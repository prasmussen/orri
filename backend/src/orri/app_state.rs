use std::path::PathBuf;
use crate::orri::encryption_key::EncryptionKey;
use crate::orri::site_key;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
}


#[derive(Clone)]
pub struct Config {
    pub encryption_key: EncryptionKey,
    pub server: ServerConfig,
    pub cookie: CookieConfig,
    pub site_key: site_key::Config,
}


#[derive(Clone)]
pub struct ServerConfig {
    pub domain: String,
    pub protocol: String,
    pub listen_addr: String,
    pub listen_port: u16,
    pub frontend_root: PathBuf,
    pub sites_root: PathBuf,
}

impl ServerConfig {
    pub fn domain_with_port(&self) -> String {
        if self.listen_port == 80 || self.listen_port == 443 {
            self.domain.clone()
        } else {
            format!("{}:{}", self.domain, self.listen_port)
        }
    }

    pub fn other_domain_with_port(&self, domain: &str) -> String {
        if self.listen_port == 80 || self.listen_port == 443 {
            domain.to_string().clone()
        } else {
            format!("{}:{}", domain, self.listen_port)
        }
    }

    pub fn base_url(&self) -> String {
        format!("{}://{}", self.protocol, self.domain_with_port())
    }

    pub fn other_base_url(&self, domain: &str) -> String {
        format!("{}://{}", self.protocol, self.other_domain_with_port(domain))
    }

    pub fn listen_addr_with_port(&self) -> String {
        format!("{}:{}", self.listen_addr, self.listen_port)
    }


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
