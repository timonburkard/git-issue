mod common;
use common::{TestEnv, disable_auto_commit, load_yaml_values, run_command, run_command_with_stdin, save_yaml_values};

#[test]
fn test_set_labels() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();
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
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that assignee is empty
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains(" -"));

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
    assert!(stdout.contains(" -"));
}

#[test]
fn test_set_reporter() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that reporter is empty
    let output = run_command(&["list", "--columns", "title,reporter"]).expect("list with reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains(" -"));

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
    assert!(stdout.contains(" -"));
}

#[test]
fn test_set_state() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

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
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that type is empty
    let output = run_command(&["list", "--columns", "title,type"]).expect("list with type failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("type"));
    assert!(stdout.contains(" -"));

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
    assert!(stdout.contains(" -"));
}

#[test]
fn test_set_priority() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that priority is empty
    let output = run_command(&["list", "--columns", "title,priority"]).expect("list with priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("priority"));
    assert!(stdout.contains(" -"));

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
    assert!(stdout.contains(" -"));
}

#[test]
fn test_set_due_date() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1"]).expect("new 1 failed");

    // List to check that due_date is empty
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains(" -"));

    // Set valid due_date
    run_command(&["set", "1", "--due-date", "2026-01-15"]).expect("set due_date failed");
    // List to check that due_date was set
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("2026-01-15"));

    // Set invalid due_date
    run_command(&["set", "1", "--due-date", "not-a-date"]).expect_err("set due_date successful but should fail");

    // List to check that non-existing due_date was not set
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("2026-01-15"));
    assert!(!stdout.contains("not-a-date"));

    // Set invalid due_date format
    run_command(&["set", "1", "--due-date", "15.01.2026"]).expect_err("set due_date successful but should fail");

    // List to check that non-existing due_date was not set
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("2026-01-15"));
    assert!(!stdout.contains("15.01.2026"));

    // Set invalid due_date date
    run_command(&["set", "1", "--due-date", "2026-02-30"]).expect_err("set due_date successful but should fail");

    // List to check that non-existing due_date was not set
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains("2026-01-15"));
    assert!(!stdout.contains("2026-02-30"));

    // Remove due_date
    run_command(&["set", "1", "--due-date", ""]).expect("remove due_date failed");

    // List to check that due_date was removed
    let output = run_command(&["list", "--columns", "title,due_date"]).expect("list with due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("due_date"));
    assert!(stdout.contains(" -"));
    assert!(!stdout.contains("2026-01-15"));
}

