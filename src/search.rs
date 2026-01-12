use std::fs;

use crate::model::{Settings, apply_style, issues_dir, load_settings, named_color_to_style};

const HEADER_ID: &str = "id";
const HEADER_VALUE: &str = "search result";

pub fn run(text: String) -> Result<(), String> {
    let settings = load_settings()?;
    let issues_dir = issues_dir()?;

    let mut search_results: Vec<(String, String)> = Vec::new();

    for issue_dir in fs::read_dir(issues_dir).map_err(|e| format!("Failed to read issues directory: {e}"))? {
        let issue_dir = match issue_dir {
            Ok(value) => value,
            Err(_) => continue,
        };

        let issue_id = match issue_dir.file_name().to_string_lossy().parse::<u32>() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let issue_desc_path = issue_dir.path().join("description.md");

        let issue_desc = match fs::read_to_string(&issue_desc_path) {
            Ok(value) => value,
            Err(_) => continue,
        };

        for line in issue_desc.lines() {
            if line.contains(&text) {
                search_results.push((issue_id.to_string(), line.to_string()));
            }
        }
    }

    let (id_width, value_width) = calculate_column_widths(&search_results);

    // Print header
    let header_id_colored = colorize_header(&settings, HEADER_ID);
    let header_value_colored = colorize_header(&settings, HEADER_VALUE);

    let id_padding = id_width.saturating_sub(HEADER_ID.len());
    let value_padding = value_width.saturating_sub(HEADER_VALUE.len());

    println!(
        "{header_id_colored}{}{header_value_colored}{}",
        " ".repeat(id_padding),
        " ".repeat(value_padding)
    );

    if settings.search_formatting.header_separator {
        println!("{}", "-".repeat(id_width + value_width));
    }

    for (id, value) in search_results {
        let line_highlighted = value.replace(
            &text,
            &apply_style(&text, named_color_to_style(settings.search_formatting.colors.results)),
        );
        let id_padding = id_width.saturating_sub(id.len());
        let value_padding = value_width.saturating_sub(value.len());
        println!("{id}{}{line_highlighted}{}", " ".repeat(id_padding), " ".repeat(value_padding));
    }

    Ok(())
}

fn calculate_column_widths(values: &Vec<(String, String)>) -> (usize, usize) {
    let mut id_width = HEADER_ID.len();
    let mut value_width = HEADER_VALUE.len();

    // Update with max content widths
    for (id, value) in values {
        if id.len() > id_width {
            id_width = id.len();
        }

        if value.len() > value_width {
            value_width = value.len();
        }
    }

    // Add padding (2 spaces)
    id_width += 2;
    value_width += 2;

    (id_width, value_width)
}

fn colorize_header(settings: &Settings, header: &str) -> String {
    let color = settings.search_formatting.colors.header;

    apply_style(header, named_color_to_style(color))
}
