use std::fs;
use std::path::Path;

use chrono::Utc;

pub fn run(title: String) -> Result<(), String> {
    // Step 1: Allocate the next issue ID
    let issue_id = allocate_id()?;

    // Step 2: Create the issue directory
    let issue_dir = format!(".gitissues/issues/{}", issue_id);
    fs::create_dir_all(&issue_dir)
        .map_err(|e| format!("Failed to create issue directory: {}", e))?;

    // Step 3: Write issue.md
    let issue_md_path = format!("{}/issue.md", issue_dir);
    let issue_md_content = format!("# {}\n\n## Description\n\nTBD\n", title);
    fs::write(&issue_md_path, issue_md_content)
        .map_err(|e| format!("Failed to write issue.md: {}", e))?;

    // Step 4: Write meta.yml
    let meta_yaml_path = format!("{}/meta.yaml", issue_dir);
    let timestamp = current_timestamp();
    let meta_content = format!(
        "id: {}\ntitle: \"{}\"\nstate: new\ncreated: {}\nupdated: {}\n",
        issue_id, title, timestamp, timestamp
    );
    fs::write(&meta_yaml_path, meta_content)
        .map_err(|e| format!("Failed to write meta.yaml: {}", e))?;

    println!("Created issue {}", issue_id);
    Ok(())
}

/// Allocate the next sequential issue ID.
/// Scans .gitissues/issues/ for existing numeric directories,
/// finds the max, and returns max+1 formatted as 10-digit zero-padded.
fn allocate_id() -> Result<String, String> {
    let issues_base = ".gitissues/issues";
    let path = Path::new(issues_base);

    // Precondition: .gitissues/issues must exist (user must run init first)
    if !path.exists() {
        return Err(
            "Not initialized: .gitissues/issues does not exist. Run `git issue init` first."
                .to_string(),
        );
    }

    let mut max_id = 0u32;

    // Read directory entries and find the highest numeric ID
    for entry in
        fs::read_dir(path).map_err(|e| format!("Failed to read issues directory: {}", e))?
    {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let file_name = entry.file_name();
        let name_str = file_name.to_string_lossy();

        // Try to parse the directory name as a u32
        if let Ok(id) = name_str.parse::<u32>() {
            if id > max_id {
                max_id = id;
            }
        }
        // Silently skip non-numeric directory names
    }

    let next_id = max_id + 1;
    Ok(format!("{:010}", next_id))
}

/// Generate a proper ISO 8601 timestamp using chrono.
fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}
