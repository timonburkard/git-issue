use std::fs;

use crate::model::{
    Priority, cache_path, current_timestamp, git_commit, is_valid_iso_date, is_valid_state, is_valid_type, issue_dir, issue_meta_path,
    load_meta, user_handle_me,
};

#[allow(clippy::too_many_arguments)]
pub fn run(
    ids: Vec<String>,
    state: Option<String>,
    title: Option<String>,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
    labels_add: Option<Vec<String>>,
    labels_remove: Option<Vec<String>>,
) -> Result<(), String> {
    let ids: Vec<u32> = if ids.len() == 1 && ids[0] == "*" {
        read_cached_issue_ids()?
    } else {
        if ids.iter().any(|t| t == "*") {
            return Err("Cannot mix '*' with explicit IDs".to_string());
        }

        ids.iter()
            .map(|t| t.parse::<u32>().map_err(|_| format!("Invalid issue ID: {t}")))
            .collect::<Result<Vec<u32>, _>>()?
    };

    // Precondition: .gitissues/issues/ID must exist
    for id in &ids {
        let dir = issue_dir(*id);
        let path = dir.as_path();

        if !path.exists() {
            return Err("Not available: ID does not exist.".to_string());
        }
    }

    let mut any_issue_updated = false;

    for id in ids {
        // Load meta.yaml
        let meta_path = issue_meta_path(id);
        let meta = load_meta(&meta_path)?;

        // Update meta fields
        let mut updated_meta = meta.clone();

        let mut fields = Vec::new();

        if let Some(value) = title.as_deref()
            && updated_meta.title != value
        {
            updated_meta.title = value.to_string();
            fields.push("title");
        }

        if let Some(value) = state.as_deref()
            && updated_meta.state != value
        {
            match is_valid_state(value) {
                Ok(true) => { /* valid, continue */ }
                Ok(false) => return Err("Invalid state: Check config.yaml:states".to_string()),
                Err(e) => return Err(format!("Config error: {e}")),
            }

            updated_meta.state = value.to_string();
            fields.push("state");
        }

        if let Some(value) = type_.as_deref()
            && updated_meta.type_ != value
        {
            match is_valid_type(value) {
                Ok(true) => { /* valid, continue */ }
                Ok(false) => return Err("Invalid type: Check config.yaml:types".to_string()),
                Err(e) => return Err(format!("Config error: {e}")),
            }

            updated_meta.type_ = value.to_string();
            fields.push("type");
        }

        if let Some(value) = reporter.as_ref()
            && updated_meta.reporter != *value
        {
            match crate::model::is_valid_user(value) {
                Ok(true) => { /* valid, continue */ }
                Ok(false) => return Err("Invalid reporter: Check users.yaml:users:id, 'me' or ''".to_string()),
                Err(e) => return Err(format!("Config error: {e}")),
            }

            let mut value = value.clone();

            user_handle_me(&mut value)?;

            if updated_meta.reporter != *value {
                updated_meta.reporter = value.to_string();
                fields.push("reporter");
            }
        }

        if let Some(value) = assignee.as_ref()
            && updated_meta.assignee != *value
        {
            match crate::model::is_valid_user(value) {
                Ok(true) => { /* valid, continue */ }
                Ok(false) => return Err("Invalid assignee: Check users.yaml:users:id, 'me' or ''".to_string()),
                Err(e) => return Err(format!("Config error: {e}")),
            }

            let mut value = value.clone();

            user_handle_me(&mut value)?;

            if updated_meta.assignee != value {
                updated_meta.assignee = value;
                fields.push("assignee");
            }
        }

        if let Some(value) = priority
            && updated_meta.priority != value
        {
            updated_meta.priority = value;
            fields.push("priority");
        }

        if let Some(value) = due_date.as_deref()
            && updated_meta.due_date != value
        {
            match is_valid_iso_date(value) {
                Ok(true) => { /* valid, continue */ }
                Ok(false) => return Err("Invalid due_date format: Use 'YYYY-MM-DD' or ''".to_string()),
                Err(e) => return Err(format!("Error: {e}")),
            }

            updated_meta.due_date = value.to_string();
            fields.push("due_date");
        }

        if let Some(value) = labels.as_ref()
            && updated_meta.labels != *value
        {
            if value == &vec![""] {
                updated_meta.labels = Vec::new();
            } else {
                updated_meta.labels = value.clone();
            }

            fields.push("labels");
        }

        if let Some(value) = labels_add.as_ref() {
            for label in value {
                if !updated_meta.labels.contains(label) {
                    updated_meta.labels.push(label.clone());

                    if !fields.contains(&"labels") {
                        fields.push("labels");
                    }
                }
            }
        }

        if let Some(value) = labels_remove.as_ref() {
            for label in value {
                if updated_meta.labels.contains(label) {
                    updated_meta.labels.retain(|l| l != label);

                    if !fields.contains(&"labels") {
                        fields.push("labels");
                    }
                }
            }
        }

        if fields.is_empty() {
            continue;
        } else {
            any_issue_updated = true;
        }

        updated_meta.updated = current_timestamp();

        let updated_yaml = serde_yaml::to_string(&updated_meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;

        fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;

        git_commit(id, updated_meta.title, &format!("set {}", fields.join(",")))?;
    }

    if !any_issue_updated {
        return Err("No fields changed".to_string());
    }

    println!("Updated issue field(s)");

    Ok(())
}

fn read_cached_issue_ids() -> Result<Vec<u32>, String> {
    let cache_file = cache_path();

    let cache_content = fs::read_to_string(&cache_file).map_err(|e| format!("Failed to read cache file {}: {e}", cache_file.display()))?;
    let issue_ids: Result<Vec<u32>, _> = cache_content.split(',').map(|s| s.trim().parse::<u32>()).collect();

    if let Ok(value) = issue_ids {
        Ok(value)
    } else {
        Err("Failed to parse issue IDs from cache".to_string())
    }
}
