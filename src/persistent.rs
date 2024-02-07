use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::service::Service;

const DATAFILE_NAME: &str = ".god-data";

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub name: String,
    pub service: Service,
}

#[derive(Serialize, Deserialize)]
pub struct PersistentData {
    pub username: String,
    pub accounts: HashMap<Service, Account>,
    pub possible_aliases: Vec<String>,
}

pub fn read_datafile(path: &Path) -> Result<PersistentData> {
    let data = std::fs::read_to_string(path)?;
    let data: PersistentData = toml::from_str(&data)?;
    Ok(data)
}

pub fn write_datafile(path: &Path, data: &PersistentData) -> Result<()> {
    let data = toml::to_string(data)?;
    std::fs::write(path, data)?;
    Ok(())
}

/// Get the datafile's path. Defaults to [`~/.god-data`].
pub fn get_datafile(path: Option<&Path>) -> PathBuf {
    match path {
        Some(path) => path.to_owned(),
        None => {
            use dirs::home_dir;
            let home = home_dir().expect("No home directory. Seriously?");
            home.join(DATAFILE_NAME)
        }
    }
}
