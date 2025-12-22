use std::fs;
use std::path::Path;
use std::process::Command;

use crate::model::{Config, Meta, git_commit};

pub fn run(id: u32) -> Result<(), String> {
    let id_str = format!("{id:010}");
    let desc_path = format!(".gitissues/issues/{id_str}/description.md");
    let path = Path::new(&desc_path);

    // Precondition: .gitissues/issues/ID/description.md must exist
    if !path.exists() {
        return Err("Not available: ID/description.md does not exist.".to_string());
    }

    // Load configuration
    let config_path = Path::new(".gitissues/config.yaml");
    let config_raw = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => return Err("config.yaml not found.".to_string()),
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(_) => return Err("config.yaml malformatted.".to_string()),
    };

    open_editor(config.editor, desc_path)?;

    // Load meta.yaml to get title for commit message
    let meta = format!(".gitissues/issues/{id_str}/meta.yaml");
    let meta_path = Path::new(&meta);
    let meta_raw = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(_) => return Err("meta.yaml not found.".to_string()),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err("meta.yaml malformatted.".to_string()),
    };

    git_commit(id, meta.title, "edit description");

    Ok(())
}

fn open_editor(mut editor: String, desc_path: String) -> Result<(), String> {
    if editor == "git" {
        // Read git default editor
        let output = Command::new("git")
            .args(["config", "--get", "core.editor"])
            .output()
            .map_err(|e| format!("Failed to get git editor: {e}"))?;

        if !output.status.success() {
            return Err("Git config failed or core.editor not set".to_string());
        }

        editor = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse git output: {e}"))?
            .trim()
            .to_string();
    }

    // Parse editor command (handles quoted paths with arguments)
    let editor_parts =
        shell_words::split(&editor).map_err(|e| format!("Failed to parse editor command: {e}"))?;

    if editor_parts.is_empty() {
        return Err("No editor command specified".to_string());
    }

    // First part is the program, rest are arguments
    let program = &editor_parts[0];
    let mut cmd = Command::new(program);

    // Add any existing arguments from the editor config
    for arg in &editor_parts[1..] {
        cmd.arg(arg);
    }

    // Add the file to edit
    cmd.arg(&desc_path);

    // Execute editor
    let status = cmd
        .status()
        .map_err(|e| format!("Failed to open editor: {e}"))?;

    if !status.success() {
        return Err(format!(
            "Editor exited with error code: {:?}",
            status.code()
        ));
    }

    Ok(())
}
