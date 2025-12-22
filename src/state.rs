use std::fs;
use std::path::Path;

use crate::model::{Meta, current_timestamp, git_commit};

pub fn run(id: u32, state: String) -> Result<(), String> {
    let id_str = format!("{id:010}");
    let issue_path = format!(".gitissues/issues/{id_str}");
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
    let mut updated_meta = meta;

    updated_meta.state = state;
    updated_meta.updated = updated_timestamp;

    let updated_yaml = serde_yaml::to_string(&updated_meta)
        .map_err(|_| "Failed to serialize meta.yaml".to_string())?;

    fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;

    // git commit
    git_commit(id, updated_meta.title, "state change");

    println!("Updated issue state");

    Ok(())
}
