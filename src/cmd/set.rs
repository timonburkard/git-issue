use std::fs;

use crate::cmd::util::{git_commit, load_meta, user_handle_me};
use crate::model::{
    Priority, current_timestamp, is_valid_iso_date, is_valid_state, is_valid_type, is_valid_user, issue_dir, issue_meta_path, load_config,
    load_settings, load_users,
};
use crate::{Cmd, CmdResult};

/// Set metadata fields of issues
/// Returns number of issues updated and optional info messages
#[allow(clippy::too_many_arguments)]
pub fn set(
    ids: Vec<u32>,
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
) -> Cmd<u32> {
    let config = load_config()?;
    let (settings, mut infos) = load_settings()?;
    let users = load_users()?;

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
                updated_meta.reporter = value;
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

        let info_commit = git_commit(id, updated_meta.title, &format!("set {}", fields.join(",")))?;

        infos.extend(info_commit);

        num_updated_issues += 1;
    }

    Ok(CmdResult {
        value: num_updated_issues,
        infos,
    })
}
