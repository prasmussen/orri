use actix_web::{web, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
use crate::orri::http as http_helper;
use http::header;


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let html = build_page(&state.config.server).render();

    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .set_header(header::CONTENT_TYPE, "text/html")
        .body(html)
}

fn build_page(server_config: &ServerConfig) -> Page {
    Page{
        head: Head{
            title: "Find site - orri".to_string(),
            elements: vec![],
        },
        body: build_body(server_config)
    }
}


fn build_body(server_config: &ServerConfig) -> Vec<Html> {
    let my_sites_route = Route::MySites();
    let site_exist_base_route = Route::SiteExist("".to_string());

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("Find site", Route::FindSite()),
            ]),
        ),
        html::div(&[attrs::class("container"), attrs::id("content")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("column col-6 col-mx-auto")], &[
                    page::error_alert(),
                ]),
            ]),
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("column col-6 col-mx-auto")], &[
                    html::form(
                        &[
                            attrs::id("form"),
                            attrs::attribute_trusted_name("data-api-base-url", &site_exist_base_route.to_string())
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
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group margin-top-40")], &[
                            html::button(
                                &[
                                    attrs::class("btn btn-primary btn-lg"),
                                    attrs::type_("submit"),
                                    attrs::id("submit-button"),
                                ],
                                &[html::text("Manage site")]
                            ),
                            html::a(
                                &[
                                    attrs::class("btn btn-lg"),
                                    attrs::href(&my_sites_route.to_string()),
                                ],
                                &[html::text("My sites")]
                            ),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/find_site.js")], &[]),
    ]
}
