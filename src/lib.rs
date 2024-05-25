//!
//! totally not gonna regret starting this project in
//! like a week from now. ^w^
//!
//! \- &Cherry, 2/7/2024
//!

#![allow(unused)]
#![feature(try_blocks)]

// use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptionsBuilder};

use crate::prelude::*;
mod configs;
mod services;

pub use configs::{Configs, State};

mod prelude {
    pub use crate::configs::{Configs, Scraped};
    pub use crate::services::Service;
    pub use crate::State;

    pub use anyhow::Result;
    pub use headless_chrome::{Browser, Tab};

    pub use std::collections::HashMap;
    pub use std::sync::Arc;
}

pub fn _browse_wikipedia(conf: &Configs) -> Result<()> {
    let browser = new_browser(conf)?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;

    assert!(tab.get_url().ends_with("WebKit"));

    Ok(())
}

pub fn new_browser(conf: &Configs) -> Result<Browser> {
    // launch options
    let mut lops = LaunchOptionsBuilder::default();

    lops.headless(conf.headless);
    lops.path(conf.chromium.clone());

    let lops = lops.build()?;

    Browser::new(lops)
}
