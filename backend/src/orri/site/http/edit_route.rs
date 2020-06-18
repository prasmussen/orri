use actix_web::{web, HttpRequest, HttpResponse};
use actix_session::Session;
use crate::orri::app_state::AppState;
use crate::orri::domain::{self, Domain};
use crate::orri::encryption_key::{EncryptionKey};
use crate::orri::site::{self, Site, GetSiteError, File, RouteInfo};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
use crate::orri::url_path::{self, UrlPath};
use crate::orri::session_data::{SessionData};
use crate::orri::util;
use crate::orri::http as http_helper;
use http::header;
use std::path::PathBuf;
use std::io;
use serde::Deserialize;
use std::str::FromStr;




#[derive(Deserialize)]
pub struct QueryParams {
    path: String,
}

struct RequestData {
    domain: String,
    path: String,
}

struct ViewData {
    site: Site,
    path: UrlPath,
}

enum Error {
    ParseDomainError(domain::Error),
    ParsePathError(url_path::Error),
    RouteDoesNotExist(),
    GetSiteError(GetSiteError),
}

pub async fn handler(state: web::Data<AppState>, session: Session, domain: web::Path<String>, query: web::Query<QueryParams>) -> HttpResponse {

    let request_data = RequestData{
        domain: domain.to_string(),
        path: query.path.clone(),
    };

    handle(&state, &request_data)
        .map(|view_data| prepare_response(view_data, &session, &state.config.encryption_key))
        .unwrap_or_else(handle_error)
}

fn handle(state: &AppState, request_data: &RequestData) -> Result<ViewData, Error> {

    let domain = Domain::from_str(&request_data.domain)
        .map_err(Error::ParseDomainError)?;

    let path = UrlPath::from_str(&request_data.path)
        .map_err(Error::ParsePathError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    util::ensure(site.routes.contains_key(&path), Error::RouteDoesNotExist())?;

    Ok(ViewData{
        site: site,
        path: path,
    })
}


fn prepare_response(view_data: ViewData, session: &Session, encryption_key: &EncryptionKey) -> HttpResponse {

    let client_has_key = SessionData::from_session(session)
        .and_then(|session_data| session_data.get_site_key(&view_data.site.domain))
        .and_then(|key_from_session| view_data.site.key.verify(&key_from_session, encryption_key).ok())
        .unwrap_or(false);

    let html = build_page(&view_data, client_has_key).to_string();

    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .set_header(header::CONTENT_TYPE, "text/html")
        .body(html)
}


fn build_page(view_data: &ViewData, client_has_key: bool) -> Page {
    Page{
        head: Head{
            title: format!("Edit route - {} - orri", &view_data.site.domain),
            elements: vec![],
        },
        body: build_body(view_data, client_has_key),
    }
}


fn build_body(view_data: &ViewData, client_has_key: bool) -> Vec<Html> {
    let edit_route = Route::EditRouteJson();
    let delete_route = Route::DeleteRouteJson();

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("Sites", Route::MySites()),
                page::breadcrumb(&view_data.site.domain.to_string(), Route::ManageSite(view_data.site.domain.to_string())),
                page::breadcrumb("Edit route", Route::EditRoute(view_data.site.domain.to_string(), Some(view_data.path.to_string()))),
            ]),
        ),
        html::div(&[attrs::class("container")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("column col-6 col-mx-auto")], &[
                    page::error_alert(),
                    html::form(
                        &[
                            attrs::id("form"),
                            attrs::attribute_trusted_name("data-api-method", &edit_route.request_method().to_string()),
                            attrs::attribute_trusted_name("data-api-url", &edit_route.to_string()),
                        ], &[
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("Domain")]),
                                html::input(&[
                                    attrs::type_("text"),
                                    attrs::class("form-input"),
                                    attrs::name("domain"),
                                    attrs::value(&view_data.site.domain.to_string()),
                                    attrs::readonly(),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("Route")]),
                                html::input(&[
                                    attrs::type_("text"),
                                    attrs::class("form-input"),
                                    attrs::name("path"),
                                    attrs::value(&view_data.path.to_string()),
                                    attrs::readonly(),
                                ]),
                            ]),
                        ]),
                        html::conditional(client_has_key == false,
                            html::div(&[attrs::class("form-group")], &[
                                html::label(&[attrs::class("form-label")], &[
                                    html::div(&[], &[html::text("Site key")]),
                                    html::input(&[
                                        attrs::type_("password"),
                                        attrs::class("form-input"),
                                        attrs::name("key"),
                                        attrs::required(),
                                    ]),
                                ]),
                            ]),
                        ),
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("File")]),
                                html::input(&[
                                    attrs::type_("file"),
                                    attrs::class("form-input"),
                                    attrs::id("file"),
                                    attrs::required(),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group margin-top-20")], &[
                            html::button(
                                &[
                                    attrs::type_("submit"),
                                    attrs::class("btn btn-primary btn-lg"),
                                    attrs::id("submit-button")
                                ], &[html::text("Update route")]
                            ),
                            html::button(
                                &[
                                    attrs::id("remove-route"),
                                    attrs::class("btn btn-error btn-lg"),
                                    attrs::type_("button"),
                                    attrs::class("button-outline"),
                                    attrs::attribute_trusted_name("data-api-method", &delete_route.request_method().to_string()),
                                    attrs::attribute_trusted_name("data-api-url", &delete_route.to_string()),
                                    attrs::attribute_trusted_name("data-api-body-domain", &view_data.site.domain.to_string()),
                                    attrs::attribute_trusted_name("data-api-body-path", &view_data.path.to_string()),
                                ],
                                &[html::text("Remove route")]
                            ),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/route_form.js")], &[]),
        html::script(&[attrs::src("/static/remove_route.js")], &[]),
    ]
}


fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::ParsePathError(err) =>
            handle_parse_path_error(err),

        Error::RouteDoesNotExist() =>
            HttpResponse::NotFound()
                .body("Path not found"),

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
        },
    }
}

fn handle_parse_path_error(err: url_path::Error) -> HttpResponse {
    match err {
        url_path::Error::MustStartWithSlash() =>
            HttpResponse::BadRequest()
                .body("The path must start with a slash"),

        url_path::Error::TooLong() =>
            HttpResponse::BadRequest()
                .body("The path is too long"),

        url_path::Error::ContainsDisallowedChars() =>
            HttpResponse::BadRequest()
                .body("The path contains disallowed characters"),

        url_path::Error::ContainsDoubleDot() =>
            HttpResponse::BadRequest()
                .body("The path cannot contain double dots"),
    }
}

fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            HttpResponse::NotFound()
                .body("Site not found")
        },

        GetSiteError::FailedToReadSiteJson(err) => {
            log::error!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}


