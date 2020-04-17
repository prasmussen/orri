use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::domain::Domain;
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


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let html = build_page().to_html().to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn build_page() -> Page {
    Page{
        head: Head{
            title: format!("orri.new_site()"),
            elements: vec![
                html::script(&[attrs::src("/static/orri.js")], &[]),
            ]
        },
        body: build_body()
    }
}


fn build_body() -> Vec<Html> {
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
                                attrs::placeholder("i.e. name.orri.dev"),
                                attrs::title("Please provide a valid domain name, the subdomain must be at least 3 characters"),
                                attrs::pattern("[a-z-]{3,}[.][a-z]+[.][a-z]+"),
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
