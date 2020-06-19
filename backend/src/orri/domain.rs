use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::orri::util;


#[derive(Clone, Serialize, Deserialize, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Domain(String);


#[derive(Debug)]
pub enum Error {
    TooLong(),
    SubdomainTooShort(),
    InvalidChar(),
    InvalidHyphenPosition(),
    EmptyDomainValue(),
    MissingSecondLevelDomain(),
    MissingSubDomain(),
    OnlyOneSubdomainAllowed(),
}


impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Domain {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        util::ensure(s.len() < 100, Error::TooLong())?;

        let host = s.to_lowercase();

        let reversed_parts = host.split(".")
            .collect::<Vec<&str>>()
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<&str>>();

        let parts_has_allowed_chars = reversed_parts
            .iter()
            .map(|part| part.chars().all(is_allowed_char))
            .all(std::convert::identity);

        util::ensure(parts_has_allowed_chars, Error::InvalidChar())?;

        let parts_has_allowed_hyphen_position = reversed_parts
            .iter()
            .map(|part| part.starts_with("-") == false && part.ends_with("-") == false)
            .all(std::convert::identity);

        util::ensure(parts_has_allowed_hyphen_position, Error::InvalidHyphenPosition())?;

        match *reversed_parts.as_slice() {
            [] =>
                Err(Error::EmptyDomainValue()),

            [_tld] =>
                Err(Error::MissingSecondLevelDomain()),

            [_tld, _sld] =>
                Err(Error::MissingSubDomain()),

            [_tld, _sld, subdomain] => {
                util::ensure(subdomain.len() >= 5, Error::SubdomainTooShort())?;
                Ok(Domain(host))
            },

            _ =>
                Err(Error::OnlyOneSubdomainAllowed()),
        }
    }
}

fn is_allowed_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-'
}
