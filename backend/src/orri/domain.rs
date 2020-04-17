use std::fmt;
use serde::{Deserialize, Serialize};
use crate::orri::util;


#[derive(Clone, Serialize, Deserialize)]
pub struct Domain(String);


#[derive(Debug)]
pub enum Error {
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

impl Domain {
    pub fn from_str(s: &str) -> Result<Domain, Error> {
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

        // TODO: add setting subdomain setting: OnlyOne | OneOrMore | NoneOrOne | NoLimit
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
