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
    let domain = Domain::from_str(&domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

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
            title: format!("orri.list_routes(\"{}\")", &site.domain),
            elements: vec![
                html::node("script", &[attrs::attribute("src", "/static/orri.js")], &[]),
            ]
        },
        body: build_body(site)
    }
}

fn build_body(site: &Site) -> Vec<Html> {
    let new_route_url = format!("/sites/{}/add-route", site.domain);

    let rows = site.routes
        .iter()
        .map(|(route, route_info)| table_row(site, route, route_info))
        .collect::<Vec<Html>>();

    vec![
        html::node("div", &[attrs::attribute("class", "container")], &[
            html::node("table", &[], &[
                html::node("thead", &[], &[
                    html::node("tr", &[], &[
                        html::node("th", &[], &[html::text("Route")]),
                        html::node("th", &[], &[html::text("Mime")]),
                        html::node("th", &[], &[html::text("Size")]),
                        html::node("th", &[], &[html::text("Delete")]),
                    ]),
                ]),
                html::node("tbody", &[], &rows),
            ]),
            html::node("a", &[
                attrs::attribute("href", &new_route_url),
                attrs::attribute("class", "button"),
            ],
            &[html::text("Add route")]),
        ]),
    ]
}

fn table_row(site: &Site, route: &str, route_info: &RouteInfo) -> Html {
    // TODO: get protocol and port from ServerConfig
    let route_url = format!("https://{}{}", site.domain, route);

    html::node("tr", &[], &[
        html::node("td", &[], &[
            html::node("a", &[attrs::attribute("href", &route_url)], &[html::text(route)]),
        ]),
        html::node("td", &[], &[html::text(&route_info.file_info.mime)]),
        html::node("td", &[], &[html::text(&route_info.file_info.size.to_string())]),
        html::node("td", &[], &[
            html::node("a", &[attrs::attribute("href", "#")], &[html::text("Delete")]),
        ]),
    ])
}
