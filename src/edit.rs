use crate::model::{git_commit, issue_desc_path, issue_title, load_settings, open_editor};

pub fn run(id: u32) -> Result<(), String> {
    let desc = issue_desc_path(id)?;
    let path = desc.as_path();

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    let settings = load_settings()?;

    open_editor(settings.editor, desc.to_string_lossy().to_string())?;

    let title = issue_title(id)?;

    git_commit(id, title, "edit description")?;

    Ok(())
}