#[test]
fn test_set_me() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create an issue
    run_command(&["new", "Issue 1", "--reporter", "alice", "--assignee", "alice"]).expect("new 1 failed");

    // List to check that reporter and assignee are 'alice'
    let output = run_command(&["list", "--columns", "title,reporter,assignee"]).expect("list with reporter/assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("alice"));

    // Set reporter and assignee with 'me' to empty (default user)
    run_command(&["set", "1", "--reporter", "me", "--assignee", "me"]).expect("set reporter/assignee failed");

    // List to check that reporter and assignee were set to empty (default user)
    let output = run_command(&["list", "--columns", "title,reporter,assignee"]).expect("list with reporter/assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains(" -"));
    assert!(!stdout.contains("alice"));

    // Change default user in settings
    let settings_path = ".gitissues/settings.yaml";
    let mut settings = load_yaml_values(settings_path);
    settings["user"] = serde_yaml::Value::String("bob".to_string());
    save_yaml_values(settings_path, &settings);

    // Set reporter and assignee with 'me' to 'bob'
    run_command(&["set", "1", "--reporter", "me", "--assignee", "me"]).expect("set reporter/assignee failed");

    // List to check that reporter and assignee were set to 'bob'
    let output = run_command(&["list", "--columns", "title,reporter,assignee"]).expect("list with reporter/assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("reporter"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("bob"));
    assert!(!stdout.contains(" -"));
}

#[test]
fn test_set_bulk() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1", "--assignee", "alice"]).expect("new 1 failed");
    run_command(&["new", "Issue 2", "--assignee", "alice"]).expect("new 2 failed");
    run_command(&["new", "Issue 3", "--assignee", "alice"]).expect("new 3 failed");
    run_command(&["new", "Issue 4", "--assignee", "alice"]).expect("new 4 failed");

    // Bulk set valid state
    run_command(&["set", "1,2,3", "--state", "active"]).expect("bulk set state failed");

    // List to check that state was changed for ID 1,2 and 3
    let output = run_command(&["list", "--columns", "title,state", "--filter", "id=1,2,3"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(!stdout.contains("Issue 4"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("active"));
    assert!(!stdout.contains("new"));

    // List to check that state was not changed for ID 4
    let output = run_command(&["list", "--columns", "title,state", "--filter", "id=4"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(!stdout.contains("Issue 1"));
    assert!(!stdout.contains("Issue 2"));
    assert!(!stdout.contains("Issue 3"));
    assert!(stdout.contains("Issue 4"));
    assert!(stdout.contains("state"));
    assert!(!stdout.contains("active"));
    assert!(stdout.contains("new"));

    // Bulk set invalid state
    run_command(&["set", "1,2,3", "--state", "perfect"]).expect_err("bulk set state successful but should fail");

    // List to check that invalid state was not set
    let output = run_command(&["list", "--columns", "title,state", "--filter", "id=1,2,3"]).expect("list with state failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("state"));
    assert!(stdout.contains("active"));
    assert!(!stdout.contains("perfect"));

    // Bulk set empty assignee for ID 1,2 and 3
    run_command(&["set", "1,2,3", "--assignee", ""]).expect("bulk set assignee failed");

    // List to check that assignee is empty for ID 1,2 and 3
    let output = run_command(&["list", "--columns", "title,assignee", "--filter", "id=1,2,3"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains(" -"));
    assert!(!stdout.contains("alice"));

    // List to check that assignee is unchanged for ID 4
    let output = run_command(&["list", "--columns", "title,assignee", "--filter", "id=4"]).expect("list with assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 4"));
    assert!(stdout.contains("assignee"));
    assert!(!stdout.contains(" -"));
    assert!(stdout.contains("alice"));
}

#[test]
fn test_set_bulk_wildcard() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1", "--assignee", "alice"]).expect("new 1 failed");
    run_command(&["new", "Issue 2", "--assignee", "bob"]).expect("new 2 failed");
    run_command(&["new", "Issue 3", "--assignee", "alice"]).expect("new 3 failed");

    run_command(&["set", "*", "--assignee", "charlie"]).expect_err("bulk set with wildcard should fail but succeeded");

    // List to enable wildcard selection
    let output = run_command(&["list", "--columns", "title,assignee", "--filter", "assignee=alice"]).expect("list with filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(!stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("assignee"));
    assert!(stdout.contains("alice"));
    assert!(!stdout.contains("bob"));

    // Bulk set with wildcard: change all issues assigned to 'alice' to 'carol'
    run_command_with_stdin(&["set", "*", "--assignee", "carol"], "y\n").expect("bulk set with wildcard failed");

    // List to check bulk set with wildcard
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("assignee"));
    assert!(!stdout.contains("alice"));
    assert!(stdout.contains("bob"));
    assert!(stdout.contains("carol"));

    // List to enable wildcard selection
    let output = run_command(&["list", "--columns", "title,assignee", "--filter", "assignee=carol"]).expect("list with filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(!stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("assignee"));
    assert!(!stdout.contains("alice"));
    assert!(!stdout.contains("bob"));
    assert!(stdout.contains("carol"));

    // Bulk set with wildcard: don't change all issues assigned to 'carol' to 'alice'
    run_command_with_stdin(&["set", "*", "--assignee", "alice"], "n\n")
        .expect_err("bulk set with wildcard and 'n' reply succeeded but should fail");

    // List to check bulk set with wildcard and 'no' reply did not succeed
    let output = run_command(&["list", "--columns", "title,assignee"]).expect("list with filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("title"));
    assert!(stdout.contains("Issue 1"));
    assert!(stdout.contains("Issue 2"));
    assert!(stdout.contains("Issue 3"));
    assert!(stdout.contains("assignee"));
    assert!(!stdout.contains("alice"));
    assert!(stdout.contains("bob"));
    assert!(stdout.contains("carol"));
}
