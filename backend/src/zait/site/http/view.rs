use actix_web::{web, HttpRequest, HttpResponse, Either};
use actix_files::NamedFile;
use crate::zait::app_state::AppState;
use crate::zait::domain::{Domain, ParseDomainError};
use crate::zait::site::{self, Site, GetSiteError};
use http::header;
use std::path::PathBuf;
use std::io;


enum Error {
    ParseDomainError(ParseDomainError),
    GetSiteError(GetSiteError),
    RouteNotFound(),
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>) -> Either<Result<NamedFile, io::Error>, HttpResponse> {

    match handle(&req, &state) {
        Ok(data_path) => {
            Either::A(NamedFile::open(data_path))
        },

        Err(err) => {
            Either::B(handle_error(err))
        },
    }
}

fn handle(req: &HttpRequest, state: &AppState) -> Result<PathBuf, Error> {
    let path = req.uri().path();
    let host = get_host_header_string(&req)
        .unwrap_or(String::new());

    let domain = Domain::from_str(&host)
        .map_err(Error::ParseDomainError)?;

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, domain);

    let site = site::get(&site_root)
        .map_err(Error::GetSiteError)?;

    let route = site.routes.get(path)
        .ok_or(Error::RouteNotFound())?;

    Ok(site_root.data_file_path(&route.source_hash))
}

fn handle_error(err: Error) -> HttpResponse {
    match err {
        Error::ParseDomainError(err) => {
            HttpResponse::BadRequest().finish()
        },

        Error::GetSiteError(err) => {
            handle_get_site_error(err)
        },

        Error::RouteNotFound() => {
            println!("route not found");
            HttpResponse::NotFound().finish()
        },
    }
}


fn handle_get_site_error(err: GetSiteError) -> HttpResponse {
    match err {
        GetSiteError::SiteNotFound() => {
            println!("site not found");
            HttpResponse::NotFound().finish()
        },

        GetSiteError::FailedToReadSiteJson(err) => {
            println!("Failed to read site json: {}", err);
            HttpResponse::InternalServerError().finish()
        },
    }
}

enum HostHeaderError {
    NotFound(),
    ToStrError(header::ToStrError),
}

fn get_host_header_string(req: &HttpRequest) -> Result<String, HostHeaderError> {
    let value = req
        .headers()
        .get("host")
        .map_or(Err(HostHeaderError::NotFound()), Ok)?;

    value
        .to_str()
        .map(|s| s.to_string())
        .map_err(HostHeaderError::ToStrError)
}
