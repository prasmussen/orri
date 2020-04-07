pub mod http;

use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::orri::file;
use crate::orri::util;
use crate::orri::domain::Domain;


#[derive(Deserialize, Serialize, Clone)]
pub struct Site {
    pub key: String,
    pub routes: HashMap<String, RouteInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RouteInfo {
    pub file_info: FileInfo,
}




pub enum CreateSiteError {
    SiteAlreadyExist(),
    FailedToCreateDomainDir(io::Error),
    FailedToWriteFile(file::WriteError),
    FailedToSaveSiteJson(file::WriteJsonError),
}


// TODO: Two users can create the same site at the same time
pub fn create(site_root: SiteRoot, file_info: FileInfo, file_data: &[u8]) -> Result<Site, CreateSiteError> {
    site_root.prepare_directories()
        .map_err(CreateSiteError::FailedToCreateDomainDir)?;

    util::err_if_false(site_root.site_json_path().exists() == false, CreateSiteError::SiteAlreadyExist())?;

    file::write(&site_root.data_file_path(&file_info.hash), file_data)
        .map_err(CreateSiteError::FailedToWriteFile)?;

    let key = util::random_string(32);

    let route_info = RouteInfo{
        file_info: file_info,
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


pub enum GetSiteError {
    SiteNotFound(),
    FailedToReadSiteJson(file::ReadJsonError),
}

pub fn get(site_root: &SiteRoot) -> Result<Site, GetSiteError> {
    util::err_if_false(site_root.site_json_path().exists(), GetSiteError::SiteNotFound())?;

    file::read_json(&site_root.site_json_path())
        .map_err(GetSiteError::FailedToReadSiteJson)
}



pub struct SiteRoot {
    site_root: PathBuf,
}

impl SiteRoot {
    pub fn new(root_path: &PathBuf, domain: Domain) -> SiteRoot {
        let site_root = root_path.join(PathBuf::from(domain.to_string()));

        SiteRoot{
            site_root: site_root,
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


#[derive(Deserialize, Serialize, Clone)]
pub struct FileInfo {
    mime: String,
    hash: String,
    size: usize,
    timestamp: u64,
}

impl FileInfo {
    pub fn new(data: &[u8], mime: String) -> FileInfo {
        let file_hash = util::sha256(&data);
        let timestamp = util::unix_timestamp();

        FileInfo{
            mime: mime,
            hash: file_hash,
            size: data.len(),
            timestamp: timestamp,
        }
    }
}


pub struct File {
    pub metadata: FileInfo,
    pub data: Vec<u8>,
}

pub fn read_route_file(site_root: &SiteRoot, route: &RouteInfo) -> Result<File, io::Error> {
    let path = site_root.data_file_path(&route.file_info.hash);
    let data = fs::read(path)?;

    Ok(File{
        metadata: route.file_info.clone(),
        data: data,
    })

}
