//!
//! totally not gonna regret starting this project in
//! like a week from now. ^w^
//!
//! \- &Cherry, 2/7/2024
//!

#![allow(unused)]

// use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use anyhow::Result;

mod persistent;
mod service;

pub use persistent::{get_datafile, read_datafile};

pub fn _browse_wikipedia() -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;

    assert!(tab.get_url().ends_with("WebKit"));

    Ok(())
}
