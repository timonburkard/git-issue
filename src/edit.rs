use std::path::Path;

use crate::model::{git_commit, load_config, load_meta, open_editor};

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
    let meta_path_str = format!(".gitissues/issues/{id_str}/meta.yaml");
    let meta_path = Path::new(&meta_path_str);
    let meta = load_meta(meta_path)?;

    git_commit(id, meta.title, "edit description")?;

    Ok(())
}
