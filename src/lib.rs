//!
//! totally not gonna regret starting this project in
//! like a week from now. ^w^
//!
//! \- &Cherry, 2/7/2024
//!

#![allow(unused)]

// use headless_chrome::protocol::cdp::Page;
use headless_chrome::{Browser, LaunchOptionsBuilder};

use anyhow::Result;

mod services;
mod stateful;

pub use stateful::{get_datafile, read_datafile, State};

pub fn _browse_wikipedia(state: &State) -> Result<()> {
    let browser = new_browser(state)?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;

    assert!(tab.get_url().ends_with("WebKit"));

    Ok(())
}

pub fn new_browser(state: &State) -> Result<Browser> {
    // launch options
    let mut lops = LaunchOptionsBuilder::default();

    lops.headless(state.headless);
    lops.path(state.chromium.clone());

    let lops = lops.build()?;

    Browser::new(lops)
}
