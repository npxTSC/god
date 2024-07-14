//!
//! The actual `Service` trait.
//! Implementors go in separate files in this folder.
//!

/// submodules for each service will prob use these
use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod github;
pub mod youtube;

/// Represents a scraper for a specific service.
pub trait Service {
    fn srv_name() -> &'static str;

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool;

    fn scan(tab: &mut Browser, user: &str) -> Vec<Scraped>;
}

/// Scan all services for a username.
pub fn scan_all(browser: &mut Browser, user: &str) -> HashMap<String, Vec<Scraped>> {
    let mut res = HashMap::new();

    // this is kinda a hack lol refactor later when there are more services
    res.insert("GitHub".to_string(), github::GitHub::scan(browser, user));
    res.insert("YouTube".to_string(), youtube::YouTube::scan(browser, user));

    // TODO compare the results and see which services have the most links in common

    res
}
