use std::fs;
use std::path::Path;

use crate::model::Meta;

pub fn run() -> Result<(), String> {
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
        let meta_raw = match fs::read_to_string(&meta_path) {
            Ok(s) => s,
            Err(_) => continue, // skip entries without readable meta
        };

        let meta: Meta = match serde_yaml::from_str(&meta_raw) {
            Ok(m) => m,
            Err(_) => continue, // skip malformed meta
        };

        issues.push(meta);
    }

    // Sort by numeric ID
    issues.sort_by_key(|m| m.id);

    // Print header and rows
    println!("{:<10} {:<10} Title", "ID", "State");
    for meta in issues {
        println!("{:<10} {:<10} {}", meta.id, meta.state, meta.title);
    }

    Ok(())
}
