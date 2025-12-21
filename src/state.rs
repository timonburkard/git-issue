use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use chrono::Utc;

pub fn run(id: u32, state: String) -> Result<(), String> {
    let id_str = format!("{:010}", id);
    let issue_path = format!(".gitissues/issues/{}", id_str);
    let path = Path::new(&issue_path);

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    // Load meta.yaml
    let meta_path = path.join("meta.yaml");
    let meta_raw = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(_) => return Err("meta.yaml not found.".to_string()),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err("meta.yaml malformatted.".to_string()),
    };

    // Update state and updated timestamp
    let updated_timestamp = current_timestamp();
    let updated_meta = Meta {
        id: meta.id,
        title: meta.title,
        state: state,
        created: meta.created,
        updated: updated_timestamp,
    };

    let updated_yaml = serde_yaml::to_string(&updated_meta)
        .map_err(|_| "Failed to serialize meta.yaml".to_string())?;

    fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;

    println!("Updated issue state");

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Meta {
    id: String,
    title: String,
    state: String,
    created: String,
    updated: String,
}

/// Generate a proper ISO 8601 timestamp using chrono.
fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
