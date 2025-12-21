use std::fs;
use std::path::Path;

pub fn run() -> Result<(), String> {
    let root = ".gitissues";
    let issues_dir = Path::new(".gitissues").join("issues");

    if Path::new(root).exists() {
        return Err("Already initialized: .gitissues already exists".to_string());
    }

    // Create the directory structure
    fs::create_dir_all(&issues_dir)
        .map_err(|e| format!("Failed to create {}: {}", issues_dir.display(), e))?;

    println!("Initialization done");

    Ok(())
}
