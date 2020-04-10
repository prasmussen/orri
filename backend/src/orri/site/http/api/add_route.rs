use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::orri::app_state::AppState;
use crate::orri::site::{self, Site, CreateSiteError, FileInfo, GetSiteError};
use crate::orri::http;
use crate::orri::file;
use crate::orri::domain::{Domain, ParseDomainError};
use data_url::{DataUrl, DataUrlError, mime, forgiving_base64};
use std::time::SystemTime;


#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    domain: String,
    path: String,
    data_url: String,
}


//#[derive(Serialize)]
//pub struct Response {
//    key: String,
//}

enum Error {
    FailedToProcessDataUrl(DataUrlError),
    FailedToDecodeDataUrl(forgiving_base64::InvalidBase64),
    ParseDomainError(ParseDomainError),
    GetSiteError(GetSiteError),
    PersistFileError(file::WriteError),
    PersistSiteError(file::WriteJsonError),
}

pub async fn handler(state: web::Data<AppState>, request_data: web::Json<Request>) -> HttpResponse {

    handle(&state, &request_data)
        .map(handle_site)
        .unwrap_or_else(handle_error)
}

// TODO: validate path (add newtype?)
// TODO: check minimum subdomain length
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

    // TODO: check if route exist
    let mut site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    site.add_route(&site_root, &request_data.path, file_info, &file_data)
        .map_err(Error::PersistFileError);

    site.persist(&site_root)
        .map_err(Error::PersistSiteError);

    Ok(site)
}

fn handle_site(site: Site) -> HttpResponse {
    HttpResponse::Ok().finish()
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

        Error::GetSiteError(err) =>
            handle_get_site_error(err),

        Error::PersistFileError(err) => {
            println!("Failed to persist file: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist file"))
        },

        Error::PersistSiteError(err) => {
            println!("Failed to persist site: {}", err);
            HttpResponse::InternalServerError()
                .json(http::Error::from_str("Failed to persist site"))
        },
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
