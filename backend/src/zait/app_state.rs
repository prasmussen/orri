#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}


#[derive(Clone)]
pub struct Config {
    pub server: ServerConfig,
}


#[derive(Clone)]
pub struct ServerConfig {
    pub main_domain: String,
    pub frontend_root: String,
    pub sites_root: String,
}

impl ServerConfig {
    pub fn static_path(&self) -> String {
        // TODO: use path type
        format!("{}/static", self.frontend_root)
    }

    pub fn frontend_file_path(&self, name: &str) -> String {
        // TODO: use path type
        format!("{}/{}", self.frontend_root, name)
    }
}
