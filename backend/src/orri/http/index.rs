use actix_web::{web, HttpResponse};
use http::header;
use crate::orri::app_state::AppState;
use crate::orri::page::{self, Page, Head};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::route::Route;


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let html = build_page().to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn build_page() -> Page {
    Page{
        head: Head{
            title: format!("Home - orri"),
            elements: vec![]
        },
        body: build_body()
    }
}


fn build_body() -> Vec<Html> {
    let new_site_route = Route::NewSite();
    let list_sites_route = Route::ListSites();

    vec![
        page::navbar(),
        html::div(&[attrs::class("container")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("hero hero-lg p-centered")], &[
                    html::div(&[attrs::class("hero-body")], &[
                        html::div(&[attrs::class("column col-12")], &[
                            html::h1(&[], &[html::text("Get started")]),
                            html::p(&[], &[html::text("Publish your site in seconds, no account required!")]),
                            html::a(&[attrs::class("btn btn-large btn-primary"), attrs::href(&new_site_route.to_string())], &[html::text("New site")]),
                            html::a(&[attrs::class("btn btn-large"), attrs::href(&list_sites_route.to_string())], &[html::text("Manage site")]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ]
}
