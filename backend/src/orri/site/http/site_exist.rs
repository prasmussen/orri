use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::{self, Domain};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{Page, Head};
use crate::orri::route::Route;
use crate::orri::util;
use crate::orri::page;
use http::header;
use std::path::PathBuf;
use std::io;
use std::str::FromStr;
use std::time::SystemTime;


enum Error {
    ParseDomainError(domain::Error),
    GetSiteError(GetSiteError),
}


pub async fn handler(state: web::Data<AppState>, domain: web::Path<String>) -> HttpResponse {
    let base_url = &state.config.server.sites_base_url(&domain);

    handle(&state, &domain)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {
    let domain = Domain::from_str(&domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSiteError)
}


fn prepare_response(site: Site) -> HttpResponse {
    HttpResponse::NoContent()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .finish()
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
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
