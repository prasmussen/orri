use actix_web::{web, HttpRequest, HttpResponse};
use crate::orri::app_state::AppState;
use crate::orri::url_path::{self, UrlPath};
use crate::orri::http as http_helper;
use http::header;
use std::io;
use std::fs;
use std::str::FromStr;
use mime_guess;


enum Error {
    ParsePath(url_path::Error),
    ReadFile(io::Error),
}

struct File {
    data: Vec<u8>,
    mime: String,
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>) -> HttpResponse {

    handle(&req, &state)
        .map(prepare_response)
        .unwrap_or_else(handle_error)
}


fn handle(req: &HttpRequest, state: &AppState) -> Result<File, Error> {
    let url_path = UrlPath::from_str(req.uri().path())
        .map_err(Error::ParsePath)?;

    let path = state.config.server.frontend_root.join(url_path.relative_path());

    let data = fs::read(&path)
        .map_err(Error::ReadFile)?;

    let mime = mime_guess::from_path(&path).first_or_octet_stream();

    Ok(File{
        data,
        mime: mime.to_string(),
    })
}

fn prepare_response(file: File) -> HttpResponse {
    http_helper::no_cache_headers(&mut HttpResponse::Ok())
        .set_header(header::CONTENT_TYPE, file.mime)
        .body(file.data)
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParsePath(_err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::ReadFile(err) => {
            log::error!("Failed to read file: {}", err);
            HttpResponse::NotFound().finish()
        },
    }
}
