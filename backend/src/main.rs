use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_files::{Files, NamedFile};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use ring::digest;
use hex;
use rand::{Rng};
use rand::distributions;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Write};
use std::fmt;
use tempfile::NamedTempFile;
use std::time::{SystemTime, UNIX_EPOCH};


#[derive(Clone)]
struct Config {
    server: ServerConfig,
}

#[derive(Clone)]
struct ServerConfig {
    main_domain: String,
    frontend_root: String,
    sites_root: String,
}

impl ServerConfig {
    pub fn static_path(&self) -> String {
        // TODO: use path type
        format!("{}/static", self.frontend_root)
    }

    pub fn frontend_file_path(&self, name: &str) -> String {
        // TODO: use path type
        format!("{}/{}", self.frontend_root, name)
    }
}


enum Host {
    Main(),
    Domain(String),
}


impl Host {
    pub fn from_req(req: &HttpRequest, main_domain: &str) -> Option<Host> {
        let value = req.headers().get("host")?;
        let host = value.to_str().ok()?;

        // TODO: make sure host only contains allowed characters (add newtype?)
        if host == main_domain {
            Some(Host::Main())

        } else {
            Some(Host::Domain(host.to_string()))
        }
    }
}


async fn index(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    println!("{:?}", req);

    let host = Host::from_req(&req, &state.config.server.main_domain)
        .unwrap_or(Host::Main());

    match host {
        Host::Main() =>
            index_main(req, state),

        Host::Domain(domain) =>
            index_domain(req, state, &domain),
    }
}


fn index_main(req: HttpRequest, state: web::Data<AppState>) -> Result<NamedFile, io::Error> {
    NamedFile::open(state.config.server.frontend_file_path("index.html"))
}

fn index_domain(req: HttpRequest, state: web::Data<AppState>, domain: &str) -> Result<NamedFile, io::Error> {
    NamedFile::open(state.config.server.frontend_file_path("index.html"))
}

async fn new_handler(req: HttpRequest, state: web::Data<AppState>) -> impl Responder {
    NamedFile::open(state.config.server.frontend_file_path("new.html"))
}


#[derive(Deserialize)]
struct NewSiteRequest {
    domain: String,
    source: String,
}


#[derive(Serialize)]
struct NewSiteResponse {
    key: String,
}


async fn new_site_handler(req: HttpRequest, state: web::Data<AppState>, payload: web::Json<NewSiteRequest>) -> impl Responder {

    match create_site(&state.config.server.sites_root, &payload) {
        Ok(site_config) =>
            HttpResponse::Ok()
                .json(NewSiteResponse{
                    key: site_config.key,
                }),

        Err(err) =>
            handle_new_site_error(&err),
    }
}


#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn error_response(err: &str) -> ErrorResponse {
    ErrorResponse {
        error: err.to_string(),
    }
}


fn handle_new_site_error(err: &CreateSiteError) -> HttpResponse {
    match err {
        CreateSiteError::SiteAlreadyExist() => {
            HttpResponse::Conflict()
                .json(error_response("Site already exist"))
        },

        CreateSiteError::FailedToCreateDomainDir(err) => {
            println!("Failed to create domain: {}", err);
            HttpResponse::InternalServerError()
                .json(error_response("Failed to create domain dir"))
        },

        CreateSiteError::FailedToWriteSourceFile(err) => {
            println!("Failed to write source file: {}", err);
            HttpResponse::InternalServerError()
                .json(error_response("Failed to write source file"))
        }

        CreateSiteError::FailedToSaveConfig(err) => {
            println!("Failed to save config: {}", err);
            HttpResponse::InternalServerError()
                .json(error_response("Failed to save config"))
        }
    }

}

enum CreateSiteError {
    SiteAlreadyExist(),
    FailedToCreateDomainDir(io::Error),
    FailedToWriteSourceFile(WriteFileError),
    FailedToSaveConfig(WriteConfigError),
}


// TODO: Two users can create the same site at the same time
fn create_site(root_path: &str, payload: &NewSiteRequest) -> Result<SiteConfig, CreateSiteError> {
    let domain_path = Path::new(root_path)
        .join(PathBuf::from(&payload.domain));

    let data_path = domain_path.join(PathBuf::from("data"));

    fs::create_dir_all(&data_path)
        .map_err(CreateSiteError::FailedToCreateDomainDir)?;

    let config_path = domain_path.join(PathBuf::from("config.json"));
    let source_hash = sha256(&payload.source);
    let source_path = data_path.join(PathBuf::from(&source_hash));

    err_if_false(config_path.exists() == false, CreateSiteError::SiteAlreadyExist())?;

    write_file_atomic(&source_path, &payload.source)
        .map_err(CreateSiteError::FailedToWriteSourceFile)?;

    let key = random_string(32);

    let timestamp = unix_timestamp();


    let route_info = RouteInfo{
        source_hash: source_hash,
        source_last_modified: timestamp,
    };

    let routes: HashMap<String, RouteInfo> = [("/".to_string(), route_info)]
        .iter()
        .cloned()
        .collect();

    let config = SiteConfig{
        key: key,
        routes: routes,
    };

    write_config(&config_path, &config)
        .map_err(CreateSiteError::FailedToSaveConfig)?;

    Ok(config)
}

