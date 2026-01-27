use std::fs;
use std::path::PathBuf;

use crate::cmd::util::git_commit_non_templated;
use crate::model::{config_path, create_settings_if_missing, gitissues_base, users_path};
use crate::{Cmd, CmdResult};

pub fn init(no_commit: bool) -> Cmd<()> {
    let root = PathBuf::from(".gitissues");

    if root.exists() {
        return Err("Already initialized: .gitissues already exists".to_string());
    }

    let issues_dir = root.join("issues");

    // Create the directory structure
    fs::create_dir_all(&issues_dir).map_err(|e| format!("Failed to create {}: {e}", issues_dir.display()))?;

    // Copy default config file
    const DEFAULT_CONFIG: &str = include_str!("../../config/config-default.yaml");
    let config_dst = config_path()?;
    fs::write(&config_dst, DEFAULT_CONFIG).map_err(|e| format!("Failed to write default config to {}: {e}", config_dst.display()))?;

    // Copy default settings file
    let mut infos = create_settings_if_missing(false)?;

    // Copy default users file
    const DEFAULT_USERS: &str = include_str!("../../config/users-default.yaml");
    let users_dst = users_path()?;
    fs::write(&users_dst, DEFAULT_USERS).map_err(|e| format!("Failed to write default users to {}: {e}", users_dst.display()))?;

    // Copy default description file
    const DEFAULT_DESC: &str = include_str!("../../config/description-default.md");
    let desc_dst = gitissues_base()?.join("description.md");
    fs::write(&desc_dst, DEFAULT_DESC).map_err(|e| format!("Failed to write default description to {}: {e}", desc_dst.display()))?;

    if !no_commit {
        match git_commit_non_templated("init") {
            Ok(infos_commit) => {
                infos.extend(infos_commit);
                return Ok(CmdResult { value: (), infos });
            }
            Err(e) => return Err(e),
        }
    }

    Ok(CmdResult { value: (), infos })
}
