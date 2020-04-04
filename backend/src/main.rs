mod zait;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use std::io;
use zait::app_state::{self, AppState};
use zait::site::http::api as site_api;
use zait::site::http as site_http;



enum Host {
    Main(),
    Domain(String),
}


impl Host {
    pub fn from_req(req: &HttpRequest, main_domain: &str) -> Option<Host> {
        let value = req.headers().get("host")?;
        let host = value.to_str().ok()?;

        // TODO: make sure host only contains allowed characters (add newtype?)
        if host == main_domain {
            Some(Host::Main())

        } else {
            Some(Host::Domain(host.to_string()))
        }
    }
}


async fn index(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("{:?}", req);

    let host = Host::from_req(&req, &state.config.server.main_domain)
        .unwrap_or(Host::Main());

    match host {
        Host::Main() =>
            index_main(req, state),

        Host::Domain(domain) =>
            index_domain(req, state, &domain),
    }
}


fn index_main(req: HttpRequest, state: web::Data<AppState>) -> Result<NamedFile, io::Error> {
    NamedFile::open(state.config.server.frontend_file_path("index.html"))
}

fn index_domain(req: HttpRequest, state: web::Data<AppState>, domain: &str) -> Result<NamedFile, io::Error> {
    NamedFile::open(state.config.server.frontend_file_path("index.html"))
}




#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let state = app_state::AppState{
        config: app_state::Config{
            server: app_state::ServerConfig{
                main_domain: "zait.io".to_string(),
                frontend_root: "../frontend".to_string(),
                sites_root: "../sites".to_string(),
            }
        }
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(index))
            .route("/new", web::get().to(site_http::create::handler))
            .route("/api/sites", web::post().to(site_api::create::handler))
            .service(Files::new("/static", state.config.server.static_path()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
