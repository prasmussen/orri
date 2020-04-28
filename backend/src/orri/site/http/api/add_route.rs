use actix_web::{web, HttpRequest, HttpResponse};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use crate::orri::app_state::AppState;
use crate::orri::site::{self, Site, CreateSiteError, FileInfo, GetSiteError};
use crate::orri::http;
use crate::orri::file;
use crate::orri::util;
use crate::orri::domain::{self, Domain};
use crate::orri::site_key::{self, SiteKey};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::session_data::{SessionData};
use crate::orri::route::Route;
use data_url::{DataUrl, DataUrlError, mime, forgiving_base64};
use std::time::SystemTime;
use std::str::FromStr;


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    domain: String,
    path: String,
    data_url: String,
    key: Option<String>,
}


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    manage_url: String,
}

enum Error {
    FailedToProcessDataUrl(DataUrlError),
    FailedToDecodeDataUrl(forgiving_base64::InvalidBase64),
    ParseDomainError(domain::Error),
    ParsePathError(url_path::Error),
    NoKeyProvided(),
    VerifyKeyError(site_key::VerifyError),
    GetSiteError(GetSiteError),
    RouteAlreadyExist(),
    InvalidKey(),
    FailedToAddRoute(site::AddRouteError),
    PersistSiteError(file::WriteJsonError),
}

pub async fn handler(state: web::Data<AppState>, session: Session, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, session, &request_data)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, session: Session, request_data: &Request) -> Result<Site, Error> {
    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomainError)?;

    let path = UrlPath::from_str(&request_data.path)
        .map_err(Error::ParsePathError)?;

    let url = DataUrl::process(&request_data.data_url)
        .map_err(Error::FailedToProcessDataUrl)?;

    let (file_data, _) = url.decode_to_vec()
        .map_err(Error::FailedToDecodeDataUrl)?;

    let time = SystemTime::now();
    let mime_type = format!("{}", url.mime_type());
    let file_info = FileInfo::new(&file_data, mime_type, time);
    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let mut site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;


    util::ensure(site.routes.contains_key(&path) == false, Error::RouteAlreadyExist())?;

    let mut session_data = SessionData::from_session(&session)
        .unwrap_or(SessionData::new());

    let provided_key = get_provided_key(&request_data, &session_data, &site.domain)
        .ok_or(Error::NoKeyProvided())?;

    let has_valid_key = site.key.verify(&provided_key, &state.config.encryption_key)
        .map_err(Error::VerifyKeyError)?;

    util::ensure(has_valid_key, Error::InvalidKey())?;

    site.add_route(&state.config.site, &site_root, path, file_info, &file_data)
        .map_err(Error::FailedToAddRoute)?;

    site.persist(&site_root)
        .map_err(Error::PersistSiteError)?;

    match &request_data.key {
        Some(key) => {
            session_data.add_site(&site, &state.config.site, key);
            session_data.update_session(&session)
        },

        None =>
            ()
    }

    Ok(site)
}

fn get_provided_key(request_data: &Request, session_data: &SessionData, domain: &Domain) -> Option<String> {
    let key_from_session = session_data.get_site_key(domain);

    request_data.key.clone().or(key_from_session)
}


fn prepare_response(site: Site) -> HttpResponse {
    let manage_route = Route::ManageSite(site.domain.to_string());

    HttpResponse::Ok().json(Response{
        manage_url: manage_route.to_string(),
    })
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::FailedToProcessDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to parse data url")),

        Error::FailedToDecodeDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to decode base64 in data url")),

        Error::ParseDomainError(err) =>
            handle_parse_domain_error(err),

        Error::ParsePathError(err) =>
            handle_parse_path_error(err),

        Error::GetSiteError(err) =>
            handle_get_site_error(err),

        Error::RouteAlreadyExist() =>
            HttpResponse::Conflict()
                .json(http::Error::from_str("Route already exists")),

        Error::NoKeyProvided() =>
            HttpResponse::Unauthorized()
                .json(http::Error::from_str("No key provided")),

        Error::InvalidKey() =>
            HttpResponse::Unauthorized()
                .json(http::Error::from_str("Invalid key")),

        Error::VerifyKeyError(err) => {
            println!("Failed to verify key: {:?}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to verify key"))
        },

        Error::FailedToAddRoute(err) => {
            handle_failed_to_add_route(err)
        },

        Error::PersistSiteError(err) => {
            println!("Failed to persist site: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist site"))
        },
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

fn handle_parse_path_error(err: url_path::Error) -> HttpResponse {
    match err {
        url_path::Error::MustStartWithSlash() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The path must start with a slash")),

        url_path::Error::TooLong() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The path is too long")),

        url_path::Error::ContainsDisallowedChars() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The path contains disallowed characters")),

        url_path::Error::ContainsDoubleDot() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The path cannot contain double dots")),
    }
}

fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        GetSiteError::FailedToReadSiteJson(err) => {
            println!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
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

        site::AddRouteError::WriteFileError(err) => {
            println!("Failed to write file: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to write file"))
        },
    }
}
