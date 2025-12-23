use clap::{Parser, Subcommand};

mod edit;
mod init;
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
    },

    /// List all issues
    List {
        #[arg(long, value_delimiter = ',')]
        columns: Option<Vec<String>>,
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

        /// Issue meta field: state
        #[arg(long)]
        state: Option<String>,

        /// Issue meta field: title
        #[arg(long)]
        title: Option<String>,

        /// Issue meta field: type
        #[arg(long)]
        type_: Option<String>,

        /// Issue meta field: assignee
        #[arg(long)]
        assignee: Option<String>,

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
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init { no_commit } => {
            if let Err(e) = init::run(no_commit) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::New { title } => {
            if let Err(e) = new::run(title) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::List { columns } => {
            if let Err(e) = list::run(columns) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Show { id } => {
            if let Err(e) = show::run(id) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Set {
            id,
            state,
            title,
            type_,
            assignee,
            labels,
            labels_add,
            labels_remove,
        } => {
            if let Err(e) = set::run(
                id,
                state,
                title,
                type_,
                assignee,
                labels,
                labels_add,
                labels_remove,
            ) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }

        Commands::Edit { id } => {
            if let Err(e) = edit::run(id) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    }
}
