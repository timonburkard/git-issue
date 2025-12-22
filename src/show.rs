use std::fs;
use std::path::Path;

use regex::Regex;

use crate::model::{Config, Meta, open_editor};

pub fn run(id: u32) -> Result<(), String> {
    let id_str = format!("{id:010}");
    let issue_path = format!(".gitissues/issues/{id_str}");
    let path = Path::new(&issue_path);

    // Precondition: .gitissues/issues/ID must exist
    if !path.exists() {
        return Err("Not available: ID does not exist.".to_string());
    }

    // Load meta.yaml
    let meta_path = path.join("meta.yaml");
    let meta_raw = match fs::read_to_string(&meta_path) {
        Ok(s) => s,
        Err(_) => return Err("meta.yaml not found.".to_string()),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err("meta.yaml malformatted.".to_string()),
    };

    // Create tmp directory structure if not already exists

    let tmp = ".gitissues/.tmp";
    let tmp_dir = Path::new(tmp);

    if !Path::new(tmp_dir).exists() {
        // Create the directory structure
        fs::create_dir_all(&tmp_dir)
            .map_err(|e| format!("Failed to create {}: {e}", tmp_dir.display()))?;
    }

    let mut content: String = generate_content_metadata(id, &meta);

    add_content_description(path, &mut content)?;

    // Copy attachments and rewrite links in content
    let attachments_src = path.join("attachments");
    let tmp_attachments = format!(".gitissues/.tmp/attachments-{id}");
    let tmp_attachments_dir = Path::new(&tmp_attachments);

    if attachments_src.exists() {
        copy_dir_recursive(&attachments_src, tmp_attachments_dir)?;
        // Rewrite markdown links to point at temp attachments folder
        content = content
            .replace("(attachments/", &format!("(attachments-{id}/"))
            .replace("](attachments/", &format!("](attachments-{id}/"))
            .replace("src=\"attachments/", &format!("src=\"attachments-{id}/"))
            .replace("href=\"attachments/", &format!("href=\"attachments-{id}/"));
    }

    // Write file
    let tmp_file = format!(".gitissues/.tmp/show-{id}.md");
    fs::write(&tmp_file, content).map_err(|e| format!("Failed to write {}: {e}", tmp_file))?;

    // Load configuration
    let config_path = Path::new(".gitissues/config.yaml");
    let config_raw = match fs::read_to_string(config_path) {
        Ok(s) => s,
        Err(_) => return Err("config.yaml not found.".to_string()),
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(_) => return Err("config.yaml malformatted.".to_string()),
    };

    open_editor(config.editor, tmp_file.clone())?;

    // Clean up temp artifacts
    let _ = fs::remove_file(&tmp_file);
    if attachments_src.exists() {
        let _ = fs::remove_dir_all(tmp_attachments_dir);
    }

    Ok(())
}

fn generate_content_metadata(id: u32, meta: &Meta) -> String {
    let mut content = String::new();

    content.push_str("<!-- READ-ONLY VIEW -->\n");
    content.push_str("\n");
    content.push_str(&format!("# Issue #{} -- {}\n", id, meta.title));
    content.push_str("\n");
    content.push_str("## Meta Data\n");
    content.push_str("\n");
    content.push_str("| **field**    | **value** |\n");
    content.push_str("| ------------ | --------- |\n");
    content.push_str(&format!("| **id**       | {} |\n", meta.id));
    content.push_str(&format!("| **title**    | {} |\n", meta.title));
    content.push_str(&format!("| **state**    | {} |\n", meta.state));
    content.push_str(&format!(
        "| **type**     | {} |\n",
        if meta.type_.is_empty() {
            "-".to_string()
        } else {
            meta.type_.clone()
        },
    ));
    content.push_str(&format!(
        "| **labels**   | {} |\n",
        if meta.labels.is_empty() {
            "-".to_string()
        } else {
            meta.labels.join(",")
        }
    ));
    content.push_str(&format!(
        "| **assignee** | {} |\n",
        if meta.assignee.is_empty() {
            "-".to_string()
        } else {
            meta.assignee.clone()
        }
    ));
    content.push_str(&format!("| **created**  | {} |\n", meta.created));
    content.push_str(&format!("| **updated**  | {} |\n", meta.updated));

    return content;
}

fn add_content_description(path: &Path, content: &mut String) -> Result<(), String> {
    // Load description.md
    let desc_path = path.join("description.md");
    let desc_raw = match fs::read_to_string(&desc_path) {
        Ok(s) => s,
        Err(_) => return Err("description.md not found.".to_string()),
    };

    let re = Regex::new(r"(?m)^#").unwrap(); // (?m) enables multi-line mode

    // Replace # with ###
    let desc_nested = re.replace_all(&desc_raw, "###");

    content.push_str("\n");
    content.push_str(&format!("## Description\n"));
    content.push_str("\n");
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
        let ty = entry
            .file_type()
            .map_err(|e| format!("Failed to read file type: {e}"))?;
        let name = entry.file_name();
        let src_path = entry.path();
        let dst_path = dst.join(&name);

        if ty.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path).map_err(|e| {
                format!(
                    "Failed to copy {} -> {}: {e}",
                    src_path.display(),
                    dst_path.display()
                )
            })?;
        }
    }

    Ok(())
}
