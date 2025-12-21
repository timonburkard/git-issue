use clap::{Parser, Subcommand};

mod init;
mod list;
mod new;
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
    List,

    /// Show issue details
    Show {
        /// Issue ID
        id: u32,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            if let Err(e) = init::run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::New { title } => {
            if let Err(e) = new::run(title) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::List => {
            if let Err(e) = list::run() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::Show { id } => {
            if let Err(e) = show::run(id) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
