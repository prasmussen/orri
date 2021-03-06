use std::fmt;
use serde::{Deserialize, Serialize};
use crate::orri::util;
use crate::orri::encryption_key;
use argon2;


#[derive(Clone, Serialize, Deserialize)]
pub struct SiteKey(String);


#[derive(Clone, Debug)]
pub struct Config {
    pub min_length: usize,
    pub max_length: usize,
    pub hash_iterations: u32,
    pub hash_memory_size: u32,
}


#[derive(Debug)]
pub enum Error {
    TooShort(),
    TooLong(),
    HashError(argon2::Error),
}


#[derive(Debug)]
pub enum VerifyError {
    HashError(argon2::Error),
}

impl fmt::Display for SiteKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SiteKey {
    pub fn verify(&self, key: &str) -> Result<bool, VerifyError> {
        argon2::verify_encoded(&self.0, key.as_bytes())
            .map_err(VerifyError::HashError)
    }
}

pub fn from_str(config: &Config, key: &str) -> Result<SiteKey, Error> {
    util::ensure(key.len() >= config.min_length, Error::TooShort())?;
    util::ensure(key.len() <= config.max_length, Error::TooLong())?;

    let salt = encryption_key::random_string(16);
    let hash_config = argon2::Config{
        mem_cost: config.hash_memory_size,
        time_cost: config.hash_iterations,
        ..argon2::Config::default()
    };

    argon2::hash_encoded(key.as_bytes(), salt.as_bytes(), &hash_config)
        .map(SiteKey)
        .map_err(Error::HashError)
}
