use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

use regex::Regex;

use crate::cmd::model::{
    Config, Filter, Meta, Operator, Priority, Settings, Sorting, Users, dash_if_empty, issue_desc_path, issues_dir, load_config,
    load_description, load_meta, load_settings, load_users, user_handle_me,
};

// (ID, {column: value, ...})
pub type IssueData = (u32, HashMap<String, String>);

// (Vec<IssueData>, Vec<column_names>, Option<info>)
pub type ListResult = (Vec<IssueData>, Vec<String>, Option<String>);

/// List issues with optional columns, filters, and sorting
pub fn list(columns: Option<Vec<String>>, filter: Option<Vec<Filter>>, sort: Option<Vec<Sorting>>) -> Result<ListResult, String> {
    let config = load_config()?;
    let (settings, info) = load_settings()?;

    let mut issues = get_issues_metadata()?;

    sort_issues(&config, &mut issues, sort)?;

    filter_issues(&config, &settings, &mut issues, filter)?;

    let mut cols = match &columns {
        Some(value) => value.clone(),
        None => config.list_columns.clone(),
    };

    let context = if columns.is_some() {
        "--columns"
    } else {
        "config.yaml:list_columns"
    };

    wildcard_expansion(&config, &mut cols);

    validate_column_names(&config, &mut cols, context)?;

    let mut issues_data: Vec<(u32, HashMap<String, String>)> = Vec::new();

    for meta in issues {
        let mut map = HashMap::new();

        for col in &cols {
            let value = get_column_value(col, &meta)?;
            map.insert(col.clone(), value);
        }

        issues_data.push((meta.id, map));
    }

    Ok((issues_data, cols, info))
}

fn get_issues_metadata() -> Result<Vec<Meta>, String> {
    let path = issues_dir()?;
    let mut issues: Vec<Meta> = Vec::new();

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err("Not initialized: .gitissues/issues does not exist. Run `git issue init` first.".to_string());
    }

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

    Ok(issues)
}

fn get_all_column_names(config: &Config) -> Vec<String> {
    let mut columns = vec![
        "id".to_string(),
        "title".to_string(),
        "state".to_string(),
        "type".to_string(),
        "labels".to_string(),
        "reporter".to_string(),
        "assignee".to_string(),
        "priority".to_string(),
        "due_date".to_string(),
    ];

    columns.extend(config.relationships.keys().cloned().collect::<Vec<String>>());

    columns.extend(vec!["created".to_string(), "updated".to_string()]);

    columns
}

fn validate_column_names(config: &Config, columns: &mut [String], context: &str) -> Result<(), String> {
    for col in columns.iter_mut() {
        // normalize aliases
        if col == "due-date" {
            *col = "due_date".to_string();
        }

        let mut valid_columns = get_all_column_names(config);

        if context == "--filter" {
            valid_columns.push("description".to_string());
        }

        if !valid_columns.contains(col) {
            return Err(format!("Invalid column name in {}: {}", context, col));
        }
    }

    Ok(())
}

fn wildcard_expansion(config: &Config, columns: &mut Vec<String>) {
    if columns.contains(&"*".to_string()) {
        *columns = get_all_column_names(config);
    }
}

fn filter_issues(config: &Config, settings: &Settings, issues: &mut Vec<Meta>, filters: Option<Vec<Filter>>) -> Result<(), String> {
    if let Some(mut filters) = filters {
        // Validate all filter fields
        let mut filter_fields: Vec<String> = filters.iter().map(|f| f.field.clone()).collect();
        validate_column_names(config, &mut filter_fields, "--filter")?;

        // Update the actual filter struct with normalized field names
        for (filter, normalized) in filters.iter_mut().zip(filter_fields) {
            filter.field = normalized;
        }

        validate_filters(&filters)?;

        let users = load_users()?;

        // Apply filters
        issues.retain(|meta| {
            filters.iter().all(|filter| match filter.operator {
                Operator::Eq => filter_eq(filter, meta, settings, &users),
                Operator::Gt => filter_gt(filter, meta).unwrap_or(false),
                Operator::Lt => filter_lt(filter, meta).unwrap_or(false),
            })
        });
    }

    Ok(())
}

