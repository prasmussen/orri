use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use std::io;


#[derive(Clone)]
struct Config {
    server: ServerConfig,
}

#[derive(Clone)]
struct ServerConfig {
    domain: String,
}


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

    let host = Host::from_req(&req, &state.config.server.domain)
        .unwrap_or(Host::Main());

    match host {
        Host::Main() =>
            index_main(req),

        Host::Domain(domain) =>
            index_domain(req, &domain),
    }
}


fn index_main(req: HttpRequest) -> Result<NamedFile, io::Error> {
    NamedFile::open("../frontend/index.html")
}

fn index_domain(req: HttpRequest, domain: &str) -> Result<NamedFile, io::Error> {
    NamedFile::open("../frontend/index.html")
}



#[derive(Clone)]
struct AppState {
    config: Config,
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let state = AppState{
        config: Config{
            server: ServerConfig{
                domain: "zait.io".to_string(),
            }
        }
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(index))
            .service(Files::new("/static", "../frontend/static"))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
