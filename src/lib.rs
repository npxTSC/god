//!
//! totally not gonna regret starting this project in
//! like a week from now. ^w^
//!
//! \- &Cherry, 2/7/2024
//!

use std::path::{Path, PathBuf};

// use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use anyhow::Result;

const DATAFILE_NAME: &str = ".god-data";

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

/// Get the datafile's path. Defaults to [`~/.god-data`].
pub fn get_datafile(path: Option<&Path>) -> PathBuf {
    match path {
        Some(path) => path.to_owned(),
        None => {
            use dirs::home_dir;
            let home = home_dir().expect("No home directory. Seriously?");
            home.join(DATAFILE_NAME)
        }
    }
}
