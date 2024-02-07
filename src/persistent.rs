use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::services::Service;

const DATAFILE_NAME: &str = ".god-data";

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    /// the username of the current target...
    pub username: String,

    /// map of services to list of usernames
    pub accounts: HashMap<String, Vec<String>>,
}

pub fn read_datafile(path: &Path) -> Result<PersistentData> {
    let data = fs::read_to_string(path)?;
    let data: PersistentData = toml::from_str(&data)?;
    Ok(data)
}

pub fn write_datafile(path: &Path, data: &PersistentData) -> Result<()> {
    let data = toml::to_string(data)?;
    fs::write(path, data)?;
    Ok(())
}

/// Get the datafile's path. Defaults to [`~/.god-data`].
pub fn get_datafile(path: Option<&Path>) -> PathBuf {
    match path {
        Some(path) => path.to_owned(),

        None => dirs::home_dir()
            .expect("No home directory. Seriously?")
            .join(DATAFILE_NAME),
    }
}
