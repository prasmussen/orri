use std::fmt;
use serde::{Deserialize, Serialize};
use crate::orri::util;
use crate::orri::encryption_key::EncryptionKey;
use argonautica::{self, Hasher, Verifier};


#[derive(Clone, Serialize, Deserialize)]
pub struct SiteKey(String);


#[derive(Debug)]
pub enum Error {
    TooShort(),
    TooLong(),
    HashError(argonautica::Error),
}


#[derive(Debug)]
pub enum VerifyError {
    HashError(argonautica::Error),
}

impl fmt::Display for SiteKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SiteKey {
    pub fn verify(&self, key: &str, secret: &EncryptionKey) -> Result<bool, VerifyError> {
        let is_valid = Verifier::default()
            .with_hash(&self.0)
            .with_password(key)
            .with_secret_key(&secret.to_string())
            .verify()
            .map_err(VerifyError::HashError)?;

        Ok(is_valid)
    }
}

pub fn from_str(key: &str, secret: &EncryptionKey) -> Result<SiteKey, Error> {
    util::ensure(key.len() > 19, Error::TooShort())?;
    util::ensure(key.len() < 100, Error::TooLong())?;

    // TODO: get options as arguments
    let hash = Hasher::default()
        .configure_iterations(1)
        .with_password(key)
        .with_secret_key(&secret.to_string())
        .hash()
        .map_err(Error::HashError)?;

    Ok(SiteKey(hash))
}
