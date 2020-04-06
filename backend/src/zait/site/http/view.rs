use actix_web::{web, HttpRequest, HttpResponse};
use crate::zait::app_state::AppState;
use crate::zait::domain::{Domain, ParseDomainError};
use crate::zait::site::{self, Site, GetSiteError, FileInfo};
use http::header;
use std::path::PathBuf;
use std::io;


enum Error {
    ParseDomainError(ParseDomainError),
    GetSiteError(GetSiteError),
    RouteNotFound(),
    FailedToReadRouteData(io::Error),
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {

    handle(&req, &state)
        .map(handle_file)
        .unwrap_or_else(handle_error)
}


fn handle(req: &HttpRequest, state: &AppState) -> Result<FileInfo, Error> {
    let path = req.uri().path();
    let host = get_host_header_string(&req)
        .unwrap_or(String::new());

    let domain = Domain::from_str(&host)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    let route = site.routes.get(path)
        .ok_or(Error::RouteNotFound())?;

    site::read_route_file(&site_root, &route)
        .map_err(Error::FailedToReadRouteData)
}

fn handle_file(file: site::FileInfo) -> HttpResponse {
    HttpResponse::Ok()
        .set_header(header::ETAG, file.hash)
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(file.data)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
        },

        Error::RouteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        Error::FailedToReadRouteData(err) => {
            println!("Failed to read route data: {}", err);
            HttpResponse::NotFound().finish()
        },
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

enum HostHeaderError {
    NotFound(),
    ToStrError(header::ToStrError),
}

fn get_host_header_string(req: &HttpRequest) -> Result<String, HostHeaderError> {
    let value = req
        .headers()
        .get("host")
        .map_or(Err(HostHeaderError::NotFound()), Ok)?;

    value
        .to_str()
        .map(|s| s.to_string())
        .map_err(HostHeaderError::ToStrError)
}
