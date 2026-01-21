use std::fs;
use std::io::{self, Write};
use std::time::Duration;

use crate::model::{Priority, cache_path};
use crate::new;
use crate::set;

pub fn new(
    title: String,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    let (issue_id, info) = new::new(title, type_, reporter, assignee, priority, due_date, labels)?;

    if let Some(info) = info {
        println!("{}", info);
    }

    println!("Created issue #{issue_id}");

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn set(
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

    let (num_updated_issues, infos) = set::set(
        ids,
        state,
        title,
        type_,
        reporter,
        assignee,
        priority,
        due_date,
        labels,
        labels_add,
        labels_remove,
    )?;

    if let Some(infos) = infos {
        for info in infos {
            println!("{}", info);
        }
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
