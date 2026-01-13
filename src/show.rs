use std::collections::HashMap;
use std::fs;
use std::path::Path;

use indexmap::IndexMap;

use regex::Regex;

use crate::model::{
    Meta, dash_if_empty, issue_attachments_dir, issue_dir, issue_meta_path, issue_tmp_show_dir, load_description, load_meta, load_settings,
    open_editor,
};

pub fn run(id: u32) -> Result<(), String> {
    let path = issue_dir(id)?;

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    // Load meta.yaml
    let meta = load_meta(&issue_meta_path(id)?)?;

    // Create per-issue tmp directory
    let tmp_issue_path = issue_tmp_show_dir(id)?;
    fs::create_dir_all(&tmp_issue_path).map_err(|e| format!("Failed to create {}: {e}", tmp_issue_path.display()))?;

    // Generate markdown content
    let mut content: String = generate_content_metadata(id, &meta);
    add_content_description(path.as_path(), &mut content)?;

    // Write markdown file
    let tmp_file = tmp_issue_path.join("show.md");
    fs::write(&tmp_file, content).map_err(|e| format!("Failed to write {}: {e}", tmp_file.display()))?;

    // Copy attachments to tmp directory
    let attachments_src = issue_attachments_dir(id)?;
    if attachments_src.exists() {
        let tmp_attachments = tmp_issue_path.join("attachments");
        copy_dir_recursive(&attachments_src, &tmp_attachments)?;
    }

    let settings = load_settings()?;

    open_editor(settings.editor, tmp_file.to_string_lossy().to_string())?;

    // Clean up entire per-issue tmp directory
    let _ = fs::remove_dir_all(tmp_issue_path);

    Ok(())
}

fn generate_content_metadata(id: u32, meta: &Meta) -> String {
    let mut content = String::new();

    let (width, values) = get_values(meta);

    content.push_str("<!-- READ-ONLY VIEW -->\n");
    content.push('\n');
    content.push_str(&format!("# Issue #{} -- {}\n", id, meta.title));
    content.push('\n');
    content.push_str("## Meta Data\n");
    content.push('\n');
    content.push_str(&format!("| **field**         | {:width$} |\n", "**value**"));
    content.push_str(&format!("| ----------------- | {} |\n", "-".repeat(width)));
    content.push_str(&format!("| **id**            | {:width$} |\n", values["id"]));
    content.push_str(&format!("| **title**         | {:width$} |\n", values["title"]));
    content.push_str(&format!("| **state**         | {:width$} |\n", values["state"]));
    content.push_str(&format!("| **type**          | {:width$} |\n", values["type"]));
    content.push_str(&format!("| **labels**        | {:width$} |\n", values["labels"]));
    content.push_str(&format!("| **reporter**      | {:width$} |\n", values["reporter"]));
    content.push_str(&format!("| **assignee**      | {:width$} |\n", values["assignee"]));
    content.push_str(&format!("| **priority**      | {:width$} |\n", values["priority"]));
    content.push_str(&format!("| **due_date**      | {:width$} |\n", values["due_date"]));
    content.push_str(&format!("| **relationships** | {}", values["relationships"]));
    content.push_str(&format!("| **created**       | {:width$} |\n", values["created"]));
    content.push_str(&format!("| **updated**       | {:width$} |\n", values["updated"]));

    content
}

fn get_values(meta: &Meta) -> (usize, HashMap<String, String>) {
    let mut values = HashMap::new();

    values.insert("id".to_string(), meta.id.to_string());
    values.insert("title".to_string(), meta.title.clone());
    values.insert("state".to_string(), meta.state.clone());
    values.insert("type".to_string(), dash_if_empty(&meta.type_));
    values.insert("labels".to_string(), dash_if_empty(&meta.labels.join(",")));
    values.insert("reporter".to_string(), dash_if_empty(&meta.reporter));
    values.insert("assignee".to_string(), dash_if_empty(&meta.assignee));
    values.insert("priority".to_string(), format!("{:?}", meta.priority));
    values.insert("due_date".to_string(), dash_if_empty(&meta.due_date));
    values.insert("created".to_string(), meta.created.clone());
    values.insert("updated".to_string(), meta.updated.clone());

    let mut max_width = values.values().map(|value| value.len()).max().unwrap_or(0);

    let (relationships_width, relationships_str) = content_relationships(&meta.relationships, max_width);

    values.insert("relationships".to_string(), relationships_str);

    if relationships_width > max_width {
        max_width = relationships_width;
    }

    (max_width, values)
}

fn content_relationships(relationships: &IndexMap<String, Vec<u32>>, min_width: usize) -> (usize, String) {
    let mut content = String::new();
    let mut max_width = 1;

    if relationships.is_empty() {
        content.push_str(&format!("{:min_width$} |\n", "-"));
        return (max_width, content);
    }

    let mut first = true;
    for (rel_type, ids) in relationships {
        let ids_str = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ");
        let string = format!("{}: {}", rel_type, ids_str);

        if string.len() > max_width {
            max_width = string.len();
        }

        if first {
            content.push_str(&format!("{:min_width$} |\n", string));
            first = false;
        } else {
            content.push_str(&format!("|                   | {:min_width$} |\n", string));
        }
    }

    (max_width, content)
}

fn add_content_description(path: &Path, content: &mut String) -> Result<(), String> {
    // Load description.md
    let desc_path = path.join("description.md");
    let desc_raw = load_description(&desc_path)?;

    let re = Regex::new(r"(?m)^#").map_err(|e| format!("Invalid regex for description headers: {e}"))?; // (?m) enables multi-line mode

    // Replace # with ###
    let desc_nested = re.replace_all(&desc_raw, "###");

    content.push('\n');
    content.push_str("## Description\n");
    content.push('\n');
    content.push_str(&format!("{desc_nested}"));

    Ok(())
}
fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    if !src.exists() {
        return Ok(()); // nothing to copy
    }

    fs::create_dir_all(dst).map_err(|e| format!("Failed to create {}: {e}", dst.display()))?;

    for entry in fs::read_dir(src).map_err(|e| format!("Failed to read {}: {e}", src.display()))? {
        let entry = entry.map_err(|e| format!("Failed to read entry: {e}"))?;
        let ty = entry.file_type().map_err(|e| format!("Failed to read file type: {e}"))?;
        let name = entry.file_name();
        let src_path = entry.path();
        let dst_path = dst.join(&name);

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| format!("Failed to copy {} -> {}: {e}", src_path.display(), dst_path.display()))?;
        }
    }

    Ok(())
}
