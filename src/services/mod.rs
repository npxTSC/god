//!
//! The actual `Service` trait.
//! Implementors go in separate files in this folder.
//!

/// submodules for each service will prob use these
use crate::prelude::*;
use serde::{Deserialize, Serialize};

pub mod github;

/// Represents a scraper for a specific service.
pub trait Service {
    fn srv_name() -> &'static str;

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool;

    fn scan(tab: Arc<Tab>, user: &str) -> Vec<Scraped>;
}
