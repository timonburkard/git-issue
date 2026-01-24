use std::fmt;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{NaiveDate, Utc};
use clap::ValueEnum;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::cmd::util::load_meta;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Priority {
    // clap default to lower case, so add aliases for upper case too
    #[serde(rename = "")]
    #[value(name = "''")]
    #[value(alias = "")]
    Empty,
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
    pub fn as_int(&self) -> i8 {
        match self {
            Priority::Empty => -1,
            Priority::P0 => 0,
            Priority::P1 => 1,
            Priority::P2 => 2,
            Priority::P3 => 3,
            Priority::P4 => 4,
        }
    }
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "" => Ok(Priority::Empty),
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

impl fmt::Debug for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Priority::Empty => write!(f, "-"),
            Priority::P0 => write!(f, "P0"),
            Priority::P1 => write!(f, "P1"),
            Priority::P2 => write!(f, "P2"),
            Priority::P3 => write!(f, "P3"),
            Priority::P4 => write!(f, "P4"),
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
    pub _version: u32,
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
    pub _version: u32,
    pub commit_auto: bool,
    pub commit_message: String,
    pub list_columns: Vec<String>,
    pub states: Vec<String>,
    pub types: Vec<String>,
    pub relationships: IndexMap<String, Relationship>,
    pub id_generation: IdGeneration,
    pub priority_default: Priority,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum NamedColor {
    Black,
    BrightBlack,
    Red,
    BrightRed,
    Green,
    BrightGreen,
    Yellow,
    BrightYellow,
    Blue,
    BrightBlue,
    Magenta,
    BrightMagenta,
    Cyan,
    BrightCyan,
    White,
    BrightWhite,
    Bold,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Colors {
    pub header: NamedColor,
    pub me: NamedColor,
    pub due_date_overdue: NamedColor,
    pub state: IndexMap<String, NamedColor>,
    pub priority: IndexMap<String, NamedColor>,
    #[serde(rename = "type")]
    pub type_: IndexMap<String, NamedColor>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ListFormatting {
    pub header_separator: bool,
    pub colors: Colors,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub _version: u32,
    pub editor: String,
    pub user: String,
    pub export_csv_separator: char,
    pub list_formatting: ListFormatting,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Users {
    pub _version: u32,
    pub users: Vec<User>,
}

/// Load users.yaml
pub fn load_users() -> Result<Users, String> {
    let users_path = users_path()?;
    let users_raw = match fs::read_to_string(&users_path) {
        Ok(s) => s,
        Err(_) => return Err("users.yaml not found.".to_string()),
    };

    let users: Users = match serde_yaml::from_str(&users_raw) {
        Ok(m) => m,
        Err(e) => return Err(format!("users.yaml malformatted: {e}")),
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
pub fn is_valid_state(config: &Config, s: &str) -> bool {
    config.states.contains(&s.to_string())
}

/// Validate if a type is in the list of valid types from config or empty.
pub fn is_valid_type(config: &Config, s: &str) -> bool {
    s.is_empty() || config.types.contains(&s.to_string())
}

/// Validate if an user is in the list of valid users:id from users.yaml.
pub fn is_valid_user(users: &Users, s: &str) -> bool {
    s.is_empty() || s == "me" || users.users.iter().any(|u| u.id == s)
}

pub fn load_config() -> Result<Config, String> {
    let config_path = config_path()?;
    let config_raw = match fs::read_to_string(&config_path) {
        Ok(s) => s,
        Err(_) => return Err("config.yaml not found.".to_string()),
    };

    let config: Config = match serde_yaml::from_str(&config_raw) {
        Ok(m) => m,
        Err(e) => return Err(format!("config.yaml malformatted: {e}")),
    };

    Ok(config)
}

pub fn load_settings() -> Result<(Settings, Vec<String>), String> {
    let settings_path = settings_path()?;

    let info = create_settings_if_missing(true)?;

    let settings_raw = match fs::read_to_string(&settings_path) {
        Ok(s) => s,
        Err(_) => return Err("settings.yaml not found.".to_string()),
    };

    let settings: Settings = match serde_yaml::from_str(&settings_raw) {
        Ok(m) => m,
        Err(e) => return Err(format!("settings.yaml malformatted: {e}")),
    };

    Ok((settings, info))
}

pub fn create_settings_if_missing(print: bool) -> Result<Vec<String>, String> {
    const DEFAULT_SETTINGS: &str = include_str!("../config/settings-default.yaml");
    let settings_dst = settings_path()?;

    if let Ok(true) = fs::exists(&settings_dst) {
        return Ok(vec![]);
    }

    fs::write(&settings_dst, DEFAULT_SETTINGS)
        .map_err(|e| format!("Failed to write default settings to {}: {e}", settings_dst.display()))?;

    if print {
        return Ok(vec![format!("Created default local user settings at {}", settings_dst.display())]);
    }

    Ok(vec![])
}

pub fn issue_title(id: u32) -> Result<String, String> {
    let meta_path = issue_meta_path(id)?;
    let meta = load_meta(&meta_path)?;
    Ok(meta.title)
}

pub fn padded_id(id: u32) -> String {
    format!("{id:010}")
}

/// Returns the path to the .gitissues base directory.
pub fn gitissues_base() -> Result<PathBuf, String> {
    let mut current_dir = std::env::current_dir().map_err(|e| format!("Failed to get current directory: {e}"))?;

    loop {
        let gitissues_root = current_dir.join(".gitissues");

        if let Ok(true) = fs::exists(&gitissues_root) {
            return Ok(gitissues_root);
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => return Err(".gitissues not found: Run `git issue init` first".to_string()),
        };
    }
}

pub fn issues_dir() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join("issues"))
}

/// Returns the path to the config.yaml file.
pub fn config_path() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join("config.yaml"))
}

/// Returns the path to the settings.yaml file.
pub fn settings_path() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join("settings.yaml"))
}

/// Returns the path to the users.yaml file.
pub fn users_path() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join("users.yaml"))
}

pub fn issue_dir(id: u32) -> Result<std::path::PathBuf, String> {
    Ok(issues_dir()?.join(padded_id(id)))
}

pub fn issue_meta_path(id: u32) -> Result<std::path::PathBuf, String> {
    Ok(issue_dir(id)?.join("meta.yaml"))
}

pub fn issue_desc_path(id: u32) -> Result<std::path::PathBuf, String> {
    Ok(issue_dir(id)?.join("description.md"))
}

pub fn issue_attachments_dir(id: u32) -> Result<std::path::PathBuf, String> {
    Ok(issue_dir(id)?.join("attachments"))
}

pub fn issue_tmp_dir() -> Result<std::path::PathBuf, String> {
    Ok(gitissues_base()?.join(".tmp"))
}

pub fn issue_tmp_show_dir(id: u32) -> Result<std::path::PathBuf, String> {
    Ok(issue_tmp_dir()?.join(format!("show-{id}")))
}
