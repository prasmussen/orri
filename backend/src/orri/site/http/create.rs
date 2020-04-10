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
                html::node("script", &[attrs::attribute("src", "/static/orri.js")], &[]),
            ]
        },
        body: build_body()
    }
}


fn build_body() -> Vec<Html> {
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
                                attrs::attribute("placeholder", "i.e. name.orri.dev"),
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
                        html::node("button", &[ attrs::attribute("type", "submit")], &[html::text("Create site")]),
                    ]),
                ]),
            ]),
        ]),
        html::node("script", &[attrs::attribute("src", "/static/new_site.js")], &[]),
    ]
}
