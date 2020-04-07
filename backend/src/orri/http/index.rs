use actix_web::{web, Responder};
use actix_files::NamedFile;
use crate::orri::app_state::AppState;


pub async fn handler(state: web::Data<AppState>) -> impl Responder {
    NamedFile::open(state.config.server.frontend_file_path("index.html"))
}
