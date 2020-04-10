use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::domain::{Domain, ParseDomainError};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml as html;
use crate::orri::slowhtml::Html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{Page, Head};
use http::header;
use std::path::PathBuf;
use std::io;
use serde::Deserialize;
use std::time::{Duration, Instant};


enum Error {
    ParseDomainError(ParseDomainError),
    GetSiteError(GetSiteError),
}

pub async fn handler(state: web::Data<AppState>, domain: web::Path<String>) -> HttpResponse {
    handle(&state, &domain)
        .map(handle_site)
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {

    let domain = Domain::from_str(domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    // TODO: check if route exist
    site::get(&site_root)
        .map_err(Error::GetSiteError)
}


fn handle_site(site: Site) -> HttpResponse {
    let html = render(&site);

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn render(site: &Site) -> String {
    let now = Instant::now();
    let page = build_page(site);
    let html_string = page.to_html().to_string();
    println!("{}", now.elapsed().as_micros());
    html_string
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

fn build_page(site: &Site) -> Page {
    Page{
        head: Head{
            title: format!("orri.add_route(\"{}\")", &site.domain),
            elements: vec![
                html::node("script", &[attrs::attribute("src", "/static/orri.js")], &[]),
            ]
        },
        body: build_body(site)
    }
}


fn build_body(site: &Site) -> Vec<Html> {
    vec![
        html::node("div", &[attrs::attribute("class", "container")], &[
            html::node("form", &[attrs::attribute("id", "site")], &[
                html::node("div", &[attrs::attribute("class", "row")], &[
                    html::node("div", &[attrs::attribute("class", "column")], &[
                        html::node("label", &[], &[
                            html::node("div", &[], &[html::text("Domain")]),
                            html::node_no_end("input", &[
                                attrs::attribute("type", "text"),
                                attrs::attribute("name", "domain"),
                                attrs::attribute("value", &site.domain.to_string()),
                                attrs::bool_attribute("readonly"),
                            ]),
                        ]),
                    ]),
                ]),
                html::node("div", &[attrs::attribute("class", "row")], &[
                    html::node("div", &[attrs::attribute("class", "column")], &[
                        html::node("label", &[], &[
                            html::node("div", &[], &[html::text("Route")]),
                            html::node_no_end("input", &[
                                attrs::attribute("type", "text"),
                                attrs::attribute("name", "path"),
                                attrs::attribute("placeholder", "i.e. /some-route or /app.js"),
                            ]),
                        ]),
                    ]),
                ]),
                html::node("div", &[attrs::attribute("class", "row")], &[
                    html::node("div", &[attrs::attribute("class", "column")], &[
                        html::node("label", &[], &[
                            html::node("div", &[], &[html::text("File")]),
                            html::node_no_end("input", &[
                                attrs::attribute("type", "file"),
                                attrs::attribute("id", "file"),
                            ]),
                        ]),
                    ]),
                ]),
                html::node("div", &[attrs::attribute("class", "row")], &[
                    html::node("div", &[attrs::attribute("class", "column column-25")], &[
                        html::node("button", &[ attrs::attribute("type", "submit")], &[html::text("Save route")]),
                    ]),
                ]),
            ]),
        ]),
        html::node("script", &[attrs::attribute("src", "/static/add_route.js")], &[]),
    ]
}
