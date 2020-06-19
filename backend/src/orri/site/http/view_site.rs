use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::domain::{self, Domain};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::site::{self, GetSiteError, File};
use crate::orri::http as http_helper;
use http::header;
use std::io;
use std::str::FromStr;


enum Error {
    ParseDomainError(domain::Error),
    ParsePathError(url_path::Error),
    GetSiteError(GetSiteError),
    RouteNotFound(),
    FailedToReadRouteData(io::Error),
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {

    handle(&req, &state)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}


fn handle(req: &HttpRequest, state: &AppState) -> Result<File, Error> {
    let host = get_host_header_string(&req)
        .unwrap_or_default();

    let domain = Domain::from_str(&host)
        .map_err(Error::ParseDomainError)?;

    let path = UrlPath::from_str(req.uri().path())
        .map_err(Error::ParsePathError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    let route = site.routes.get(&path)
        .ok_or(Error::RouteNotFound())?;

    site::read_route_file(&site_root, &route)
        .map_err(Error::FailedToReadRouteData)
}

fn prepare_response(file: site::File) -> HttpResponse {
    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        //.set_header(header::ETAG, file.metadata.hash)
        .set_header(header::CONTENT_TYPE, file.metadata.mime)
        .body(file.data)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::ParsePathError(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
        },

        Error::RouteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        Error::FailedToReadRouteData(err) => {
            log::error!("Failed to read route data: {}", err);
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
            log::error!("Failed to read site json: {}", err);
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

    let host = value
        .to_str()
        .map(|s| s.to_string())
        .map_err(HostHeaderError::ToStrError)?;

    let parts = host.split(':')
        .collect::<Vec<&str>>();

    if parts.is_empty() {
        Ok(host)
    } else {
        Ok(parts[0].to_string())
    }

}
