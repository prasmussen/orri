use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::zait::app_state::AppState;
use crate::zait::site::{self, Site, CreateSiteError};
use crate::zait::http;
use crate::zait::domain::{Domain, ParseDomainError};


#[derive(Deserialize)]
pub struct Request {
    domain: String,
    data: String,
}


#[derive(Serialize)]
pub struct Response {
    key: String,
}

enum Error {
    ParseDomainError(ParseDomainError),
    CreateSiteError(CreateSiteError),
}

pub async fn handler(state: web::Data<AppState>, payload: web::Json<Request>) -> HttpResponse {

    handle(&state, &payload)
        .map(handle_site)
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, payload: &Request) -> Result<Site, Error> {
    // TODO: check minimum subdomain length
    let domain = Domain::from_str(&payload.domain)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::create(site_root, &payload.data)
        .map_err(Error::CreateSiteError)
}

fn handle_site(site: Site) -> HttpResponse {
    HttpResponse::Ok()
        .json(Response{
            key: site.key,
        })
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) =>
            handle_parse_domain_error(err),

        Error::CreateSiteError(err) =>
            handle_create_site_error(err),
    }
}

fn handle_parse_domain_error(err: ParseDomainError) -> HttpResponse {
    match err {
        ParseDomainError::NotAlphabetic() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain can only contain characters in the range a-z")),

        ParseDomainError::EmptyDomainValue() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("The domain cannot be empty")),

        ParseDomainError::MissingSecondLevelDomain() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("A second level domain is required")),

        ParseDomainError::MissingSubDomain() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("A sub domain is required")),

        ParseDomainError::OnlyOneSubdomainAllowed() =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Only one subdomain is allowed")),
    }
}

fn handle_create_site_error(err: CreateSiteError) -> HttpResponse {
    match err {
        CreateSiteError::SiteAlreadyExist() => {
            HttpResponse::Conflict()
                .json(http::Error::from_str("Site already exist"))
        },

        CreateSiteError::FailedToCreateDomainDir(err) => {
            println!("Failed to create domain: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to create domain dir"))
        },

        CreateSiteError::FailedToWriteFile(err) => {
            println!("Failed to write file: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to write file"))
        }

        CreateSiteError::FailedToSaveSiteJson(err) => {
            println!("Failed to save config: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to save config"))
        }
    }
}
