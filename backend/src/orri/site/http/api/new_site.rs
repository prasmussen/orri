use std::time::SystemTime;
use std::str::FromStr;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_session::Session;
use serde::{Deserialize, Serialize};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::site::{self, Site, CreateSiteError, FileInfo};
use crate::orri::http;
use crate::orri::domain::{self, Domain};
use crate::orri::site_key::{self, SiteKey};
use data_url::{DataUrl, DataUrlError, mime, forgiving_base64};


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
    FailedToProcessDataUrl(DataUrlError),
    FailedToDecodeDataUrl(forgiving_base64::InvalidBase64),
    ParseDomainError(domain::Error),
    SiteKeyError(site_key::Error),
    CreateSiteError(CreateSiteError),
}

pub async fn handler(state: web::Data<AppState>, session: Session, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, &request_data)
        .map(|site| handle_site(session, &state.config.server, &request_data, site))
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, request_data: &Request) -> Result<Site, Error> {
    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomainError)?;

    let url = DataUrl::process(&request_data.data_url)
        .map_err(Error::FailedToProcessDataUrl)?;

    let (file_data, _) = url.decode_to_vec()
        .map_err(Error::FailedToDecodeDataUrl)?;

    let time = SystemTime::now();
    let mime_type = format!("{}", url.mime_type());
    let file_info = FileInfo::new(&file_data, mime_type, time);
    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site_key = site_key::from_str(&request_data.key, &state.config.encryption_key)
        .map_err(Error::SiteKeyError)?;

    site::create(site_root, site_key, file_info, &file_data)
        .map_err(Error::CreateSiteError)
}

fn handle_site(session: Session, server_config: &ServerConfig, request_data: &Request, site: Site) -> HttpResponse {
    let manage_url = format!("/sites/{}", &site.domain);
    let site_url = server_config.other_base_url(&site.domain.to_string());

    session.set(&site.domain.to_string(), &request_data.key);

    HttpResponse::Ok()
        .json(Response{
            manage_url: manage_url,
            site_url: site_url,
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

        Error::SiteKeyError(err) =>
            handle_site_key_error(err),

        Error::CreateSiteError(err) =>
            handle_create_site_error(err),
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
            println!("Failed to hash key: {:?}", err);
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
