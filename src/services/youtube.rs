use crate::prelude::*;

pub struct YouTube;

impl Service for YouTube {
    fn srv_name() -> &'static str {
        "YouTube"
    }

    fn username_exists(tab: Arc<Tab>, user: &str) -> bool {
        todo!()
    }

    fn scan(browser: &mut Browser, user: &str) -> Vec<Scraped> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    // #[test]
    // fn test_dexie_youtube() -> Result<()> {}
}
