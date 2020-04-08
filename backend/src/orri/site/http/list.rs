use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::domain::{Domain, ParseDomainError};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml as html;
use crate::orri::slowhtml::Html;
use crate::orri::slowhtml::attributes as attrs;
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


fn handle(state: &AppState, domain: &str) -> Result<Site, Error> {
    let domain = Domain::from_str(&domain)
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
    let html = page_html(site);
    let html_string = &html.to_string();
    println!("{}", now.elapsed().as_micros());
    html_string.to_string()
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

fn page_html(site: &Site) -> Html {
    let head_elements = &[
        vec![
            html::node("title", &[], &[html::text("Orri")])
        ],
        milligram_styles(),
    ].concat();

    html::node("html",
        &[attrs::attribute("lang", "en")],
        &[
            html::node("head", &[], head_elements),
            html::node("body", &[], &[
                table(site)
            ]),
        ]
    )
}

fn milligram_styles() -> Vec<Html> {
    vec![
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://fonts.googleapis.com/css?family=Roboto:300,300italic,700,700italic"),
        ]),
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/normalize/5.0.0/normalize.css"),
        ]),
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/milligram/1.3.0/milligram.css"),
        ]),
    ]
}

fn table(site: &Site) -> Html {
    let rows = site.routes
        .iter()
        .map(|(route, route_info)| table_row(site, route, route_info))
        .collect::<Vec<Html>>();

    html::node("table", &[], &[
        html::node("thead", &[], &[
            html::node("tr", &[], &[
                html::node("th", &[], &[html::text("Route")]),
                html::node("th", &[], &[html::text("Mime")]),
                html::node("th", &[], &[html::text("Size")]),
                html::node("th", &[], &[html::text("Edit")]),
                html::node("th", &[], &[html::text("Delete")]),
            ]),
        ]),
        html::node("tbody", &[], &rows),
    ])
}

fn table_row(site: &Site, route: &str, route_info: &RouteInfo) -> Html {
    let edit_url = format!("/sites/{}/edit{}", site.domain, route);

    html::node("tr", &[], &[
        html::node("td", &[], &[html::text(route)]),
        html::node("td", &[], &[html::text(&route_info.file_info.mime)]),
        html::node("td", &[], &[html::text(&route_info.file_info.size.to_string())]),
        html::node("td", &[], &[
            html::node("a", &[attrs::attribute("href", &edit_url)], &[html::text("Edit")]),
        ]),
        html::node("td", &[], &[
            html::node("a", &[attrs::attribute("href", "#")], &[html::text("Delete")]),
        ]),
    ])
}
