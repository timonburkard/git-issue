use crate::model::Priority;
use crate::new;

pub fn new(
    title: String,
    type_: Option<String>,
    reporter: Option<String>,
    assignee: Option<String>,
    priority: Option<Priority>,
    due_date: Option<String>,
    labels: Option<Vec<String>>,
) -> Result<(), String> {
    let (issue_id, info) = new::new(title, type_, reporter, assignee, priority, due_date, labels)?;

    if let Some(info) = info {
        println!("{}", info);
    }

    println!("Created issue #{issue_id}");

    Ok(())
}
