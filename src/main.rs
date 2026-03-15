use clap::Parser;

/// Simple Windows file searcher written in Rust
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// The pattern to search for
    pattern: String,

    /// The directory to search
    path: String,

    /// Enable case-insensitive matching
    #[arg(short, long)]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    // Initialize and verify entry path
    let path = std::path::Path::new(&args.path);
    if !path.exists() {
        eprintln!("Error: path '{}' does not exist", args.path);
        std::process::exit(1);
    }

    // Ignore case if selected
    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern
    };

    // Search
    if path.is_dir() {
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                let name = e.file_name().to_string_lossy();
                if args.ignore_case {
                    name.to_lowercase().contains(&pattern)
                } else {
                    name.contains(&pattern)
                }
            })
        {
            println!("{}", entry.path().to_string_lossy());
        }
    }
}
