use clap::{Parser, Subcommand};

mod init;

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

        Commands::New { title } => println!("Running: new with title '{}'", title),
    }
}
