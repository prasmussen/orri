use actix_web::{web, Responder};
use crate::zait::app_state::AppState;
use actix_files::NamedFile;


pub async fn handler(state: web::Data<AppState>) -> impl Responder {
    NamedFile::open(state.config.server.frontend_file_path("new.html"))
}
