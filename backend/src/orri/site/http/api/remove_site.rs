use actix_web::{web, HttpResponse};
use actix_session::Session;
use serde::{Deserialize};
use crate::orri::app_state::AppState;
use crate::orri::site::{self, Site, GetSiteError};
use crate::orri::http;
use crate::orri::util;
use crate::orri::domain::{self, Domain};
use crate::orri::site_key;
use crate::orri::session_data::{SessionData};
use crate::orri::http as http_helper;
use std::str::FromStr;
use std::io;


#[derive(Deserialize)]
pub struct Request {
    domain: String,
    key: Option<String>,
}


enum Error {
    ParseDomainError(domain::Error),
    NoKeyProvided(),
    VerifyKeyError(site_key::VerifyError),
    GetSiteError(GetSiteError),
    InvalidKey(),
    RemoveSiteError(io::Error),
}

pub async fn handler(state: web::Data<AppState>, session: Session, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, session, &request_data)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, session: Session, request_data: &Request) -> Result<Site, Error> {
    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    let mut session_data = SessionData::from_session(&session)
        .unwrap_or_else(SessionData::new);

    let provided_key = get_provided_key(&request_data, &session_data, &site.domain)
        .ok_or(Error::NoKeyProvided())?;

    let has_valid_key = site.key.verify(&provided_key, &state.config.encryption_key)
        .map_err(Error::VerifyKeyError)?;

    util::ensure(has_valid_key, Error::InvalidKey())?;

    site_root.remove()
        .map_err(Error::RemoveSiteError)?;

    session_data.remove_site(&site.domain);
    let _ = session_data.update_session(&session);

    Ok(site)
}

fn get_provided_key(request_data: &Request, session_data: &SessionData, domain: &Domain) -> Option<String> {
    let key_from_session = session_data.get_site_key(domain);

    request_data.key.clone().or(key_from_session)
}


fn prepare_response(_site: Site) -> HttpResponse {
    http_helper::no_cache_headers(&mut HttpResponse::NoContent())
        .finish()
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) =>
            handle_parse_domain_error(err),

        Error::GetSiteError(err) =>
            handle_get_site_error(err),

        Error::NoKeyProvided() =>
            HttpResponse::Unauthorized()
                .json(http::Error::from_str("No key provided")),

        Error::InvalidKey() =>
            HttpResponse::Unauthorized()
                .json(http::Error::from_str("Invalid key")),

        Error::VerifyKeyError(err) => {
            log::error!("Failed to verify key: {:?}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to verify key"))
        },

        Error::RemoveSiteError(err) => {
            log::error!("Failed to remove site: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to remove site"))
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

fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        GetSiteError::FailedToReadSiteJson(err) => {
            log::error!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}
