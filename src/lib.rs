// use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;

use anyhow::Result;

pub fn hello() -> Result<()> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.navigate_to("https://www.wikipedia.org")?;
    tab.wait_for_element("input#searchInput")?.click()?;
    tab.type_str("WebKit")?.press_key("Enter")?;
    tab.wait_for_element("#firstHeading")?;

    assert!(tab.get_url().ends_with("WebKit"));

    Ok(())
}
