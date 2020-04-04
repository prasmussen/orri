pub mod http;

use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::zait::file;
use crate::zait::util;


#[derive(Deserialize, Serialize, Clone)]
pub struct Site {
    pub key: String,
    pub routes: HashMap<String, RouteInfo>,
}


#[derive(Deserialize, Serialize, Clone)]
pub struct RouteInfo {
    pub source_hash: String,
    pub source_last_modified: u64,
}


pub enum CreateSiteError {
    SiteAlreadyExist(),
    FailedToCreateDomainDir(io::Error),
    FailedToWriteSourceFile(file::WriteError),
    FailedToSaveSiteJson(file::WriteJsonError),
}


// TODO: Two users can create the same site at the same time
pub fn create(root_path: &str, domain: &str, source: &str) -> Result<Site, CreateSiteError> {
    let domain_path = Path::new(root_path)
        .join(PathBuf::from(domain));

    let data_path = domain_path.join(PathBuf::from("data"));

    fs::create_dir_all(&data_path)
        .map_err(CreateSiteError::FailedToCreateDomainDir)?;

    let config_path = domain_path.join(PathBuf::from("site.json"));
    let source_hash = util::sha256(&source);
    let source_path = data_path.join(PathBuf::from(&source_hash));

    util::err_if_false(config_path.exists() == false, CreateSiteError::SiteAlreadyExist())?;

    file::write(&source_path, &source)
        .map_err(CreateSiteError::FailedToWriteSourceFile)?;

    let key = util::random_string(32);
    let timestamp = util::unix_timestamp();

    let route_info = RouteInfo{
        source_hash: source_hash,
        source_last_modified: timestamp,
    };

    let routes: HashMap<String, RouteInfo> = [("/".to_string(), route_info)]
        .iter()
        .cloned()
        .collect();

    let site = Site{
        key: key,
        routes: routes,
    };

    file::write_json(&config_path, &site)
        .map_err(CreateSiteError::FailedToSaveSiteJson)?;

    Ok(site)
}
