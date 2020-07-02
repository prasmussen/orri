pub mod index;
pub mod guard;
pub mod static_files;


use serde::Serialize;
use actix_web::dev::HttpResponseBuilder;
use http::header;
use actix_http::http as actix_http_helper;

#[derive(Serialize)]
pub struct Error {
    error: String,
}

impl Error {
    pub fn from_str(err: &str) -> Error {
        Error {
            error: err.to_string(),
        }
    }
}


pub fn no_cache_headers(builder: &mut HttpResponseBuilder) -> &mut HttpResponseBuilder {
    builder
        .set_header(header::CACHE_CONTROL, "no-cache, no-store, max-age=0, must-revalidate")
        .set_header(header::EXPIRES, "Mon, 01 Jan 1990 00:00:00 GMT")
        .set_header(header::PRAGMA, "no-cache")
}


#[derive(Debug)]
pub struct Host(pub actix_http_helper::header::HeaderValue);


pub fn get_host(headers: &actix_http_helper::header::HeaderMap) -> Host {
    let host = headers.get("Host")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(':').next())
        .and_then(|value| actix_http_helper::header::HeaderValue::from_str(value).ok())
        .unwrap_or_else(|| actix_http_helper::header::HeaderValue::from_static(""));

    Host(host)
}
