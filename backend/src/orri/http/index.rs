use actix_web::{web, HttpResponse};
use http::header;
use crate::orri::app_state::AppState;
use crate::orri::page::{self, Page, Head};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::route::Route;
use crate::orri::site;


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let html = build_page(&state.config.site).to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn build_page(site_config: &site::Config) -> Page {
    Page{
        head: Head{
            title: format!("Home - orri"),
            elements: vec![]
        },
        body: build_body(site_config)
    }
}


fn build_body(site_config: &site::Config) -> Vec<Html> {
    let new_site_route = Route::NewSite();
    let my_sites_route = Route::MySites();
    let max_routes = site_config.quota_nano.max_routes;
    let max_size_megabyte = site_config.quota_nano.max_size / 1000 / 1000;

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
            ]),
        ),
        html::div(&[attrs::class("container max-width-976 text-center")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("hero hero-lg p-centered")], &[
                    html::div(&[attrs::class("hero-body")], &[
                        html::div(&[attrs::class("column col-12")], &[
                            html::h1(&[], &[html::text("Get started")]),
                            html::p(&[], &[html::text("Publish your website in seconds, no account required!")]),
                            html::a(&[attrs::class("btn btn-lg btn-primary"), attrs::href(&new_site_route.to_string())], &[html::text("New site")]),
                            html::a(&[attrs::class("btn btn-lg"), attrs::href(&my_sites_route.to_string())], &[html::text("Manage site")]),
                        ]),
                    ]),
                ]),
            ]),
            html::div(&[attrs::class("columns margin-top-40")], &[
                html::div(&[attrs::class("column col-12")], &[
                    html::h3(&[], &[html::text("What you get")]),
                ]),
            ]),
            html::div(&[attrs::class("columns features")], &[
                html::div(&[attrs::class("column col-4 col-xs-12")], &[
                    html::div(&[attrs::class("card text-center")], &[
                        html::div(&[attrs::class("card-header")], &[
                            html::span(&[attrs::class("card-title")], &[html::text("Subdomain")]),
                        ]),
                        html::div(&[attrs::class("card-header")], &[
                            html::text("Custom subdomain with SSL certificate from Let's Encrypt")
                        ]),
                    ]),
                ]),
                html::div(&[attrs::class("column col-4 col-xs-12")], &[
                    html::div(&[attrs::class("card text-center")], &[
                        html::div(&[attrs::class("card-header")], &[
                            html::span(&[attrs::class("card-title")], &[html::text("Storage")]),
                        ]),
                        html::div(&[attrs::class("card-header")], &[
                            html::text(&format!("{} MB storage per site", max_size_megabyte))
                        ]),
                    ]),
                ]),
                html::div(&[attrs::class("column col-4 col-xs-12")], &[
                    html::div(&[attrs::class("card text-center")], &[
                        html::div(&[attrs::class("card-header")], &[
                            html::span(&[attrs::class("card-title")], &[html::text("Routes")]),
                        ]),
                        html::div(&[attrs::class("card-header")], &[
                            html::text(&format!("Up to {} routes per site", max_routes))
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ]
}
