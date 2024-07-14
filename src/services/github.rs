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
        let mut res = vec![];

        let _: Result<_> = try {
            tab.navigate_to(&format!("https://github.com/{}", user))?;
            tab.wait_until_navigated();

            if let Ok(email) = tab.find_element("[itemprop=email]") {
                let email = email.get_content().unwrap();
                println!("found email! {:?}", email);
            } else {
                println!("no email found! (probably requires login)");
            }

            if let Ok(socials) = tab.find_elements("li[itemprop=social] > a") {
                let socials = socials
                    .into_iter()
                    .filter_map(|v| v.get_attribute_value("href").ok().flatten())
                    .map(|v| crate::resolver::link_to_social(&v))
                    .collect::<Vec<_>>();

                println!("socials: {:?}", socials);
            } else {
                println!("failed to get social links");
            }

            if let Ok(link) = tab.find_element(".Link--primary") {
                if let Some(link) = link.get_attribute_value("href")? {
                    println!("found profile website link! {:?}", link);
                    res.push(Scraped::Link(ProfileLink::Generic(link)));
                } else {
                    println!("link found, but no href");
                }
            } else {
                println!("no link found!");
            }

            // debug
            std::thread::sleep(std::time::Duration::from_secs(60));
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
