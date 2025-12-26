use chrono::Utc;
use serde_yaml::Value;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tempfile::TempDir;

/// Global lock to ensure only one test changes directory at a time
static DIR_LOCK: Mutex<()> = Mutex::new(());

/// Helper to setup a temporary working directory for tests
struct TestEnv {
    _temp_dir: TempDir,
    original_dir: PathBuf,
    _lock: std::sync::LockResult<std::sync::MutexGuard<'static, ()>>,
}

impl TestEnv {
    fn new() -> Self {
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
fn run_command(args: &[&str]) -> Result<std::process::Output, String> {
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

fn load_meta_value(path: &str) -> Value {
    let content = fs::read_to_string(path).expect("Failed to read meta.yaml");
    serde_yaml::from_str::<Value>(&content).expect("Failed to parse meta.yaml")
}

#[test]
fn test_basic_workflow() {
    let _env = TestEnv::new();

    // Step 1: Initialize without git commit
    run_command(&["init", "--no-commit"]).expect("init failed");

    // Verify .gitissues structure
    assert!(PathBuf::from(".gitissues").exists());
    assert!(PathBuf::from(".gitissues/config.yaml").exists());
    assert!(PathBuf::from(".gitissues/description.md").exists());
    assert!(PathBuf::from(".gitissues/issues").exists());

    // Step 2: Create 3 issues
    let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    run_command(&["new", "First issue"]).expect("new 1 failed");
    run_command(&["new", "Second issue"]).expect("new 2 failed");
    run_command(&["new", "Third issue"]).expect("new 3 failed");

    // Verify issues were created
    assert!(PathBuf::from(".gitissues/issues/0000000001").exists());
    assert!(PathBuf::from(".gitissues/issues/0000000002").exists());
    assert!(PathBuf::from(".gitissues/issues/0000000003").exists());

    // Verify attachments directory and description for issue #1
    let attach_dir = PathBuf::from(".gitissues/issues/0000000001/attachments");
    assert!(attach_dir.exists());
    assert!(attach_dir.join(".gitkeep").exists());

    let desc_path = PathBuf::from(".gitissues/issues/0000000001/description.md");
    assert!(desc_path.exists());
    let content = fs::read_to_string(&desc_path).expect("Failed to read description");
    assert!(content.contains("# Description"));

    // Step 3: Verify issue metadata
    let meta1 = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta1["id"].as_i64().unwrap(), 1);
    assert_eq!(meta1["title"].as_str().unwrap(), "First issue");
    assert_eq!(meta1["state"].as_str().unwrap(), "new"); // default
    assert_eq!(meta1["type"].as_str().unwrap(), "");
    assert_eq!(meta1["assignee"].as_str().unwrap(), "");
    assert_eq!(meta1["priority"].as_str().unwrap(), "P2"); // default
    assert_eq!(meta1["due_date"].as_str().unwrap(), "");

    let created = meta1["created"].as_str().unwrap();
    let updated = meta1["created"].as_str().unwrap();
    assert_eq!(created, now);
    assert_eq!(created, updated);

    let meta2 = load_meta_value(".gitissues/issues/0000000002/meta.yaml");
    assert_eq!(meta2["id"].as_i64().unwrap(), 2);
    assert_eq!(meta2["title"].as_str().unwrap(), "Second issue");

    let meta3 = load_meta_value(".gitissues/issues/0000000003/meta.yaml");
    assert_eq!(meta3["id"].as_i64().unwrap(), 3);
    assert_eq!(meta3["title"].as_str().unwrap(), "Third issue");

    // Small delay to ensure timestamp will be different
    thread::sleep(Duration::from_millis(1100));

    // Step 4: Set fields on issue 2
    run_command(&[
        "set",
        "2",
        "--state",
        "active",
        "--type",
        "bug",
        "--labels",
        "cli,fw",
        "--assignee",
        "alice",
        "--priority",
        "P1",
        "--due-date",
        "2026-06-15",
    ])
    .expect("set failed");

    // Step 5: Verify changes
    let prev_updated = meta2["updated"].as_str().unwrap().to_string();
    let meta2_updated = load_meta_value(".gitissues/issues/0000000002/meta.yaml");
    assert_eq!(meta2_updated["state"].as_str().unwrap(), "active");
    assert_eq!(meta2_updated["type"].as_str().unwrap(), "bug");
    let labels: Vec<String> = meta2_updated["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "fw"]);
    assert_eq!(meta2_updated["assignee"].as_str().unwrap(), "alice");
    assert_eq!(meta2_updated["priority"].as_str().unwrap(), "P1");
    assert_eq!(meta2_updated["due_date"].as_str().unwrap(), "2026-06-15");
    assert!(meta2_updated["updated"].as_str().unwrap() >= prev_updated.as_str());
}

#[test]
fn test_new_with_initial_metadata() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create issue with all initial metadata
    run_command(&[
        "new",
        "Complex issue",
        "--type",
        "feature",
        "--assignee",
        "alice",
        "--priority",
        "P0",
        "--due-date",
        "2026-01-15",
        "--labels",
        "frontend,ui",
    ])
    .expect("new with metadata failed");

    // Verify metadata
    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Complex issue");
    assert_eq!(meta["type"].as_str().unwrap(), "feature");
    assert_eq!(meta["assignee"].as_str().unwrap(), "alice");
    assert_eq!(meta["priority"].as_str().unwrap(), "P0");
    assert_eq!(meta["due_date"].as_str().unwrap(), "2026-01-15");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["frontend", "ui"]);
    assert_eq!(meta["state"].as_str().unwrap(), "new"); // always starts as new
}

#[test]
fn test_set_labels() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    run_command(&["new", "Label test"]).expect("new failed");

