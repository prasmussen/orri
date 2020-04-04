use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::zait::app_state::AppState;
use crate::zait::site::{self, CreateSiteError};
use crate::zait::http::Error;


#[derive(Deserialize)]
pub struct Request {
    domain: String,
    source: String,
}


#[derive(Serialize)]
pub struct Response {
    key: String,
}


pub async fn handler(req: HttpRequest, state: web::Data<AppState>, payload: web::Json<Request>) -> HttpResponse {

    let site_root = site::SiteRoot::new(&state.config.server.sites_root, &payload.domain);

    match site::create(site_root, &payload.source) {
        Ok(site) =>
            HttpResponse::Ok()
                .json(Response{
                    key: site.key,
                }),

        Err(err) =>
            handle_error(&err),
    }
}


fn handle_error(err: &CreateSiteError) -> HttpResponse {
    match err {
        CreateSiteError::SiteAlreadyExist() => {
            HttpResponse::Conflict()
                .json(Error::from_str("Site already exist"))
        },

        CreateSiteError::FailedToCreateDomainDir(err) => {
            println!("Failed to create domain: {}", err);
            HttpResponse::InternalServerError()
                .json(Error::from_str("Failed to create domain dir"))
        },

        CreateSiteError::FailedToWriteSourceFile(err) => {
            println!("Failed to write source file: {}", err);
            HttpResponse::InternalServerError()
                .json(Error::from_str("Failed to write source file"))
        }

        CreateSiteError::FailedToSaveSiteJson(err) => {
            println!("Failed to save config: {}", err);
            HttpResponse::InternalServerError()
                .json(Error::from_str("Failed to save config"))
        }
    }
}
