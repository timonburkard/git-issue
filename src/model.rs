use std::fs;
use std::path::Path;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub id: u32,
    pub title: String,
    pub state: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub labels: Vec<String>,
    pub assignee: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub commit_auto: bool,
    pub commit_message: String,
    pub editor: String,
}

/// Generate a proper ISO 8601 timestamp using chrono.
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// git commit
pub fn git_commit(id: u32, title: String, action: &str) {
    use std::process::Command;
    let path = Path::new(".gitissues/");

    // Load configuration
    let config_path = path.join("config.yaml");
    let config_raw = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => return,
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(_) => return,
    };

    // Check if auto-commit is enabled
    if !config.commit_auto {
        return;
    }

    // Prepare commit message
    let commit_message_template = config.commit_message;
    let commit_message = commit_message_template
        .replace("{action}", action)
        .replace("{id}", &format!("{id}"))
        .replace("{title}", &title);

    // Execute git add
    let add_result = Command::new("git").args(["add", ".gitissues"]).output();
    if let Err(e) = add_result {
        eprintln!("Warning: failed to stage .gitissues: {e}");
        return;
    }

    // Execute git commit
    let commit_result = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output();
    if let Err(e) = commit_result {
        eprintln!("Warning: failed to commit: {e}");
    }
}
