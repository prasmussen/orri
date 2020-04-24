use std::fmt;


pub enum Route {
    // User facing routes
    NewSite(),
    ManageSite(String),
    AddRoute(String),
    EditRoute(String),

    // Json routes
    NewSiteJson(),
    AddRouteJson(),
    UpdateRouteJson(),
    DeleteRouteJson(),
}


impl fmt::Display for Route {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Route::NewSite() =>
                write!(f, "/new"),

            Route::ManageSite(domain) =>
                write!(f, "/sites/{}", domain),

            Route::AddRoute(domain) =>
                write!(f, "/sites/{}/routes/add", domain),

            Route::EditRoute(domain) =>
                write!(f, "/sites/{}/routes/edit", domain),

            Route::NewSiteJson() =>
                write!(f, "/json/sites"),

            Route::AddRouteJson() =>
                write!(f, "/json/sites/add-route"),

            Route::UpdateRouteJson() =>
                write!(f, "/json/sites/update-route"),

            Route::DeleteRouteJson() =>
                write!(f, "/json/sites/delete-route"),
        }
    }
}