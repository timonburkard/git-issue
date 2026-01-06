use std::process::Command;

mod common;
use common::{TestEnv, get_binary_path, load_meta_value, run_command};

#[test]
fn test_new() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create simple issue
    run_command(&["new", "Simple issue"]).expect("new failed");

    // Verify metadata
    let meta = load_meta_value(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Simple issue");
    assert_eq!(meta["type"].as_str().unwrap(), "");
    assert_eq!(meta["assignee"].as_str().unwrap(), "");
    assert_eq!(meta["reporter"].as_str().unwrap(), "");
    assert_eq!(meta["priority"].as_str().unwrap(), "");
    assert_eq!(meta["due_date"].as_str().unwrap(), "");
    assert_eq!(meta["labels"].as_sequence().unwrap().len(), 0);
    assert_eq!(meta["state"].as_str().unwrap(), "new");
}

#[test]
fn test_new_initial_metadata() {
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
        "--reporter",
        "bob",
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
    assert_eq!(meta["reporter"].as_str().unwrap(), "bob");
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
fn test_new_invalid_due_date() {
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
