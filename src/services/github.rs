use crate::prelude::*;

pub struct GitHub;

impl Service for GitHub {
    fn srv_name() -> &'static str {
        "GitHub"
    }

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool {
        todo!()
    }

    fn scan(browser: &mut Browser, user: &str) -> Vec<Scraped> {
        let tab = browser.new_tab().unwrap();
        let res = vec![];

        let _: Result<_> = try {
            tab.navigate_to(&format!("https://github.com/{}", user))?;
            tab.wait_until_navigated();

            if let Ok(email) = tab.find_element("[itemprop=email]") {
                let email = email.get_content().unwrap();
                println!("found email! {:?}", email);
            } else {
                println!("no email found!");
            }
        };

        res
    }
}

/// these tests are kinda not meant to be stable...
/// just to make sure it's working properly rn.
///
/// should prob be disabled later in case someone
/// changes their username or we get rate-limited or
/// some other crap.
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_primeagen_github() -> Result<()> {
        let mut browser = crate::new_browser(&Configs {
            chromium: None,
            headless: true,
        })?;

        let aliases = GitHub::scan(&mut browser, "ThePrimeagen");

        // TODO should find `ThePrimeTimeagen` (youtube link)

        Ok(())
    }
}
