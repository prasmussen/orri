use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use http::header;
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;


#[derive(Deserialize)]
pub struct Request {
    domain: String,
    key: String,
}


#[derive(Serialize)]
pub struct Response {
    html: String,
}

pub async fn handler(state: web::Data<AppState>, request_data: web::Json<Request>) -> HttpResponse {

    let html = build_html(&request_data, &state.config.server).to_string();

    HttpResponse::Ok()
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .json(Response{
            html: html,
        })
}

fn build_html(request_data: &Request, server_config: &ServerConfig) -> Html {
    let manage_url = format!("/sites/{}", &request_data.domain);
    let site_url = server_config.other_base_url(&request_data.domain);

    html::div(&[attrs::class("container"), attrs::id("content")], &[
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column")], &[
                html::h3(&[attrs::class("margin-top-40 margin-bottom-10")], &[
                    html::text("Congrats!"),
                ]),
                html::div(&[], &[
                    html::text("Your site "),
                    html::strong(&[], &[
                        html::em(&[], &[
                            html::text(&request_data.domain),
                        ]),
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
                        html::em(&[], &[
                            html::text(&request_data.key) ,
                        ]),
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
                        html::a(&[attrs::href(&site_url), attrs::target("_blank")], &[
                            html::text("Go to site"),
                        ]),
                    ]),
                    html::li(&[], &[
                        html::a(&[attrs::href(&manage_url), attrs::target("_blank")], &[
                            html::text("Manage site"),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ])
}
