use std::fs;
use std::path::Path;

use crate::model::Meta;

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

    println!("id:       {}", meta.id);
    println!("title:    {}", meta.title);
    println!("state:    {}", meta.state);
    println!(
        "type:     {}",
        if meta.type_.is_empty() {
            "-".to_string()
        } else {
            meta.type_
        }
    );
    println!(
        "labels:   {}",
        if meta.labels.is_empty() {
            "-".to_string()
        } else {
            meta.labels.join(",")
        }
    );
    println!(
        "assignee: {}",
        if meta.assignee.is_empty() {
            "-".to_string()
        } else {
            meta.assignee
        }
    );
    println!("created:  {}", meta.created);
    println!("updated:  {}", meta.updated);

    // Load description.md
    let md_path = path.join("description.md");
    let md_raw = match fs::read_to_string(&md_path) {
        Ok(s) => s,
        Err(_) => return Err("description.md not found.".to_string()),
    };

    println!("description:\n{md_raw}");

    Ok(())
}
