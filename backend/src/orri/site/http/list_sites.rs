use actix_web::{web, HttpRequest, HttpResponse};
use actix_session::Session;
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::Domain;
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
use crate::orri::session_data::SessionData;
use http::header;
use std::path::PathBuf;
use std::io;
use serde::Deserialize;
use std::time::{Duration, Instant};
use std::str::FromStr;


pub async fn handler(state: web::Data<AppState>, session: Session) -> HttpResponse {
    let session_data = SessionData::from_session(&session)
        .unwrap_or(SessionData::new());


    let html = build_page(&state.config.server, session_data).to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn build_page(server_config: &ServerConfig, session_data: SessionData) -> Page {
    Page{
        head: Head{
            title: format!("Sites - orri"),
            elements: vec![],
        },
        body: build_body(server_config, session_data)
    }
}


fn build_body(server_config: &ServerConfig, session_data: SessionData) -> Vec<Html> {
    let rows = session_data
        .list_sites()
        .iter()
        .map(|domain| table_row(server_config, domain))
        .collect::<Vec<Html>>();

    let have_session_sites = rows.len() > 0;

    vec![
        html::div(&[attrs::class("container"), attrs::id("content")], &[
            html::div(&[attrs::class("row")], &[
                html::div(&[attrs::class("column")], &[
                    page::error_alert(),
                ]),
            ]),
            html::div(&[], &[
                html::div(
                    &[
                        attrs::id("sites-table"),
                        attrs::class_list(&[("display-none", have_session_sites == false)]),
                    ], &[
                        html::div(&[attrs::class("row")], &[
                            html::div(&[attrs::class("column")], &[
                                html::table(&[], &[
                                    html::thead(&[], &[
                                        html::tr(&[], &[
                                            html::th(&[], &[html::text("Domain")]),
                                            html::th(&[], &[]),
                                        ]),
                                    ]),
                                    html::tbody(&[], &rows),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("row")], &[
                            html::div(&[attrs::class("column column-25")], &[
                                html::button(&[attrs::type_("button"), attrs::id("manage-other")], &[html::text("Manage other")]),
                            ]),
                        ]),
                    ]
                ),
                html::div(
                    &[
                        attrs::id("manage-other-form"),
                        attrs::class_list(&[("display-none", have_session_sites)]),
                    ], &[
                        site_form(&server_config, have_session_sites),
                    ]
                ),
            ]),
        ]),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/list_sites.js")], &[]),
    ]
}


fn site_form(server_config: &ServerConfig, have_session_sites: bool) -> Html {
    let list_sites_route = Route::ListSites();

    html::form(
        &[
            attrs::id("form"),
            attrs::attribute_trusted_name("data-api-base-url", &list_sites_route.to_string())
        ], &[
        html::div(&[attrs::class("site-form row")], &[
            html::div(&[attrs::class("column")], &[
                html::label(&[], &[
                    html::div(&[], &[html::text("Domain")]),
                    html::input(&[
                        attrs::type_("text"),
                        attrs::name("sitesDomain"),
                        attrs::value(&server_config.sites_domain),
                        attrs::readonly(),
                    ]),
                ]),
            ]),
        ]),
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column")], &[
                html::label(&[], &[
                    html::div(&[], &[html::text("Subdomain")]),
                    html::input(&[
                        attrs::type_("text"),
                        attrs::name("subdomain"),
                        attrs::placeholder("i.e. my-cool-site"),
                        attrs::title("Please provide a valid subdomain, at least 4 characters"),
                        attrs::pattern("[a-z0-9-]{4,}"),
                        attrs::required(),
                    ]),
                ]),
            ]),
        ]),
        html::div(&[attrs::class("row")], &[
            html::div(&[attrs::class("column column-25")], &[
                html::button(&[attrs::type_("submit"), attrs::id("submit-button")], &[html::text("Manage")]),
            ]),
            html::div(
                &[
                    attrs::class_list(&[
                        ("column", true),
                        ("column-25", true),
                        ("display-none", have_session_sites == false),
                    ]),
                ], &[
                    html::button(&[attrs::type_("button"), attrs::id("show-my-sites")], &[html::text("Show my sites") ]),
                ]
            ),
        ]),
    ])
}

fn table_row(server_config: &ServerConfig, domain: &Domain) -> Html {
    let manage_route = Route::ManageSite(domain.to_string());
    let site_url = server_config.sites_base_url(&domain.to_string());

    html::tr(&[], &[
        html::td(&[], &[
            html::a(&[attrs::href(&site_url)], &[html::text(&domain.to_string())]),
        ]),
        html::td(&[], &[
            html::a(&[attrs::href(&manage_route.to_string())], &[html::text("Manage")]),
        ]),
    ])
}
