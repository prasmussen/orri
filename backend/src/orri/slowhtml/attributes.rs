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

#[allow(dead_code)]
pub fn attribute(name: &str, value: &str) -> Attribute {
    Attribute{
        name: htmlescape::encode_attribute(name),
        value: Some(htmlescape::encode_minimal(value)),
    }
}

pub fn attribute_trusted_name(name: &'static str, value: &str) -> Attribute {
    Attribute{
        name: name.to_string(),
        value: Some(htmlescape::encode_minimal(value)),
    }
}

#[allow(dead_code)]
pub fn bool_attribute(name: &str) -> Attribute {
    Attribute{
        name: htmlescape::encode_attribute(name),
        value: None,
    }
}

pub fn bool_attribute_trusted(name: &'static str) -> Attribute {
    Attribute{
        name: name.to_string(),
        value: None,
    }
}


pub fn lang(value: &str) -> Attribute {
    attribute_trusted_name("lang", value)
}

pub fn charset(value: &str) -> Attribute {
    attribute_trusted_name("charset", value)
}

pub fn http_equiv(value: &str) -> Attribute {
    attribute_trusted_name("http-equiv", value)
}

pub fn content(value: &str) -> Attribute {
    attribute_trusted_name("content", value)
}

pub fn name(value: &str) -> Attribute {
    attribute_trusted_name("name", value)
}

pub fn rel(value: &str) -> Attribute {
    attribute_trusted_name("rel", value)
}

pub fn id(value: &str) -> Attribute {
    attribute_trusted_name("id", value)
}

pub fn class(value: &str) -> Attribute {
    attribute_trusted_name("class", value)
}

pub fn href(value: &str) -> Attribute {
    attribute_trusted_name("href", value)
}

pub fn target(value: &str) -> Attribute {
    attribute_trusted_name("target", value)
}

pub fn src(value: &str) -> Attribute {
    attribute_trusted_name("src", value)
}

pub fn type_(value: &str) -> Attribute {
    attribute_trusted_name("type", value)
}

pub fn value(value: &str) -> Attribute {
    attribute_trusted_name("value", value)
}

pub fn placeholder(value: &str) -> Attribute {
    attribute_trusted_name("placeholder", value)
}

pub fn title(value: &str) -> Attribute {
    attribute_trusted_name("title", value)
}

pub fn pattern(value: &str) -> Attribute {
    attribute_trusted_name("pattern", value)
}

pub fn readonly() -> Attribute {
    bool_attribute_trusted("readonly")
}

pub fn required() -> Attribute {
    bool_attribute_trusted("required")
}

pub fn class_list(list: &[(&str, bool)]) -> Attribute {
    let classes = list
        .iter()
        .filter(|tuple| tuple.1)
        .map(|tuple| tuple.0)
        .collect::<Vec<&str>>()
        .join(" ");

    attribute_trusted_name("class", &classes)
}
