use serde::Serialize;

pub mod index;


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

