use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;


pub struct Page {
    pub head: Head,
    pub body: Vec<Html>,
}

impl Page {
    pub fn to_html(self) -> Html {
        html::node("html",
            &[attrs::attribute("lang", "en")],
            &[
                html::node("head", &[], &self.head.to_html()),
                html::node("body", &[], &self.body),
            ]
        )
    }
}


pub struct Head {
    pub title: String,
    pub elements: Vec<Html>,
}


impl Head {
    pub fn to_html(self) -> Vec<Html> {
        let common = vec![
            html::node_no_end("meta", &[attrs::attribute("charset", "utf-8")]),
            html::node_no_end("meta", &[attrs::attribute("http-equiv", "X-UA-Compatible"), attrs::attribute("content", "IE=edge")]),
            html::node_no_end("meta", &[attrs::attribute("name", "viewport"), attrs::attribute("content", "width=device-width, initial-scale=1")]),
            html::node_no_end("meta", &[attrs::attribute("name", "description"), attrs::attribute("content", "Create websites, no account required")]),
            html::node("title", &[], &[html::text(&self.title)]),
        ];

        vec![
            common,
            milligram_styles(),
            self.elements,
        ].concat()
    }
}


fn milligram_styles() -> Vec<Html> {
    vec![
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://fonts.googleapis.com/css?family=Roboto:300,300italic,700,700italic"),
        ]),
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/normalize/5.0.0/normalize.css"),
        ]),
        html::node_no_end("link", &[
            attrs::attribute("rel", "stylesheet"),
            attrs::attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/milligram/1.3.0/milligram.css"),
        ]),
    ]
}
