use std::fs;
use std::path::Path;

use indexmap::IndexMap;

use regex::Regex;

use crate::model::{
    Meta, dash_if_empty, issue_attachments_dir, issue_dir, issue_meta_path, issue_tmp_show_dir, load_description, load_meta, load_settings,
    open_editor,
};

pub fn run(id: u32) -> Result<(), String> {
    let path = issue_dir(id);

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    // Load meta.yaml
    let meta = load_meta(&issue_meta_path(id))?;

    // Create per-issue tmp directory
    let tmp_issue_path = issue_tmp_show_dir(id);
    fs::create_dir_all(&tmp_issue_path).map_err(|e| format!("Failed to create {}: {e}", tmp_issue_path.display()))?;

    // Generate markdown content
    let mut content: String = generate_content_metadata(id, &meta);
    add_content_description(path.as_path(), &mut content)?;

    // Write markdown file
    let tmp_file = tmp_issue_path.join("show.md");
    fs::write(&tmp_file, content).map_err(|e| format!("Failed to write {}: {e}", tmp_file.display()))?;

    // Copy attachments to tmp directory
    let attachments_src = issue_attachments_dir(id);
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

    content.push_str("<!-- READ-ONLY VIEW -->\n");
    content.push('\n');
    content.push_str(&format!("# Issue #{} -- {}\n", id, meta.title));
    content.push('\n');
    content.push_str("## Meta Data\n");
    content.push('\n');
    content.push_str("| **field**         | **value** |\n");
    content.push_str("| ----------------- | --------- |\n");
    content.push_str(&format!("| **id**            | {} |\n", meta.id));
    content.push_str(&format!("| **title**         | {} |\n", meta.title));
    content.push_str(&format!("| **state**         | {} |\n", meta.state));
    content.push_str(&format!("| **type**          | {} |\n", dash_if_empty(&meta.type_),));
    content.push_str(&format!("| **labels**        | {} |\n", dash_if_empty(&meta.labels.join(","))));
    content.push_str(&format!("| **assignee**      | {} |\n", dash_if_empty(&meta.assignee)));
    content.push_str(&format!("| **priority**      | {:?} |\n", meta.priority));
    content.push_str(&format!("| **due_date**      | {} |\n", dash_if_empty(&meta.due_date)));
    content.push_str(content_relationships(&meta.relationships).as_str());
    content.push_str(&format!("| **created**       | {} |\n", meta.created));
    content.push_str(&format!("| **updated**       | {} |\n", meta.updated));

    content
}

fn content_relationships(relationships: &IndexMap<String, Vec<u32>>) -> String {
    let mut content = String::new();

    if relationships.is_empty() {
        content.push_str("| **relationships** | - |\n");
        return content;
    }

    let mut first = true;
    for (rel_type, ids) in relationships {
        let ids_str = ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(", ");
        if first {
            content.push_str(&format!("| **relationships** | {}: {} |\n", rel_type, ids_str));
            first = false;
        } else {
            content.push_str(&format!("|                   | {}: {} |\n", rel_type, ids_str));
        }
    }

    content
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
