// TODO: enable warnings
#![allow(warnings)]

mod orri;

use std::io;
use std::path::PathBuf;
use actix_web::{web, App, HttpServer, guard};
use actix_files::Files;
use orri::app_state::{self, AppState};
use orri::http::index;
use orri::site::http::api as site_api;
use orri::site::http as site_http;




fn main_domain_routes(config: &mut web::ServiceConfig, state: &AppState, host: &'static str) {
    config.service(
        web::scope("/")
            .guard(guard::Header("Host", host))
            .route("", web::get().to(index::handler))
            .route("/new", web::get().to(site_http::create::handler))
            .route("/api/sites", web::post().to(site_api::create::handler))
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
            server: app_state::ServerConfig{
                main_domain: "orri.loc:8000".to_string(),
                frontend_root: PathBuf::from("../frontend"),
                sites_root: PathBuf::from("../sites"),
            }
        }
    };

    // TODO: This is probably ok, but is it possible to have a 'static String in the config?
    let main_domain = to_static_str(state.config.server.main_domain.clone());

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10))
            .configure(|cfg| main_domain_routes(cfg, &state, main_domain))
            .configure(other_domains_routes)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
