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
pub fn create(site_root: SiteRoot, source: &str) -> Result<Site, CreateSiteError> {
    site_root.prepare_directories()
        .map_err(CreateSiteError::FailedToCreateDomainDir)?;

    util::err_if_false(site_root.site_json_path().exists() == false, CreateSiteError::SiteAlreadyExist())?;

    let source_hash = util::sha256(&source);

    file::write(&site_root.data_file_path(&source_hash), &source)
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

    file::write_json(&site_root.site_json_path(), &site)
        .map_err(CreateSiteError::FailedToSaveSiteJson)?;

    Ok(site)
}


pub struct SiteRoot {
    site_root: PathBuf,
}

impl SiteRoot {
    pub fn new(root_path: &str, domain: &str) -> SiteRoot {
        SiteRoot{
            site_root: Path::new(root_path).join(PathBuf::from(domain)),
        }
    }

    pub fn site_json_path(&self) -> PathBuf {
        self.site_root.join(PathBuf::from("site.json"))
    }

    pub fn data_path(&self) -> PathBuf {
        self.site_root.join(PathBuf::from("data"))
    }


    pub fn data_file_path(&self, data_hash: &str) -> PathBuf {
        self.data_path().join(PathBuf::from(data_hash))
    }

    pub fn prepare_directories(&self) -> Result<(), io::Error> {
        fs::create_dir_all(self.data_path())
    }
}