fn validate_filters(filters: &[Filter]) -> Result<(), String> {
    for filter in filters {
        match filter.field.as_str() {
            "id" => {
                for id in filter.value.split(',') {
                    if id.parse::<u32>().is_err() {
                        return Err("ID must be an integer".to_string());
                    }
                }
            }
            "priority" => {
                for priority in filter.value.split(',') {
                    if Priority::from_str(priority).is_err() {
                        return Err("Invalid priority value".to_string());
                    }
                }
            }
            _ => {}
        }

        match filter.operator {
            Operator::Eq => { /* all fields support '=' */ }
            Operator::Gt | Operator::Lt => match filter.field.as_str() {
                "id" | "priority" | "due_date" | "created" | "updated" => { /* supported */ }
                _ => return Err(format!("Operator '>' and '<' not supported for field: {}", filter.field)),
            },
        }
    }
    Ok(())
}

fn filter_eq(filter: &Filter, meta: &Meta, settings: &Settings, users: &Users) -> bool {
    match filter.field.as_str() {
        "id" => do_strings_match(&meta.id.to_string(), &filter.value),
        "title" => do_strings_match(&meta.title, &filter.value),
        "state" => do_strings_match(&meta.state, &filter.value),
        "type" => do_strings_match(&meta.type_, &filter.value),
        "labels" => is_in_str_list(&meta.labels, &filter.value),
        "assignee" => do_strings_match_with_me(&meta.assignee, &filter.value, settings, users),
        "reporter" => do_strings_match_with_me(&meta.reporter, &filter.value, settings, users),
        "priority" => do_strings_match(&format!("{:?}", meta.priority).replace("-", ""), &filter.value),
        "due_date" => do_strings_match(&meta.due_date, &filter.value),
        "created" => do_strings_match(&meta.created, &filter.value),
        "updated" => do_strings_match(&meta.updated, &filter.value),
        "description" => does_description_match(meta.id, &filter.value),
        relationship => {
            if let Some(ids) = meta.relationships.get(relationship) {
                is_in_u32_list(ids, &filter.value)
            } else {
                filter.value.is_empty()
            }
        }
    }
}

fn filter_gt(filter: &Filter, meta: &Meta) -> Result<bool, String> {
    match filter.field.as_str() {
        "id" => Ok(meta.id > filter.value.parse::<u32>().map_err(|e| format!("Parse error: {e}"))?),
        "priority" => Ok(meta.priority.as_int() > Priority::from_str(&filter.value)?.as_int()),
        "due_date" => Ok(meta.due_date.cmp(&filter.value) == Ordering::Greater),
        "created" => Ok(meta.created.cmp(&filter.value) == Ordering::Greater),
        "updated" => Ok(meta.updated.cmp(&filter.value) == Ordering::Greater),
        _ => unreachable!(
            "Operator '>' not supported for field: {}. Should have been caught by `validate_filters()`.",
            filter.field
        ),
    }
}

fn filter_lt(filter: &Filter, meta: &Meta) -> Result<bool, String> {
    match filter.field.as_str() {
        "id" => Ok(meta.id < filter.value.parse::<u32>().map_err(|e| format!("Parse error: {e}"))?),
        "priority" => Ok(meta.priority.as_int() < Priority::from_str(&filter.value)?.as_int()),
        "due_date" => Ok(meta.due_date.cmp(&filter.value) == Ordering::Less),
        "created" => Ok(meta.created.cmp(&filter.value) == Ordering::Less),
        "updated" => Ok(meta.updated.cmp(&filter.value) == Ordering::Less),
        _ => unreachable!(
            "Operator '<' not supported for field: {}. Should have been caught by `validate_filters()`.",
            filter.field
        ),
    }
}

/// Check if value matches any pattern with wildcard support
fn do_strings_match(value: &str, pattern: &str) -> bool {
    let value = value.trim().to_lowercase();
    let pattern = pattern.trim().to_lowercase();

    for str in pattern.split(',') {
        let str = str.trim();

        // Escape regex special characters except '*'
        let regex_pattern = regex::escape(str).replace(r"\*", ".*");

        // Add anchors to match the whole string
        let regex_pattern = format!("(?s)^{}$", regex_pattern);

        // Compile the regex
        let re = match Regex::new(&regex_pattern) {
            Ok(re) => re,
            Err(_) => return false, // invalid regex
        };

        if re.is_match(&value) {
            return true;
        }
    }

    false
}

