use anyhow;
use dirs;
use indicatif::ProgressBar;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tachyon_core::{build_index, index::Index, query::QueryType, query_index};

// Helpers
fn get_index_path() -> Result<PathBuf, anyhow::Error> {
    // Find the local data dir
    let local_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get local app data directory"))?;

    // Create index file
    Ok(local_dir.join("tachyon").join("index.bin"))
}

fn get_progress_bar() -> ProgressBar {
    ProgressBar::new_spinner().with_style(
        indicatif::ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    )
}

// Main command functions
pub(crate) fn build(drive: &Path) -> Result<(), anyhow::Error> {
    // Verify the path exists and is a directory
    if !drive.exists() {
        return Err(anyhow::anyhow!(
            "Drive path does not exist: {}",
            drive.display()
        ));
    }

    if !drive.is_dir() {
        return Err(anyhow::anyhow!(
            "Path is not a directory: {}",
            drive.display()
        ));
    }

    // Get the index path
    let index_path = get_index_path()?;

    // Create a spinner to show activity
    let spinner = get_progress_bar();
    spinner.set_message(format!("Indexing {}...", drive.display()));
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Build and save index
    let index = build_index(drive)?;

    spinner.finish_with_message(format!("Index built with {} entries", index.entries.len()));

    // Save index
    index.save(&index_path)?;

    println!("✓ Index saved to {}", index_path.display());
    Ok(())
}

pub(crate) fn search(
    pattern: &String,
    case_insensitive: bool,
    glob: bool,
) -> Result<(), anyhow::Error> {
    let query_type = if glob {
        QueryType::Glob {
            pattern: pattern.clone(),
            case_insensitive,
        }
    } else {
        QueryType::Substring {
            pattern: pattern.clone(),
            case_insensitive,
        }
    };

    // Create a spinner to show activity
    let spinner = get_progress_bar();
    spinner.enable_steady_tick(Duration::from_millis(100));

    // Load index
    spinner.set_message(format!("Loading index..."));
    let index = Index::load(&get_index_path()?)?;

    // Query index
    spinner.set_message(format!("Searching index..."));
    let results = query_index(&index, query_type)?;

    // Print results
    for result in &results {
        println!("{}", result.display());
    }

    spinner.finish_with_message(format!("✓ Search completed with {} results", results.len()));

    Ok(())
}
