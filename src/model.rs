use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Meta {
    pub id: u32,
    pub title: String,
    pub state: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub labels: Vec<String>,
    pub assignee: String,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub commit_auto: bool,
    pub commit_message: String,
    pub editor: String,
}

/// Generate a proper ISO 8601 timestamp using chrono.
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn load_config() -> Result<Config, String> {
    let config_path = Path::new(".gitissues/config.yaml");
    let config_raw = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(_) => return Err("config.yaml not found.".to_string()),
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(_) => return Err("config.yaml malformatted.".to_string()),
    };

    Ok(config)
}

pub fn load_meta(path: &Path) -> Result<Meta, String> {
    let meta_raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Err(format!("meta.yaml not found: {}", path.display())),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err(format!("meta.yaml malformatted: {}", path.display())),
    };

    Ok(meta)
}

/// git commit based on template from config
pub fn git_commit(id: u32, title: String, action: &str) -> Result<(), String> {
    use std::process::Command;

    let config = load_config()?;

    // Check if auto-commit is enabled
    if !config.commit_auto {
        return Ok(());
    }

    // Prepare commit message
    let commit_message_template = config.commit_message;

    let commit_message = commit_message_template
        .replace("{action}", action)
        .replace("{id}", &format!("{id}"))
        .replace("{title}", &title);

    // Execute git add
    let add_result = Command::new("git").args(["add", ".gitissues"]).output();
    if let Err(e) = add_result {
        return Err(format!("Failed to stage .gitissues: {e}"));
    }

    // Execute git commit
    let commit_result = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output();
    if let Err(e) = commit_result {
        return Err(format!("Failed to commit: {e}"));
    }

    Ok(())
}

/// Simple commit message not based on template or config
pub fn git_commit_non_templated(msg: &str) -> Result<(), String> {
    use std::process::Command;

    // Prepare commit message
    let commit_message = format!("[issue] {msg}");

    // Execute git add
    let add_result = Command::new("git").args(["add", ".gitissues"]).output();
    if let Err(e) = add_result {
        return Err(format!("Failed to stage .gitissues: {e}"));
    }

    // Execute git commit
    let commit_result = Command::new("git")
        .args(["commit", "-m", &commit_message])
        .output();
    if let Err(e) = commit_result {
        return Err(format!("Failed to commit: {e}"));
    }

    Ok(())
}

pub fn open_editor(mut editor: String, path: String) -> Result<(), String> {
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
    cmd.arg(&path);

    // Execute editor (with Windows-specific shell fallback for PATH/PATHEXT resolution)
    #[allow(unused_mut)]
    let mut status = cmd.status();

    #[cfg(windows)]
    if let Err(e) = &status {
        use std::io::ErrorKind;
        if e.kind() == ErrorKind::NotFound {
            // Build a single command line for cmd.exe to resolve (e.g., code -> code.cmd)
            let quoted_path = if path.contains(' ') {
                format!("\"{}\"", path)
            } else {
                path.clone()
            };
            let full = format!("{} {}", editor, quoted_path);
            status = Command::new("cmd").args(["/c", &full]).status();
        }
    }

    let status = status.map_err(|e| format!("Failed to open editor: {e}"))?;
    if !status.success() {
        return Err(format!(
            "Editor exited with error code: {:?}",
            status.code()
        ));
    }

    Ok(())
}