fn do_strings_match_with_me(value: &str, pattern: &str, settings: &Settings, users: &Users) -> bool {
    let mut pattern_me_replaced = Vec::new();

    for word in pattern.split(',') {
        let mut word = word.to_string();
        match user_handle_me(users, settings, &mut word) {
            Ok(_) => { /* continue */ }
            Err(_) => return false,
        }

        pattern_me_replaced.push(word);
    }

    do_strings_match(value, pattern_me_replaced.join(",").as_str())
}

/// Check if pattern matches any string in the list
fn is_in_str_list(list: &[String], pattern: &str) -> bool {
    if pattern.is_empty() && list.is_empty() {
        return true;
    }

    list.iter().any(|str| do_strings_match(str, pattern))
}

/// Check if pattern matches any u32 in the list
fn is_in_u32_list(list: &[u32], pattern: &str) -> bool {
    if pattern.is_empty() && list.is_empty() {
        return true;
    }

    list.iter().any(|id| do_strings_match(&id.to_string(), pattern))
}

fn does_description_match(id: u32, pattern: &str) -> bool {
    let description_path = match issue_desc_path(id) {
        Ok(value) => value,
        Err(_) => return false,
    };

    let description = match load_description(description_path.as_path()) {
        Ok(value) => value,
        Err(_) => return false,
    };

    do_strings_match(&description, pattern)
}

fn sort_issues(config: &Config, issues: &mut [Meta], sorts: Option<Vec<Sorting>>) -> Result<(), String> {
    if let Some(mut sorts) = sorts {
        // Validate all sort fields
        let mut sort_fields: Vec<String> = sorts.iter().map(|s| s.field.clone()).collect();
        validate_column_names(config, &mut sort_fields, "--sort")?;

        // Update the actual sort struct with normalized field names
        for (sort, normalized) in sorts.iter_mut().zip(sort_fields) {
            sort.field = normalized;
        }

        issues.sort_by(|a, b| {
            for sort in &sorts {
                let ordering = match sort.field.as_str() {
                    "id" => a.id.cmp(&b.id),
                    "title" => a.title.cmp(&b.title),
                    "state" => a.state.cmp(&b.state),
                    "type" => a.type_.cmp(&b.type_),
                    "labels" => a.labels.cmp(&b.labels),
                    "reporter" => a.reporter.cmp(&b.reporter),
                    "assignee" => a.assignee.cmp(&b.assignee),
                    "priority" => a.priority.as_int().cmp(&b.priority.as_int()),
                    "due_date" => a.due_date.cmp(&b.due_date),
                    "created" => a.created.cmp(&b.created),
                    "updated" => a.updated.cmp(&b.updated),
                    relationship => {
                        if let Some(a_ids) = a.relationships.get(relationship) {
                            if let Some(b_ids) = b.relationships.get(relationship) {
                                a_ids.cmp(b_ids)
                            } else {
                                Ordering::Greater
                            }
                        } else if b.relationships.get(relationship).is_some() {
                            Ordering::Less
                        } else {
                            Ordering::Equal
                        }
                    }
                };
                let ordering = match sort.order {
                    crate::cmd::model::Order::Asc => ordering,
                    crate::cmd::model::Order::Desc => ordering.reverse(),
                };
                if ordering != Ordering::Equal {
                    return ordering;
                }
            }
            Ordering::Equal
        });
    } else {
        issues.sort_by_key(|m| Reverse(m.id));
    }

    Ok(())
}

fn get_relationship_value(col: &str, meta: &Meta) -> String {
    match meta.relationships.get(col) {
        Some(ids) => {
            let ids_joined = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",");
            dash_if_empty(&ids_joined)
        }
        None => "-".to_string(),
    }
}

fn get_column_value(col: &str, meta: &Meta) -> Result<String, String> {
    match col {
        "id" => Ok(meta.id.to_string()),
        "title" => Ok(meta.title.clone()),
        "state" => Ok(meta.state.clone()),
        "type" => Ok(dash_if_empty(&meta.type_)),
        "labels" => Ok(dash_if_empty(&meta.labels.join(","))),
        "reporter" => Ok(dash_if_empty(&meta.reporter)),
        "assignee" => Ok(dash_if_empty(&meta.assignee)),
        "priority" => Ok(format!("{:?}", meta.priority)),
        "due_date" => Ok(dash_if_empty(&meta.due_date)),
        "created" => Ok(meta.created.clone()),
        "updated" => Ok(meta.updated.clone()),
        _ => Ok(get_relationship_value(col, meta)),
    }
}
