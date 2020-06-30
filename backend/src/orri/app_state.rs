use std::path::PathBuf;
use crate::orri::encryption_key::EncryptionKey;
use crate::orri::site_key;
use crate::orri::site;

#[derive(Clone, Debug)]
pub struct AppState {
    pub config: Config,
}


#[derive(Clone, Debug)]
pub struct Config {
    pub encryption_key: EncryptionKey,
    pub server: ServerConfig,
    pub cookie: CookieConfig,
    pub site_key: site_key::Config,
    pub site: site::Config,
}


#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub app_domain: String,
    pub sites_domain: String,
    pub protocol: String,
    pub listen_addr: String,
    pub listen_port: u16,
    pub frontend_root: PathBuf,
    pub sites_root: PathBuf,
}

impl ServerConfig {
    pub fn sites_domain_with_port(&self, domain: &str) -> String {
        if self.listen_port == 80 || self.listen_port == 443 {
            domain.to_string()
        } else {
            format!("{}:{}", domain, self.listen_port)
        }
    }

    pub fn sites_base_url(&self, domain: &str) -> String {
        format!("{}://{}", self.protocol, self.sites_domain_with_port(domain))
    }

    pub fn listen_addr_with_port(&self) -> String {
        format!("{}:{}", self.listen_addr, self.listen_port)
    }


    pub fn static_path(&self) -> PathBuf {
        self.frontend_root.join(PathBuf::from("static"))
    }
}


#[derive(Clone, Debug)]
pub struct CookieConfig {
    pub secure: bool,
    pub max_age: i64,
}
