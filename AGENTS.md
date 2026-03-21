# AGENTS.md

## Project Overview

**Tachyon** is a Windows file searcher written in Rust. It's a workspace project with two crates:

- `tachyon-core` — Library crate for indexing and file system operations
- `tachyon-cli` — Command-line interface

## Build Commands

### Standard Cargo Operations

```bash
# Build the entire workspace
cargo build

# Build a specific crate
cargo build -p tachyon-core
cargo build -p tachyon-cli

# Build in release mode
cargo build --release

# Type-check without building
cargo check
cargo check -p tachyon-core
```

### Testing

```bash
# Run all tests in the workspace
cargo test

# Run tests for a specific crate
cargo test -p tachyon-core
cargo test -p tachyon-cli
```

### Linting and Formatting

```bash
# Format all code (run before committing)
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check

# Run clippy lints
cargo clippy
cargo clippy -- -D warnings  # Treat warnings as errors

# Run clippy with specific checks
cargo clippy --all-targets --all-features
```

### Documentation

```bash
# Build and view documentation
cargo doc
cargo doc --open

# Check documentation builds
cargo doc --no-deps
```

### Running the CLI

```bash
# Run the CLI (builds automatically)
cargo run -- build --drive C:\

# Run with specific arguments
cargo run -- -h
```

## Code Style Guidelines

### Formatting

- Run `cargo fmt` before every commit
- Use the default Rust formatting style (rustfmt defaults)
- 4-space indentation (rustfmt default)
- No trailing whitespace
- Maximum line length: 100 characters (rustfmt default)

### Error Handling

- **Library code (`tachyon-core`)**: Use `thiserror` to define custom error enums
  - Use `#[from]` for automatic error conversion
  - Include context in error messages (paths, underlying errors)
  - Example pattern from `error.rs`:
    ```rust
    #[derive(Debug, Error)]
    pub enum TachyonError {
        #[error("I/O error at '{path}': {source}")]
        Io {
            path: std::path::PathBuf,
            #[source]
            source: std::io::Error,
        },
        #[error("Serialization error: {0}")]
        Serialization(#[from] bincode::error::EncodeError),
    }
    ```

- **CLI code (`tachyon-cli`)**: Use `anyhow::Result<()>` for main functions
  - Use `anyhow::anyhow!()` for quick error creation
  - Return `Ok(())` on success

### Module Organization

- Use `pub mod` and `pub use` for intentional public APIs
- Keep internal details private (`mod`)
- Group related functionality in submodules
- Structure:
  ```
  src/
    lib.rs          # Public API and module declarations
    error.rs        # Error types (for library crates)
    index.rs        # Module re-exports
    index/
      builder.rs    # Implementation
      entry.rs      # Types
    tests.rs        # Integration tests
  ```

### Naming Conventions

- **Modules**: `snake_case` (e.g., `index_builder`, not `indexBuilder`)
- **Types/Structs/Enums**: `PascalCase` (e.g., `FileEntry`, `EntryKind`)
- **Functions/Methods**: `snake_case` (e.g., `build_index`, `load_index`)
- **Variables**: `snake_case` (e.g., `temp_dir`, `index_path`)
- **Constants**: `SCREAMING_SNAKE_CASE` for true constants
- **Crate naming**: `kebab-case` in Cargo.toml

### Documentation

- Add module-level doc comments (`//!`) for public modules
- Document public functions with `///` comments
- Include usage examples for complex functions
- Document error cases when non-obvious

### Imports

- Use absolute paths for external crates: `use crate::...` for internal
- Group imports logically (extern crates, then std, then local)
- Remove unused imports (clippy will catch this)
- Use `use` statements for frequently used items

### Testing

- Unit tests go in `#[cfg(test)]` modules within each source file
- Integration tests in `tests.rs` or `tests/` directory
- Use descriptive test names: `test_<function>_<scenario>`
- Use `tempfile::TempDir` for file system tests
- Test both success and error paths
- Use `assert!`, `assert_eq!`, `assert_ne!` appropriately

### Dependencies

- **Core types**: `bincode` (serialization), `thiserror` (errors)
- **CLI**: `clap` (CLI parsing), `anyhow` (error handling), `dirs` (paths)
- **Testing**: `tempfile` (temporary directories)
- Avoid adding dependencies without justification
- Specify minimal version requirements

### Code Patterns

- Use `Result` for fallible operations
- Use `map_err` for error context conversion
- Prefer `?` operator over `match` for simple error propagation
- Use `unwrapped()` sparingly in tests (acceptable for setup)
- Handle errors gracefully — don't panic on recoverable errors
- Use buffered I/O (`BufReader`, `BufWriter`) for file operations

### Performance Considerations

- Use `Vec::push` and `Vec::append` for collection building
- Consider capacity hints for large collections
- Use `walk_directory` pattern for recursive operations
- Buffer file I/O for serialization operations

## Architecture Notes

### Indexing Flow

1. `build_index(root, dest)` — Recursively walks directory tree
2. `walk_directory(root)` — Collects `FileEntry` structs
3. `load_index(src)` — Deserializes index file

### Data Model

- `FileEntry`: Contains `name` (String), `path` (PathBuf), `kind` (EntryKind)
- `EntryKind`: Enum with `File` and `Directory` variants
- Binary serialization via `bincode`

### Windows-Specific

- Default drive: `C:\`
- Paths use backslashes (Windows)
- Uses `dirs` crate for local app data paths
