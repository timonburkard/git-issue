use crate::model::Priority;
use crate::new;
use crate::set;

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
    let (num_updated_issues, infos) = set::set(
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

    if let Some(infos) = infos {
        for info in infos {
            println!("{}", info);
        }
    }

    match num_updated_issues {
        0 => return Err("No fields changed".to_string()),
        1 => println!("Updated issue field(s)"),
        _ => println!("Updated {} issues' field(s)", num_updated_issues),
    };

    Ok(())
}
