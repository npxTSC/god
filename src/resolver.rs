use crate::prelude::*;

fn until_slash_or_question(frag: &str) -> &str {
    frag.split(&['/', '?'][..]).next().unwrap()
}

pub fn link_to_social(link: &str) -> ProfileLink {
    let original = link;

    // take off the http(s):// part
    let link = link
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.")
        .to_lowercase(); // potential source of bugs

    //////////////////////////////
    // match the social network //
    //////////////////////////////

    if link.starts_with("twitch.tv") {
        let link = until_slash_or_question(link.trim_start_matches("twitch.tv/"));

        return ProfileLink::Twitch(link.to_string());
    }

    // youtube links are so diverse! we can disambiguate those later
    if link.starts_with("youtube.com") {
        let link = link
            .trim_start_matches("youtube.com/")
            .trim_start_matches("channel/")
            .trim_start_matches("@");
        return ProfileLink::YouTube(link.to_string());
    }

    if link.starts_with("twitter.com") {
        let link = until_slash_or_question(link.trim_start_matches("twitter.com/"));
        return ProfileLink::Twitter(link.to_string());
    }

    ProfileLink::Generic(original.to_string())
}