    // Add labels
    run_command(&["set", "1", "--labels-add", "cli,critical"]).expect("set labels_add failed");

    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "critical"]);

    // Add more labels
    run_command(&["set", "1", "--labels-add", "ui"]).expect("set labels_add 2 failed");

    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "critical", "ui"]);

    // Remove a label
    run_command(&["set", "1", "--labels-remove", "critical"]).expect("set labels_remove failed");

    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "ui"]);

    // Overwrite all labels
    run_command(&["set", "1", "--labels", "new-label"]).expect("set labels (overwrite) failed");

    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["new-label"]);

    // Clear all labels
    run_command(&["set", "1", "--labels", ""]).expect("set labels clear failed");

    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(labels.is_empty());
}

#[test]
fn test_invalid_due_date() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Try to create issue with invalid date
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["new", "Bad date", "--due-date", "not-a-date"])
        .output()
        .expect("Failed to execute command");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid due_date format"));

    // Create valid issue first
    run_command(&["new", "Valid issue"]).expect("new failed");

    // Try to set invalid date
    let output = Command::new(&binary)
        .args(&["set", "1", "--due-date", "invalid-date"])
        .output()
        .expect("Failed to execute command");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid due_date format"));
}

#[test]
fn test_list_basic() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create a few issues
    run_command(&["new", "Issue 1"]).expect("new 1 failed");
    run_command(&["new", "Issue 2"]).expect("new 2 failed");
    run_command(&["new", "Issue 3"]).expect("new 3 failed");

    // List should not fail and should show issues
    let output = run_command(&["list"]).expect("list failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("title"));
    assert!(!stdout.contains("priority"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));

    // List with custom columns
    let output =
        run_command(&["list", "--columns", "id,title,priority"]).expect("list with columns failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("state"));
    assert!(!stdout.contains("assignee"));
    assert!(stdout.contains("title"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));

    // List with wildcard
    let output = run_command(&["list", "--columns", "*"]).expect("list with wildcard failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("title"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("labels"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("created"));
    assert!(stdout.contains("updated"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
}
