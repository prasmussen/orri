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
            title: format!("My sites - orri"),
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
    let find_site_route = Route::FindSite();

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("My sites", Route::MySites()),
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
                    html::table(&[attrs::class("table")], &[
                        html::thead(&[], &[
                            html::tr(&[], &[
                                html::th(&[], &[html::text("Domain")]),
                                html::th(&[], &[]),
                            ]),
                        ]),
                        html::tbody(&[], &rows),
                    ]),
                    html::div(&[attrs::class("form-group margin-top-40")], &[
                        html::a(
                            &[
                                attrs::href(&find_site_route.to_string()),
                                attrs::class("btn btn-lg"), 
                            ],
                            &[html::text("Manage other")]
                        ),
                    ]),
                ]),
            ]),
        ]),
    ]
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