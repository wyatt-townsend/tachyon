use anyhow;
use dirs;
use std::path::Path;
use tachyon_core::index::builder::build_index;

pub(crate) fn run(drive: &Path) -> Result<(), anyhow::Error> {
    // Find the local data dir
    let local_dir = dirs::data_local_dir()
        .ok_or_else(|| anyhow::anyhow!("Failed to get local app data directory"))?;

    // Get or create index directory
    let dest = local_dir.join("tachyon");
    std::fs::create_dir_all(&dest)?;

    // Build the index
    let index = dest.join("index.bin");
    build_index(drive, &index)?;

    print!("Index saved to {}", dest.display());
    Ok(())
}
