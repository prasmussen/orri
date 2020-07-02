use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::domain::{self, Domain};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::site::{self, GetSiteError, File};
use crate::orri::http as http_helper;
use crate::orri::http::{Host};
use actix_http::http::{header};
use std::io;
use std::str::FromStr;


enum Error {
    ParseDomain(domain::Error),
    ParsePath(url_path::Error),
    GetSite(GetSiteError),
    RouteNotFound(),
    ReadRouteData(io::Error),
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {

    handle(&req, &state)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}


fn handle(req: &HttpRequest, state: &AppState) -> Result<File, Error> {
    let extensions = req.extensions();
    let maybe_host: Option<&Host> = extensions.get();
    let host_str = maybe_host
        .map(|host| host.0.to_str().unwrap_or_default())
        .unwrap_or("");

    let domain = Domain::from_str(host_str)
        .map_err(Error::ParseDomain)?;


    let path = UrlPath::from_str(req.uri().path())
        .map_err(Error::ParsePath)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSite)?;

    let route = site.routes.get(&path)
        .ok_or(Error::RouteNotFound())?;

    site::read_route_file(&site_root, &route)
        .map_err(Error::ReadRouteData)
}

fn prepare_response(file: site::File) -> HttpResponse {
    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .set_header(header::CONTENT_TYPE, file.metadata.mime)
        .body(file.data)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomain(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::ParsePath(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSite(err) => {
            handle_get_site_error(err)
        },

        Error::RouteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        Error::ReadRouteData(err) => {
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

        GetSiteError::ReadSiteJson(err) => {
            log::error!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}
