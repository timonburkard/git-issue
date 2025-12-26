#![deny(warnings, clippy::unwrap_used, clippy::expect_used)]
use clap::{Parser, Subcommand};

use crate::model::{Filter, Priority, RelationshipLink, Sorting};

mod edit;
mod init;
mod link;
mod list;
mod model;
mod new;
mod set;
mod show;

#[derive(Parser)]
#[command(name = "git-issue")]
#[command(about = "Git-native issue tracker", long_about = None)]
#[command(version = concat!("v", env!("CARGO_PKG_VERSION")))]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize .gitissues in the current repository
    Init {
        /// Don't create an initial git commit
        #[arg(long, default_value_t = false)]
        no_commit: bool,
    },

    /// Create a new issue
    New {
        /// Issue title
        title: String,

        /// Issue meta field: type [possible values: see config.yaml:types or '']
        #[arg(long)]
        type_: Option<String>,

        /// Issue meta field: assignee [possible values: see users.yaml:users:id or '']
        #[arg(long)]
        assignee: Option<String>,

        /// Issue meta field: priority
        #[arg(long)]
        priority: Option<Priority>,

        /// Issue meta field: due_date [possible values: YYYY-MM-DD or '']
        #[arg(long, alias = "due_date")]
        due_date: Option<String>,

        /// Issue meta field: labels
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
    },

    /// List all issues
    List {
        /// Columns to display
        #[arg(long, value_delimiter = ',')]
        columns: Option<Vec<String>>,

        /// Filter issues by meta fields [<field>=<value>]
        #[arg(long, num_args = 1..)]
        filter: Option<Vec<Filter>>,

        /// Sort issues by meta fields  [<field>=asc|desc]
        #[arg(long, num_args = 1..)]
        sort: Option<Vec<Sorting>>,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: u32,
    },

    /// Change issue meta fields
    Set {
        /// Issue ID
        id: u32,

        /// Issue meta field: title
        #[arg(long)]
        title: Option<String>,

        /// Issue meta field: state [possible values: see config.yaml:states]
        #[arg(long)]
        state: Option<String>,

        /// Issue meta field: type [possible values: see config.yaml:types or '']
        #[arg(long)]
        type_: Option<String>,

        /// Issue meta field: assignee [possible values: see users.yaml:users:id or '']
        #[arg(long)]
        assignee: Option<String>,

        /// Issue meta field: priority
        #[arg(long)]
        priority: Option<Priority>,

        /// Issue meta field: due_date [possible values: YYYY-MM-DD or '']
        #[arg(long, alias = "due_date")]
        due_date: Option<String>,

        /// Issue meta field: labels
        #[arg(long, value_delimiter = ',', conflicts_with_all = ["labels_add", "labels_remove"])]
        labels: Option<Vec<String>>,

        /// Issue meta field: labels-add
        #[arg(long, value_delimiter = ',', conflicts_with_all = ["labels"])]
        labels_add: Option<Vec<String>>,

        /// Issue meta field: labels-remove
        #[arg(long, value_delimiter = ',', conflicts_with_all = ["labels"])]
        labels_remove: Option<Vec<String>>,
    },

    /// Edit issue description (markdown)
    Edit {
        /// Issue ID
        id: u32,
    },

    /// Link issue to other issues via relationships
    Link {
        /// Issue ID
        id: u32,

        /// Relationship link
        #[arg(long, num_args = 1.., required_unless_present = "remove")]
        add: Option<Vec<RelationshipLink>>,

        /// Relationship link
        #[arg(long, num_args = 1.., required_unless_present = "add")]
        remove: Option<Vec<RelationshipLink>>,
    },
}

fn main() {
    let args = Args::parse();

    let result = match args.command {
        Commands::Init { no_commit } => init::run(no_commit),

        Commands::New {
            title,
            type_,
            assignee,
            priority,
            due_date,
            labels,
        } => new::run(title, type_, assignee, priority, due_date, labels),

        Commands::List { columns, filter, sort } => list::run(columns, filter, sort),

        Commands::Show { id } => show::run(id),

        Commands::Set {
            id,
            state,
            title,
            type_,
            assignee,
            priority,
            due_date,
            labels,
            labels_add,
            labels_remove,
        } => set::run(
            id,
            state,
            title,
            type_,
            assignee,
            priority,
            due_date,
            labels,
            labels_add,
            labels_remove,
        ),

        Commands::Edit { id } => edit::run(id),

        Commands::Link { id, add, remove } => link::run(id, add, remove),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
