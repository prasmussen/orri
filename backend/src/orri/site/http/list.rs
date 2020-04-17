use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::{self, Domain};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{Page, Head};
use http::header;
use std::path::PathBuf;
use std::io;
use std::time::{Duration, Instant};
use std::str::FromStr;


enum Error {
    ParseDomainError(domain::Error),
    GetSiteError(GetSiteError),
}


pub async fn handler(state: web::Data<AppState>, domain: web::Path<String>) -> HttpResponse {
    let base_url = &state.config.server.other_base_url(&domain);

    handle(&state, &domain)
        .map(|site| handle_site(site, base_url))
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {
    let domain = Domain::from_str(&domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSiteError)
}


fn handle_site(site: Site, base_url: &str) -> HttpResponse {
    let html = render(&site, base_url);

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn render(site: &Site, base_url: &str) -> String {
    let now = Instant::now();
    let page = build_page(site, base_url);
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

fn build_page(site: &Site, base_url: &str) -> Page {
    Page{
        head: Head{
            title: format!("orri.list_routes(\"{}\")", &site.domain),
            elements: vec![
                html::script(&[attrs::src("/static/orri.js")], &[]),
            ]
        },
        body: build_body(site, base_url)
    }
}

fn build_body(site: &Site, base_url: &str) -> Vec<Html> {
    let new_route_url = format!("/sites/{}/add-route", site.domain);

    let rows = site.routes
        .iter()
        .map(|(route, route_info)| table_row(site, route, route_info, base_url))
        .collect::<Vec<Html>>();

    vec![
        html::div(&[attrs::class("container")], &[
            html::table(&[], &[
                html::thead(&[], &[
                    html::tr(&[], &[
                        html::th(&[], &[html::text("Route")]),
                        html::th(&[], &[html::text("Mime")]),
                        html::th(&[], &[html::text("Size")]),
                        html::th(&[], &[html::text("Delete")]),
                    ]),
                ]),
                html::tbody(&[], &rows),
            ]),
            html::a(
                &[
                    attrs::href(&new_route_url),
                    attrs::class("button"),
                ],
                &[html::text("Add route")]
            ),
        ]),
    ]
}

fn table_row(site: &Site, route: &UrlPath, route_info: &RouteInfo, base_url: &str) -> Html {
    let route_url = format!("{}{}", base_url, route);

    html::tr(&[], &[
        html::td(&[], &[
            html::a(&[attrs::href(&route_url)], &[html::text(&route.to_string())]),
        ]),
        html::td(&[], &[html::text(&route_info.file_info.mime)]),
        html::td(&[], &[html::text(&route_info.file_info.size.to_string())]),
        html::td(&[], &[
            html::a(&[attrs::href("#")], &[html::text("Delete")]),
        ]),
    ])
}
