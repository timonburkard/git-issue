#![allow(dead_code)] // Test helpers may not be used by all test binaries

use serde_yaml::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use tempfile::TempDir;

/// Global lock to ensure only one test changes directory at a time
pub static DIR_LOCK: Mutex<()> = Mutex::new(());

/// Helper to setup a temporary working directory for tests
pub struct TestEnv {
    _temp_dir: TempDir,
    original_dir: PathBuf,
    _lock: std::sync::LockResult<std::sync::MutexGuard<'static, ()>>,
}

impl TestEnv {
    pub fn new() -> Self {
        // Use lock() which handles poisoned mutexes by clearing the poison
        let lock = DIR_LOCK.lock();
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let original_dir = env::current_dir().expect("Failed to get current dir");
        env::set_current_dir(temp_dir.path()).expect("Failed to change to temp dir");

        TestEnv {
            _temp_dir: temp_dir,
            original_dir,
            _lock: lock,
        }
    }
}

impl Drop for TestEnv {
    fn drop(&mut self) {
        let _ = env::set_current_dir(&self.original_dir);
    }
}

/// Helper to get the path to the git-issue binary
fn get_binary_path() -> PathBuf {
    // Use the binary built by cargo test
    let mut path = env::current_exe().expect("Failed to get test executable path");
    path.pop(); // Remove test executable name
    path.pop(); // Remove "deps" directory
    path.push("git-issue.exe");

    if !path.exists() {
        path.pop();
        path.push("git-issue"); // Try without .exe for non-Windows
    }

    path
}

/// Run a git-issue command and return the result
pub fn run_command(args: &[&str]) -> Result<std::process::Output, String> {
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output)
}

/// Run a git-issue command with stdin input and return the result
pub fn run_command_with_stdin(args: &[&str], stdin_input: &str) -> Result<std::process::Output, String> {
    use std::io::Write;
    use std::process::Stdio;

    let binary = get_binary_path();
    let mut child = Command::new(&binary)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn command: {}", e))?;

    // Write to stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(stdin_input.as_bytes())
            .map_err(|e| format!("Failed to write to stdin: {}", e))?;
    }

    let output = child.wait_with_output().map_err(|e| format!("Failed to wait for command: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Command failed with exit code {:?}\nstdout: {}\nstderr: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(output)
}

pub fn load_yaml_values(path: &str) -> Value {
    let content = fs::read_to_string(path).unwrap_or_else(|_| panic!("Failed to read {path}"));
    serde_yaml::from_str::<Value>(&content).expect("Failed to parse meta.yaml")
}

pub fn save_yaml_values(path: &str, value: &Value) {
    let content = serde_yaml::to_string(value).unwrap_or_else(|_| panic!("Failed to serialize {path}"));
    fs::write(path, content).unwrap_or_else(|_| panic!("Failed to write {path}"));
}
