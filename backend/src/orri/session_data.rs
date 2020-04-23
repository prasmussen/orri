use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_json;
use crate::orri::util;
use crate::orri::domain::Domain;
use crate::orri::site::{self, Site};
use crate::orri::encryption_key::EncryptionKey;
use std::collections::HashMap;
use actix_session::Session;


#[derive(Clone, Serialize, Deserialize)]
pub struct SessionData {
    sites: HashMap<Domain, String>
}

const SESSION_KEY_NAME: &'static str = "data";
const MAX_COOKIE_SIZE: usize = 4096;


impl SessionData {
    pub fn new() -> SessionData {
        SessionData{
            sites: HashMap::new(),
        }
    }

    pub fn from_session(session: &Session) -> Option<SessionData> {
        session.get(SESSION_KEY_NAME)
            .unwrap_or(None)
    }

    pub fn update_session(&self, session: &Session) {
        session.set(SESSION_KEY_NAME, self);
    }

    pub fn add_site(&mut self, site: &Site, site_config: &site::Config, key: &str) -> Result<(), Error> {
        let limits = site.quota.limits(site_config);

        util::ensure(self.sites.len() < limits.max_sites, Error::QuotaMaxSites())?;
        util::ensure(self.estimated_cookie_size() < MAX_COOKIE_SIZE, Error::SessionDataTooLarge())?;

        self.sites.insert(site.domain.clone(), key.to_string());

        Ok(())
    }

    pub fn get_site_key(&self, domain: &Domain) -> Option<String> {
        self.sites.get(domain)
            .map(|s| s.to_string())
    }

    // Estimate encrypted cookie size
    fn estimated_cookie_size(&self) -> usize {
        serde_json::to_string(self)
            .unwrap_or(String::new())
            .len() * 2
    }
}

pub enum Error {
    QuotaMaxSites(),
    SessionDataTooLarge(),
}
