use chrono::Utc;
use std::time::Duration;

mod common;
use common::{TestEnv, disable_auto_commit, load_yaml_values, run_command, save_yaml_values};

#[test]
fn test_show_empty() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Change settings: Set 'editor' to 'cat' resp. 'cat'-equivalent
    let editor = if cfg!(windows) { "type" } else { "cat" };
    let settings_path = ".gitissues/settings.yaml";
    let mut settings = load_yaml_values(settings_path);
    settings["editor"] = serde_yaml::Value::String(editor.to_string());
    save_yaml_values(settings_path, &settings);

    // Current time
    let t = Utc::now();
    let now = t.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let now_plus_1s = (t + Duration::from_secs(1)).format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Create new issue
    run_command(&["new", "Simple issue"]).expect("new failed");

    let expected_template = include_str!("includes/show_empty.md").replace("\r\n", "\n");
    let expected = expected_template.replace("__DATE__", &now);
    let expected_plus_1 = expected_template.replace("__DATE__", &now_plus_1s);

    let output = run_command(&["show", "1"]).expect("show failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_str = stdout.as_ref().replace("\r\n", "\n");

    assert!((stdout_str == expected) || (stdout_str == expected_plus_1));
}

#[test]
fn test_show_metadata() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Change settings: Set 'editor' to 'cat' resp. 'cat'-equivalent
    let editor = if cfg!(windows) { "type" } else { "cat" };
    let settings_path = ".gitissues/settings.yaml";
    let mut settings = load_yaml_values(settings_path);
    settings["editor"] = serde_yaml::Value::String(editor.to_string());
    save_yaml_values(settings_path, &settings);

    // Current time
    let t = Utc::now();
    let now = t.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let now_plus_1s = (t + Duration::from_secs(1)).format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Create new issue
    run_command(&["new", "Very important new feature"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");

    // Set some metadata
    run_command(&["set", "1", "--state", "active"]).expect("set failed");
    run_command(&["set", "1", "--type", "feature"]).expect("set failed");
    run_command(&["set", "1", "--priority", "P1"]).expect("set failed");
    run_command(&["set", "1", "--labels", "ui,driver"]).expect("set failed");
    run_command(&["set", "1", "--reporter", "alice"]).expect("set failed");
    run_command(&["set", "1", "--assignee", "bob"]).expect("set failed");
    run_command(&["set", "1", "--due_date", "2026-06-24"]).expect("set failed");
    run_command(&["link", "1", "--add", "related=2"]).expect("set failed");
    run_command(&["link", "1", "--add", "child=2,3"]).expect("set failed");

    let expected_template = include_str!("includes/show_metadata.md").replace("\r\n", "\n");
    let expected = expected_template.replace("__DATE__", &now);
    let expected_plus_1 = expected_template.replace("__DATE__", &now_plus_1s);

    let output = run_command(&["show", "1"]).expect("show failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_str = stdout.as_ref().replace("\r\n", "\n");

    assert!((stdout_str == expected) || (stdout_str == expected_plus_1));
}
