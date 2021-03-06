use std::fmt;
use actix_web::http::Method;


pub enum Route {
    // User facing routes
    Index(),
    NewSite(),
    MySites(),
    FindSite(),
    SiteExist(String),
    ManageSite(String),
    AddRoute(String),
    EditRoute(String, Option<String>),

    // Json routes
    NewSiteJson(),
    AddRouteJson(),
    EditRouteJson(),
    DeleteRouteJson(),
    DeleteSiteJson(),
}

impl Route {
    pub fn request_method(&self) -> Method {
        match self {
            Route::Index() =>
                Method::GET,

            Route::NewSite() =>
                Method::GET,

            Route::MySites() =>
                Method::GET,

            Route::FindSite() =>
                Method::GET,

            Route::SiteExist(_) =>
                Method::HEAD,

            Route::ManageSite(_) =>
                Method::GET,

            Route::AddRoute(_) =>
                Method::GET,

            Route::EditRoute(_, _) =>
                Method::GET,

            Route::NewSiteJson() =>
                Method::POST,

            Route::AddRouteJson() =>
                Method::POST,

            Route::EditRouteJson() =>
                Method::PUT,

            Route::DeleteRouteJson() =>
                Method::DELETE,

            Route::DeleteSiteJson() =>
                Method::DELETE,
        }
    }
}

impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Route::Index() =>
                write!(f, "/"),

            Route::NewSite() =>
                write!(f, "/new"),

            Route::MySites() =>
                write!(f, "/my-sites"),

            Route::FindSite() =>
                write!(f, "/find-site"),

            Route::ManageSite(domain) =>
                write!(f, "/sites/{}", domain),

            Route::SiteExist(domain) =>
                write!(f, "/sites/{}", domain),

            Route::AddRoute(domain) =>
                write!(f, "/sites/{}/routes/add", domain),

            Route::EditRoute(domain, route) =>
                match route {
                    Some(path) =>
                        write!(f, "/sites/{}/routes/edit?path={}", domain, path),

                    None =>
                        write!(f, "/sites/{}/routes/edit", domain),
                },

            Route::NewSiteJson() =>
                write!(f, "/json/sites"),

            Route::AddRouteJson() =>
                write!(f, "/json/sites/add-route"),

            Route::EditRouteJson() =>
                write!(f, "/json/sites/edit-route"),

            Route::DeleteRouteJson() =>
                write!(f, "/json/sites/delete-route"),

            Route::DeleteSiteJson() =>
                write!(f, "/json/sites"),
        }
    }
}
