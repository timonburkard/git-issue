use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

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

fn init() -> Result<(), String> {
    let root = ".gitissues";
    let issues_dir = Path::new(".gitissues").join("issues");

    if Path::new(root).exists() {
        return Err("Already initialized: .gitissues already exists".to_string());
    }

    // Create the directory structure
    fs::create_dir_all(&issues_dir)
        .map_err(|e| format!("Failed to create {}: {}", issues_dir.display(), e))?;

    println!("Initialization done");
    Ok(())
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            if let Err(e) = init() {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }

        Commands::New { title } => println!("Running: new with title '{}'", title),
    }
}