fn unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

// SITE CONFIG

#[derive(Deserialize, Serialize, Clone)]
struct SiteConfig {
    key: String,
    routes: HashMap<String, RouteInfo>,
}


#[derive(Deserialize, Serialize, Clone)]
struct RouteInfo {
    source_hash: String,
    source_last_modified: u64,
}

enum ReadConfigError {
    FailedToOpen(io::Error),
    FailedToDeserialize(serde_json::error::Error),
}

fn read_config(path: &Path) -> Result<SiteConfig, ReadConfigError> {
    let file = File::open(path)
        .map_err(ReadConfigError::FailedToOpen)?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(ReadConfigError::FailedToDeserialize)
}


enum WriteConfigError {
    FailedToDetermineDir(),
    FailedToCreateTempFile(io::Error),
    FailedToSerialize(serde_json::error::Error),
    FailedToPersist(io::Error),
}

impl fmt::Display for WriteConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteConfigError::FailedToDetermineDir() =>
                write!(f, "Invalid file path"),

            WriteConfigError::FailedToCreateTempFile(err) =>
                write!(f, "Failed to create temp file: {}", err),

            WriteConfigError::FailedToSerialize(err) =>
                write!(f, "Failed to serialize config: {}", err),

            WriteConfigError::FailedToPersist(err) =>
                write!(f, "Failed to persist file: {}", err),

        }
    }
}


fn write_config(path: &Path, config: &SiteConfig) -> Result<(), WriteConfigError> {
    let dir = path.parent()
        .ok_or(WriteConfigError::FailedToDetermineDir())?;

    let file = NamedTempFile::new_in(dir)
        .map_err(WriteConfigError::FailedToCreateTempFile)?;

    serde_json::to_writer_pretty(&file, config)
        .map_err(WriteConfigError::FailedToSerialize)?;

    // TODO: do we need to close/flush the file?
    file.persist(path)
        .map_err(|err| WriteConfigError::FailedToPersist(err.error))?;

    Ok(())
}


enum WriteFileError {
    FailedToDetermineDir(),
    FailedToCreateTempFile(io::Error),
    FailedToWriteFile(io::Error),
    FailedToPersist(io::Error),
}

impl fmt::Display for WriteFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteFileError::FailedToDetermineDir() =>
                write!(f, "Invalid file path"),

            WriteFileError::FailedToCreateTempFile(err) =>
                write!(f, "Failed to create temp file: {}", err),

            WriteFileError::FailedToWriteFile(err) =>
                write!(f, "Failed to write file: {}", err),

            WriteFileError::FailedToPersist(err) =>
                write!(f, "Failed to persist file: {}", err),
        }
    }
}

fn write_file_atomic(path: &Path, data: &str) -> Result<(), WriteFileError> {
    let dir = path.parent()
        .ok_or(WriteFileError::FailedToDetermineDir())?;

    let mut file = NamedTempFile::new_in(dir)
        .map_err(WriteFileError::FailedToCreateTempFile)?;

    file.write_all(data.as_bytes())
        .map_err(WriteFileError::FailedToWriteFile)?;

    file.persist(path)
        .map_err(|err| WriteFileError::FailedToPersist(err.error))?;

    Ok(())
}


fn random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&distributions::Alphanumeric)
        .take(len)
        .collect()
}




fn err_if_false<E>(value: bool, err: E) -> Result<(), E> {
    if value {
        Ok(())
    } else {
        Err(err)
    }
}

fn sha256(str: &str) -> String {
    let digest = digest::digest(&digest::SHA256, str.as_bytes());
    hex::encode(digest.as_ref())
}


#[derive(Clone)]
struct AppState {
    config: Config,
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Starting...");

    let state = AppState{
        config: Config{
            server: ServerConfig{
                main_domain: "zait.io".to_string(),
                frontend_root: "../frontend".to_string(),
                sites_root: "../sites".to_string(),
            }
        }
    };

    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .route("/", web::get().to(index))
            .route("/new", web::get().to(new_handler))
            .route("/api/sites", web::post().to(new_site_handler))
            .service(Files::new("/static", state.config.server.static_path()))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
