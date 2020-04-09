use std::fmt;
use htmlescape;


#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            Some(value) =>
                write!(f, "{}=\"{}\"", self.name, value),

            None =>
                write!(f, "{}", self.name),
        }
    }
}

pub fn attribute(name: &str, value: &str) -> Attribute {
    Attribute{
        name: htmlescape::encode_attribute(name),
        value: Some(htmlescape::encode_attribute(value)),
    }
}

pub fn bool_attribute(name: &str) -> Attribute {
    Attribute{
        name: htmlescape::encode_attribute(name),
        value: None,
    }
}
