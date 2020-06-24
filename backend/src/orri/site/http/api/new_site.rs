use std::time::SystemTime;
use std::str::FromStr;
use actix_web::{web, HttpResponse};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use crate::orri::app_state::{AppState, Config};
use crate::orri::site::{self, Site, CreateSiteError, FileInfo};
use crate::orri::http;
use crate::orri::domain::{self, Domain};
use crate::orri::session_data::{self, SessionData};
use crate::orri::site_key;
use crate::orri::route::Route;
use crate::orri::http as http_helper;
use data_url::{DataUrl, DataUrlError, forgiving_base64};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    domain: String,
    key: String,
    data_url: String,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    manage_url: String,
    site_url: String,
}


enum Error {
    ProcessDataUrl(DataUrlError),
    DecodeDataUrl(forgiving_base64::InvalidBase64),
    ParseDomain(domain::Error),
    SiteKey(site_key::Error),
    CreateSite(CreateSiteError),
    PersistSite(site::PersistSiteError),
    SessionData(session_data::Error),
}

pub async fn handler(state: web::Data<AppState>, session: Session, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, &session, &request_data)
        .map(|site| prepare_response(&state.config, site))
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, session: &Session, request_data: &Request) -> Result<Site, Error> {
    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomain)?;

    let url = DataUrl::process(&request_data.data_url)
        .map_err(Error::ProcessDataUrl)?;

    let (file_data, _) = url.decode_to_vec()
        .map_err(Error::DecodeDataUrl)?;

    let time = SystemTime::now();
    let mime_type = format!("{}", url.mime_type());
    let file_info = FileInfo::new(&file_data, mime_type, time);
    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site_key = site_key::from_str(&state.config.site_key, &request_data.key)
        .map_err(Error::SiteKey)?;

    let site = site::create(&state.config.site, &site_root, site_key, file_info, &file_data)
        .map_err(Error::CreateSite)?;

    site.persist(&site_root)
        .map_err(Error::PersistSite)?;

    let mut session_data = SessionData::from_session(&session)
        .unwrap_or_else(SessionData::new);

    let session_data_result = session_data.add_site(&site, &state.config.site, &request_data.key)
        .map_err(Error::SessionData);

    match session_data_result {
        Ok(()) => {
            let _ = session_data.update_session(&session);
            Ok(())
        },

        Err(err) => {
            let _ = site_root.remove();
            Err(err)
        },
    }?;


    Ok(site)
}

fn prepare_response(config: &Config, site: Site) -> HttpResponse {
    let manage_route = Route::ManageSite(site.domain.to_string());
    let site_url = config.server.sites_base_url(&site.domain.to_string());

    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .json(Response{
            manage_url: manage_route.to_string(),
            site_url,
        })
}


fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ProcessDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to parse data url")),

        Error::DecodeDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to decode base64 in data url")),

        Error::ParseDomain(err) =>
            handle_parse_domain_error(err),

        Error::SiteKey(err) =>
            handle_site_key_error(err),

        Error::CreateSite(err) =>
            handle_create_site_error(err),

        Error::PersistSite(err) =>
            handle_persist_site_error(err),

        Error::SessionData(err) =>
            handle_session_data_error(err),
    }
}

fn handle_parse_domain_error(err: domain::Error) -> HttpResponse {
    match err {
        domain::Error::TooLong() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain is too long")),

        domain::Error::SubdomainTooShort() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The subdomain is too short")),

        domain::Error::InvalidChar() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain can only contain alphanumeric characters and hyphens")),

        domain::Error::InvalidHyphenPosition() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain cannot start or end with a hyphen")),

        domain::Error::EmptyDomainValue() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain cannot be empty")),

        domain::Error::MissingSecondLevelDomain() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("A second level domain is required")),

        domain::Error::MissingSubDomain() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("A sub domain is required")),

        domain::Error::OnlyOneSubdomainAllowed() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Only one subdomain is allowed")),
    }
}


fn handle_site_key_error(err: site_key::Error) -> HttpResponse {
    match err {
        site_key::Error::TooShort() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Key is too short")),

        site_key::Error::TooLong() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The key is too long")),

        site_key::Error::HashError(err) => {
            log::error!("Failed to hash key: {:?}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to hash key"))
        },
    }
}

fn handle_create_site_error(err: CreateSiteError) -> HttpResponse {
    match err {
        CreateSiteError::SiteAlreadyExist() => {
            HttpResponse::Conflict()
                .json(http::Error::from_str("Site already exist"))
        },

        CreateSiteError::AddRoute(err) => {
            handle_failed_to_add_route(err)
        },
    }
}


fn handle_failed_to_add_route(err: site::AddRouteError) -> HttpResponse {
    match err {
        site::AddRouteError::QuotaMaxSize() => {
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Max total size reached"))
        },

        site::AddRouteError::QuotaMaxRoutes() => {
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Max routes reached"))
        },
    }
}


fn handle_session_data_error(err: session_data::Error) -> HttpResponse {
    match err {
        session_data::Error::QuotaMaxSites() => {
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Max total sites reached"))
        },

        session_data::Error::SessionDataTooLarge() => {
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The session cookie is not able to store more sites"))
        },
    }
}

fn handle_persist_site_error(err: site::PersistSiteError) -> HttpResponse {
    match err {
        site::PersistSiteError::CreateDomainDir(err) => {
            log::error!("Failed to create domain: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist site"))
        },

        site::PersistSiteError::WriteFileError(err) => {
            log::error!("Failed to write file: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist site"))
        },

        site::PersistSiteError::WriteSiteJsonError(err) => {
            log::error!("Failed to write site json: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist site"))
        },
    }
}
