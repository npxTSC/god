use super::prelude::*;
pub struct GitHub;

impl GitHub {
    fn get_profile_links(tab: Arc<Tab>, user: &str) -> Vec<String> {
        todo!()
    }
}

impl Service for GitHub {
    fn srv_name() -> &'static str {
        "GitHub"
    }

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool {
        todo!()
    }

    fn find_aliases(tab: Arc<Tab>, user: &str) -> Vec<String> {
        todo!()
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
        let browser = crate::new_browser(false)?;
        let tab = browser.new_tab()?;

        let aliases = GitHub::find_aliases(tab, "ThePrimeagen");

        // should also find `ThePrimeTimeagen` (youtube link)

        Ok(())
    }
}
