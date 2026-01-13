use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;

use chrono::Utc;

use indexmap::IndexMap;

use crate::model::{
    IdGeneration, Meta, Priority, current_timestamp, git_commit, gitissues_base, is_valid_iso_date, is_valid_type, is_valid_user,
    issue_attachments_dir, issue_desc_path, issue_dir, issue_meta_path, issues_dir, load_config, load_settings, load_users, padded_id,
    user_handle_me,
};

pub fn run(
    title: String,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    // Step 1: Allocate the next issue ID
    let issue_id = generate_id()?;

    // Step 2: Read config
    let config = load_config()?;
    let settings = load_settings()?;
    let users = load_users()?;

    if config.states.is_empty() {
        return Err("No states defined in config.yaml.".to_string());
    }

    // Step 3: Validate user inputs
    let type_val = type_.unwrap_or_default();

    if !is_valid_type(&config, &type_val) {
        return Err(format!(
            "Invalid type \"{type_val}\". Valid options: {:?} | Configurable in config.yaml:types",
            config.types
        ));
    }

    let mut reporter_val = match reporter {
        Some(value) => {
            if !is_valid_user(&users, &value) {
                return Err(format!(
                    "Invalid reporter \"{value}\". Valid options: {:?} | Configurable in users.yaml:users",
                    users.users.iter().map(|u| u.id.as_str()).chain(["me", ""]).collect::<Vec<_>>()
                ));
            } else {
                value
            }
        }
        None => {
            let settings = load_settings()?;
            if !is_valid_user(&users, &settings.user) {
                return Err(format!(
                    "Invalid reporter \"{}\": settings.yaml::user must be part of users.yaml:users or ''",
                    settings.user
                ));
            } else {
                settings.user
            }
        }
    };

    user_handle_me(&users, &settings, &mut reporter_val)?;

    let mut assignee_val = assignee.unwrap_or_default();
    if !is_valid_user(&users, &assignee_val) {
        return Err(format!(
            "Invalid assignee \"{assignee_val}\". Valid options: {:?} | Configurable in users.yaml:users",
            users.users.iter().map(|u| u.id.as_str()).chain(["me", ""]).collect::<Vec<_>>()
        ));
    }

    user_handle_me(&users, &settings, &mut assignee_val)?;

    let due_date_val = due_date.clone().unwrap_or_default();
    match is_valid_iso_date(&due_date_val) {
        Ok(true) => { /* valid, continue */ }
        Ok(false) => return Err(" Invalid due_date format: Use 'YYYY-MM-DD' or ''".to_string()),
        Err(e) => return Err(format!("Error: {e}")),
    }

    let mut labels_val = labels.unwrap_or_default();
    labels_val.retain(|label| !label.is_empty());

    // Step 4: Create meta fields and validate
    let timestamp = current_timestamp();

    let meta = Meta {
        _version: 1,
        id: issue_id,
        title: title.clone(),
        state: config.states.first().cloned().unwrap_or_else(|| "new".to_string()),
        type_: type_val,
        labels: labels_val,
        reporter: reporter_val,
        assignee: assignee_val,
        priority: priority.unwrap_or(config.priority_default),
        due_date: due_date.unwrap_or_default(),
        relationships: IndexMap::new(),
        created: timestamp.clone(),
        updated: timestamp,
    };

    // Step 5: Create the issue directory
    let dir = issue_dir(issue_id)?;
    fs::create_dir_all(&dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 6: Write description.md
    let desc_path = issue_desc_path(issue_id)?;

    let template_path = gitissues_base()?.join("description.md");
    fs::copy(&template_path, &desc_path).map_err(|e| format!("Failed to write description.md: {e}"))?;

    // Step 7: Create attachments directory
    let attachment_dir = issue_attachments_dir(issue_id)?;
    fs::create_dir_all(&attachment_dir).map_err(|e| format!("Failed to create issue directory: {e}"))?;

    // Step 7.1: Add .gitkeep
    fs::write(attachment_dir.join(".gitkeep"), "").map_err(|e| format!("Failed to write .gitkeep: {e}"))?;

    // Step 8: Write meta.yaml
    let meta_yaml_path = issue_meta_path(issue_id)?;

    let meta_yaml = serde_yaml::to_string(&meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;
    fs::write(&meta_yaml_path, meta_yaml).map_err(|e| format!("Failed to write meta.yaml: {e}"))?;

    // Step 9: git commit
    git_commit(issue_id, title, "new")?;

    println!("Created issue #{issue_id}");

    Ok(())
}

/// Generates new ID
fn generate_id() -> Result<u32, String> {
    let path = issues_dir()?;

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err("Not initialized: .gitissues/issues does not exist. Run `git issue init` first.".to_string());
    }

    let config = load_config()?;

    let id = match config.id_generation {
        IdGeneration::Sequential => generate_id_sequential(&path)?,
        IdGeneration::Timestamp => generate_id_timestamp(&path)?,
    };

    Ok(id)
}

fn generate_id_sequential(issues_dir: &Path) -> Result<u32, String> {
    let mut max_id = 0u32;

    // Read directory entries and find the highest numeric ID
    for entry in fs::read_dir(issues_dir).map_err(|e| format!("Failed to read issues directory: {e}"))? {
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

    Ok(max_id + 1)
}

fn generate_id_timestamp(issues_dir: &Path) -> Result<u32, String> {
    const START_2025: i64 = 1735689600;

    let mut id = (Utc::now().timestamp() - START_2025) as u32;

    let mut issue_dir = issues_dir.join(padded_id(id));

    if issue_dir.exists() {
        // In the rare case of a collision, wait one second and try again
        thread::sleep(Duration::from_secs(1));

        id = (Utc::now().timestamp() - START_2025) as u32;

        issue_dir = issues_dir.join(padded_id(id));

        if issue_dir.exists() {
            return Err("Failed to generate unique ID using timestamp due to collision.".to_string());
        }
    }

    Ok(id)
}
