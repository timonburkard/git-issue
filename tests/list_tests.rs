use std::fs;

mod common;
use common::{TestEnv, disable_auto_commit, load_yaml_values, run_command, save_yaml_values};

#[test]
fn test_list_columns() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

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

#[test]
fn test_list_filter() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

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
        "2026-01-30",
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
        "carol",
        "--labels",
        "ui,gui",
        "--priority",
        "P3",
        "--due_date",
        "2027-01-02",
    ])
    .expect("new 2 failed");
    run_command(&[
        "new",
        "Issue 3",
        "--type",
        "bug",
        "--assignee",
        "carol",
        "--reporter",
        "carol",
        "--labels",
        "fw",
        "--priority",
        "",
        "--due_date",
        "2026-06-16",
    ])
    .expect("new 3 failed");

    // List without filter
    let output = run_command(&["list", "--columns", "id"]).expect("list without filters failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '=' on ID
    let output = run_command(&["list", "--columns", "id", "--filter", "id=2"]).expect("list with filter on ID failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on ID (OR)
    let output = run_command(&["list", "--columns", "id", "--filter", "id=1,3"]).expect("list with filter on ID failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '=' on assignee
    let output = run_command(&["list", "--columns", "id", "--filter", "assignee=alice"]).expect("list with filter on assignee failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on reporter
    let output = run_command(&["list", "--columns", "id", "--filter", "reporter=bob"]).expect("list with filter on reporter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on priority
    let output = run_command(&["list", "--columns", "id", "--filter", "priority=P3"]).expect("list with filter on priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on empty priority
    // In CLI this corresponds to priority=''
    let output = run_command(&["list", "--columns", "id", "--filter", "priority="]).expect("list with filter on priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '=' on type
    let output = run_command(&["list", "--columns", "id", "--filter", "type=bug"]).expect("list with filter on type failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '=' on labels (Simple1)
    let output = run_command(&["list", "--columns", "id", "--filter", "labels=ui"]).expect("list with filter on labels failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on labels (Simple2)
    let output = run_command(&["list", "--columns", "id", "--filter", "labels=fw"]).expect("list with filter on labels failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '=' on labels (OR)
    let output = run_command(&["list", "--columns", "id", "--filter", "labels=gui,cli"]).expect("list with filter on labels failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on labels (AND)
    let output =
        run_command(&["list", "--columns", "id", "--filter", "labels=ui", "labels=gui"]).expect("list with filter on labels failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on labels (no match)
    let output = run_command(&["list", "--columns", "id", "--filter", "labels=bla"]).expect("list with filter on labels failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '=' on due_date
    let output = run_command(&["list", "--columns", "id", "--filter", "due_date=2026-06-16"]).expect("list with filter on due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with filter '<' and '>' on ID
    // In CLI this corresponds to id\>1 id\<3
    let output = run_command(&["list", "--columns", "id", "--filter", "id>1", "id<3"]).expect("list with filter on ID failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // List with filter '<' on priority
    // In CLI this corresponds to priority\<P2
    let output = run_command(&["list", "--columns", "id", "--filter", "priority<P2"]).expect("list with filter on priority failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3")); // empty is considered smallest

    // List with filter '<' on due_date
    // In CLI this corresponds to due_date\<2027
    let output = run_command(&["list", "--columns", "id", "--filter", "due_date<2027"]).expect("list with filter on due_date failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(stdout.contains("1"));
    assert!(!stdout.contains("2"));
    assert!(stdout.contains("3"));

    // List with two filters (AND)
    let output = run_command(&["list", "--columns", "id", "--filter", "labels=ui", "reporter=carol"]).expect("list with and filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));

    // Add description to issue 2
    fs::write(
        ".gitissues/issues/0000000002/description.md",
        "This is a detailed description about the driver problem in this issue.",
    )
    .expect("failed to write description.md");

    // List with filter on description.md
    let output = run_command(&["list", "--columns", "id", "--filter", "assignee=alice", "description=*driver*"])
        .expect("list with description filter failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("id"));
    assert!(!stdout.contains("1"));
    assert!(stdout.contains("2"));
    assert!(!stdout.contains("3"));
}

#[test]
fn test_list_sort() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

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
        "2026-01-30",
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
        "carol",
        "--labels",
        "ui,gui",
        "--priority",
        "P3",
        "--due_date",
        "2027-01-02",
    ])
    .expect("new 2 failed");
    run_command(&[
        "new",
        "Issue 3",
        "--type",
        "bug",
        "--assignee",
        "carol",
        "--reporter",
        "carol",
        "--labels",
        "fw",
        "--priority",
        "",
        "--due_date",
        "2026-06-16",
    ])
    .expect("new 3 failed");

    // List without sort (default is desc ID)
    let output = run_command(&["list", "--columns", "id"]).expect("list without filters failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position3);
    assert!(position3 < position2);
    assert!(position2 < position1);

    // List with sort: asc ID
    let output = run_command(&["list", "--columns", "id", "--sort", "id=asc"]).expect("list with sort failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position1);
    assert!(position1 < position2);
    assert!(position2 < position3);

    // List with sort: desc ID
    let output = run_command(&["list", "--columns", "id", "--sort", "id=desc"]).expect("list with sort failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position3);
    assert!(position3 < position2);
    assert!(position2 < position1);

    // List with sort: asc due_date
    let output = run_command(&["list", "--columns", "id", "--sort", "due_date=asc"]).expect("list with sort failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position1);
    assert!(position1 < position3);
    assert!(position3 < position2);

    // List with sort: desc due_date
    let output = run_command(&["list", "--columns", "id", "--sort", "due_date=desc"]).expect("list with sort failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position2);
    assert!(position2 < position3);
    assert!(position3 < position1);

    // List with sort: asc assignee, desc reporter
    let output = run_command(&["list", "--columns", "id", "--sort", "assignee=asc", "reporter=desc"]).expect("list with sort failed");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let position_header = stdout.find("id").expect("id header not found");
    let position1 = stdout.find("1").expect("id 1 not found");
    let position2 = stdout.find("2").expect("id 2 not found");
    let position3 = stdout.find("3").expect("id 3 not found");
    assert!(position_header < position2);
    assert!(position2 < position1);
    assert!(position1 < position3);
}
