// TODO: enable warnings
#![allow(warnings)]


mod orri;

use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use actix_web::{web, App, HttpServer, guard};
use actix_files::Files;
use actix_session::CookieSession;
use actix_http::cookie::SameSite;
use orri::app_state::{self, AppState};
use orri::http::index;
use orri::site::http::api as site_api;
use orri::site::http as site_http;
use crate::orri::encryption_key::EncryptionKey;
use orri::site_key;
use orri::site;
use orri::route::Route;
use orri::util;




fn app_domain_routes(config: &mut web::ServiceConfig, state: &AppState, host: &'static str) {
    let cookie_session = CookieSession::private(state.config.encryption_key.as_bytes())
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(state.config.cookie.secure)
        .max_age(state.config.cookie.max_age);

    config.service(
        web::scope("/")
            .guard(guard::Header("Host", host))
            .wrap(cookie_session)

            // User facing routes
            .route("", web::get().to(index::handler))
            .route(&Route::NewSite().to_string(), web::get().to(site_http::new_site::handler))
            .route(&Route::ListSites().to_string(), web::get().to(site_http::list_sites::handler))
            .route(&Route::SiteExist("{domain}".to_string()).to_string(), web::head().to(site_http::site_exist::handler))
            .route(&Route::ManageSite("{domain}".to_string()).to_string(), web::get().to(site_http::manage_site::handler))
            .route(&Route::AddRoute("{domain}".to_string()).to_string(), web::get().to(site_http::add_route::handler))
            .route(&Route::EditRoute("{domain}".to_string(), None).to_string(), web::get().to(site_http::edit_route::handler))

            // Json routes
            .route(
                &Route::NewSiteJson().to_string(),
                web::method(Route::NewSiteJson().request_method()).to(site_api::new_site::handler)
            )
            .route(
                &Route::AddRouteJson().to_string(),
                web::method(Route::AddRouteJson().request_method()).to(site_api::add_route::handler)
            )
            .route(
                &Route::EditRouteJson().to_string(),
                web::method(Route::EditRouteJson().request_method()).to(site_api::edit_route::handler)
            )
            .route(
                &Route::DeleteRouteJson().to_string(),
                web::method(Route::DeleteRouteJson().request_method()).to(site_api::remove_route::handler)
            )
            .route(
                &Route::DeleteSiteJson().to_string(),
                web::method(Route::DeleteSiteJson().request_method()).to(site_api::remove_site::handler)
            )

            // Static files
            .service(Files::new("/static", state.config.server.static_path()))
    );
}

fn sites_domain_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .route("", web::get().to(site_http::view_site::handler))
            .route("{tail:.*}", web::get().to(site_http::view_site::handler))
    );
}


#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    println!("Starting...");

    let state = app_state::AppState{
        config: app_state::Config{
            encryption_key: "YdotmVZtV5R3PRnzfCiKBV3gtitSFg70".parse().unwrap(),
            server: app_state::ServerConfig{
                app_domain: "orri.devz".to_string(),
                sites_domain: "orri.pagez".to_string(),
                protocol: "http".to_string(),
                listen_addr: "127.0.0.1".to_string(),
                listen_port: 8000,
                frontend_root: PathBuf::from("../frontend"),
                sites_root: PathBuf::from("../sites"),
            },
            cookie: app_state::CookieConfig{
                secure: false,
                max_age: 315576000,
            },
            site_key: site_key::Config{
                min_length: 20,
                max_length: 50,
                hash_iterations: 1,
                hash_memory_size: 4096,
            },
            site: site::Config{
                quota_nano: site::QuotaLimits{
                    max_size: 1 * 1024 * 1024,
                    max_routes: 100,
                    max_sites: 10,
                },
            },
        }
    };

    // TODO: This is probably ok, but is it possible to have a 'static String in the config?
    let domain = util::to_static_str(state.config.server.app_domain_with_port());
    let listen_addr = &state.config.server.listen_addr_with_port();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10))
            .configure(|cfg| app_domain_routes(cfg, &state, domain))
            .configure(sites_domain_routes)
    })
    .bind(listen_addr)?
    .run()
    .await
}
