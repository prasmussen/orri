use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::Domain;
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use http::header;
use std::path::PathBuf;
use std::io;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::str::FromStr;


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let html = build_page(&state.config.server).to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn build_page(server_config: &ServerConfig) -> Page {
    Page{
        head: Head{
            title: format!("orri.new_site()"),
            elements: vec![
                html::script(&[attrs::src("/static/orri.js")], &[]),
            ]
        },
        body: build_body(server_config)
    }
}


fn build_body(server_config: &ServerConfig) -> Vec<Html> {
    vec![
        html::div(&[attrs::class("container"), attrs::id("content")], &[
            page::error_alert(),
            html::form(&[attrs::id("site")], &[
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column")], &[
                        html::label(&[], &[
                            html::div(&[], &[html::text("Subdomain")]),
                            html::input(&[
                                attrs::type_("text"),
                                attrs::name("subdomain"),
                                attrs::placeholder("i.e. mycoolsite"),
                                attrs::title("Please provide a valid subdomain, at least 3 characters"),
                                attrs::pattern("[a-z0-9]{4,}"),
                                attrs::required(),
                            ]),
                        ]),
                    ]),
                ]),
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
                html::input(&[
                    attrs::type_("hidden"),
                    attrs::name("mainDomain"),
                    attrs::value(&server_config.domain),
                ]),
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column column-25")], &[
                        html::button(&[ attrs::type_("submit")], &[html::text("Create site")]),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/new_site.js")], &[]),
    ]
}
