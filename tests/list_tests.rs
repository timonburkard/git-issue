mod common;
use common::{TestEnv, load_yaml_values, run_command, save_yaml_values};

#[test]
fn test_list_columns() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create a few issues
    run_command(&[
        "new",
        "Issue 1",
        "--type",
        "feature",
        "--assignee",
        "alice",
        "--reporter",
        "bob",
        "--labels",
        "ui,cli",
        "--priority",
        "P1",
        "--due_date",
        "2026-01-02",
    ])
    .expect("new 1 failed");
    run_command(&[
        "new",
        "Issue 2",
        "--type",
        "feature",
        "--assignee",
        "alice",
        "--reporter",
        "bob",
        "--labels",
        "ui,cli",
        "--priority",
        "P1",
        "--due_date",
        "2026-01-02",
    ])
    .expect("new 2 failed");
    run_command(&[
        "new",
        "Issue 3",
        "--type",
        "feature",
        "--assignee",
        "alice",
        "--reporter",
        "bob",
        "--labels",
        "ui,cli",
        "--priority",
        "P1",
        "--due_date",
        "2026-01-02",
    ])
    .expect("new 3 failed");

    // List should not fail and should show issues with default columns
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
    assert!(stdout.contains("alice"));
    assert!(!stdout.contains("bob"));

    // Change default columns in config
    let config_path = ".gitissues/config.yaml";
    let mut config = load_yaml_values(config_path);
    config["list_columns"] = serde_yaml::Value::Sequence(vec![
        serde_yaml::Value::String("type".to_string()),
        serde_yaml::Value::String("priority".to_string()),
        serde_yaml::Value::String("labels".to_string()),
        serde_yaml::Value::String("reporter".to_string()),
        serde_yaml::Value::String("due_date".to_string()),
        serde_yaml::Value::String("related".to_string()),
        serde_yaml::Value::String("parent".to_string()),
        serde_yaml::Value::String("child".to_string()),
        serde_yaml::Value::String("created".to_string()),
        serde_yaml::Value::String("updated".to_string()),
    ]);
    save_yaml_values(config_path, &config);

    // List with changed default columns
    let output = run_command(&["list"]).expect("list with changed default columns failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("type"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains("labels"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("related"));
    assert!(stdout.contains("parent"));
    assert!(stdout.contains("child"));
    assert!(stdout.contains("created"));
    assert!(stdout.contains("updated"));
    assert!(!stdout.contains("id"));
    assert!(!stdout.contains("state"));
    assert!(!stdout.contains("assignee"));
    assert!(!stdout.contains("title"));
    assert!(!stdout.contains("Issue 1"));
    assert!(!stdout.contains("Issue 2"));
    assert!(!stdout.contains("Issue 3"));
    assert!(!stdout.contains("alice"));
    assert!(stdout.contains("bob"));
    assert!(stdout.contains("feature"));
    assert!(stdout.contains("ui,cli"));
    assert!(stdout.contains("P1"));
    assert!(stdout.contains("2026-01-02"));

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
    assert!(stdout.contains("new"));
    assert!(stdout.contains("alice"));
    assert!(stdout.contains("bob"));
    assert!(stdout.contains("feature"));
    assert!(stdout.contains("ui,cli"));
    assert!(stdout.contains("P1"));
    assert!(stdout.contains("2026-01-02"));
}
