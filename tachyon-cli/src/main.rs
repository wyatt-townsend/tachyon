use clap::{Parser, Subcommand};
use std::path::PathBuf;
mod commands;

/// Simple Windows file searcher written in Rust
#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Build the drive file index
    Build {
        /// Optionally specify the drive to index
        #[arg(short, long, default_value = "C:\\")]
        drive: PathBuf,
    },
    Search {
        /// The substring pattern to search for  
        pattern: String,
        /// Perform a case-insensitive search
        #[arg(short, long)]
        case_insensitive: bool,
        /// Use glob pattern to filter results (e.g. *.txt)
        #[arg(short, long)]
        glob: bool,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Build { drive } => {
            commands::build(&drive)?;
        }
        Commands::Search {
            pattern,
            case_insensitive,
            glob,
        } => {
            commands::search(&pattern, case_insensitive, glob)?;
        }
    }

    Ok(())
}
