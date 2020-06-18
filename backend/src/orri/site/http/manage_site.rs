use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::{AppState, ServerConfig};
use crate::orri::domain::{self, Domain};
use crate::orri::url_path::{self, UrlPath};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
use crate::orri::util;
use http::header;
use std::path::PathBuf;
use std::io;
use std::str::FromStr;
use std::time::SystemTime;


enum Error {
    ParseDomainError(domain::Error),
    GetSiteError(GetSiteError),
}


pub async fn handler(state: web::Data<AppState>, domain: web::Path<String>) -> HttpResponse {
    let base_url = &state.config.server.sites_base_url(&domain);

    handle(&state, &domain)
        .map(|site| prepare_response(site, base_url))
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {
    let domain = Domain::from_str(&domain_str)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSiteError)
}


fn prepare_response(site: Site, base_url: &str) -> HttpResponse {
    let html = build_page(&site, base_url).to_string();

    HttpResponse::Ok()
        .set_header(header::CONTENT_TYPE, "text/html")
        .set_header(header::CACHE_CONTROL, "no-cache")
        .set_header(header::PRAGMA, "no-cache")
        .body(html)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
        },
    }
}


fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        GetSiteError::FailedToReadSiteJson(err) => {
            log::error!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

fn build_page(site: &Site, base_url: &str) -> Page {
    Page{
        head: Head{
            title: format!("Manage {} - orri", &site.domain),
            elements: vec![]
        },
        body: build_body(site, base_url)
    }
}

fn build_body(site: &Site, base_url: &str) -> Vec<Html> {
    let add_route_route = Route::AddRoute(site.domain.to_string());
    let delete_site_route = Route::DeleteSiteJson();

    let now = SystemTime::now();

    let rows = site.routes
        .iter()
        .map(|(route, route_info)| table_row(site, route, route_info, base_url, now))
        .collect::<Vec<Html>>();

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("Sites", Route::MySites()),
                page::breadcrumb(&site.domain.to_string(), Route::ManageSite(site.domain.to_string())),
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
                                html::th(&[], &[html::text("Route")]),
                                html::th(&[], &[html::text("Mime")]),
                                html::th(&[], &[html::text("Size")]),
                                html::th(&[], &[]),
                            ]),
                        ]),
                        html::tbody(&[], &rows),
                    ]),
                    html::div(&[attrs::class("form-group margin-top-40")], &[
                        html::a(
                            &[
                                attrs::href(&add_route_route.to_string()),
                                attrs::class("btn btn-primary btn-lg"),
                            ],
                            &[html::text("Add route")]
                        ),
                        html::button(
                            &[
                                attrs::id("remove-site"),
                                attrs::type_("button"),
                                attrs::class("btn btn-error btn-lg"),
                                attrs::attribute_trusted_name("data-api-method", &delete_site_route.request_method().to_string()),
                                attrs::attribute_trusted_name("data-api-url", &delete_site_route.to_string()),
                                attrs::attribute_trusted_name("data-api-body-domain", &site.domain.to_string()),
                            ],
                            &[html::text("Remove site")]
                        ),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/manage_site.js")], &[]),
    ]
}

fn table_row(site: &Site, route: &UrlPath, route_info: &RouteInfo, base_url: &str, now: SystemTime) -> Html {
    let route_url = format!("{}{}", base_url, route);
    let edit_url = Route::EditRoute(site.domain.to_string(), Some(route.to_string())).to_string();
    let age_in_seconds = util::unix_timestamp(now) - route_info.file_info.timestamp;
    let recently_added = site.routes.len() > 1 && age_in_seconds < 5;

    html::tr(&[attrs::class_list(&[("success-fade", recently_added)])], &[
        html::td(&[], &[
            html::a(&[attrs::href(&route_url)], &[html::text(&route.to_string())]),
        ]),
        html::td(&[], &[html::text(&route_info.file_info.mime)]),
        html::td(&[], &[html::text(&route_info.file_info.size.to_string())]),
        html::td(&[], &[
            html::a(&[attrs::href(&edit_url)], &[html::text("Edit")]),
        ]),
    ])
}
