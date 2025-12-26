use std::fs;
use std::path::Path;

use crate::model::{config_path, git_commit_non_templated, gitissues_base, users_path};

pub fn run(no_commit: bool) -> Result<(), String> {
    let root = gitissues_base();
    let issues_dir = Path::new(gitissues_base()).join("issues");

    if Path::new(root).exists() {
        return Err("Already initialized: .gitissues already exists".to_string());
    }

    // Create the directory structure
    fs::create_dir_all(&issues_dir).map_err(|e| format!("Failed to create {}: {e}", issues_dir.display()))?;

    // Copy default config file
    const DEFAULT_CONFIG: &str = include_str!("../config/config-default.yaml");
    let config_dst = config_path();
    fs::write(&config_dst, DEFAULT_CONFIG).map_err(|e| format!("Failed to write default config to {}: {e}", config_dst.display()))?;

    // Copy default users file
    const DEFAULT_USERS: &str = include_str!("../config/users-default.yaml");
    let users_dst = users_path();
    fs::write(&users_dst, DEFAULT_USERS).map_err(|e| format!("Failed to write default users to {}: {e}", users_dst.display()))?;

    // Copy default description file
    const DEFAULT_DESC: &str = include_str!("../config/description-default.md");
    let desc_dst = Path::new(gitissues_base()).join("description.md");
    fs::write(&desc_dst, DEFAULT_DESC).map_err(|e| format!("Failed to write default description to {}: {e}", desc_dst.display()))?;

    if !no_commit {
        git_commit_non_templated("init")?;
    }

    println!("Initialization done");

    Ok(())
}
