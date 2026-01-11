use std::fs;
use std::io::{self, Write};
use std::time::Duration;

use crate::model::{
    Priority, cache_path, current_timestamp, git_commit, is_valid_iso_date, is_valid_state, is_valid_type, is_valid_user, issue_dir,
    issue_meta_path, load_config, load_meta, load_settings, load_users, user_handle_me,
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
    let config = load_config()?;
    let settings = load_settings()?;
    let users = load_users()?;

    let using_wildcard = ids.len() == 1 && ids[0] == "*";

    let ids: Vec<u32> = if using_wildcard {
        read_cached_issue_ids()?
    } else {
        if ids.iter().any(|t| t == "*") {
            return Err("Cannot mix '*' with explicit IDs".to_string());
        }

        ids.iter()
            .map(|t| t.parse::<u32>().map_err(|_| format!("Invalid issue ID: {t}")))
            .collect::<Result<Vec<u32>, _>>()?
    };

    if using_wildcard {
        wildcard_confirmation(ids.len())?;
    }

    // Precondition: .gitissues/issues/ID must exist
    for id in &ids {
        let dir = issue_dir(*id)?;
        let path = dir.as_path();

        if !path.exists() {
            return Err(format!("Not available: ID #{id} does not exist."));
        }
    }

    let mut num_updated_issues = 0;

    for id in ids {
        // Load meta.yaml
        let meta_path = issue_meta_path(id)?;
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
            if !is_valid_state(&config, value) {
                return Err(format!(
                    "Invalid state \"{value}\". Valid options: {:?} | Configurable in config.yaml:states",
                    config.states
                ));
            }

            updated_meta.state = value.to_string();
            fields.push("state");
        }

        if let Some(value) = type_.as_deref()
            && updated_meta.type_ != value
        {
            if !is_valid_type(&config, value) {
                return Err(format!(
                    "Invalid type \"{value}\". Valid options: {:?} | Configurable in config.yaml:types",
                    config.types
                ));
            }

            updated_meta.type_ = value.to_string();
            fields.push("type");
        }

        if let Some(value) = reporter.as_ref()
            && updated_meta.reporter != *value
        {
            if !is_valid_user(&users, value) {
                return Err(format!(
                    "Invalid reporter \"{value}\". Valid options: {:?} | Configurable in users.yaml:users",
                    users.users.iter().map(|u| u.id.as_str()).chain(["me", ""]).collect::<Vec<_>>()
                ));
            }

            let mut value = value.clone();

            user_handle_me(&users, &settings, &mut value)?;

            if updated_meta.reporter != *value {
                updated_meta.reporter = value.to_string();
                fields.push("reporter");
            }
        }

        if let Some(value) = assignee.as_ref()
            && updated_meta.assignee != *value
        {
            if !is_valid_user(&users, value) {
                return Err(format!(
                    "Invalid assignee \"{value}\". Valid options: {:?} | Configurable in users.yaml:users",
                    users.users.iter().map(|u| u.id.as_str()).chain(["me", ""]).collect::<Vec<_>>()
                ));
            }

            let mut value = value.clone();

            user_handle_me(&users, &settings, &mut value)?;

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
            let mut labels_val = value.clone();
            labels_val.retain(|label| !label.is_empty());

            updated_meta.labels = labels_val;
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
        }

        updated_meta.updated = current_timestamp();

        let updated_yaml = serde_yaml::to_string(&updated_meta).map_err(|_| "Failed to serialize meta.yaml".to_string())?;

        fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;

        git_commit(id, updated_meta.title, &format!("set {}", fields.join(",")))?;

        num_updated_issues += 1;
    }

    match num_updated_issues {
        0 => return Err("No fields changed".to_string()),
        1 => println!("Updated issue field(s)"),
        _ => println!("Updated {} issues' field(s)", num_updated_issues),
    };

    Ok(())
}

fn read_cached_issue_ids() -> Result<Vec<u32>, String> {
    let cache_file = cache_path()?;

    // ensure cache file is not too old
    let metadata = fs::metadata(&cache_file).map_err(|_| "Cached ID list is empty; run 'git issue list' first.".to_string())?;
    if let Ok(modified) = metadata.modified()
        && let Ok(elapsed) = modified.elapsed()
        && elapsed > Duration::from_secs(300)
    {
        return Err("Cached ID list is stale; run 'git issue list' first.".to_string());
    }

    let cache_content = fs::read_to_string(&cache_file).map_err(|_| "Cached ID list is empty; run 'git issue list' first.".to_string())?;
    let issue_ids: Result<Vec<u32>, _> = cache_content.split(',').map(|s| s.trim().parse::<u32>()).collect();

    if let Ok(value) = issue_ids {
        Ok(value)
    } else {
        Err("Cached ID list is empty; run 'git issue list' first.".to_string())
    }
}

fn wildcard_confirmation(num_of_ids: usize) -> Result<(), String> {
    println!("Modify {} issues from last list cache.", num_of_ids);
    print!("Continue? [y/N] ");
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {e}"))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    if !input.trim().eq_ignore_ascii_case("y") {
        return Err("Cancelled".to_string());
    }

    Ok(())
}
