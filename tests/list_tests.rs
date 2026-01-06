mod common;
use common::{TestEnv, run_command};

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
    let output = run_command(&["list", "--columns", "id,title,priority"]).expect("list with columns failed");
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
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("title"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("labels"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("related"));
    assert!(stdout.contains("child"));
    assert!(stdout.contains("parent"));
    assert!(stdout.contains("created"));
    assert!(stdout.contains("updated"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
}
