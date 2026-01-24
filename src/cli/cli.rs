use std::collections::HashMap;
use std::fs;
use std::io::IsTerminal;
use std::io::{self, Write};
use std::time::Duration;

use anstyle::{AnsiColor, Effects, Reset, Style};
use chrono::Utc;

use git_issue::list::IssueData;
use git_issue::model::{
    Filter, NamedColor, Priority, RelationshipLink, Settings, Sorting, cache_path, current_timestamp, issue_exports_dir, load_settings,
    open_editor,
};

pub fn init(no_commit: bool) -> Result<(), String> {
    let result = git_issue::init(no_commit)?;

    for info in result.infos {
        println!("{}", info);
    }

    println!("Initialization done");

    Ok(())
}

pub fn new(
    title: String,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    let result = git_issue::new(title, type_, reporter, assignee, priority, due_date, labels)?;

    for info in result.infos {
        println!("{}", info);
    }

    println!("Created issue #{}", result.value);

    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn set(
    ids: Vec<String>,
    state: Option<String>,
    title: Option<String>,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
    labels_add: Option<Vec<String>>,
    labels_remove: Option<Vec<String>>,
) -> Result<(), String> {
    let using_wildcard = ids.len() == 1 && ids[0] == "*";

    let ids: Vec<u32> = if using_wildcard {
        read_cached_issue_ids()?
    } else {
        if ids.iter().any(|t| t == "*") {
            return Err("Cannot mix '*' with explicit IDs".to_string());
        }

        ids.iter()
            .map(|t| t.parse::<u32>().map_err(|_| format!("Invalid issue ID: {t}")))
            .collect::<Result<Vec<u32>, _>>()?
    };

    if using_wildcard {
        wildcard_confirmation(ids.len())?;
    }

    let result = git_issue::set(
        ids,
        state,
        title,
        type_,
        reporter,
        assignee,
        priority,
        due_date,
        labels,
        labels_add,
        labels_remove,
    )?;

    for info in result.infos {
        println!("{}", info);
    }

    match result.value {
        0 => return Err("No fields changed".to_string()),
        1 => println!("Updated issue field(s)"),
        num => println!("Updated {} issues' field(s)", num),
    };

    Ok(())
}

pub fn link(id: u32, add: Option<Vec<RelationshipLink>>, remove: Option<Vec<RelationshipLink>>) -> Result<(), String> {
    let result = git_issue::link(id, add, remove)?;

    for info in result.infos {
        println!("{}", info);
    }

    println!("Updated issue relationship(s)");

    Ok(())
}

pub fn list(
    columns: Option<Vec<String>>,
    filter: Option<Vec<Filter>>,
    sort: Option<Vec<Sorting>>,
    print_csv: bool,
    no_color: bool,
) -> Result<(), String> {
    let (settings, infos) = load_settings()?;

    for info in infos {
        println!("{}", info);
    }

    let result = git_issue::list(columns, filter, sort)?;

    for info in result.infos {
        println!("{}", info);
    }

    print_list(&settings, &result.value.issues, &result.value.columns, print_csv, no_color)?;

    Ok(())
}

pub fn show(id: u32) -> Result<(), String> {
    let (settings, infos) = load_settings()?;

    for info in infos {
        println!("{}", info);
    }

    let result = git_issue::show(id)?;

    for info in result.infos {
        println!("{}", info);
    }

    open_editor(settings.editor, result.value.to_string_lossy().to_string())?;

    Ok(())
}

pub fn edit(id: u32) -> Result<(), String> {
    let (settings, infos) = load_settings()?;

    for info in infos {
        println!("{}", info);
    }

    let result = git_issue::edit_start(id)?;

    for info in result.infos {
        println!("{}", info);
    }

    open_editor(settings.editor, result.value.to_string_lossy().to_string())?;

    let result = git_issue::edit_end(id)?;

    for info in result.infos {
        println!("{}", info);
    }

    Ok(())
}

fn read_cached_issue_ids() -> Result<Vec<u32>, String> {
    let cache_file = cache_path()?;

    // ensure cache file is not too old
    let metadata = fs::metadata(&cache_file).map_err(|_| "Cached ID list is empty; run 'git issue list' first.".to_string())?;
    if let Ok(modified) = metadata.modified()
        && let Ok(elapsed) = modified.elapsed()
        && elapsed > Duration::from_secs(300)
    {
        return Err("Cached ID list is stale; run 'git issue list' first.".to_string());
    }

    let cache_content = fs::read_to_string(&cache_file).map_err(|_| "Cached ID list is empty; run 'git issue list' first.".to_string())?;
    let issue_ids: Result<Vec<u32>, _> = cache_content.split(',').map(|s| s.trim().parse::<u32>()).collect();

    if let Ok(value) = issue_ids {
        Ok(value)
    } else {
        Err("Cached ID list is empty; run 'git issue list' first.".to_string())
    }
}

fn wildcard_confirmation(num_of_ids: usize) -> Result<(), String> {
    println!("Modify {} issues from last list cache.", num_of_ids);
    print!("Continue? [y/N] ");
    io::stdout().flush().map_err(|e| format!("Failed to flush stdout: {e}"))?;

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(|e| format!("Failed to read input: {e}"))?;
    if !input.trim().eq_ignore_ascii_case("y") {
        return Err("Cancelled".to_string());
    }

    Ok(())
}

/// print list
/// - config: loaded configuration
/// - settings: loaded user settings
/// - issues: list of issue data
/// - columns: list of columns to print (None means default from config)
/// - print_csv: whether to print as CSV
/// - no_color: whether to disable color output
fn print_list(settings: &Settings, issues: &Vec<IssueData>, columns: &Vec<String>, print_csv: bool, no_color: bool) -> Result<(), String> {
    let column_widths = calculate_column_widths(issues, columns)?;

    let mut csv_content = String::new();
    let csv_separator = settings.export_csv_separator;

    // Enable colors only for interactive terminals and when NO_COLOR is not set
    let color_enabled = std::env::var("NO_COLOR").is_err() && std::io::stdout().is_terminal() && !no_color;

    // Print header
    for col in columns {
        if print_csv {
            csv_content.push_str(&to_csv_field(col, csv_separator));
        } else {
            let width = *column_widths.get(col).unwrap_or(&22);
            let styled = if color_enabled {
                colorize_header(settings, col)
            } else {
                col.to_string()
            };
            let padding = width.saturating_sub(col.len());
            print!("{}{}", styled, " ".repeat(padding));
        }
    }

    print_ln(print_csv, &mut csv_content);

    if !print_csv {
        print_header_separator(settings, columns, &column_widths);
    }

    // Print rows
    for issue in issues {
        for col in columns {
            let value = issue.data.get(col).map(String::as_str).unwrap_or("");

            if print_csv {
                csv_content.push_str(&to_csv_field(value, csv_separator));
            } else {
                let width = *column_widths.get(col).unwrap_or(&22);
                let colored_value = if color_enabled {
                    colorize_value(settings, col, value)
                } else {
                    value.to_string()
                };
                let padding = width.saturating_sub(value.len());
                print!("{}{}", colored_value, " ".repeat(padding));
            }
        }

        print_ln(print_csv, &mut csv_content);
    }

    if print_csv {
        // Create exports directory
        let export_dir = issue_exports_dir()?;
        fs::create_dir_all(&export_dir).map_err(|e| format!("Failed to create {}: {e}", export_dir.display()))?;

        // Write CSV file
        let export_file = export_dir.join(format!("{}.csv", current_timestamp().replace(":", "-")));
        fs::write(&export_file, csv_content).map_err(|e| format!("Failed to write {}: {e}", export_file.display()))?;
    }

    cache_issue_ids(&issues.iter().map(|issue| issue.id).collect::<Vec<u32>>())?; // For `set` command wildcard support

    Ok(())
}

fn calculate_column_widths(issues: &Vec<IssueData>, columns: &[String]) -> Result<std::collections::HashMap<String, usize>, String> {
    let mut widths: HashMap<String, usize> = HashMap::new();

    // Initialize with header widths
    for col in columns {
        widths.insert(col.clone(), col.len());
    }

    // Update with max content widths
    for issue in issues {
        for col in columns {
            let value = issue.data.get(col).map(String::as_str).unwrap_or("");
            let width = widths.get(col).copied().unwrap_or(0);
            widths.insert(col.clone(), width.max(value.len()));
        }
    }

    // Add padding (2 spaces)
    for width in widths.values_mut() {
        *width += 2;
    }

    Ok(widths)
}

fn print_ln(print_csv: bool, csv_content: &mut String) {
    if print_csv {
        csv_content.push('\n');
    } else {
        println!();
    }
}

fn to_csv_field(value: &str, separator: char) -> String {
    format!("\"{value}\"{separator}")
}

fn apply_style(text: &str, style: Style) -> String {
    format!("{style}{text}{reset}", reset = Reset)
}

fn fg(color: AnsiColor) -> Style {
    Style::new().fg_color(Some(color.into()))
}

fn named_color_to_style(color: NamedColor) -> Style {
    match color {
        NamedColor::White => fg(AnsiColor::White),
        NamedColor::BrightWhite => fg(AnsiColor::BrightWhite),
        NamedColor::Black => fg(AnsiColor::Black),
        NamedColor::BrightBlack => fg(AnsiColor::BrightBlack),
        NamedColor::Red => fg(AnsiColor::Red),
        NamedColor::BrightRed => fg(AnsiColor::BrightRed),
        NamedColor::Green => fg(AnsiColor::Green),
        NamedColor::BrightGreen => fg(AnsiColor::BrightGreen),
        NamedColor::Yellow => fg(AnsiColor::Yellow),
        NamedColor::BrightYellow => fg(AnsiColor::BrightYellow),
        NamedColor::Blue => fg(AnsiColor::Blue),
        NamedColor::BrightBlue => fg(AnsiColor::BrightBlue),
        NamedColor::Magenta => fg(AnsiColor::Magenta),
        NamedColor::BrightMagenta => fg(AnsiColor::BrightMagenta),
        NamedColor::Cyan => fg(AnsiColor::Cyan),
        NamedColor::BrightCyan => fg(AnsiColor::BrightCyan),
        NamedColor::Bold => Style::new().effects(Effects::BOLD),
    }
}

fn colorize_state(settings: &Settings, state: &str) -> String {
    let color = settings
        .list_formatting
        .colors
        .state
        .get(state)
        .cloned()
        .unwrap_or(NamedColor::White);

    apply_style(state, named_color_to_style(color))
}

fn colorize_priority(settings: &Settings, priority: &str) -> String {
    let color = settings
        .list_formatting
        .colors
        .priority
        .get(priority)
        .cloned()
        .unwrap_or(NamedColor::White);

    apply_style(priority, named_color_to_style(color))
}

fn colorize_type(settings: &Settings, type_: &str) -> String {
    let color = settings
        .list_formatting
        .colors
        .type_
        .get(type_)
        .cloned()
        .unwrap_or(NamedColor::White);

    apply_style(type_, named_color_to_style(color))
}

fn colorize_value(settings: &Settings, col: &str, value: &str) -> String {
    match col {
        "state" => colorize_state(settings, value),
        "priority" => colorize_priority(settings, value),
        "type" => colorize_type(settings, value),
        "assignee" | "reporter" => colorize_me(settings, value),
        "due_date" => colorize_due_date(settings, value),
        _ => value.to_string(),
    }
}

fn colorize_header(settings: &Settings, header: &str) -> String {
    let color = settings.list_formatting.colors.header;

    apply_style(header, named_color_to_style(color))
}

fn colorize_me(settings: &Settings, user: &str) -> String {
    let me = settings.user.clone();

    if user != me {
        return user.to_string();
    }

    let color = settings.list_formatting.colors.me;

    apply_style(user, named_color_to_style(color))
}

fn colorize_due_date(settings: &Settings, due_date: &str) -> String {
    let today = Utc::now().naive_utc().date();

    match chrono::NaiveDate::parse_from_str(due_date, "%Y-%m-%d") {
        Ok(due_date_date) => {
            if due_date_date >= today {
                return due_date.to_string();
            }

            let color = settings.list_formatting.colors.due_date_overdue;
            apply_style(due_date, named_color_to_style(color))
        }
        Err(_) => due_date.to_string(),
    }
}

fn print_header_separator(settings: &Settings, cols: &[String], column_widths: &HashMap<String, usize>) {
    let header_separator = settings.list_formatting.header_separator;

    if !header_separator {
        return;
    }

    // Print separator line
    for col in cols {
        let width = *column_widths.get(col).unwrap_or(&22);
        print!("{}", "-".repeat(width));
    }
    println!();
}

fn cache_issue_ids(issue_ids: &[u32]) -> Result<(), String> {
    let cache_content = issue_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>().join(",");
    let cache_file = cache_path()?;

    if let Some(parent) = cache_file.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create cache directory: {e}"))?;
    }

    fs::write(&cache_file, cache_content).map_err(|e| format!("Failed to write cache file {}: {e}", cache_file.display()))?;

    Ok(())
}
