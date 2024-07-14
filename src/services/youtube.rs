use crate::prelude::*;

pub struct YouTube;

impl Service for YouTube {
    fn srv_name(&self) -> &'static str {
        "YouTube"
    }

    fn username_exists(&self, tab: Arc<Tab>, user: &str) -> bool {
        todo!()
    }

    fn scan(&self, browser: &mut Browser, user: &str) -> Vec<Scraped> {
        let tab = browser.new_tab().unwrap();
        let mut res = vec![];

        let _: Result<_> = try {
            tab.navigate_to(&format!("https://youtube.com/user/{}", user))?;
            tab.wait_until_navigated();

            // click span with content "...more"
            if let Ok(more_btn) = tab.find_element(todo!("youtube more links")) {
                more_btn.click();
            } else {
                println!("no \"more\" button found");
            }
        };

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    // #[test]
    // fn test_dexie_youtube() -> Result<()> {}
}
