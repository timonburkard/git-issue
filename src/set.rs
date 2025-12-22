use std::fs;
use std::path::Path;

use crate::model::{Meta, current_timestamp, git_commit};

pub fn run(
    id: u32,
    state: Option<String>,
    title: Option<String>,
    type_: Option<String>,
    assignee: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    let id_str = format!("{id:010}");
    let issue_path = format!(".gitissues/issues/{id_str}");
    let path = Path::new(&issue_path);

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    // Load meta.yaml
    let meta_path = path.join("meta.yaml");
    let meta_raw = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(_) => return Err("meta.yaml not found.".to_string()),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err("meta.yaml malformatted.".to_string()),
    };

    // Update meta fields
    let mut updated_meta = meta.clone();

    let mut fields = Vec::new();

    if let Some(value) = state
        && updated_meta.state != value
    {
        updated_meta.state = value;
        fields.push("state");
    }

    if let Some(value) = title
        && updated_meta.title != value
    {
        updated_meta.title = value;
        fields.push("title");
    }

    if let Some(value) = type_
        && updated_meta.type_ != value
    {
        updated_meta.type_ = value;
        fields.push("type");
    }

    if let Some(value) = assignee
        && updated_meta.assignee != value
    {
        updated_meta.assignee = value;
        fields.push("assignee");
    }

    if let Some(value) = labels {
        for label in value {
            if !updated_meta.labels.contains(&label) {
                updated_meta.labels.push(label);

                if !fields.contains(&"labels") {
                    fields.push("labels");
                }
            }
        }
    }

    if fields.is_empty() {
        return Err("No fields changed".to_string());
    }

    updated_meta.updated = current_timestamp();

    let updated_yaml = serde_yaml::to_string(&updated_meta)
        .map_err(|_| "Failed to serialize meta.yaml".to_string())?;

    fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;

    git_commit(id, updated_meta.title, &format!("set {}", fields.join(",")));

    println!("Updated issue field(s)");

    Ok(())
}
