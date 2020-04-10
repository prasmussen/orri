use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::orri::app_state::AppState;
use crate::orri::site::{self, Site, CreateSiteError, FileInfo};
use crate::orri::http;
use crate::orri::domain::{Domain, ParseDomainError};
use data_url::{DataUrl, DataUrlError, mime, forgiving_base64};


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    domain: String,
    key: String,
    data_url: String,
}


#[derive(Serialize)]
pub struct Response {
    key: String,
}

enum Error {
    FailedToProcessDataUrl(DataUrlError),
    FailedToDecodeDataUrl(forgiving_base64::InvalidBase64),
    ParseDomainError(ParseDomainError),
    CreateSiteError(CreateSiteError),
}

pub async fn handler(state: web::Data<AppState>, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, &request_data)
        .map(handle_site)
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, request_data: &Request) -> Result<Site, Error> {
    // TODO: check minimum subdomain length
    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomainError)?;

    let url = DataUrl::process(&request_data.data_url)
        .map_err(Error::FailedToProcessDataUrl)?;

    let (file_data, _) = url.decode_to_vec()
        .map_err(Error::FailedToDecodeDataUrl)?;

    let mime_type = format!("{}", url.mime_type());
    let file_info = FileInfo::new(&file_data, mime_type);
    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::create(site_root, &request_data.key, file_info, &file_data)
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
        Error::FailedToProcessDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to parse data url")),

        Error::FailedToDecodeDataUrl(_) =>
            HttpResponse::BadRequest()
                .json(http::Error::from_str("Failed to decode base64 in data url")),

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