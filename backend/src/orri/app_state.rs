use std::path::PathBuf;
use crate::orri::encryption_key::EncryptionKey;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}


#[derive(Clone)]
pub struct Config {
    pub encryption_key: EncryptionKey,
    pub server: ServerConfig,
    pub cookie: CookieConfig,
}


#[derive(Clone)]
pub struct ServerConfig {
    pub main_domain: String,
    pub frontend_root: PathBuf,
    pub sites_root: PathBuf,
}

impl ServerConfig {
    pub fn static_path(&self) -> PathBuf {
        self.frontend_root.join(PathBuf::from("static"))
    }

    pub fn frontend_file_path(&self, name: &'static str) -> PathBuf {
        self.frontend_root.join(PathBuf::from(name))
    }
}


#[derive(Clone)]
pub struct CookieConfig {
    pub secure: bool,
}
