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
    fn srv_name(&self) -> &'static str;

    fn username_exists(&self, tab: Arc<Tab>, user: &str) -> bool;

    fn scan(&self, tab: &mut Browser, user: &str) -> Vec<Scraped>;
}

/// Scan all services for a username.
pub fn scan_all(browser: &mut Browser, user: &str) -> HashMap<String, Vec<Scraped>> {
    let services: Vec<Box<dyn Service>> =
        vec![Box::new(github::GitHub), Box::new(youtube::YouTube)];

    let res = services
        .into_iter()
        .map(|srv| (srv.srv_name().to_string(), srv.scan(browser, user)))
        .collect();

    // debugging purposes
    std::thread::sleep(std::time::Duration::from_secs(60));

    // TODO compare the results and see which services have the most links in common

    res
}
