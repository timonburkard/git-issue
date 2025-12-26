use std::fs;
use std::path::Path;

use regex::Regex;

use crate::model::{Filter, Meta, dash_if_empty, gitissues_base, load_config, load_meta};

pub fn run(columns: Option<Vec<String>>, filter: Option<Vec<Filter>>) -> Result<(), String> {
    let path = Path::new(gitissues_base()).join("issues");

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err("Not initialized: .gitissues/issues does not exist. Run `git issue init` first.".to_string());
    }

    // Collect issue metadata
    let mut issues: Vec<Meta> = Vec::new();

    for entry in fs::read_dir(path).map_err(|e| format!("Failed to read issues directory: {e}"))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        if !entry.file_type().map_err(|e| format!("Failed to read file type: {e}"))?.is_dir() {
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

    // Apply filters
    filter_issues(&mut issues, filter)?;

    // Print
    match columns {
        None => {
            print_default_list(&issues)?;
        }
        Some(cols) => {
            print_custom_list(&issues, cols)?;
        }
    }

    Ok(())
}

fn validate_column_names(columns: &mut [String], context: &str) -> Result<(), String> {
    for col in columns.iter_mut() {
        // normalize aliases
        if col == "due-date" {
            *col = "due_date".to_string();
        }

        if ![
            "id", "title", "state", "type", "labels", "assignee", "priority", "due_date", "created", "updated", "*",
        ]
        .contains(&col.as_str())
        {
            return Err(format!("Invalid column name in {}: {}", context, col));
        }
    }

    Ok(())
}

fn print_default_list(issues: &Vec<Meta>) -> Result<(), String> {
    let config = load_config()?;

    let mut columns = config.list_columns;

    validate_column_names(&mut columns, "config.yaml:list_columns")?;

    wildcard_expansion(&mut columns);

    let column_widths = calculate_column_widths(issues, &columns)?;

    // Header
    for col in &columns {
        let width = *column_widths.get(col).unwrap_or(&22);
        print!("{:<width$}", col, width = width);
    }
    println!();

    // Rows
    for meta in issues {
        for col in &columns {
            let value = get_column_value(col, meta)?;
            let width = *column_widths.get(col).unwrap_or(&22);
            print!("{:<width$}", value, width = width);
        }
        println!();
    }

    Ok(())
}

fn print_custom_list(issues: &Vec<Meta>, mut columns: Vec<String>) -> Result<(), String> {
    validate_column_names(&mut columns, "--columns")?;

    wildcard_expansion(&mut columns);

    let column_widths = calculate_column_widths(issues, &columns)?;

    // Print header
    for col in &columns {
        let width = *column_widths.get(col).unwrap_or(&22);
        print!("{:<width$}", col, width = width);
    }

    println!();

    // Print rows
    for meta in issues {
        for col in &columns {
            let value = get_column_value(col, meta)?;
            let width = *column_widths.get(col).unwrap_or(&22);
            print!("{:<width$}", value, width = width);
        }
        println!();
    }

    Ok(())
}

fn wildcard_expansion(columns: &mut Vec<String>) {
    if columns.contains(&"*".to_string()) {
        *columns = vec![
            "id".to_string(),
            "state".to_string(),
            "assignee".to_string(),
            "type".to_string(),
            "title".to_string(),
            "priority".to_string(),
            "labels".to_string(),
            "due_date".to_string(),
            "created".to_string(),
            "updated".to_string(),
        ];
    }
}

fn get_column_value(col: &str, meta: &Meta) -> Result<String, String> {
    match col {
        "id" => Ok(meta.id.to_string()),
        "title" => Ok(meta.title.clone()),
        "state" => Ok(meta.state.clone()),
        "type" => Ok(dash_if_empty(&meta.type_)),
        "labels" => Ok(dash_if_empty(&meta.labels.join(","))),
        "assignee" => Ok(dash_if_empty(&meta.assignee)),
        "priority" => Ok(format!("{:?}", meta.priority)),
        "due_date" => Ok(dash_if_empty(&meta.due_date)),
        "created" => Ok(meta.created.clone()),
        "updated" => Ok(meta.updated.clone()),
        _ => Err(format!("Unknown column: {}", col)),
    }
}

fn calculate_column_widths(issues: &[Meta], columns: &[String]) -> Result<std::collections::HashMap<String, usize>, String> {
    use std::collections::HashMap;
    let mut widths: HashMap<String, usize> = HashMap::new();

    // Initialize with header widths
    for col in columns {
        widths.insert(col.clone(), col.len());
    }

    // Update with max content widths
    for meta in issues {
        for col in columns {
            let value = get_column_value(col, meta)?;
            let width = widths.get(col).copied().unwrap_or(0);
            widths.insert(col.clone(), width.max(value.len()));
        }
    }

    // Add padding (2 spaces)
    for width in widths.values_mut() {
        *width += 2;
    }

    Ok(widths)
}

fn filter_issues(issues: &mut Vec<Meta>, filters: Option<Vec<Filter>>) -> Result<(), String> {
    if let Some(mut filters) = filters {
        // Validate all filter fields
        let mut filter_fields: Vec<String> = filters.iter().map(|f| f.field.clone()).collect();
        validate_column_names(&mut filter_fields, "--filter")?;

        // Update the actual filter struct with normalized field names
        for (filter, normalized) in filters.iter_mut().zip(filter_fields) {
            filter.field = normalized;
        }

        // Apply filters
        issues.retain(|meta| {
            filters.iter().all(|filter| match filter.field.as_str() {
                "id" => do_strings_match(&meta.id.to_string(), &filter.value),
                "title" => do_strings_match(&meta.title, &filter.value),
                "state" => do_strings_match(&meta.state, &filter.value),
                "type" => do_strings_match(&meta.type_, &filter.value),
                "labels" => meta.labels.iter().any(|l| do_strings_match(l, &filter.value)),
                "assignee" => do_strings_match(&meta.assignee, &filter.value),
                "priority" => do_strings_match(&format!("{:?}", meta.priority), &filter.value),
                "due_date" => do_strings_match(&meta.due_date, &filter.value),
                "created" => do_strings_match(&meta.created, &filter.value),
                "updated" => do_strings_match(&meta.updated, &filter.value),
                _ => unreachable!("All filter fields should have been validated above"),
            })
        });
    }
    Ok(())
}

fn do_strings_match(value: &str, pattern: &str) -> bool {
    let value = value.trim().to_lowercase();
    let pattern = pattern.trim().to_lowercase();

    // Escape regex special characters except '*'
    let regex_pattern = regex::escape(&pattern).replace(r"\*", ".*");

    // Add anchors to match the whole string
    let regex_pattern = format!("^{}$", regex_pattern);

    // Compile the regex
    let re = match Regex::new(&regex_pattern) {
        Ok(re) => re,
        Err(_) => return false, // invalid regex
    };

    re.is_match(&value)
}
