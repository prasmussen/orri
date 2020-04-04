use serde::{Serialize};
use serde::de::DeserializeOwned;
use serde_json;
use std::io;
use std::path::{Path};
use std::fs::File;
use std::io::{BufReader, Write};
use std::fmt;
use tempfile::NamedTempFile;


pub enum WriteError {
    FailedToDetermineDir(),
    FailedToCreateTempFile(io::Error),
    FailedToWriteFile(io::Error),
    FailedToPersist(io::Error),
}

impl fmt::Display for WriteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteError::FailedToDetermineDir() =>
                write!(f, "Invalid file path"),

            WriteError::FailedToCreateTempFile(err) =>
                write!(f, "Failed to create temp file: {}", err),

            WriteError::FailedToWriteFile(err) =>
                write!(f, "Failed to write file: {}", err),

            WriteError::FailedToPersist(err) =>
                write!(f, "Failed to persist file: {}", err),
        }
    }
}


pub fn write(path: &Path, data: &str) -> Result<(), WriteError> {
    let dir = path.parent()
        .ok_or(WriteError::FailedToDetermineDir())?;

    let mut file = NamedTempFile::new_in(dir)
        .map_err(WriteError::FailedToCreateTempFile)?;

    file.write_all(data.as_bytes())
        .map_err(WriteError::FailedToWriteFile)?;

    file.persist(path)
        .map_err(|err| WriteError::FailedToPersist(err.error))?;

    Ok(())
}

pub enum WriteJsonError {
    FailedToDetermineDir(),
    FailedToCreateTempFile(io::Error),
    FailedToSerialize(serde_json::error::Error),
    FailedToPersist(io::Error),
}

impl fmt::Display for WriteJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WriteJsonError::FailedToDetermineDir() =>
                write!(f, "Invalid file path"),

            WriteJsonError::FailedToCreateTempFile(err) =>
                write!(f, "Failed to create temp file: {}", err),

            WriteJsonError::FailedToSerialize(err) =>
                write!(f, "Failed to serialize config: {}", err),

            WriteJsonError::FailedToPersist(err) =>
                write!(f, "Failed to persist file: {}", err),

        }
    }
}


pub fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<(), WriteJsonError> {
    let dir = path.parent()
        .ok_or(WriteJsonError::FailedToDetermineDir())?;

    let file = NamedTempFile::new_in(dir)
        .map_err(WriteJsonError::FailedToCreateTempFile)?;

    serde_json::to_writer_pretty(&file, value)
        .map_err(WriteJsonError::FailedToSerialize)?;

    file.persist(path)
        .map_err(|err| WriteJsonError::FailedToPersist(err.error))?;

    Ok(())
}



pub enum ReadJsonError {
    FailedToOpen(io::Error),
    FailedToDeserialize(serde_json::error::Error),
}

pub fn read_json<T: DeserializeOwned>(path: &Path) -> Result<T, ReadJsonError> {
    let file = File::open(path)
        .map_err(ReadJsonError::FailedToOpen)?;

    let reader = BufReader::new(file);

    serde_json::from_reader(reader)
        .map_err(ReadJsonError::FailedToDeserialize)
}


