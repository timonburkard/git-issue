use std::fs;
use std::path::Path;

use crate::model::{Meta, load_config, load_meta};

pub fn run(columns: Option<Vec<String>>) -> Result<(), String> {
    let issues_base = ".gitissues/issues";
    let path = Path::new(issues_base);

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err(
            "Not initialized: .gitissues/issues does not exist. Run `git issue init` first."
                .to_string(),
        );
    }

    // Collect issue metadata
    let mut issues: Vec<Meta> = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| format!("Failed to read issues directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        if !entry
            .file_type()
            .map_err(|e| format!("Failed to read file type: {e}"))?
            .is_dir()
        {
            continue;
        }

        let dir_name = entry.file_name();
        let name_str = dir_name.to_string_lossy().to_string();

        // Try to parse the directory name as a u32 (issue ID)
        if name_str.parse::<u32>().is_err() {
            continue; // skip non-numeric directories
        }

        // Load meta.yaml
        let meta_path = entry.path().join("meta.yaml");
        let meta = load_meta(&meta_path)?;

        issues.push(meta);
    }

    // Sort by numeric ID
    issues.sort_by_key(|m| m.id);

    if columns.is_none() {
        print_default_list(&issues)?;

        return Ok(());
    }

    print_custom_list(&issues, columns.unwrap())?;

    Ok(())
}

fn validate_config_columns(columns: &Vec<String>) -> Result<(), String> {
    for col in columns {
        if ![
            "id", "title", "state", "type", "labels", "assignee", "created", "updated",
        ]
        .contains(&col.as_str())
        {
            return Err(format!(
                "Invalid column name in config.list_columns: {}",
                col
            ));
        }
    }

    Ok(())
}

fn print_default_list(issues: &Vec<Meta>) -> Result<(), String> {
    let config = load_config()?;

    let columns = config.list_columns;

    validate_config_columns(&columns)?;

    let column_widths = calculate_column_widths(issues, &columns);

    // Header
    for col in &columns {
        print!(
            "{:<width$}",
            col,
            width = column_widths.get(col).copied().unwrap_or(22)
        );
    }
    println!();

    // Rows
    for meta in issues {
        for col in &columns {
            let value = get_column_value(col, meta);
            print!(
                "{:<width$}",
                value,
                width = column_widths.get(col).copied().unwrap_or(22)
            );
        }
        println!();
    }

    Ok(())
}

fn print_custom_list(issues: &Vec<Meta>, mut columns: Vec<String>) -> Result<(), String> {
    // Validate column names
    for col in &columns {
        if !&[
            "id", "title", "state", "type", "labels", "assignee", "created", "updated", "*",
        ]
        .contains(&col.as_str())
        {
            return Err(format!("Invalid column name: {}", col));
        }
    }

    // Wildcard
    if columns.contains(&"*".to_string()) {
        columns = vec![
            "id".to_string(),
            "state".to_string(),
            "assignee".to_string(),
            "type".to_string(),
            "labels".to_string(),
            "title".to_string(),
            "created".to_string(),
            "updated".to_string(),
        ];
    }

    let column_widths = calculate_column_widths(issues, &columns);

    // Print header
    for col in &columns {
        print!(
            "{:<width$}",
            col,
            width = column_widths.get(col).copied().unwrap_or(22)
        );
    }

    println!();

    // Print rows
    for meta in issues {
        for col in &columns {
            let value = get_column_value(col, meta);
            print!(
                "{:<width$}",
                value,
                width = column_widths.get(col).copied().unwrap_or(22)
            );
        }
        println!();
    }

    Ok(())
}

fn get_column_value(col: &str, meta: &Meta) -> String {
    match col {
        "id" => meta.id.to_string(),
        "title" => meta.title.clone(),
        "state" => meta.state.clone(),
        "type" => {
            if meta.type_.is_empty() {
                "-".to_string()
            } else {
                meta.type_.clone()
            }
        }
        "labels" => {
            if meta.labels.is_empty() {
                "-".to_string()
            } else {
                meta.labels.join(",")
            }
        }
        "assignee" => {
            if meta.assignee.is_empty() {
                "-".to_string()
            } else {
                meta.assignee.clone()
            }
        }
        "created" => meta.created.clone(),
        "updated" => meta.updated.clone(),
        _ => "-".to_string(),
    }
}

fn calculate_column_widths(
    issues: &[Meta],
    columns: &[String],
) -> std::collections::HashMap<String, usize> {
    use std::collections::HashMap;
    let mut widths: HashMap<String, usize> = HashMap::new();

    // Initialize with header widths
    for col in columns {
        widths.insert(col.clone(), col.len());
    }

    // Update with max content widths
    for meta in issues {
        for col in columns {
            let value = get_column_value(col, meta);
            let width = widths.get(col).copied().unwrap_or(0);
            widths.insert(col.clone(), width.max(value.len()));
        }
    }

    // Add padding (2 spaces)
    for width in widths.values_mut() {
        *width += 2;
    }

    widths
}
