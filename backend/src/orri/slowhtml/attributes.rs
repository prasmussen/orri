use std::fmt;
use htmlescape;


#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}=\"{}\"", self.name, self.value)
    }
}


pub fn attribute(name: &str, value: &str) -> Attribute {
    Attribute{
        name: htmlescape::encode_attribute(name),
        value: htmlescape::encode_attribute(value),
    }
}


