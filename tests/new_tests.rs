mod common;
use common::{TestEnv, load_yaml_values, run_command, save_yaml_values};

#[test]
fn test_new_simple() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create simple issue
    run_command(&["new", "Simple issue"]).expect("new failed");

    // Verify metadata
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
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
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
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
fn test_new_empty_metadata() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create issue with explicit empty initial metadata for all fields
    run_command(&[
        "new",
        "Another issue",
        "--type",
        "",
        "--assignee",
        "",
        "--reporter",
        "",
        "--priority",
        "",
        "--due-date",
        "",
        "--labels",
        "",
    ])
    .expect("new with metadata failed");

    // Verify metadata
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Another issue");
    assert_eq!(meta["type"].as_str().unwrap(), "");
    assert_eq!(meta["assignee"].as_str().unwrap(), "");
    assert_eq!(meta["reporter"].as_str().unwrap(), "");
    assert_eq!(meta["priority"].as_str().unwrap(), "");
    assert_eq!(meta["due_date"].as_str().unwrap(), "");
    assert_eq!(meta["labels"].as_sequence().unwrap().len(), 0);
    assert_eq!(meta["state"].as_str().unwrap(), "new"); // always starts as new
}

#[test]
fn test_new_changed_default_reporter() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Change default user in settings
    let settings_path = ".gitissues/settings.yaml";
    let mut settings = load_yaml_values(settings_path);
    settings["user"] = serde_yaml::Value::String("bob".to_string());
    save_yaml_values(settings_path, &settings);

    // Create issue
    run_command(&["new", "Bob's issue"]).expect("new failed");

    // Verify metadata
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Bob's issue");
    assert_eq!(meta["type"].as_str().unwrap(), "");
    assert_eq!(meta["assignee"].as_str().unwrap(), "");
    assert_eq!(meta["reporter"].as_str().unwrap(), "bob");
    assert_eq!(meta["priority"].as_str().unwrap(), "");
    assert_eq!(meta["due_date"].as_str().unwrap(), "");
    assert_eq!(meta["labels"].as_sequence().unwrap().len(), 0);
    assert_eq!(meta["state"].as_str().unwrap(), "new");
}

#[test]
fn test_new_changed_default_state() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Change default state in config
    let config_path = ".gitissues/config.yaml";
    let mut config = load_yaml_values(config_path);
    config["states"] = serde_yaml::Value::Sequence(vec![
        serde_yaml::Value::String("open".to_string()),
        serde_yaml::Value::String("active".to_string()),
        serde_yaml::Value::String("closed".to_string()),
        serde_yaml::Value::String("deleted".to_string()),
    ]);
    save_yaml_values(config_path, &config);

    // Create issue
    run_command(&["new", "Just an issue"]).expect("new failed");

    // Verify metadata
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Just an issue");
    assert_eq!(meta["type"].as_str().unwrap(), "");
    assert_eq!(meta["assignee"].as_str().unwrap(), "");
    assert_eq!(meta["reporter"].as_str().unwrap(), "");
    assert_eq!(meta["priority"].as_str().unwrap(), "");
    assert_eq!(meta["due_date"].as_str().unwrap(), "");
    assert_eq!(meta["labels"].as_sequence().unwrap().len(), 0);
    assert_eq!(meta["state"].as_str().unwrap(), "open");
}

#[test]
fn test_new_changed_default_priority() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Change default state in config
    let config_path = ".gitissues/config.yaml";
    let mut config = load_yaml_values(config_path);
    config["priority_default"] = serde_yaml::Value::String("P2".to_string());
    save_yaml_values(config_path, &config);

    // Create issue
    run_command(&["new", "Just an issue"]).expect("new failed");

    // Verify metadata
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta["id"].as_i64().unwrap(), 1);
    assert_eq!(meta["title"].as_str().unwrap(), "Just an issue");
    assert_eq!(meta["type"].as_str().unwrap(), "");
    assert_eq!(meta["assignee"].as_str().unwrap(), "");
    assert_eq!(meta["reporter"].as_str().unwrap(), "");
    assert_eq!(meta["priority"].as_str().unwrap(), "P2");
    assert_eq!(meta["due_date"].as_str().unwrap(), "");
    assert_eq!(meta["labels"].as_sequence().unwrap().len(), 0);
    assert_eq!(meta["state"].as_str().unwrap(), "new");
}

#[test]
fn test_new_invalid_metadata() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Try to create issue with invalid state
    run_command(&["new", "Title", "--state", "unknown"]).expect_err("new with invalid state successful but should fail");

    // Try to create issue with invalid type
    run_command(&["new", "Title", "--type", "unknown"]).expect_err("new with invalid type successful but should fail");

    // Try to create issue with invalid priority
    run_command(&["new", "Title", "--priority", "P5"]).expect_err("new with invalid priority successful but should fail");

    // Try to create issue with invalid reporter
    run_command(&["new", "Title", "--reporter", "unknown_user"]).expect_err("new with invalid reporter successful but should fail");

    // Try to create issue with invalid assignee
    run_command(&["new", "Title", "--assignee", "unknown_user"]).expect_err("new with invalid assignee successful but should fail");

    // Try to create issue with invalid due_date
    run_command(&["new", "Title", "--due-date", "2026-02-30"]).expect_err("new with invalid due_date successful but should fail");
}

#[test]
fn test_new_me() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");

    // Create an issue with reporter and assignee as 'me', which is empty by default
    run_command(&["new", "Issue 1", "--reporter", "me", "--assignee", "me"]).expect("new 1 failed");

    // List to check that reporter and assignee are empty (default user)
    let output = run_command(&["list", "--columns", "title,reporter,assignee"]).expect("list with reporter/assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains(" -"));

    // Change default user in settings
    let settings_path = ".gitissues/settings.yaml";
    let mut settings = load_yaml_values(settings_path);
    settings["user"] = serde_yaml::Value::String("bob".to_string());
    save_yaml_values(settings_path, &settings);

    // Create another issue with reporter and assignee as 'me', which is now 'bob'
    run_command(&["new", "Issue 2", "--reporter", "me", "--assignee", "me"]).expect("new 1 failed");

    // List to check that reporter and assignee are 'bob' (default user)
    let output =
        run_command(&["list", "--filter", "id=2", "--columns", "title,reporter,assignee"]).expect("list with reporter/assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("bob"));
    assert!(!stdout.contains(" -"));
}
