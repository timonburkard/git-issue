#![deny(warnings, clippy::unwrap_used, clippy::expect_used)]
use std::fs;
use std::io::ErrorKind;

use clap::{Parser, Subcommand};

use git_issue::cmd::model::{Filter, Priority, RelationshipLink, Sorting, cache_path};

mod cli;

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

        /// Issue meta field: reporter [possible values: see users.yaml:users:id, 'me' or '']
        #[arg(long)]
        reporter: Option<String>,

        /// Issue meta field: assignee [possible values: see users.yaml:users:id, 'me' or '']
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

        /// Filter issues by meta fields [field{=|>|<}value]
        #[arg(long, num_args = 1..)]
        filter: Option<Vec<Filter>>,

        /// Sort issues by meta fields [field=asc|desc]
        #[arg(long, num_args = 1..)]
        sort: Option<Vec<Sorting>>,

        /// Print output to CSV file
        #[arg(long, default_value_t = false)]
        csv: bool,

        /// Don't color the output
        #[arg(long, default_value_t = false)]
        no_color: bool,
    },

    /// Show issue details
    Show {
        /// Issue ID
        id: u32,
    },

    /// Change issue meta fields
    Set {
        /// Issue IDs [single ID, comma-separated IDs, or '*' to bulk update all issues from latest `list` command]
        #[arg(value_delimiter = ',', num_args = 1..)]
        ids: Vec<String>,

        /// Issue meta field: title
        #[arg(long)]
        title: Option<String>,

        /// Issue meta field: state [possible values: see config.yaml:states]
        #[arg(long)]
        state: Option<String>,

        /// Issue meta field: type [possible values: see config.yaml:types or '']
        #[arg(long)]
        type_: Option<String>,

        /// Issue meta field: reporter [possible values: see users.yaml:users:id, 'me' or '']
        #[arg(long)]
        reporter: Option<String>,

        /// Issue meta field: assignee [possible values: see users.yaml:users:id, 'me' or '']
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

    // Clear cache
    match &args.command {
        Commands::List { .. } | Commands::Set { .. } => { /* keep cache for list/set */ }
        _ => match cache_path() {
            Ok(cache_file) => {
                if let Err(e) = fs::remove_file(&cache_file)
                    && e.kind() != ErrorKind::NotFound
                {
                    eprintln!("Error: failed to clear cache {}: {e}", cache_file.display());
                }
            }
            Err(_) => { /* cache does not exist, so we don't need to clear it */ }
        },
    }

    let result = match args.command {
        Commands::Init { no_commit } => cli::init(no_commit),

        Commands::New {
            title,
            type_,
            reporter,
            assignee,
            priority,
            due_date,
            labels,
        } => cli::new(title, type_, reporter, assignee, priority, due_date, labels),

        Commands::List {
            columns,
            filter,
            sort,
            csv,
            no_color,
        } => cli::list(columns, filter, sort, csv, no_color),

        Commands::Show { id } => cli::show(id),

        Commands::Set {
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
        } => cli::set(
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
        ),

        Commands::Edit { id } => cli::edit(id),

        Commands::Link { id, add, remove } => cli::link(id, add, remove),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
