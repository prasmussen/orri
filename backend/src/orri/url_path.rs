use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use crate::orri::util;


#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Serialize, Deserialize)]
pub struct UrlPath(String);


#[derive(Debug)]
pub enum Error {
    MustStartWithSlash(),
    TooLong(),
    ContainsDisallowedChars(),
    ContainsDoubleDot(),
}

impl UrlPath {
    pub fn root() -> UrlPath {
        UrlPath("/".to_string())
    }
}


impl fmt::Display for UrlPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UrlPath {
    type Err = Error;

    fn from_str(path: &str) -> Result<Self, Error> {
        util::ensure(path.len() < 100, Error::TooLong())?;
        util::ensure(path.starts_with('/'), Error::MustStartWithSlash())?;

        let parts = path.split('/').collect::<Vec<&str>>();

        let parts_has_allowed_chars = parts
            .iter()
            .map(|part| part.chars().all(is_allowed_char))
            .all(std::convert::identity);

        util::ensure(parts_has_allowed_chars, Error::ContainsDisallowedChars())?;
        util::ensure(!path.contains(".."), Error::ContainsDoubleDot())?;

        Ok(UrlPath(path.to_string()))
    }
}


fn is_allowed_char(c: char) -> bool {
    let allowed_special_chars: Vec<char> = vec![
        '.',
        '-',
        '_',
    ];

    c.is_ascii_alphanumeric() || allowed_special_chars.contains(&c)
}
