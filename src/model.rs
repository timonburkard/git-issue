use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub id: u32,
    pub title: String,
    pub state: String,
    pub created: String,
    pub updated: String,
}

/// Generate a proper ISO 8601 timestamp using chrono.
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
