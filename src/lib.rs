//!
//! totally not gonna regret starting this project in
//! like a week from now. ^w^
//!
//! \- &Cherry, 2/7/2024
//!

#![allow(unused)]
#![feature(try_blocks)]

use headless_chrome::{Browser, LaunchOptionsBuilder};

use crate::prelude::*;
mod configs;
mod resolver;
mod services;

pub use configs::{Configs, State};

mod prelude {
    pub use crate::configs::State;
    pub use crate::configs::{Configs, ProfileLink, Scraped};
    pub use crate::services::Service;

    pub use anyhow::Result;
    pub use headless_chrome::{Browser, Tab};

    pub use std::collections::HashMap;
    pub use std::sync::Arc;
}

fn new_browser(conf: &Configs) -> Result<Browser> {
    // launch options
    let mut lops = LaunchOptionsBuilder::default();

    lops.headless(conf.headless);
    lops.path(conf.chromium.clone());

    let lops = lops.build()?;

    Browser::new(lops)
}

pub fn start_scan(conf: &Configs, passes: u8, user: &str) -> Result<Vec<Scraped>> {
    let mut browser = new_browser(conf)?;

    let users = vec![user];
    let mut scans = HashMap::new();

    for _ in 0..passes {
        // TODO add new entries to
        let new_scans = services::scan_all(&mut browser, user);
        scans.extend(new_scans);
    }

    Ok(vec![])
}
