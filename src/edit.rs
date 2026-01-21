use std::path::PathBuf;

use crate::model::{git_commit, issue_desc_path, issue_title};

/// Start editing the description of an issue
/// Returns the path to the description file
pub fn edit_start(id: u32) -> Result<PathBuf, String> {
    let desc = issue_desc_path(id)?;
    let path = desc.as_path();

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    Ok(desc)
}

/// Finalize editing the description of an issue
pub fn edit_end(id: u32) -> Result<Option<String>, String> {
    let title = issue_title(id)?;

    git_commit(id, title, "edit description")
}
