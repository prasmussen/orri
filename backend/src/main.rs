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




fn main_domain_routes(config: &mut web::ServiceConfig, state: &AppState, host: &'static str) {
    let cookie_session = CookieSession::private(state.config.encryption_key.as_bytes())
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(state.config.cookie.secure)
        .max_age(state.config.cookie.max_age);

    config.service(
        web::scope("/")
            .guard(guard::Header("Host", host))
            .wrap(cookie_session)
            .route("", web::get().to(index::handler))
            .route("/new", web::get().to(site_http::new_site::handler))
            .route("/sites/{domain}", web::get().to(site_http::list::handler))
            .route("/sites/{domain}/add-route", web::get().to(site_http::add_route::handler))
            .route("/api/sites", web::post().to(site_api::new_site::handler))
            .route("/api/sites", web::put().to(site_api::add_route::handler))
            // TODO:
            //.route("/api/sites/{domain}/{path}", web::put().to(site_api::add_route::handler))
            .service(Files::new("/static", state.config.server.static_path()))
    );
}

fn other_domains_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .route("", web::get().to(site_http::view::handler))
            .route("{tail:.*}", web::get().to(site_http::view::handler))
    );
}


fn to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    println!("Starting...");

    let state = app_state::AppState{
        config: app_state::Config{
            encryption_key: "YdotmVZtV5R3PRnzfCiKBV3gtitSFg70".parse().unwrap(),
            server: app_state::ServerConfig{
                domain: "orri.loc".to_string(),
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
                max_length: 99,
                hash_iterations: 1,
                hash_memory_size: 4096,
            },
            site: site::Config{
                quota_nano: site::QuotaLimits{
                    max_size: 1 * 1024 * 1024,
                    max_routes: 10,
                    max_sites: 10,
                },
            },
        }
    };

    // TODO: This is probably ok, but is it possible to have a 'static String in the config?
    let domain = to_static_str(state.config.server.domain_with_port());
    let listen_addr = &state.config.server.listen_addr_with_port();

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10))
            .configure(|cfg| main_domain_routes(cfg, &state, domain))
            .configure(other_domains_routes)
    })
    .bind(listen_addr)?
    .run()
    .await
}
