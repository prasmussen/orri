pub mod http;


use serde::{Deserialize, Serialize};
use std::io;
use std::fs;
use std::path::{PathBuf};
use std::collections::BTreeMap;
use crate::orri::file;
use crate::orri::util;
use crate::orri::domain::Domain;
use crate::orri::site_key::SiteKey;
use crate::orri::url_path::UrlPath;
use std::time::SystemTime;
use std::str::FromStr;
use std::ffi::OsString;


#[derive(Deserialize, Serialize, Clone)]
pub struct Site {
    pub domain: Domain,
    pub key: SiteKey,
    pub quota: Quota,
    pub routes: BTreeMap<UrlPath, RouteInfo>,

    #[serde(skip)]
    unwritten_files: Vec<File>,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub quota_nano: QuotaLimits,
}


pub enum AddRouteError {
    QuotaMaxSize(),
    QuotaMaxRoutes(),
}

pub enum UpdateRouteError {
    RouteNotFound(),
    QuotaMaxSize(),
}

pub enum PersistSiteError {
    CreateDomainDir(io::Error),
    WriteFileError(file::WriteError),
    WriteSiteJsonError(file::WriteJsonError),
}


impl Site {
    pub fn add_route(&mut self, config: &Config, path: UrlPath, file_info: FileInfo, file_data: &[u8]) -> Result<&Site, AddRouteError> {
        let limits = self.quota.limits(config);

        util::ensure(self.size() + file_info.size < limits.max_size, AddRouteError::QuotaMaxSize())?;
        util::ensure(self.routes.len() < limits.max_routes, AddRouteError::QuotaMaxRoutes())?;

        self.unwritten_files.push(File{
            metadata: file_info.clone(),
            data: file_data.to_vec(),
        });

        self.routes.insert(path, RouteInfo{
            file_info,
        });

        Ok(self)
    }

    pub fn update_route(&mut self, config: &Config, path: UrlPath, file_info: FileInfo, file_data: &[u8]) -> Result<&Site, UpdateRouteError> {
        let limits = self.quota.limits(config);

        let old_route = self.routes.get(&path)
            .ok_or(UpdateRouteError::RouteNotFound())?;

        util::ensure(self.size() - old_route.file_info.size + file_info.size < limits.max_size, UpdateRouteError::QuotaMaxSize())?;

        self.unwritten_files.push(File{
            metadata: file_info.clone(),
            data: file_data.to_vec(),
        });

        self.routes.insert(path, RouteInfo{
            file_info,
        });

        Ok(self)
    }

    pub fn remove_route(&mut self, path: UrlPath) -> &Site {
        self.routes.remove(&path);

        self
    }

    pub fn size(&self) -> usize {
        self.routes
            .iter()
            .fold(0, |acc, (_path, route_info)| acc + route_info.file_info.size)
    }

    pub fn persist(&self, site_root: &SiteRoot) -> Result<&Site, PersistSiteError> {
        site_root.prepare_directories()
            .map_err(PersistSiteError::CreateDomainDir)?;

        self.unwritten_files
            .iter()
            .try_for_each(|file|
                file::write(&site_root.data_file_path(&file.metadata.hash), &file.data)
            )
            .map_err(PersistSiteError::WriteFileError)?;

        file::write_json(&site_root.site_json_path(), self)
            .map_err(PersistSiteError::WriteSiteJsonError)?;

        let _ = self.remove_stale_data(site_root);

        Ok(self)
    }

    fn remove_stale_data(&self, site_root: &SiteRoot) -> Result<(), io::Error> {
        let fresh_hashes = self.routes.iter()
            .map(|(_path, route_info)| OsString::from(route_info.file_info.hash.clone()))
            .collect::<Vec<OsString>>();

        fs::read_dir(site_root.data_path())?
            .for_each(|res| {
                let _ = res.map(|entry| {
                    if !fresh_hashes.contains(&entry.file_name()) {
                        let _ = fs::remove_file(entry.path());
                    }
                });
            });

        Ok(())
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RouteInfo {
    pub file_info: FileInfo,
}



pub enum CreateSiteError {
    SiteAlreadyExist(),
    AddRoute(AddRouteError),
}


pub fn create(config: &Config, site_root: &SiteRoot, key: SiteKey, file_info: FileInfo, file_data: &[u8]) -> Result<Site, CreateSiteError> {
    util::ensure(!site_root.site_json_path().exists(), CreateSiteError::SiteAlreadyExist())?;

    let mut site = Site{
        domain: site_root.domain.clone(),
        key,
        quota: Quota::Nano,
        routes: BTreeMap::new(),
        unwritten_files: vec![],
    };

    site.add_route(&config, UrlPath::root(), file_info, file_data)
        .map_err(CreateSiteError::AddRoute)?;

    Ok(site)
}


pub enum GetSiteError {
    SiteNotFound(),
    ReadSiteJson(file::ReadJsonError),
}

pub fn get(site_root: &SiteRoot) -> Result<Site, GetSiteError> {
    util::ensure(site_root.site_json_path().exists(), GetSiteError::SiteNotFound())?;

    file::read_json(&site_root.site_json_path())
        .map_err(GetSiteError::ReadSiteJson)
}



pub struct SiteRoot {
    domain: Domain,
    root: PathBuf,
}

impl SiteRoot {
    pub fn new(root_path: &PathBuf, domain: Domain) -> SiteRoot {
        let root = root_path.join(PathBuf::from(domain.to_string()));

        SiteRoot{
            domain,
            root,
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

    pub fn remove(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.root)?;

        Ok(())
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
        let hash = util::sha256(&data);
        let timestamp = util::unix_timestamp(time);

        FileInfo{
            mime,
            hash,
            size: data.len(),
            timestamp,
        }
    }
}


#[derive(Clone)]
pub struct File {
    pub metadata: FileInfo,
    pub data: Vec<u8>,
}

pub fn read_route_file(site_root: &SiteRoot, route: &RouteInfo) -> Result<File, io::Error> {
    let path = site_root.data_file_path(&route.file_info.hash);
    let data = fs::read(path)?;

    Ok(File{
        metadata: route.file_info.clone(),
        data,
    })

}


#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Quota {
    Nano,
}

impl Quota {
    pub fn limits(&self, config: &Config) -> QuotaLimits {
        match self {
            Quota::Nano =>
                config.quota_nano.clone(),
        }
    }
}


pub enum QuotaFromStrError {
    UnknownQuota()
}

impl FromStr for Quota {
    type Err = QuotaFromStrError;

    fn from_str(s: &str) -> Result<Quota, QuotaFromStrError> {
        match s {
            "nano" =>
                Ok(Quota::Nano),

            _ =>
                Err(QuotaFromStrError::UnknownQuota()),
        }
    }
}


#[derive(Clone, Debug)]
pub struct QuotaLimits {
    pub max_size: usize,
    pub max_routes: usize,
    pub max_sites: usize,
}
