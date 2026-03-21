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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.cmd {
        Commands::Build { drive } => {
            commands::build(&drive)?;
        }
    }

    Ok(())
}
