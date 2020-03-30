use actix_web::{web, App, HttpRequest, HttpServer, Responder};


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

        // TODO: make sure host only contains allowed characters
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
            index_domain(req, domain),
    }
}

fn index_main(req: HttpRequest) -> String {
    format!("main!")
}

fn index_domain(req: HttpRequest, domain: String) -> String {
    format!("domain: {}", domain)
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
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
