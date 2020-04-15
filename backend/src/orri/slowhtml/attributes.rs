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


pub fn lang(value: &str) -> Attribute {
    attribute("lang", value)
}

pub fn charset(value: &str) -> Attribute {
    attribute("charset", value)
}

pub fn http_equiv(value: &str) -> Attribute {
    attribute("http-equiv", value)
}

pub fn content(value: &str) -> Attribute {
    attribute("content", value)
}

pub fn name(value: &str) -> Attribute {
    attribute("name", value)
}

pub fn rel(value: &str) -> Attribute {
    attribute("rel", value)
}

pub fn id(value: &str) -> Attribute {
    attribute("id", value)
}

pub fn class(value: &str) -> Attribute {
    attribute("class", value)
}

pub fn href(value: &str) -> Attribute {
    attribute("href", value)
}

pub fn src(value: &str) -> Attribute {
    attribute("src", value)
}

pub fn type_(value: &str) -> Attribute {
    attribute("type", value)
}

pub fn value(value: &str) -> Attribute {
    attribute("value", value)
}

pub fn placeholder(value: &str) -> Attribute {
    attribute("placeholder", value)
}

pub fn title(value: &str) -> Attribute {
    attribute("title", value)
}

pub fn pattern(value: &str) -> Attribute {
    attribute("pattern", value)
}

pub fn readonly() -> Attribute {
    bool_attribute("readonly")
}

pub fn required() -> Attribute {
    bool_attribute("required")
}
