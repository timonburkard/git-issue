use crate::model::{
    git_commit, issue_desc_path, issue_meta_path, load_config, load_meta, open_editor,
};

pub fn run(id: u32) -> Result<(), String> {
    let desc = issue_desc_path(id);
    let path = desc.as_path();

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    let config = load_config()?;

    open_editor(config.editor, desc.to_string_lossy().to_string())?;

    // Load meta.yaml to get title for commit message
    let meta_path = issue_meta_path(id);
    let meta = load_meta(&meta_path)?;

    git_commit(id, meta.title, "edit description")?;

    Ok(())
}
