use actix_web::{web, HttpResponse};
use crate::orri::app_state::{Environment, AppState};
use crate::orri::http as http_helper;
use actix_http::http::{header};


pub async fn handler(state: web::Data<AppState>) -> HttpResponse {
    let server_config = &state.config.server;

    let location = match server_config.environment {
        Environment::Production() =>
            format!("{}://{}", server_config.protocol, server_config.app_domain),

        _ =>
            format!("{}://{}:{}", server_config.protocol, server_config.app_domain, server_config.listen_port),
    };

    http_helper::no_cache_headers(&mut HttpResponse::MovedPermanently())
        .set_header(header::LOCATION, location)
        .finish()
}
