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
    Social(Social),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Social {
    Twitch(String),
    YouTube(String),
    Twitter(String), // not X. :>
    Other(String),
}
