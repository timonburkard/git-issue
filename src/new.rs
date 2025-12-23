use std::fs;
use std::path::Path;

use crate::model::{
    Meta, Priority, current_timestamp, git_commit, issue_attachments_dir, issue_desc_path,
    issue_dir, issue_meta_path,
};

pub fn run(title: String) -> Result<(), String> {
    // Step 1: Allocate the next issue ID
    let issue_id = allocate_id()?;

    // Step 2: Create the issue directory
    let dir = issue_dir(issue_id);
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 3: Write description.md
    let desc_path = issue_desc_path(issue_id);

    fs::copy(".gitissues/description.md", &desc_path)
        .map_err(|e| format!("Failed to write description.md: {e}"))?;

    // Step 4: Create attachments directory
    let attachment_dir = issue_attachments_dir(issue_id);
    fs::create_dir_all(&attachment_dir)
        .map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 4.1: Add .gitkeep
    fs::write(attachment_dir.join(".gitkeep"), "")
        .map_err(|e| format!("Failed to write .gitkeep: {e}"))?;

    // Step 5: Write meta.yaml
    let meta_yaml_path = issue_meta_path(issue_id);
    let timestamp = current_timestamp();
    let meta = Meta {
        id: issue_id,
        title: title.clone(),
        state: "new".to_string(),
        type_: "".to_string(),
        labels: vec![],
        assignee: "".to_string(),
        priority: Priority::P2,
        created: timestamp.clone(),
        updated: timestamp,
    };
    let meta_yaml =
        serde_yaml::to_string(&meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;
    fs::write(&meta_yaml_path, meta_yaml).map_err(|e| format!("Failed to write meta.yaml: {e}"))?;

    // Step 6: git commit
    git_commit(issue_id, title, "new")?;

    println!("Created issue #{issue_id}");

    Ok(())
}

/// Allocate the next sequential issue ID.
/// Scans .gitissues/issues/ for existing numeric directories,
/// finds the max, and returns max+1 as a u32.
fn allocate_id() -> Result<u32, String> {
    let issues_base = ".gitissues/issues";
    let path = Path::new(issues_base);

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err(
            "Not initialized: .gitissues/issues does not exist. Run `git issue init` first."
                .to_string(),
        );
    }

    let mut max_id = 0u32;

    // Read directory entries and find the highest numeric ID
    for entry in fs::read_dir(path).map_err(|e| format!("Failed to read issues directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Try to parse the directory name as a u32
        if let Ok(id) = name_str.parse::<u32>()
            && id > max_id
        {
            max_id = id;
        }
        // Silently skip non-numeric directory names
    }

    let next_id = max_id + 1;
    Ok(next_id)
}
