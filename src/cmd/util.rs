use std::process::Command;
use std::{fs, path::Path};

use crate::model::{Meta, Settings, Users, gitissues_base, is_valid_user, issues_dir, load_config};

pub(crate) fn user_handle_me(users: &Users, settings: &Settings, value: &mut String) -> Result<(), String> {
    if *value != "me" {
        return Ok(());
    }

    *value = if is_valid_user(users, &settings.user) {
        settings.user.clone()
    } else {
        return Err("Invalid user: settings.yaml::user must be part of users.yaml:users or ''".to_string());
    };

    Ok(())
}

pub(crate) fn dash_if_empty(value: &str) -> String {
    if value.is_empty() { "-".to_string() } else { value.to_string() }
}

/// Git commit based on template and config
/// This commits all issue changes (.gitissues/issues/)
pub(crate) fn git_commit(id: u32, title: String, action: &str) -> Result<Vec<String>, String> {
    let config = load_config()?;

    // Check if auto-commit is enabled
    if !config.commit_auto {
        return Ok(vec![]);
    }

    // Prepare commit message
    let commit_message_template = config.commit_message;

    let commit_message = commit_message_template
        .replace("{action}", action)
        .replace("{id}", &format!("{id}"))
        .replace("{title}", &title);

    run_git(&commit_message, &issues_dir()?.to_string_lossy())
}

/// Simple commit message not based on template or config
/// This commits all changes (.gitissues/)
pub(crate) fn git_commit_non_templated(msg: &str) -> Result<Vec<String>, String> {
    // Prepare commit message
    let commit_message = format!("[issue] {msg}");

    run_git(&commit_message, &gitissues_base()?.to_string_lossy())
}

fn run_git(commit_message: &str, staging_dir: &str) -> Result<Vec<String>, String> {
    // Execute git add
    let add_result = Command::new("git")
        .args(["add", staging_dir])
        .output()
        .map_err(|e| format!("Failed to stage .gitissues: {e}"))?;

    if !add_result.status.success() {
        let stderr = String::from_utf8_lossy(&add_result.stderr);
        return Err(format!("Failed to stage .gitissues: {}", stderr.trim()));
    }

    // Execute git commit
    let commit_result = Command::new("git")
        .args(["commit", "-m", commit_message])
        .output()
        .map_err(|e| format!("Failed to commit: {e}"))?;

    if !commit_result.status.success() {
        let stdout = String::from_utf8_lossy(&commit_result.stdout);
        let stderr = String::from_utf8_lossy(&commit_result.stderr);

        // Check if it's just "nothing to commit" - this is not an error
        if stdout.contains("nothing to commit")
            || stdout.contains("no changes added to commit")
            || stdout.contains("nothing added to commit")
        {
            return Ok(vec!["Info: Nothing to commit".to_string()]);
        }

        let error_msg = if !stderr.trim().is_empty() { stderr.trim() } else { stdout.trim() };

        return Err(format!("Failed to commit: {}", error_msg));
    }

    Ok(vec![])
}

pub fn load_meta(path: &Path) -> Result<Meta, String> {
    let meta_raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Err(format!("meta.yaml not found: {}", path.display())),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(e) => return Err(format!("meta.yaml malformatted: {}: {e}", path.display())),
    };

    Ok(meta)
}

pub fn load_description(path: &Path) -> Result<String, String> {
    let raw = fs::read_to_string(path).map_err(|_| format!("description.md not found: {}", path.display()))?;
    Ok(raw)
}
