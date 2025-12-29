use std::fs;
use std::path::Path;
use std::process::Command;
use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use clap::ValueEnum;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ValueEnum)]
pub enum Priority {
    // clap default to lower case, so add aliases for upper case too
    #[value(alias = "P0")]
    P0,
    #[value(alias = "P1")]
    P1,
    #[value(alias = "P2")]
    P2,
    #[value(alias = "P3")]
    P3,
    #[value(alias = "P4")]
    P4,
}

impl Priority {
    pub fn as_int(&self) -> u8 {
        match self {
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "p0" => Ok(Priority::P0),
            "P0" => Ok(Priority::P0),
            "p1" => Ok(Priority::P1),
            "P1" => Ok(Priority::P1),
            "p2" => Ok(Priority::P2),
            "P2" => Ok(Priority::P2),
            "p3" => Ok(Priority::P3),
            "P3" => Ok(Priority::P3),
            "p4" => Ok(Priority::P4),
            "P4" => Ok(Priority::P4),
            _ => Err(format!("Unknown priority: {s}")),
        }
    }
}

#[derive(Clone)]
pub struct RelationshipLink {
    pub relationship: String,
    pub target_ids: Vec<u32>,
}

impl FromStr for RelationshipLink {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (relationship, target_ids) = s.split_once('=').ok_or("expected format: <relationship>=<target_ids>")?;

        let target_ids = target_ids
            .split(',')
            .map(|id| id.trim().parse::<u32>().map_err(|_| format!("invalid target id: {id}")))
            .collect::<Result<Vec<_>, _>>()?;

        if target_ids.is_empty() {
            return Err("at least one target id is required".into());
        }

        Ok(RelationshipLink {
            relationship: relationship.to_string(),
            target_ids,
        })
    }
}

#[derive(Clone)]
pub enum Operator {
    Eq,
    Lt,
    Gt,
}

#[derive(Clone)]
pub struct Filter {
    pub field: String,
    pub operator: Operator,
    pub value: String,
}

impl FromStr for Filter {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let field: &str;
        let operator: Operator;
        let value: &str;

        if s.contains('=') {
            operator = Operator::Eq;
            (field, value) = s.split_once('=').ok_or("expected format: <field>{=|>|<}<value>")?;
        } else if s.contains('>') {
            operator = Operator::Gt;
            (field, value) = s.split_once('>').ok_or("expected format: <field>{=|>|<}<value>")?;
        } else if s.contains('<') {
            operator = Operator::Lt;
            (field, value) = s.split_once('<').ok_or("expected format: <field>{=|>|<}<value>")?;
        } else {
            return Err("expected operator: =, >, or <".to_string());
        }

        Ok(Filter {
            field: field.to_string(),
            operator,
            value: value.to_string(),
        })
    }
}

#[derive(Clone, Debug)]
pub enum Order {
    Asc,
    Desc,
}

#[derive(Clone)]
pub struct Sorting {
    pub field: String,
    pub order: Order,
}

