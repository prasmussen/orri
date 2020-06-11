use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::Domain;
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
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
            title: format!("New site - orri"),
            elements: vec![],
        },
        body: build_body(server_config)
    }
}


fn build_body(server_config: &ServerConfig) -> Vec<Html> {
    let new_site_route = Route::NewSiteJson();

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("New site", Route::NewSite()),
            ]),
        ),
        html::div(&[attrs::id("main-content"), attrs::class("container")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("column col-6 col-mx-auto")], &[
                    page::error_alert(),
                    html::form(
                        &[
                            attrs::id("form"),
                            attrs::attribute_trusted_name("data-api-method", &new_site_route.request_method().to_string()),
                            attrs::attribute_trusted_name("data-api-url", &new_site_route.to_string())
                        ], &[
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("Domain")]),
                                html::input(&[
                                    attrs::class("form-input"),
                                    attrs::type_("text"),
                                    attrs::name("sitesDomain"),
                                    attrs::value(&server_config.sites_domain),
                                    attrs::readonly(),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("Subdomain")]),
                                html::input(&[
                                    attrs::class("form-input"),
                                    attrs::type_("text"),
                                    attrs::name("subdomain"),
                                    attrs::placeholder("i.e. my-cool-site"),
                                    attrs::title("Please provide a valid subdomain, at least 5 characters"),
                                    attrs::pattern("[a-z0-9-]{5,}"),
                                    attrs::required(),
                                ]),
                                html::p(&[attrs::class("form-input-hint")], &[
                                    html::text(&format!("Minimum 5 characters. The full domain of your site will be <subdomain>.{}", &server_config.sites_domain)),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("File")]),
                                html::input(&[
                                    attrs::class("form-input"),
                                    attrs::type_("file"),
                                    attrs::id("file"),
                                    attrs::required(),
                                ]),
                                html::p(&[attrs::class("form-input-hint")], &[
                                    html::text("The selected file will be the front page of your site."),
                                    html::text(" "),
                                    html::a(
                                        &[attrs::href("https://glot.io/snippets/fo1aqwk3ec/raw/index.html"), attrs::target("_blank")],
                                        &[html::text("See this link for a minimal starting point.")]
                                    ),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group margin-top-20")], &[
                            html::button(
                                &[
                                    attrs::type_("submit"),
                                    attrs::id("submit-button"),
                                    attrs::class("btn btn-primary btn-lg")
                                ],
                                &[html::text("Publish site")]
                            ),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        build_success_content(),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/new_site.js")], &[]),
    ]
}

fn build_success_content() -> Html {
    html::div(&[attrs::id("success-content"), attrs::class("container display-none")], &[
        html::div(&[attrs::class("columns")], &[
            html::div(&[attrs::class("column col-6 col-mx-auto")], &[
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
        html::div(&[attrs::class("columns")], &[
            html::div(&[attrs::class("column col-6 col-mx-auto")], &[
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
        html::div(&[attrs::class("columns")], &[
            html::div(&[attrs::class("column col-6 col-mx-auto")], &[
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
