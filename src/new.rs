use std::fs;
use std::path::Path;

use indexmap::IndexMap;

use crate::model::{
    Meta, Priority, current_timestamp, git_commit, gitissues_base, is_valid_assignee, is_valid_iso_date, is_valid_type,
    issue_attachments_dir, issue_desc_path, issue_dir, issue_meta_path, load_config,
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

    // Step 2: Read config
    let config = load_config()?;

    if config.states.is_empty() {
        return Err("No states defined in config.yaml.".to_string());
    }

    // Step 3: Validate user inputs
    let type_val = type_.unwrap_or_default();
    match is_valid_type(&type_val) {
        Ok(true) => { /* valid, continue */ }
        Ok(false) => return Err("Invalid type: Check config.yaml:types".to_string()),
        Err(e) => return Err(format!("Config error: {e}")),
    }

    let assignee_val = assignee.unwrap_or_default();
    match is_valid_assignee(&assignee_val) {
        Ok(true) => { /* valid, continue */ }
        Ok(false) => return Err("Invalid assignee: Check users.yaml:users:id".to_string()),
        Err(e) => return Err(format!("Config error: {e}")),
    }

    let due_date_val = due_date.clone().unwrap_or_default();
    match is_valid_iso_date(&due_date_val) {
        Ok(true) => { /* valid, continue */ }
        Ok(false) => return Err(" Invalid due_date format: Use 'YYYY-MM-DD' or ''".to_string()),
        Err(e) => return Err(format!("Error: {e}")),
    }

    // Step 4: Create meta fields and validate
    let timestamp = current_timestamp();

    let meta = Meta {
        id: issue_id,
        title: title.clone(),
        state: config.states.first().cloned().unwrap_or_else(|| "new".to_string()),
        type_: type_val,
        labels: labels.unwrap_or_default(),
        assignee: assignee_val,
        priority: priority.unwrap_or(Priority::P2),
        due_date: due_date.unwrap_or_default(),
        relationships: IndexMap::new(),
        created: timestamp.clone(),
        updated: timestamp,
    };

    // Step 5: Create the issue directory
    let dir = issue_dir(issue_id);
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 6: Write description.md
    let desc_path = issue_desc_path(issue_id);

    let template_path = Path::new(gitissues_base()).join("description.md");
    fs::copy(&template_path, &desc_path).map_err(|e| format!("Failed to write description.md: {e}"))?;

    // Step 7: Create attachments directory
    let attachment_dir = issue_attachments_dir(issue_id);
    fs::create_dir_all(&attachment_dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 7.1: Add .gitkeep
    fs::write(attachment_dir.join(".gitkeep"), "").map_err(|e| format!("Failed to write .gitkeep: {e}"))?;

    // Step 8: Write meta.yaml
    let meta_yaml_path = issue_meta_path(issue_id);

    let meta_yaml = serde_yaml::to_string(&meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;
    fs::write(&meta_yaml_path, meta_yaml).map_err(|e| format!("Failed to write meta.yaml: {e}"))?;

    // Step 9: git commit
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
        return Err("Not initialized: .gitissues/issues does not exist. Run `git issue init` first.".to_string());
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