impl FromStr for Sorting {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, order) = s.split_once('=').ok_or("expected format: <field>=asc|desc")?;

        let order_enum = match order.to_lowercase().as_str() {
            "asc" => Order::Asc,
            "desc" => Order::Desc,
            _ => return Err("Unknown order, expected 'asc' or 'desc'".to_string()),
        };

        Ok(Sorting {
            field: field.to_string(),
            order: order_enum,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Meta {
    pub id: u32,
    pub title: String,
    pub state: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub labels: Vec<String>,
    pub reporter: String,
    pub assignee: String,
    pub priority: Priority,
    pub due_date: String,
    pub relationships: IndexMap<String, Vec<u32>>,
    pub created: String,
    pub updated: String,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    pub link: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IdGeneration {
    Sequential, // Sequential numbers (1, 2, 3, ...)
    Timestamp,  // Timestamps in seconds since 2025-01-01
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub commit_auto: bool,
    pub commit_message: String,
    pub list_columns: Vec<String>,
    pub states: Vec<String>,
    pub types: Vec<String>,
    pub relationships: IndexMap<String, Relationship>,
    pub export_csv_separator: char,
    pub id_generation: IdGeneration,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub editor: String,
    pub user: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub users: Vec<User>,
}

/// Load users.yaml
pub fn load_users() -> Result<Users, String> {
    let users_path = users_path();
    let users_raw = match fs::read_to_string(&users_path) {
        Ok(s) => s,
        Err(_) => return Err("users.yaml not found.".to_string()),
    };

    let users: Users = match serde_yaml::from_str(&users_raw) {
        Ok(m) => m,
        Err(_) => return Err("users.yaml malformatted.".to_string()),
    };

    Ok(users)
}

/// Generate a proper ISO 8601 timestamp using chrono.
pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

/// Validate if a string is in ISO format: YYYY-MM-DD or empty.
pub fn is_valid_iso_date(s: &str) -> Result<bool, String> {
    Ok(s.is_empty() || NaiveDate::parse_from_str(s, "%Y-%m-%d").is_ok())
}

/// Validate if a state is in the list of valid states from config.
pub fn is_valid_state(s: &str) -> Result<bool, String> {
    let config = load_config()?;
    Ok(config.states.contains(&s.to_string()))
}

/// Validate if a type is in the list of valid types from config or empty.
pub fn is_valid_type(s: &str) -> Result<bool, String> {
    let config = load_config()?;
    Ok(s.is_empty() || config.types.contains(&s.to_string()))
}

/// Validate if an user is in the list of valid users:id from users.yaml.
pub fn is_valid_user(s: &str) -> Result<bool, String> {
    let users = load_users()?;
    Ok(s.is_empty() || s == "me" || users.users.iter().any(|u| u.id == s))
}

pub fn user_handle_me(value: &mut String) -> Result<(), String> {
    if *value == "me" {
        let settings = load_settings()?;
        *value = match is_valid_user(&settings.user) {
            Ok(true) => settings.user,
            Ok(false) => return Err("Invalid user: settings.yaml::user must be part of users.yaml:users:id or ''".to_string()),
            Err(e) => return Err(format!("Config error: {e}")),
        }
    }

    Ok(())
}

pub fn load_config() -> Result<Config, String> {
    let config_path = config_path();
    let config_raw = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => return Err("config.yaml not found.".to_string()),
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(_) => return Err("config.yaml malformatted.".to_string()),
    };

    Ok(config)
}

pub fn load_settings() -> Result<Settings, String> {
    let settings_path = settings_path();
    let settings_raw = match fs::read_to_string(&settings_path) {
        Ok(s) => s,
        Err(_) => return Err("settings.yaml not found.".to_string()),
    };

    let settings: Settings = match serde_yaml::from_str(&settings_raw) {
        Ok(m) => m,
        Err(_) => return Err("settings.yaml malformatted.".to_string()),
    };

    Ok(settings)
}

pub fn load_meta(path: &Path) -> Result<Meta, String> {
    let meta_raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return Err(format!("meta.yaml not found: {}", path.display())),
    };

    let meta: Meta = match serde_yaml::from_str(&meta_raw) {
        Ok(m) => m,
        Err(_) => return Err(format!("meta.yaml malformatted: {}", path.display())),
    };

    Ok(meta)
}

pub fn load_description(path: &Path) -> Result<String, String> {
    let raw = fs::read_to_string(path).map_err(|_| format!("description.md not found: {}", path.display()))?;
    Ok(raw)
}

pub fn issue_title(id: u32) -> Result<String, String> {
    let meta_path = issue_meta_path(id);
    let meta = load_meta(&meta_path)?;
    Ok(meta.title)
}

pub fn padded_id(id: u32) -> String {
    format!("{id:010}")
}

/// Returns the path to the .gitissues base directory.
pub fn gitissues_base() -> &'static str {
    ".gitissues"
}

/// Returns the path to the config.yaml file.
pub fn config_path() -> std::path::PathBuf {
    Path::new(gitissues_base()).join("config.yaml")
}

/// Returns the path to the settings.yaml file.
pub fn settings_path() -> std::path::PathBuf {
    Path::new(gitissues_base()).join("settings.yaml")
}

/// Returns the path to the users.yaml file.
pub fn users_path() -> std::path::PathBuf {
    Path::new(gitissues_base()).join("users.yaml")
}

pub fn issue_dir(id: u32) -> std::path::PathBuf {
    Path::new(gitissues_base()).join("issues").join(padded_id(id))
}

