use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Log {
    #[serde(default)]
    pub msg: String,
    pub date: u128,
}
