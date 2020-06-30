pub mod index;
pub mod guard;


use serde::Serialize;
use actix_web::dev::HttpResponseBuilder;
use http::header;

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