pub fn issue_meta_path(id: u32) -> std::path::PathBuf {
    issue_dir(id).join("meta.yaml")
}

pub fn issue_desc_path(id: u32) -> std::path::PathBuf {
    issue_dir(id).join("description.md")
}

pub fn issue_attachments_dir(id: u32) -> std::path::PathBuf {
    issue_dir(id).join("attachments")
}

pub fn issue_tmp_show_dir(id: u32) -> std::path::PathBuf {
    Path::new(gitissues_base()).join(".tmp").join(format!("show-{id}"))
}

pub fn issue_exports_dir() -> std::path::PathBuf {
    Path::new(gitissues_base()).join("exports")
}

/// git commit based on template from config
pub fn git_commit(id: u32, title: String, action: &str) -> Result<(), String> {
    use std::process::Command;

    let config = load_config()?;

    // Check if auto-commit is enabled
    if !config.commit_auto {
        return Ok(());
    }

    // Prepare commit message
    let commit_message_template = config.commit_message;

    let commit_message = commit_message_template
        .replace("{action}", action)
        .replace("{id}", &format!("{id}"))
        .replace("{title}", &title);

    // Execute git add
    let add_result = Command::new("git").args(["add", gitissues_base()]).output();
    if let Err(e) = add_result {
        return Err(format!("Failed to stage .gitissues: {e}"));
    }

    // Execute git commit
    let commit_result = Command::new("git").args(["commit", "-m", &commit_message]).output();
    if let Err(e) = commit_result {
        return Err(format!("Failed to commit: {e}"));
    }

    Ok(())
}

/// Simple commit message not based on template or config
pub fn git_commit_non_templated(msg: &str) -> Result<(), String> {
    use std::process::Command;

    // Prepare commit message
    let commit_message = format!("[issue] {msg}");

    // Execute git add
    let add_result = Command::new("git").args(["add", gitissues_base()]).output();
    if let Err(e) = add_result {
        return Err(format!("Failed to stage .gitissues: {e}"));
    }

    // Execute git commit
    let commit_result = Command::new("git").args(["commit", "-m", &commit_message]).output();
    if let Err(e) = commit_result {
        return Err(format!("Failed to commit: {e}"));
    }

    Ok(())
}

pub fn open_editor(mut editor: String, path: String) -> Result<(), String> {
    if editor == "git" {
        // Read git default editor
        let output = Command::new("git")
            .args(["config", "--get", "core.editor"])
            .output()
            .map_err(|e| format!("Failed to get git editor: {e}"))?;

        if !output.status.success() {
            return Err("Git config failed or core.editor not set".to_string());
        }

        editor = String::from_utf8(output.stdout)
            .map_err(|e| format!("Failed to parse git output: {e}"))?
            .trim()
            .to_string();
    }

    // Parse editor command (handles quoted paths with arguments)
    let editor_parts = shell_words::split(&editor).map_err(|e| format!("Failed to parse editor command: {e}"))?;

    if editor_parts.is_empty() {
        return Err("No editor command specified".to_string());
    }

    // First part is the program, rest are arguments
    let program = &editor_parts[0];
    let mut cmd = Command::new(program);

    // Add any existing arguments from the editor config
    for arg in &editor_parts[1..] {
        cmd.arg(arg);
    }

    // Add the file to edit
    cmd.arg(&path);

    // Execute editor (with Windows-specific shell fallback for PATH/PATHEXT resolution)
    #[allow(unused_mut)]
    let mut status = cmd.status();

    #[cfg(windows)]
    if let Err(e) = &status {
        use std::io::ErrorKind;
        if e.kind() == ErrorKind::NotFound {
            // Build a single command line for cmd.exe to resolve (e.g., code -> code.cmd)
            let quoted_path = if path.contains(' ') {
                format!("\"{}\"", path)
            } else {
                path.clone()
            };
            let full = format!("{} {}", editor, quoted_path);
            status = Command::new("cmd").args(["/c", &full]).status();
        }
    }

    let status = status.map_err(|e| format!("Failed to open editor: {e}"))?;
    if !status.success() {
        return Err(format!("Editor exited with error code: {:?}", status.code()));
    }

    Ok(())
}

pub fn dash_if_empty(value: &str) -> String {
    if value.is_empty() { "-".to_string() } else { value.to_string() }
}
