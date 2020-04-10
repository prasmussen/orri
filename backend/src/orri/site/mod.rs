pub mod http;

use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;
use crate::orri::file;
use crate::orri::util;
use crate::orri::domain::Domain;
use std::time::SystemTime;


#[derive(Deserialize, Serialize, Clone)]
pub struct Site {
    pub domain: Domain,
    pub key: String,
    pub routes: BTreeMap<String, RouteInfo>,
}

impl Site {
    pub fn add_route(&mut self, site_root: &SiteRoot, path: &str, file_info: FileInfo, file_data: &[u8]) -> Result<&Site, file::WriteError> {
        file::write(&site_root.data_file_path(&file_info.hash), file_data)?;

        self.routes.insert(path.to_string(), RouteInfo{
            file_info: file_info,
        });

        Ok(self)
    }

    pub fn persist(&self, site_root: &SiteRoot) -> Result<&Site, file::WriteJsonError> {
        file::write_json(&site_root.site_json_path(), self)?;

        Ok(self)
    }
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
pub fn create(site_root: SiteRoot, key: &str, file_info: FileInfo, file_data: &[u8]) -> Result<Site, CreateSiteError> {
    site_root.prepare_directories()
        .map_err(CreateSiteError::FailedToCreateDomainDir)?;

    util::ensure(site_root.site_json_path().exists() == false, CreateSiteError::SiteAlreadyExist())?;

    let mut site = Site{
        domain: site_root.domain.clone(),
        key: key.to_string(),
        routes: BTreeMap::new(),
    };

    site.add_route(&site_root, "/", file_info, file_data)
        .map_err(CreateSiteError::FailedToWriteFile)?;

    site.persist(&site_root)
        .map_err(CreateSiteError::FailedToSaveSiteJson)?;

    Ok(site)
}


pub enum GetSiteError {
    SiteNotFound(),
    FailedToReadSiteJson(file::ReadJsonError),
}

pub fn get(site_root: &SiteRoot) -> Result<Site, GetSiteError> {
    util::ensure(site_root.site_json_path().exists(), GetSiteError::SiteNotFound())?;

    file::read_json(&site_root.site_json_path())
        .map_err(GetSiteError::FailedToReadSiteJson)
}



pub struct SiteRoot {
    domain: Domain,
    root: PathBuf,
}

impl SiteRoot {
    pub fn new(root_path: &PathBuf, domain: Domain) -> SiteRoot {
        let root = root_path.join(PathBuf::from(domain.to_string()));

        SiteRoot{
            domain: domain,
            root: root,
        }
    }

    pub fn site_json_path(&self) -> PathBuf {
        self.root.join(PathBuf::from("site.json"))
    }

    pub fn data_path(&self) -> PathBuf {
        self.root.join(PathBuf::from("data"))
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
    pub mime: String,
    pub hash: String,
    pub size: usize,
    pub timestamp: u64,
}

impl FileInfo {
    pub fn new(data: &[u8], mime: String, time: SystemTime) -> FileInfo {
        let file_hash = util::sha256(&data);
        let timestamp = util::unix_timestamp(time);

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
