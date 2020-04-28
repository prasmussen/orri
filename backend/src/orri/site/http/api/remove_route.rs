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
use std::io;


#[derive(Deserialize)]
pub struct Request {
    domain: String,
    path: String,
    key: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    manage_url: String,
}

enum Error {
    ParseDomainError(domain::Error),
    ParsePathError(url_path::Error),
    NoKeyProvided(),
    VerifyKeyError(site_key::VerifyError),
    GetSiteError(GetSiteError),
    InvalidKey(),
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

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let mut site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    let mut session_data = SessionData::from_session(&session)
        .unwrap_or(SessionData::new());

    let provided_key = get_provided_key(&request_data, &session_data, &site.domain)
        .ok_or(Error::NoKeyProvided())?;

    let has_valid_key = site.key.verify(&provided_key, &state.config.encryption_key)
        .map_err(Error::VerifyKeyError)?;

    util::ensure(has_valid_key, Error::InvalidKey())?;

    site.remove_route(path)
        .persist(&site_root)
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
        Error::ParseDomainError(err) =>
            handle_parse_domain_error(err),

        Error::ParsePathError(err) =>
            handle_parse_path_error(err),

        Error::GetSiteError(err) =>
            handle_get_site_error(err),

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
