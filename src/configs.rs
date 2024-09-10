use std::collections::HashMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

// const DATAFILE_NAME: &str = ".god-data";

#[derive(Default, Serialize, Deserialize)]
pub struct Configs {
    /// custom chromium binary path
    pub chromium: Option<PathBuf>,

    /// should the browser be headless?
    pub headless: bool,
}

/// The state of the program.
#[derive(Serialize, Deserialize)]
pub struct State {
    /// the username of the current target...
    pub username: String,

    /// map of services to list of usernames
    pub accounts: HashMap<String, Vec<Scraped>>,
}

/// Information that has been scraped from a profile.
#[derive(Serialize, Deserialize)]
pub enum Scraped {
    Username(String),
    Email(String),
    Link(ProfileLink),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProfileLink {
    Twitch(String),
    YouTube(String),
    Twitter(String), // not X. :>
    Generic(String),
}
