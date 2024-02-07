use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Service {
    Reddit,
}
