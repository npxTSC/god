use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::services::Service;

const DATAFILE_NAME: &str = ".god-data";

#[derive(Default, Serialize, Deserialize)]
pub struct Configs {
    /// custom chromium binary path
    pub chromium: Option<PathBuf>,

    /// should the browser be headless?
    pub headless: bool,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    /// the username of the current target...
    pub username: String,

    /// map of services to list of usernames
    pub accounts: HashMap<String, Vec<Scraped>>,
}

#[derive(Serialize, Deserialize)]
pub enum Scraped {
    Username(String),
    Email(String),
}

pub fn read_datafile(path: &Path) -> Configs {
    let data = fs::read_to_string(path);

    data.map(|s| toml::from_str(&s).expect("Issues opening the state file..."))
        .unwrap_or_default()
}

pub fn write_datafile(path: &Path, data: &State) -> Result<()> {
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
