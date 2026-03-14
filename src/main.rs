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

    let path = std::path::Path::new(&args.path);

    let pattern = if args.ignore_case {
        args.pattern.to_lowercase()
    } else {
        args.pattern.clone()
    };

    if path.is_dir() {
        for entry in walkdir::WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|e| {
                let name = e.file_name().to_string_lossy();
                let name = if args.ignore_case {
                    name.to_lowercase()
                }
                else {
                    name.into_owned()
                };
                name.contains(&pattern)
            })
        { 
            println!("{}", entry.path().to_string_lossy());
        }
    }
}