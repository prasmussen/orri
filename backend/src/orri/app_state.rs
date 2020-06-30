use std::str::FromStr;
use std::path::PathBuf;
use crate::orri::encryption_key::EncryptionKey;
use crate::orri::site_key;
use crate::orri::site;
use std::fmt;

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


#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Environment {
    Production(),
    Development(),
}

pub enum EnvironmentFromStrError {
    UnknownEnvironment(String)
}

impl fmt::Display for EnvironmentFromStrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EnvironmentFromStrError::UnknownEnvironment(s) =>
                write!(f, "Unknown environment «{0}»", s),
        }
    }
}

impl FromStr for Environment {
    type Err = EnvironmentFromStrError;

    fn from_str(s: &str) -> Result<Environment, EnvironmentFromStrError> {
        match s {
            "production" =>
                Ok(Environment::Production()),

            "development" =>
                Ok(Environment::Development()),

            value =>
                Err(EnvironmentFromStrError::UnknownEnvironment(value.to_string())),
        }
    }
}


#[derive(Clone, Debug)]
pub struct ServerConfig {
    pub environment: Environment,
    pub app_domain: String,
    pub sites_domain: String,
    pub protocol: String,
    pub listen_addr: String,
    pub listen_port: u16,
    pub frontend_root: PathBuf,
    pub sites_root: PathBuf,
}

impl ServerConfig {
    pub fn sites_base_url(&self, domain: &str) -> String {
        format!("{}://{}", self.protocol, self.sites_domain_with_port(domain))
    }

    pub fn listen_addr_with_port(&self) -> String {
        format!("{}:{}", self.listen_addr, self.listen_port)
    }

    pub fn static_path(&self) -> PathBuf {
        self.frontend_root.join(PathBuf::from("static"))
    }

    fn sites_domain_with_port(&self, domain: &str) -> String {
        if self.environment == Environment::Production() || self.listen_port == 80 || self.listen_port == 443 {
            domain.to_string()
        } else {
            format!("{}:{}", domain, self.listen_port)
        }
    }
}


#[derive(Clone, Debug)]
pub struct CookieConfig {
    pub secure: bool,
    pub max_age: i64,
}
