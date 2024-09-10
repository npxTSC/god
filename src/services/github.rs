use crate::prelude::*;

fn find_email(tab: &Arc<Tab>, res: &mut Vec<Scraped>) {
    if let Ok(email) = tab.find_element("[itemprop=email]") {
        let email = email.get_content().unwrap();
        println!("found email! {:?}", email);

        res.push(Scraped::Email(email));
    } else {
        println!("no email found! (probably requires login)");
    }
}

fn find_website_links(tab: &Tab, res: &mut Vec<Scraped>) -> Result<()> {
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

    Ok(())
}

fn find_socials(tab: &Arc<Tab>, _res: &mut [Scraped]) {
    if let Ok(socials) = tab.find_elements("li[itemprop=social] > a") {
        let socials = socials
            .into_iter()
            .filter_map(|v| v.get_attribute_value("href").ok().flatten())
            .map(|v| crate::resolver::link_to_social(&v))
            .collect::<Vec<_>>();

        println!("socials: {:?}", socials);
        // TODO push res
    } else {
        println!("failed to get social links");
    }
}

fn find_emails_from_patches(
    tab: &Tab,
    _res: &mut [Scraped],
    _user: &str,
) -> Result<()> {
    println!("searching for emails from patches...");

    let repo_links = tab
        .wait_for_elements("a[itemprop=\"name codeRepository\"]")
        .unwrap();

    // filter any that are from forked repos, since they
    // likely won't contain commits from the user
    let repo_links = repo_links
        .into_iter()
        .filter(|v| {
            let is_own = v.find_element_by_xpath("../span").is_err();
            if !is_own {
                println!(
                    "skipping forked repo: {:?}",
                    v.get_attribute_value("href")
                );
            }

            is_own
        })
        .filter_map(|v| v.get_attribute_value("href").unwrap())
        .map(|v| v.split('/').last().unwrap().to_owned())
        .inspect(|v| println!("found own repo link: {:?}", v))
        .collect::<Vec<_>>();

    println!("found {} own repos", repo_links.len());

    Ok(())
}

pub struct GitHub;

impl Service for GitHub {
    // fn srv_name() -> &'static str {
    //     "GitHub"
    // }
    //
    // fn username_exists(tab: Arc<Tab>, user: &str) -> bool {
    //     todo!()
    // }

    fn scan(browser: &mut Browser, user: &str) -> Vec<Scraped> {
        let tab = browser.new_tab().unwrap();
        let mut res = vec![];

        let _: Result<_> = try {
            tab.navigate_to(&format!("https://github.com/{}", user))?
                .wait_until_navigated()?;

            find_email(&tab, &mut res);
            find_socials(&tab, &mut res);
            find_website_links(&tab, &mut res)?;

            tab.navigate_to(&format!(
                "https://github.com/{}?tab=repositories",
                user
            ))?
            .wait_until_navigated()?;
            find_emails_from_patches(&tab, &mut res, user)?;

            // debug
            // std::thread::sleep(std::time::Duration::from_secs(60));
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
