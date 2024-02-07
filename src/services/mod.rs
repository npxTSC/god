use serde::{Deserialize, Serialize};

pub mod github;

pub trait Service {
    fn srv_name() -> &'static str;
    fn username_exists(username: &str) -> bool;
}
