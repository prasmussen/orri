use actix_web::{web, HttpResponse};
use http::header;
use crate::orri::app_state::AppState;
use crate::orri::page::{Page, Head};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;


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
            title: format!("orri.index()"),
            elements: vec![]
        },
        body: build_body()
    }
}


fn build_body() -> Vec<Html> {
    vec![
        html::node("div", &[attrs::attribute("class", "container")], &[
            html::node("a", &[attrs::attribute("href", "/new"), attrs::attribute("class", "button")], &[html::text("New site")]),
        ]),
    ]
}
