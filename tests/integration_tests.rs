use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod common;
use common::{TestEnv, disable_auto_commit, load_yaml_values, run_command};

#[test]
fn test_integration_basic_workflow() {
    let _env = TestEnv::new();

    // Step 1: Initialize without git commit
    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Verify .gitissues structure
    assert!(PathBuf::from(".gitissues").exists());
    assert!(PathBuf::from(".gitissues/config.yaml").exists());
    assert!(PathBuf::from(".gitissues/description.md").exists());
    assert!(PathBuf::from(".gitissues/settings.yaml").exists());
    assert!(PathBuf::from(".gitissues/users.yaml").exists());
    assert!(PathBuf::from(".gitissues/issues").exists());

    // Current time
    let t = Utc::now();
    let now = t.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let now_plus_1s = (t + Duration::from_secs(1)).format("%Y-%m-%dT%H:%M:%SZ").to_string();

    // Step 2: Create 3 issues
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
    let meta1 = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    assert_eq!(meta1["id"].as_i64().unwrap(), 1);
    assert_eq!(meta1["title"].as_str().unwrap(), "First issue");
    assert_eq!(meta1["state"].as_str().unwrap(), "new"); // default
    assert_eq!(meta1["type"].as_str().unwrap(), "");
    assert_eq!(meta1["reporter"].as_str().unwrap(), ""); // default
    assert_eq!(meta1["assignee"].as_str().unwrap(), "");
    assert_eq!(meta1["priority"].as_str().unwrap(), ""); // default
    assert_eq!(meta1["due_date"].as_str().unwrap(), "");

    let created = meta1["created"].as_str().unwrap();
    let updated = meta1["created"].as_str().unwrap();

    assert_eq!(created, updated);

    assert!(
        created == now || created == now_plus_1s,
        "expected {} or {}, got {}",
        now,
        now_plus_1s,
        created
    );

    let meta2 = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    assert_eq!(meta2["id"].as_i64().unwrap(), 2);
    assert_eq!(meta2["title"].as_str().unwrap(), "Second issue");

    let meta3 = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
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
        "--reporter",
        "bob",
        "--priority",
        "P1",
        "--due-date",
        "2026-06-15",
    ])
    .expect("set failed");

    // Step 5: Verify changes
    let prev_updated = meta2["updated"].as_str().unwrap().to_string();
    let meta2_updated = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
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
    assert_eq!(meta2_updated["reporter"].as_str().unwrap(), "bob");
    assert_eq!(meta2_updated["priority"].as_str().unwrap(), "P1");
    assert_eq!(meta2_updated["due_date"].as_str().unwrap(), "2026-06-15");
    assert!(meta2_updated["updated"].as_str().unwrap() >= prev_updated.as_str());
}
