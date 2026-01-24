use std::path::PathBuf;

use crate::model::{git_commit, issue_desc_path, issue_title};
use crate::{Cmd, CmdResult};

/// Start editing the description of an issue
/// Returns the path to the description file
pub fn edit_start(id: u32) -> Cmd<PathBuf> {
    let desc_path = issue_desc_path(id)?;
    let path = desc_path.as_path();

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    Ok(CmdResult {
        value: desc_path,
        infos: vec![],
    })
}

/// Finalize editing the description of an issue
pub fn edit_end(id: u32) -> Cmd<()> {
    let title = issue_title(id)?;

    let result = git_commit(id, title, "edit description");

    match result {
        Ok(infos) => Ok(CmdResult { value: (), infos }),
        Err(e) => Err(e),
    }
}
