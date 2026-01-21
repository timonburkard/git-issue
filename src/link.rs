use std::fs;

use crate::model::{
    Config, RelationshipLink, current_timestamp, git_commit, issue_dir, issue_meta_path, issue_title, load_config, load_meta,
};

enum Action {
    Add,
    Remove,
}

pub fn run(id: u32, add: Option<Vec<RelationshipLink>>, remove: Option<Vec<RelationshipLink>>) -> Result<(), String> {
    let dir = issue_dir(id)?;
    let path = dir.as_path();

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    let config = load_config()?;

    // Validate relationships

    if let Some(relationship) = add.as_deref() {
        validate_relationships(id, relationship, &config)?;
    }

    if let Some(relationship) = remove.as_deref() {
        validate_relationships(id, relationship, &config)?;
    }

    // Process relationships

    if let Some(relationship) = add {
        for relationship in relationship {
            update_relationship(Action::Add, id, &relationship, &config)?;
        }
    }

    if let Some(relationship) = remove {
        for relationship in relationship {
            update_relationship(Action::Remove, id, &relationship, &config)?;
        }
    }

    let title = issue_title(id)?;

    match git_commit(id, title, "links updated") {
        Ok(None) => {}
        Ok(Some(info)) => println!("{}", info),
        Err(e) => return Err(e),
    }

    println!("Updated issue relationship(s)");

    Ok(())
}

fn validate_relationships(id: u32, relationships: &[RelationshipLink], config: &Config) -> Result<(), String> {
    for relationship in relationships {
        check_relationship(&relationship.relationship, config)?;
        check_target_ids(id, &relationship.target_ids)?;
    }

    Ok(())
}

fn check_relationship(relationship: &str, config: &Config) -> Result<(), String> {
    if !config.relationships.contains_key(relationship) {
        return Err(format!(
            "Invalid relationship: \"{}\". Valid options: {:?} | Configurable in config.yaml:relationships",
            relationship,
            config.relationships.keys()
        ));
    }

    Ok(())
}

fn check_target_ids(id: u32, target_ids: &Vec<u32>) -> Result<(), String> {
    for target_id in target_ids {
        let dir = issue_dir(*target_id)?;
        let path = dir.as_path();

        if !path.exists() {
            return Err(format!("Invalid target ID: {} does not exist.", target_id));
        }

        if *target_id == id {
            return Err("Invalid target ID: cannot link issue to itself.".to_string());
        }
    }

    Ok(())
}

fn update_relationship(action: Action, id: u32, relationship: &RelationshipLink, config: &Config) -> Result<(), String> {
    let current_timestamp = current_timestamp();

    let meta_path = issue_meta_path(id)?;
    let meta = load_meta(&meta_path)?;

    let mut meta_updated = meta.clone();

    // update relationships of ID

    let relationship_category = meta_updated
        .relationships
        .entry(relationship.relationship.clone())
        .or_insert_with(Vec::new);

    let mut updated = false;
    for target_id in &relationship.target_ids {
        match action {
            Action::Add => {
                if !relationship_category.contains(target_id) {
                    relationship_category.push(*target_id);
                    updated = true;
                }
            }
            Action::Remove => {
                if relationship_category.contains(target_id) {
                    relationship_category.retain(|x| x != target_id);
                    updated = true;
                }
            }
        }
    }

    if updated {
        meta_updated.updated = current_timestamp.clone();
    }

    // update relationships of target ID

    let mut target_metas = Vec::new();
    let mut target_metas_paths = Vec::new();
    let mut target_metas_updated = Vec::new();

    if let Some(rel_config) = config.relationships.get(&relationship.relationship)
        && let Some(link) = &rel_config.link
    {
        // link is the name of the relationship that needs to be updated for the target IDs

        for target_id in &relationship.target_ids {
            let target_meta_path = issue_meta_path(*target_id)?;
            let target_meta = load_meta(&target_meta_path)?;

            let mut target_meta_updated = target_meta.clone();

            let relationship_category = target_meta_updated.relationships.entry(link.clone()).or_insert_with(Vec::new);

            match action {
                Action::Add => {
                    if !relationship_category.contains(&id) {
                        relationship_category.push(id);

                        target_meta_updated.updated = current_timestamp.clone();
                    }
                }
                Action::Remove => {
                    if relationship_category.contains(&id) {
                        relationship_category.retain(|x| x != &id);
                        target_meta_updated.updated = current_timestamp.clone();
                    }
                }
            }

            target_metas.push(target_meta);
            target_metas_updated.push(target_meta_updated);
            target_metas_paths.push(target_meta_path);
        }
    }

    // Save updated meta files

    if meta_updated != meta {
        let updated_yaml = serde_yaml::to_string(&meta_updated).map_err(|_| "Failed to serialize meta.yaml".to_string())?;

        fs::write(&meta_path, updated_yaml).map_err(|_| "Failed to write meta.yaml".to_string())?;
    } else {
        return Err("No changes made to relationships".to_string());
    }

    for ((target_meta, target_meta_updated), target_meta_path) in target_metas.into_iter().zip(target_metas_updated).zip(target_metas_paths)
    {
        if target_meta_updated == target_meta {
            continue;
        }

        let updated_target_yaml =
            serde_yaml::to_string(&target_meta_updated).map_err(|_| "Failed to serialize target meta.yaml".to_string())?;

        fs::write(&target_meta_path, updated_target_yaml).map_err(|_| "Failed to write target meta.yaml".to_string())?;
    }

    Ok(())
}
