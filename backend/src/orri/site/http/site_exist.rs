use actix_web::{web, HttpResponse};
use crate::orri::domain::{self, Domain};
use crate::orri::site::{self, Site, GetSiteError};
use crate::orri::http as http_helper;
use crate::orri::app_state::{AppState};
use http::header;
use std::str::FromStr;


enum Error {
    ParseDomain(domain::Error),
    GetSite(GetSiteError),
}


pub async fn handler(state: web::Data<AppState>, domain: web::Path<String>) -> HttpResponse {
    handle(&state, &domain)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {
    let domain = Domain::from_str(&domain_str)
        .map_err(Error::ParseDomain)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSite)
}


fn prepare_response(_site: Site) -> HttpResponse {
    http_helper::no_cache_headers(&mut HttpResponse::NoContent())
        .set_header(header::CONTENT_TYPE, "text/html")
        .finish()
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomain(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSite(err) => {
            handle_get_site_error(err)
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
