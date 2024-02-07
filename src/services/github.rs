use super::Service;
pub struct GitHub;

impl Service for GitHub {
    fn srv_name() -> &'static str {
        "GitHub"
    }

    fn username_exists(username: &str) -> bool {
        unimplemented!()
    }
}
