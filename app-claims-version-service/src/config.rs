use serde::{Deserialize, Serialize};
use claims_core::config::Log;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    pub log: Log,
}