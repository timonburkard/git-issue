mod common;
use common::{TestEnv, disable_auto_commit, load_yaml_values, run_command, save_yaml_values};

#[test]
fn test_link_add_bidirectional_symmetric() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");

    // Add related relationship from 1 to 2
    run_command(&["link", "1", "--add", "related=2"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![2]);

    // Verify that the relationship was added to issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);

    // Add related relationship from 1 to 3
    run_command(&["link", "1", "--add", "related=3"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![2, 3]);

    // Verify that the relationship was added to issue 3
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);

    // Verify that the relationship of issue 2 are unchanged
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);
}

#[test]
fn test_link_add_bidirectional_asymmetric() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");
    run_command(&["new", "Issue 4"]).expect("new failed");
    run_command(&["new", "Issue 5"]).expect("new failed");

    // Add child relationship from 1 to 2
    run_command(&["link", "1", "--add", "child=2"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let child: Vec<u32> = meta["relationships"]["child"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(child, vec![2]);

    // Verify that the relationship was added to issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);

    // Add child relationship from 1 to 3
    run_command(&["link", "1", "--add", "child=3"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let child: Vec<u32> = meta["relationships"]["child"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(child, vec![2, 3]);

    // Verify that the relationship was added to issue 3
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);

    // Verify that the relationship of issue 2 are unchanged
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);

    // Add parent relationship from 4 to 5
    run_command(&["link", "4", "--add", "parent=5"]).expect("link --add failed");

    // Verify that the relationship was added to issue 4
    let meta = load_yaml_values(".gitissues/issues/0000000004/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![5]);

    // Verify that the relationship was added to issue 5
    let meta = load_yaml_values(".gitissues/issues/0000000005/meta.yaml");
    let child: Vec<u32> = meta["relationships"]["child"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(child, vec![4]);
}

#[test]
fn test_link_add_unidirectional() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Change config: add relationship type "inspired_by" which is unidirectional
    let config_path = ".gitissues/config.yaml";
    let mut config = load_yaml_values(config_path);
    config["relationships"]["inspired_by"]["link"] = serde_yaml::Value::Null;
    save_yaml_values(config_path, &config);

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");

    // Add inspired_by relationship from issue 1 to 2
    run_command(&["link", "1", "--add", "inspired_by=2"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let inspired_by: Vec<u32> = meta["relationships"]["inspired_by"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(inspired_by, vec![2]);

    // Verify that the relationship was not added to issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    assert_eq!(meta["relationships"]["inspired_by"], serde_yaml::Value::Null);
}

#[test]
fn test_link_add_invalid() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");

    // Try to add inspired_by relationship from issue 1 to 2
    run_command(&["link", "1", "--add", "inspired_by=2"]).expect_err("link --add succeeded but should have failed");

    // Try to link to itself
    run_command(&["link", "1", "--add", "related=1"]).expect_err("link --add self succeeded but should have failed");

    // Try to link to invalid issue ID
    run_command(&["link", "1", "--add", "related=3"]).expect_err("link --add invalid id succeeded but should have failed");

    // Try to link to duplicated issue ID
    run_command(&["link", "1", "--add", "related=2"]).expect("link --add failed");
    run_command(&["link", "1", "--add", "related=2"]).expect_err("link --add duplicate succeeded but should have failed");
}

#[test]
fn test_link_remove_bidirectional_symmetric() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");

    // Add related relationship from 1 to 2 and 3
    run_command(&["link", "1", "--add", "related=2,3"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![2, 3]);

    // Verify that the relationship was added to issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);

    // Verify that the relationship was added to issue 3
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);

    // Remove related relationship from 1 to 2
    run_command(&["link", "1", "--remove", "related=2"]).expect("link --remove failed");

    // Verify that the relationship was removed from issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![3]);

    // Verify that the relationship was removed from issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![]);

    // Verify that the relationship was not from issue 3
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let related: Vec<u32> = meta["relationships"]["related"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(related, vec![1]);
}

#[test]
fn test_link_remove_bidirectional_asymmetric() {
    let _env = TestEnv::new();

    run_command(&["init", "--no-commit"]).expect("init failed");
    disable_auto_commit();

    // Create some issues
    run_command(&["new", "Issue 1"]).expect("new failed");
    run_command(&["new", "Issue 2"]).expect("new failed");
    run_command(&["new", "Issue 3"]).expect("new failed");

    // Add child relationship from 1 to 2 and 3
    run_command(&["link", "1", "--add", "child=2,3"]).expect("link --add failed");

    // Verify that the relationship was added to issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let child: Vec<u32> = meta["relationships"]["child"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(child, vec![2, 3]);

    // Verify that the relationship was added to issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);

    // Verify that the relationship was added to issue 3
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);

    // Remove child relationship from 1 to 2
    run_command(&["link", "1", "--remove", "child=2"]).expect("link --remove failed");

    // Verify that the relationship was removed from issue 1
    let meta = load_yaml_values(".gitissues/issues/0000000001/meta.yaml");
    let child: Vec<u32> = meta["relationships"]["child"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(child, vec![3]);

    // Verify that the relationship was removed from issue 2
    let meta = load_yaml_values(".gitissues/issues/0000000002/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![]);

    // Verify that the relationship of issue 3 is unchanged
    let meta = load_yaml_values(".gitissues/issues/0000000003/meta.yaml");
    let parent: Vec<u32> = meta["relationships"]["parent"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect();
    assert_eq!(parent, vec![1]);
}
