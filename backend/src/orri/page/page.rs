use std::fmt;
use crate::Route;
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


pub fn navbar(center_content: Html) -> Html {
    html::header(&[attrs::class("navbar")], &[
        html::section(&[attrs::class("navbar-section")], &[
            html::a(&[attrs::href("/"), attrs::class("btn btn-link")], &[html::text("Orri")])
        ]),
        html::section(&[attrs::class("navbar-center")], &[
            center_content
        ]),
        html::section(&[attrs::class("navbar-section")], &[
            html::a(&[attrs::href("https://github.com/prasmussen/orri"), attrs::class("btn btn-link")], &[html::text("Github")])
        ]),
    ])
}

pub struct Breadcrumb {
    name: String,
    route: Route,
}

pub fn breadcrumb(name: &str, route: Route) -> Breadcrumb {
    Breadcrumb{
        name: name.to_string(),
        route: route,
    }
}

pub fn breadcrumbs(items: &[Breadcrumb]) -> Html {
    let breadcrumb_items = items.iter()
        .map(|item| breadcrumb_item(item))
        .collect::<Vec<Html>>();

    html::ul(&[attrs::class("breadcrumb")], &breadcrumb_items)
}

fn breadcrumb_item(item: &Breadcrumb) -> Html {
    html::li(&[attrs::class("breadcrumb-item")], &[
        html::a(&[attrs::href(&item.route.to_string())], &[html::text(&item.name)])
    ])
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
            html::link(&[attrs::rel("stylesheet"), attrs::href("/static/vendor/spectre.min.css")]),
            html::link(&[attrs::rel("stylesheet"), attrs::href("/static/orri.css")]),
        ];

        vec![
            common,
            self.elements,
        ].concat()
    }
}


pub fn error_alert() -> Html {
    html::div(&[attrs::class("toast toast-error display-none"), attrs::id("alert-error")], &[])
}
