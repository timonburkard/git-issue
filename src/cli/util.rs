use std::process::Command;

use git_issue::model::{gitissues_base, issue_tmp_dir};

/// Returns the path to the cache.txt file.
pub(crate) fn cache_path() -> Result<std::path::PathBuf, String> {
    Ok(issue_tmp_dir()?.join("cache.txt"))
}

pub(crate) fn issue_exports_dir() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join("exports"))
}

pub(crate) fn open_editor(mut editor: String, path: String) -> Result<(), String> {
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
    let editor_parts = shell_words::split(&editor).map_err(|e| format!("Failed to parse editor command: {e}"))?;

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
        return Err(format!("Editor exited with error code: {:?}", status.code()));
    }

    Ok(())
}
