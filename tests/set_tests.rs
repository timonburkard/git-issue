use std::process::Command;

mod common;
use common::{TestEnv, get_binary_path, load_yaml_values, run_command};

#[test]
fn test_set_labels() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    run_command(&["new", "Label test"]).expect("new failed");

    // Add labels
    run_command(&["set", "1", "--labels-add", "cli,critical"]).expect("set labels_add failed");

    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "critical"]);

    // Add more labels
    run_command(&["set", "1", "--labels-add", "ui"]).expect("set labels_add 2 failed");

    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "critical", "ui"]);

    // Remove a label
    run_command(&["set", "1", "--labels-remove", "critical"]).expect("set labels_remove failed");

    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["cli", "ui"]);

    // Overwrite all labels
    run_command(&["set", "1", "--labels", "new-label"]).expect("set labels (overwrite) failed");

    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert_eq!(labels, vec!["new-label"]);

    // Clear all labels
    run_command(&["set", "1", "--labels", ""]).expect("set labels clear failed");

    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let labels: Vec<String> = meta["labels"]
        .as_sequence()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();
    assert!(labels.is_empty());
}

#[test]
fn test_set_assignee() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that assignee is empty
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("-"));

    // Set valid assignee
    run_command(&["set", "1", "--assignee", "bob"]).expect("set assignee failed");

    // List to check that assignee was set
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("bob"));

    // Set invalid assignee
    run_command(&["set", "1", "--assignee", "duck"]).expect_err("set assignee successful but should fail");

    // List to check that invalid assignee was not set
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("bob"));
    assert!(!stdout.contains("duck"));

    // Remove assignee
    run_command(&["set", "1", "--assignee", ""]).expect("remove assignee failed");

    // List to check that assignee was removed
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("assignee"));
    assert!(!stdout.contains("bob"));
    assert!(stdout.contains("-"));
}

#[test]
fn test_set_reporter() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that reporter is empty
    let output = run_command(&["list", "--columns", "title,reporter"]).expect("list with reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("-"));

    // Set valid reporter
    run_command(&["set", "1", "--reporter", "bob"]).expect("set reporter failed");

    // List to check that reporter was set
    let output = run_command(&["list", "--columns", "title,reporter"]).expect("list with reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("bob"));

    // Set invalid reporter
    run_command(&["set", "1", "--reporter", "duck"]).expect_err("set reporter successful but should fail");

    // List to check that invalid reporter was not set
    let output = run_command(&["list", "--columns", "title,reporter"]).expect("list with reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("bob"));
    assert!(!stdout.contains("duck"));

    // Remove reporter
    run_command(&["set", "1", "--reporter", ""]).expect("remove reporter failed");

    // List to check that reporter was removed
    let output = run_command(&["list", "--columns", "title,reporter"]).expect("list with reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(!stdout.contains("bob"));
    assert!(stdout.contains("-"));
}

#[test]
fn test_set_state() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that state is new
    let output = run_command(&["list", "--columns", "title,state"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("new"));

    // Set valid state
    run_command(&["set", "1", "--state", "active"]).expect("set state failed");

    // List to check that state was set
    let output = run_command(&["list", "--columns", "title,state"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("active"));

    // Set invalid state
    run_command(&["set", "1", "--state", "perfect"]).expect_err("set state successful but should fail");

    // List to check that invalid state was not set
    let output = run_command(&["list", "--columns", "title,state"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("active"));
    assert!(!stdout.contains("perfect"));

    // Try to remove state
    run_command(&["set", "1", "--state", ""]).expect_err("remove state successful but should fail");

    // List to check that state was not removed
    let output = run_command(&["list", "--columns", "title,state"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("active"));
}

#[test]
fn test_set_type() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that type is empty
    let output = run_command(&["list", "--columns", "title,type"]).expect("list with type failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("-"));

    // Set valid type
    run_command(&["set", "1", "--type", "feature"]).expect("set type failed");

    // List to check that state was set
    let output = run_command(&["list", "--columns", "title,type"]).expect("list with type failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("feature"));

    // Set invalid type
    run_command(&["set", "1", "--type", "experiment"]).expect_err("set type successful but should fail");
    // List to check that invalid type was not set
    let output = run_command(&["list", "--columns", "title,type"]).expect("list with type failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("feature"));
    assert!(!stdout.contains("experiment"));

    // Remove type
    run_command(&["set", "1", "--type", ""]).expect("remove type failed");

    // List to check that type was removed
    let output = run_command(&["list", "--columns", "title,type"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("-"));
}

#[test]
fn test_set_priority() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that priority is empty
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("-"));

    // Set valid priority
    run_command(&["set", "1", "--priority", "P1"]).expect("set priority failed");

    // List to check that priority was set
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("P1"));

    // Set valid priority in lowercase
    run_command(&["set", "1", "--priority", "p0"]).expect("set priority failed");

    // List to check that priority was set
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("P0"));

    // Set non-existing priority
    run_command(&["set", "1", "--priority", "P5"]).expect_err("set priority successful but should fail");

    // List to check that non-existing priority was not set
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("P0"));
    assert!(!stdout.contains("P5"));

    // Set invalid priority
    run_command(&["set", "1", "--priority", "3"]).expect_err("set priority successful but should fail");

    // List to check that non-existing priority was not set
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("P0"));
    assert!(!stdout.contains("3"));

    // Remove priority
    run_command(&["set", "1", "--priority", ""]).expect("remove priority failed");

    // List to check that priority was removed
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(!stdout.contains("P0"));
    assert!(stdout.contains("-"));
}

#[test]
fn test_set_due_date() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create valid issue first
    run_command(&["new", "Valid issue"]).expect("new failed");

    // Try to set invalid date
    let binary = get_binary_path();
    let output = Command::new(&binary)
        .args(&["set", "1", "--due-date", "invalid-date"])
        .output()
        .expect("Failed to execute command");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Invalid due_date format"));
}
