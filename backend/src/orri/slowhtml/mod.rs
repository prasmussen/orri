use std::fmt;
use htmlescape;
use crate::orri::slowhtml::attributes::Attribute;

pub mod attributes;

#[derive(Clone, Debug)]
pub enum Html {
    Tag(HtmlTag),
    Text(String),
}

impl fmt::Display for Html {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Html::Tag(tag) =>
                tag.fmt(f),

            Html::Text(text) =>
                text.fmt(f),
        }
    }
}


#[derive(Clone, Debug)]
struct HtmlTag {
    name: String,
    attributes: Vec<Attribute>,
    children: Vec<Html>,
    has_end_tag: bool,
}

impl fmt::Display for HtmlTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let attributes = self.attributes
            .iter()
            .map(|attr| attr.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let children = self.children
            .iter()
            .map(|child| child.to_string())
            .collect::<Vec<String>>()
            .join("");

        let attributesSpace = if attributes.is_empty() { "" } else { " " };

        if self.has_end_tag {
            write!(f, "<{}{}{}>{}</{}>", &self.name, attributesSpace, attributes, children, &self.name)
        } else {
            write!(f, "<{}{}{}>", &self.name, attributesSpace, attributes)
        }
    }
}


pub fn text(text: &str) -> Html {
    Html::Text(htmlescape::encode_minimal(text))
}

pub fn node(name: &str, attributes: &[Attribute], children: &[Html]) -> Html {
    Html::Tag(HtmlTag{
        name: htmlescape::encode_minimal(name),
        attributes: attributes.to_vec(),
        children: children.to_vec(),
        has_end_tag: true,
    })
}


pub fn node_no_end(name: &str, attributes: &[Attribute]) -> Html {
    Html::Tag(HtmlTag{
        name: htmlescape::encode_minimal(name),
        attributes: attributes.to_vec(),
        children: vec![],
        has_end_tag: false,
    })
}
