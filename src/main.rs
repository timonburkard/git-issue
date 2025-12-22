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
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize .gitissues in the current repository
    Init,

    /// Create a new issue
    New {
        /// Issue title
        title: String,
    },

    /// List all issues
    List {
        #[arg(long, value_delimiter = ',')]
        column: Option<Vec<String>>,
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
        #[arg(long, value_delimiter = ',')]
        labels: Option<Vec<String>>,
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
        Commands::Init => {
            if let Err(e) = init::run() {
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

        Commands::List { column } => {
            if let Err(e) = list::run(column) {
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
        } => {
            if let Err(e) = set::run(id, state, title, type_, assignee, labels) {
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
