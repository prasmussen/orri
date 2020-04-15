use std::fmt;
use htmlescape;
use crate::orri::slowhtml::attributes::Attribute;

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


pub fn html(attributes: &[Attribute], children: &[Html]) -> Html {
    node("html", attributes, children)
}

pub fn head(attributes: &[Attribute], children: &[Html]) -> Html {
    node("head", attributes, children)
}

pub fn body(attributes: &[Attribute], children: &[Html]) -> Html {
    node("body", attributes, children)
}

pub fn title(attributes: &[Attribute], children: &[Html]) -> Html {
    node("title", attributes, children)
}

pub fn div(attributes: &[Attribute], children: &[Html]) -> Html {
    node("div", attributes, children)
}

pub fn a(attributes: &[Attribute], children: &[Html]) -> Html {
    node("a", attributes, children)
}

pub fn form(attributes: &[Attribute], children: &[Html]) -> Html {
    node("form", attributes, children)
}

pub fn label(attributes: &[Attribute], children: &[Html]) -> Html {
    node("label", attributes, children)
}

pub fn button(attributes: &[Attribute], children: &[Html]) -> Html {
    node("button", attributes, children)
}

pub fn table(attributes: &[Attribute], children: &[Html]) -> Html {
    node("table", attributes, children)
}

pub fn thead(attributes: &[Attribute], children: &[Html]) -> Html {
    node("thead", attributes, children)
}

pub fn tbody(attributes: &[Attribute], children: &[Html]) -> Html {
    node("tbody", attributes, children)
}

pub fn th(attributes: &[Attribute], children: &[Html]) -> Html {
    node("th", attributes, children)
}

pub fn tr(attributes: &[Attribute], children: &[Html]) -> Html {
    node("tr", attributes, children)
}

pub fn td(attributes: &[Attribute], children: &[Html]) -> Html {
    node("td", attributes, children)
}

pub fn input(attributes: &[Attribute]) -> Html {
    node_no_end("input", attributes)
}

pub fn meta(attributes: &[Attribute]) -> Html {
    node_no_end("meta", attributes)
}

pub fn link(attributes: &[Attribute]) -> Html {
    node_no_end("link", attributes)
}

pub fn script(attributes: &[Attribute], children: &[Html]) -> Html {
    node("script", attributes, children)
}
