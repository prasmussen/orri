use actix_web::{web, HttpResponse};
use actix_session::Session;
use crate::orri::app_state::AppState;
use crate::orri::domain::{self, Domain};
use crate::orri::site::{self, Site, GetSiteError};
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;
use crate::orri::page::{self, Page, Head};
use crate::orri::route::Route;
use crate::orri::session_data::{SessionData};
use crate::orri::http as http_helper;
use http::header;
use std::str::FromStr;


enum Error {
    ParseDomain(domain::Error),
    GetSite(GetSiteError),
}

pub async fn handler(state: web::Data<AppState>, session: Session, domain: web::Path<String>) -> HttpResponse {
    handle(&state, &domain)
        .map(|site| prepare_response(site, &session))
        .unwrap_or_else(handle_error)
}


fn handle(state: &AppState, domain_str: &str) -> Result<Site, Error> {

    let domain = Domain::from_str(domain_str)
        .map_err(Error::ParseDomain)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    site::get(&site_root)
        .map_err(Error::GetSite)
}


fn prepare_response(site: Site, session: &Session) -> HttpResponse {

    let client_has_key = SessionData::from_session(session)
        .and_then(|session_data| session_data.get_site_key(&site.domain))
        .and_then(|key_from_session| site.key.verify(&key_from_session).ok())
        .unwrap_or(false);

    let html = build_page(&site, client_has_key).render();

    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .set_header(header::CONTENT_TYPE, "text/html")
        .body(html)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomain(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSite(err) => {
            handle_get_site_error(err)
        },
    }
}


fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            HttpResponse::NotFound().finish()
        },

        GetSiteError::ReadSiteJson(err) => {
            log::error!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

fn build_page(site: &Site, client_has_key: bool) -> Page {
    Page{
        head: Head{
            title: format!("Add route - {} - orri", &site.domain),
            elements: vec![],
        },
        body: build_body(site, client_has_key),
    }
}


fn build_body(site: &Site, client_has_key: bool) -> Vec<Html> {
    let add_route = Route::AddRouteJson();

    vec![
        page::navbar(
            page::breadcrumbs(&[
                page::breadcrumb("Home", Route::Index()),
                page::breadcrumb("Sites", Route::MySites()),
                page::breadcrumb(&site.domain.to_string(), Route::ManageSite(site.domain.to_string())),
                page::breadcrumb("Add route", Route::AddRoute(site.domain.to_string())),
            ]),
        ),
        html::div(&[attrs::class("container")], &[
            html::div(&[attrs::class("columns")], &[
                html::div(&[attrs::class("column col-6 col-mx-auto")], &[
                    page::error_alert(),
                    html::form(
                        &[
                            attrs::id("form"),
                            attrs::attribute_trusted_name("data-api-method", &add_route.request_method().to_string()),
                            attrs::attribute_trusted_name("data-api-url", &add_route.to_string())
                        ], &[
                        html::div(&[attrs::class("form-group")], &[
                            html::label(&[attrs::class("form-label")], &[
                                html::div(&[], &[html::text("Domain")]),
                                html::input(&[
                                    attrs::type_("text"),
                                    attrs::class("form-input"),
                                    attrs::name("domain"),
                                    attrs::value(&site.domain.to_string()),
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
                                    attrs::placeholder("i.e. /some-page or /some-styles.css"),
                                    attrs::title("The path to the file, it must start with a slash"),
                                    attrs::pattern("/.+"),
                                    attrs::required(),
                                ]),
                                html::p(&[attrs::class("form-input-hint")], &[
                                    html::text("The route must start with a /"),
                                ]),
                            ]),
                        ]),
                        html::conditional(!client_has_key,
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
                                html::p(&[attrs::class("form-input-hint")], &[
                                    html::text("Most formats are supported, i.e. html, js, css, jpg."),
                                ]),
                            ]),
                        ]),
                        html::div(&[attrs::class("form-group margin-top-20")], &[
                            html::button(
                                &[
                                    attrs::type_("submit"),
                                    attrs::class("btn btn-primary btn-lg"),
                                    attrs::id("submit-button")
                                ],
                                &[html::text("Add route")]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
        html::script(&[attrs::src("/static/orri.js")], &[]),
        html::script(&[attrs::src("/static/route_form.js")], &[]),
    ]
}
