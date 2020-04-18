use std::fmt;
use crate::orri::slowhtml::html::Html;
use crate::orri::slowhtml::html;
use crate::orri::slowhtml::attributes as attrs;


pub struct Page {
    pub head: Head,
    pub body: Vec<Html>,
}

impl Page {
    pub fn to_html(self) -> Vec<Html> {
        vec![
            html::doctype_html(),
            html::html(
                &[attrs::lang("en")],
                &[
                    html::head(&[], &self.head.to_html()),
                    html::body(&[], &self.body),
                ]
            ),
        ]
    }

    pub fn to_string(self) -> String {
        self.to_html()
            .iter()
            .map(|html| html.to_string())
            .collect::<Vec<String>>()
            .join("")

    }
}


pub struct Head {
    pub title: String,
    pub elements: Vec<Html>,
}


impl Head {
    pub fn to_html(self) -> Vec<Html> {
        let common = vec![
            html::meta(&[attrs::charset("utf-8")]),
            html::meta(&[attrs::http_equiv("X-UA-Compatible"), attrs::content("IE=edge")]),
            html::meta(&[attrs::name("viewport"), attrs::content("width=device-width, initial-scale=1")]),
            html::meta(&[attrs::name("description"), attrs::content("Create websites, no account required")]),
            html::title(&[], &[html::text(&self.title)]),
            html::link(&[attrs::rel("stylesheet"), attrs::href("/static/orri.css")]),
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
        html::link(&[
            attrs::rel("stylesheet"),
            attrs::href("https://fonts.googleapis.com/css?family=Roboto:300,300italic,700,700italic"),
        ]),
        html::link(&[
            attrs::rel("stylesheet"),
            attrs::href("/static/vendor/normalize.min.css"),
        ]),
        html::link(&[
            attrs::rel("stylesheet"),
            attrs::href("/static/vendor/milligram.min.css"),
        ]),
    ]
}
