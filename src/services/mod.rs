//!
//! The actual `Service` trait.
//! Implementors go in separate files in this folder.
//!

/// submodules for each service will prob use these
mod prelude {
    pub use super::Service;
    pub use crate::State;

    pub use headless_chrome::Tab;

    pub use std::collections::HashMap;
    pub use std::sync::Arc;
}

use prelude::*;
use serde::{Deserialize, Serialize};

pub mod github;

/// Represents a scraper for a specific service.
pub trait Service {
    fn srv_name() -> &'static str;

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool;

    fn find_aliases(tab: Arc<Tab>, user: &str) -> Vec<String>;
}
