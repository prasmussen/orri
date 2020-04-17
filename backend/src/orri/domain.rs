use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::orri::util;


#[derive(Clone, Serialize, Deserialize)]
pub struct Domain(String);


#[derive(Debug)]
pub enum Error {
    TooLong(),
    NotAlphanumeric(),
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

        let parts_are_alphanumeric = reversed_parts
            .iter()
            .map(|part| part.chars().all(|c| c.is_ascii_alphanumeric()))
            .all(std::convert::identity);

        util::ensure(parts_are_alphanumeric, Error::NotAlphanumeric())?;

        match *reversed_parts.as_slice() {
            [] =>
                Err(Error::EmptyDomainValue()),

            [tld] =>
                Err(Error::MissingSecondLevelDomain()),

            [tld, sld] =>
                Err(Error::MissingSubDomain()),

            [tld, sld, subdomain] =>
                Ok(Domain(host)),

            _ =>
                Err(Error::OnlyOneSubdomainAllowed()),
        }
    }

}
