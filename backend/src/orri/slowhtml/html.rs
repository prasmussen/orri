use std::fmt;
use htmlescape;
use crate::orri::slowhtml::attributes::{self, Attribute};

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
    Html::Text(htmlescape::encode_attribute(text))
}

pub fn node(name: &str, attributes: &[Attribute], children: &[Html]) -> Html {
    Html::Tag(HtmlTag{
        name: htmlescape::encode_attribute(name),
        attributes: attributes.to_vec(),
        children: children.to_vec(),
        has_end_tag: true,
    })
}

pub fn node_trusted_name(name: &'static str, attributes: &[Attribute], children: &[Html]) -> Html {
    Html::Tag(HtmlTag{
        name: name.to_string(),
        attributes: attributes.to_vec(),
        children: children.to_vec(),
        has_end_tag: true,
    })
}

pub fn node_no_end(name: &str, attributes: &[Attribute]) -> Html {
    Html::Tag(HtmlTag{
        name: htmlescape::encode_attribute(name),
        attributes: attributes.to_vec(),
        children: vec![],
        has_end_tag: false,
    })
}

pub fn node_no_end_trusted_name(name: &'static str, attributes: &[Attribute]) -> Html {
    Html::Tag(HtmlTag{
        name: name.to_string(),
        attributes: attributes.to_vec(),
        children: vec![],
        has_end_tag: false,
    })
}


pub fn empty() -> Html {
    Html::Text("".to_string())
}


pub fn conditional(condition: bool, html: Html) -> Html{
    if condition {
        html
    } else {
        empty()
    }
}

pub fn doctype_html() -> Html {
    node_no_end_trusted_name("!DOCTYPE", &[
        attributes::bool_attribute_trusted("html"),
    ])
}

pub fn html(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("html", attributes, children)
}

pub fn head(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("head", attributes, children)
}

pub fn body(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("body", attributes, children)
}

pub fn title(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("title", attributes, children)
}

pub fn h1(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h1", attributes, children)
}

pub fn h2(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h2", attributes, children)
}

pub fn h3(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h3", attributes, children)
}

pub fn h4(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h4", attributes, children)
}

pub fn h5(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h5", attributes, children)
}

pub fn h6(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("h6", attributes, children)
}

pub fn div(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("div", attributes, children)
}

pub fn a(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("a", attributes, children)
}

pub fn form(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("form", attributes, children)
}

pub fn label(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("label", attributes, children)
}

pub fn button(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("button", attributes, children)
}

pub fn table(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("table", attributes, children)
}

pub fn thead(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("thead", attributes, children)
}

pub fn tbody(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("tbody", attributes, children)
}

pub fn th(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("th", attributes, children)
}

pub fn tr(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("tr", attributes, children)
}

pub fn td(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("td", attributes, children)
}

pub fn ul(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("ul", attributes, children)
}

pub fn li(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("li", attributes, children)
}

pub fn strong(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("strong", attributes, children)
}

pub fn em(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("em", attributes, children)
}

pub fn input(attributes: &[Attribute]) -> Html {
    node_no_end_trusted_name("input", attributes)
}

pub fn meta(attributes: &[Attribute]) -> Html {
    node_no_end_trusted_name("meta", attributes)
}

pub fn link(attributes: &[Attribute]) -> Html {
    node_no_end_trusted_name("link", attributes)
}

pub fn script(attributes: &[Attribute], children: &[Html]) -> Html {
    node_trusted_name("script", attributes, children)
}
