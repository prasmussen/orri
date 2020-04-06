use crate::zait::util;


pub struct Domain(String);


#[derive(Debug)]
pub enum ParseDomainError {
    NotAlphabetic(),
    EmptyDomainValue(),
    MissingSecondLevelDomain(),
    MissingSubDomain(),
    OnlyOneSubdomainAllowed(),
}


impl Domain {
    pub fn from_str(s: &str) -> Result<Domain, ParseDomainError> {
        let host = s.to_lowercase();

        let reversed_parts = host.split(".")
            .collect::<Vec<&str>>()
            .iter()
            .cloned()
            .rev()
            .collect::<Vec<&str>>();

        let parts_are_alphabetic = reversed_parts
            .iter()
            .map(|part| part.chars().all(char::is_alphabetic))
            .all(std::convert::identity);

        util::err_if_false(parts_are_alphabetic, ParseDomainError::NotAlphabetic())?;

        // TODO: add setting subdomain setting: OnlyOne | OneOrMore | NoneOrOne | NoLimit
        match *reversed_parts.as_slice() {
            [] =>
                Err(ParseDomainError::EmptyDomainValue()),

            [tld] =>
                Err(ParseDomainError::MissingSecondLevelDomain()),

            [tld, sld] =>
                Err(ParseDomainError::MissingSubDomain()),

            [tld, sld, subdomain] =>
                Ok(Domain(host)),

            _ =>
                Err(ParseDomainError::OnlyOneSubdomainAllowed()),
        }
    }

    pub fn to_string(self) -> String {
        self.0
    }
}