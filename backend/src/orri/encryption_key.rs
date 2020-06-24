use std::fmt;
use std::str::FromStr;
use rand::{Rng};
use rand::distributions;
use crate::orri::util;



#[derive(Clone, Debug)]
pub struct EncryptionKey(String);


impl EncryptionKey {
    #[allow(dead_code)]
    pub fn new() -> EncryptionKey {
        let s = random_string(32);

        EncryptionKey(s)
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl fmt::Display for EncryptionKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl FromStr for EncryptionKey {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        let string = s.to_string();

        util::ensure(string.len() == 32, Error::InvalidLength())?;

        Ok(EncryptionKey(string))
    }
}


#[derive(Debug)]
pub enum Error {
    InvalidLength()
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid key length, key must be exacly 32 chars")
    }
}


pub fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(len)
        .collect()
}
