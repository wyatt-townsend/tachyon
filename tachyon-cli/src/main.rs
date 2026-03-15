use clap::Parser;

/// Simple Windows file searcher written in Rust
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// The directory to search
    path: std::path::PathBuf,

    /// Enable case-insensitive matching
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    print!(
        "Searching for '{}' in '{}'",
        args.pattern,
        args.path.display()
    );
}
