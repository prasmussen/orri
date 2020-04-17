use actix_web::{web, HttpRequest, HttpResponse};
use actix_session::Session;
use crate::orri::app_state::AppState;
use crate::orri::domain::{self, Domain};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{Page, Head};
use http::header;
use std::path::PathBuf;
use std::io;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::str::FromStr;


enum Error {
    ParseDomainError(domain::Error),
    GetSiteError(GetSiteError),
}

pub async fn handler(state: web::Data<AppState>, session: Session, domain: web::Path<String>) -> HttpResponse {
    let site_key: Option<String> = session.get(&domain)
        .unwrap_or(None);

    handle(&state, &domain)
        .map(|site| handle_site(site, site_key))
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {

    let domain = Domain::from_str(domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSiteError)
}


fn handle_site(site: Site, site_key: Option<String>) -> HttpResponse {

    let client_provided_key = site_key == Some(site.key.clone());
    let html = render(&site, client_provided_key);

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn render(site: &Site, client_provided_key: bool) -> String {
    let now = Instant::now();
    let page = build_page(site, client_provided_key);
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

fn build_page(site: &Site, client_provided_key: bool) -> Page {
    Page{
        head: Head{
            title: format!("orri.add_route(\"{}\")", &site.domain),
            elements: vec![
                html::script(&[attrs::src("/static/orri.js")], &[]),
            ]
        },
        body: build_body(site, client_provided_key),
    }
}


fn build_body(site: &Site, client_provided_key: bool) -> Vec<Html> {
    vec![
        html::div(&[attrs::class("container")], &[
            html::form(&[attrs::id("site")], &[
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column")], &[
                        html::label(&[], &[
                            html::div(&[], &[html::text("Domain")]),
                            html::input(&[
                                attrs::type_("text"),
                                attrs::name("domain"),
                                attrs::value(&site.domain.to_string()),
                                attrs::readonly(),
                            ]),
                        ]),
                    ]),
                ]),
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column")], &[
                        html::label(&[], &[
                            html::div(&[], &[html::text("Route")]),
                            html::input(&[
                                attrs::type_("text"),
                                attrs::name("path"),
                                attrs::placeholder("i.e. /some-page or /some-styles.css"),
                                attrs::title("The path to the file, it must start with a slash"),
                                attrs::pattern("/.+"),
                                attrs::required(),
                            ]),
                        ]),
                    ]),
                ]),
                html::conditional(client_provided_key == false,
                    html::div(&[attrs::class("row")], &[
                        html::div(&[attrs::class("column")], &[
                            html::label(&[], &[
                                html::div(&[], &[html::text("Site key")]),
                                html::input(&[
                                    attrs::type_("password"),
                                    attrs::name("key"),
                                    attrs::required(),
                                ]),
                            ]),
                        ]),
                    ])
                ),
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column")], &[
                        html::label(&[], &[
                            html::div(&[], &[html::text("File")]),
                            html::input(&[
                                attrs::type_("file"),
                                attrs::id("file"),
                                attrs::required(),
                            ]),
                        ]),
                    ]),
                ]),
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column column-25")], &[
                        html::button(&[attrs::type_("submit")], &[html::text("Save route")]),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/add_route.js")], &[]),
    ]
}
