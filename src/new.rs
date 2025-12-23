use std::fs;
use std::path::Path;

use crate::model::{
    Meta, Priority, current_timestamp, git_commit, is_valid_iso_date, issue_attachments_dir,
    issue_desc_path, issue_dir, issue_meta_path,
};

pub fn run(
    title: String,
    type_: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    // Step 1: Allocate the next issue ID
    let issue_id = allocate_id()?;

    // Step 2: Create meta fields and validate
    let timestamp = current_timestamp();
    let meta = Meta {
        id: issue_id,
        title: title.clone(),
        state: "new".to_string(),
        type_: type_.unwrap_or_default(),
        labels: labels.unwrap_or_default(),
        assignee: assignee.unwrap_or_default(),
        priority: priority.unwrap_or(Priority::P2),
        due_date: due_date.unwrap_or_default(),
        created: timestamp.clone(),
        updated: timestamp,
    };

    if !meta.due_date.is_empty() && !is_valid_iso_date(&meta.due_date) {
        return Err("Invalid due_date format: Use YYYY-MM-DD".to_string());
    }

    // Step 3: Create the issue directory
    let dir = issue_dir(issue_id);
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 4: Write description.md
    let desc_path = issue_desc_path(issue_id);

    fs::copy(".gitissues/description.md", &desc_path)
        .map_err(|e| format!("Failed to write description.md: {e}"))?;

    // Step 5: Create attachments directory
    let attachment_dir = issue_attachments_dir(issue_id);
    fs::create_dir_all(&attachment_dir)
        .map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 5.1: Add .gitkeep
    fs::write(attachment_dir.join(".gitkeep"), "")
        .map_err(|e| format!("Failed to write .gitkeep: {e}"))?;

    // Step 6: Write meta.yaml
    let meta_yaml_path = issue_meta_path(issue_id);

    let meta_yaml =
        serde_yaml::to_string(&meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;
    fs::write(&meta_yaml_path, meta_yaml).map_err(|e| format!("Failed to write meta.yaml: {e}"))?;

    // Step 7: git commit
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
