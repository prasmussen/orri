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
        html::div(&[attrs::id("main-content"), attrs::class("container")], &[
            page::error_alert(),
            html::form(&[attrs::id("form")], &[
                html::div(&[attrs::class("row")], &[
                    html::div(&[attrs::class("column")], &[
                        html::label(&[], &[
                            html::div(&[], &[html::text("Subdomain")]),
                            html::input(&[
                                attrs::type_("text"),
                                attrs::name("subdomain"),
                                attrs::placeholder("i.e. mycoolsite"),
                                attrs::title("Please provide a valid subdomain, at least 3 characters"),
                                attrs::pattern("[a-z0-9-]{4,}"),
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
                        html::button(&[attrs::type_("submit"), attrs::id("submit-button")], &[html::text("Create site")]),
                    ]),
                ]),
            ]),
        ]),
        build_success_content(),
        html::script(&[attrs::src("/static/new_site.js")], &[]),
    ]
}

fn build_success_content() -> Html {
    html::div(&[attrs::id("success-content"), attrs::class("container display-none")], &[
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column")], &[
                html::h3(&[attrs::class("margin-top-40 margin-bottom-10")], &[
                    html::text("Congrats!"),
                ]),
                html::div(&[], &[
                    html::text("Your site "),
                    html::strong(&[], &[
                        html::em(&[attrs::id("domain-placeholder")], &[]),
                    ]),
                    html::text(" is ready."),
                ]),
            ]),
        ]),
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column")], &[
                html::h3(&[attrs::class("margin-top-40 margin-bottom-10")], &[
                    html::text("Important!"),
                ]),
                html::div(&[], &[
                    html::text("This is your site key: "),
                    html::strong(&[], &[
                        html::em(&[attrs::id("key-placeholder")], &[]),
                    ]),
                ]),
                html::div(&[], &[
                    html::text("The key is needed to manage your site, so please save it. It's "),
                    html::em(&[], &[html::text("not")]),
                    html::text(" possible to recover the key later."),
                ]),
            ]),
        ]),
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column")], &[
                html::h3(&[attrs::class("margin-top-40 margin-bottom-10")], &[
                    html::text("What's next?"),
                ]),
                html::ul(&[], &[
                    html::li(&[], &[
                        html::a(&[attrs::href("#"), attrs::target("_blank"), attrs::id("site-url-placeholder")], &[
                            html::text("Go to site"),
                        ]),
                    ]),
                    html::li(&[], &[
                        html::a(&[attrs::href("#"), attrs::target("_blank"), attrs::id("manage-url-placeholder")], &[
                            html::text("Manage site"),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ])
}
