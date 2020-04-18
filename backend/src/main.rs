// TODO: enable warnings
#![allow(warnings)]


mod orri;

use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use actix_web::{web, App, HttpServer, guard};
use actix_files::Files;
use actix_session::CookieSession;
use orri::app_state::{self, AppState};
use orri::http::index;
use orri::site::http::api as site_api;
use orri::site::http as site_http;
use crate::orri::encryption_key::EncryptionKey;




fn main_domain_routes(config: &mut web::ServiceConfig, state: &AppState, host: &'static str) {
    // TODO: set SameSite, etc
    let cookie_session = CookieSession::private(state.config.encryption_key.as_bytes())
        .secure(state.config.cookie.secure);

    config.service(
        web::scope("/")
            .guard(guard::Header("Host", host))
            .wrap(cookie_session)
            .route("", web::get().to(index::handler))
            .route("/new", web::get().to(site_http::create::handler))
            .route("/sites/{domain}", web::get().to(site_http::list::handler))
            .route("/sites/{domain}/add-route", web::get().to(site_http::add_route::handler))
            .route("/api/sites", web::post().to(site_api::create::handler))
            .route("/api/sites", web::put().to(site_api::add_route::handler))
            .route("/api/sites/site-created", web::post().to(site_api::create_success::handler))
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
