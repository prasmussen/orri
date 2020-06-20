mod orri;

use std::io;
use std::process;
use actix_web::{web, App, HttpServer, guard};
use actix_files::Files;
use actix_session::CookieSession;
use actix_http::cookie::SameSite;
use orri::app_state::{self, AppState};
use orri::http::index;
use orri::site::http::api as site_api;
use orri::site::http as site_http;
use orri::site_key;
use orri::site;
use orri::route::Route;
use orri::util;
use orri::environment::{self, Environment};
use log;




fn app_domain_routes(config: &mut web::ServiceConfig, state: &AppState, host: &'static str) {
    let cookie_session = CookieSession::private(state.config.encryption_key.as_bytes())
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(state.config.cookie.secure)
        .max_age(state.config.cookie.max_age);

    config.service(
        web::scope("/")
            .guard(guard::Header("Host", host))
            .wrap(cookie_session)

            // User facing routes
            .route("", web::get().to(index::handler))
            .route(&Route::NewSite().to_string(), web::get().to(site_http::new_site::handler))
            .route(&Route::MySites().to_string(), web::get().to(site_http::my_sites::handler))
            .route(&Route::FindSite().to_string(), web::get().to(site_http::find_site::handler))
            .route(&Route::SiteExist("{domain}".to_string()).to_string(), web::head().to(site_http::site_exist::handler))
            .route(&Route::ManageSite("{domain}".to_string()).to_string(), web::get().to(site_http::manage_site::handler))
            .route(&Route::AddRoute("{domain}".to_string()).to_string(), web::get().to(site_http::add_route::handler))
            .route(&Route::EditRoute("{domain}".to_string(), None).to_string(), web::get().to(site_http::edit_route::handler))

            // Json routes
            .route(
                &Route::NewSiteJson().to_string(),
                web::method(Route::NewSiteJson().request_method()).to(site_api::new_site::handler)
            )
            .route(
                &Route::AddRouteJson().to_string(),
                web::method(Route::AddRouteJson().request_method()).to(site_api::add_route::handler)
            )
            .route(
                &Route::EditRouteJson().to_string(),
                web::method(Route::EditRouteJson().request_method()).to(site_api::edit_route::handler)
            )
            .route(
                &Route::DeleteRouteJson().to_string(),
                web::method(Route::DeleteRouteJson().request_method()).to(site_api::remove_route::handler)
            )
            .route(
                &Route::DeleteSiteJson().to_string(),
                web::method(Route::DeleteSiteJson().request_method()).to(site_api::remove_site::handler)
            )

            // Static files
            .service(Files::new("/static", state.config.server.static_path()))
    );
}

fn sites_domain_routes(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/")
            .route("", web::get().to(site_http::view_site::handler))
            .route("{tail:.*}", web::get().to(site_http::view_site::handler))
    );
}


fn build_server_config(env: &Environment) -> Result<app_state::ServerConfig, environment::Error> {
    let app_domain = environment::lookup(env, "SERVER_APP_DOMAIN")?;
    let sites_domain = environment::lookup(env, "SERVER_SITES_DOMAIN")?;
    let protocol = environment::lookup(env, "SERVER_PROTOCOL")?;
    let listen_addr = environment::lookup(env, "SERVER_LISTEN_ADDR")?;
    let listen_port = environment::lookup(env, "SERVER_LISTEN_PORT")?;
    let frontend_root = environment::lookup(env, "SERVER_FRONTEND_ROOT")?;
    let sites_root = environment::lookup(env, "SERVER_SITES_ROOT")?;

    Ok(app_state::ServerConfig{
        app_domain,
        sites_domain,
        protocol,
        listen_addr,
        listen_port,
        frontend_root,
        sites_root,
    })
}

fn build_site_key_config(env: &Environment) -> Result<site_key::Config, environment::Error> {
    let min_length = environment::lookup(env, "SITE_KEY_MIN_LENGTH")?;
    let max_length = environment::lookup(env, "SITE_KEY_MAX_LENGTH")?;
    let hash_iterations = environment::lookup(env, "SITE_KEY_HASH_ITERATIONS")?;
    let hash_memory_size = environment::lookup(env, "SITE_KEY_HASH_MEMORY_SIZE")?;

    Ok(site_key::Config{
        min_length,
        max_length,
        hash_iterations,
        hash_memory_size,
    })
}

fn build_site_quota_limits_nano(env: &Environment) -> Result<site::QuotaLimits, environment::Error> {
    let max_size = environment::lookup(env, "SITE_QUOTA_NANO_MAX_SIZE")?;
    let max_routes = environment::lookup(env, "SITE_QUOTA_NANO_MAX_ROUTES")?;
    let max_sites = environment::lookup(env, "SITE_QUOTA_NANO_MAX_SITES")?;

    Ok(site::QuotaLimits{
        max_size,
        max_routes,
        max_sites,
    })
}

fn build_cookie_config(env: &Environment) -> Result<app_state::CookieConfig, environment::Error> {
    let secure = environment::lookup(env, "COOKIE_SECURE")?;
    let max_age = environment::lookup(env, "COOKIE_MAX_AGE")?;

    Ok(app_state::CookieConfig{
        secure,
        max_age,
    })
}

fn build_config(env: &Environment) -> Result<app_state::Config, environment::Error> {
    let encryption_key = environment::lookup(env, "ENCRYPTION_KEY")?;
    let server = build_server_config(env)?;
    let cookie = build_cookie_config(env)?;
    let site_key = build_site_key_config(env)?;
    let quota_nano = build_site_quota_limits_nano(env)?;

    Ok(app_state::Config{
        encryption_key,
        server,
        cookie,
        site_key,
        site: site::Config{
            quota_nano,
        }
    })
}


fn prepare_app_state() -> app_state::AppState {
    let env = environment::get_environment();

    match build_config(&env) {
        Ok(config) => {
            app_state::AppState{
                config
            }
        },

        Err(err) => {
            log::error!("Failed to build config: {:?}", err);
            process::exit(1)
        },
    }
}

#[actix_rt::main]
async fn main() -> Result<(), io::Error> {
    env_logger::init();

    let state = prepare_app_state();

    // TODO: This is probably ok, but is it possible to have a 'static String in the config?
    let domain = util::to_static_str(state.config.server.app_domain_with_port());
    let listen_addr = &state.config.server.listen_addr_with_port();

    log::info!("Starting server on {}", listen_addr);

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 10))
            .configure(|cfg| app_domain_routes(cfg, &state, domain))
            .configure(sites_domain_routes)
    })
    .bind(listen_addr)?
    .run()
    .await
}
