use std::fs;
use std::path::Path;

use crate::model::{Meta, git_commit, load_config, open_editor};

pub fn run(id: u32) -> Result<(), String> {
    let id_str = format!("{id:010}");
    let desc_path = format!(".gitissues/issues/{id_str}/description.md");
    let path = Path::new(&desc_path);

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    let config = load_config()?;

    open_editor(config.editor, desc_path)?;

    // Load meta.yaml to get title for commit message
    let meta = format!(".gitissues/issues/{id_str}/meta.yaml");
    let meta_path = Path::new(&meta);
    let meta_raw = match fs::read_to_string(meta_path) {
        Ok(s) => s,
        Err(_) => return Err("meta.yaml not found.".to_string()),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err("meta.yaml malformatted.".to_string()),
    };

    git_commit(id, meta.title, "edit description")?;

    Ok(())
}
