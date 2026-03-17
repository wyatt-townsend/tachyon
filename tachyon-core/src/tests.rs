mod index_tests {
    use crate::index::builder::{build_index, load_index, walk_directory};
    use crate::index::entry::EntryKind;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

    /// Tests for the `walk_directory`

    #[test]
    fn test_walk_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = walk_directory(temp_dir.path()).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_walk_single_file() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

        let result = walk_directory(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "file.txt");
        assert_eq!(result[0].kind, EntryKind::File);
    }

    #[test]
    fn test_walk_nested_directories() {
        let temp_dir = TempDir::new().unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("subdir/nested.txt"), "data").unwrap();

        let result = walk_directory(temp_dir.path()).unwrap();
        assert_eq!(result.len(), 2);
        assert!(
            result
                .iter()
                .any(|e| e.kind == EntryKind::Directory && e.name == "subdir")
        );
        assert!(
            result
                .iter()
                .any(|e| e.kind == EntryKind::File && e.name == "nested.txt")
        );
    }

    #[test]
    fn test_walk_nonexistent_path() {
        let result = walk_directory(Path::new("/nonexistent/path"));
        assert!(result.is_err());
    }

    /// Tests for the `build_index` and `load_index` functions

    #[test]
    fn test_build_index_creates_file() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

        let index_path = temp_dir.path().join("index.bin");
        let result = build_index(temp_dir.path(), &index_path);

        assert!(result.is_ok());
        assert!(index_path.exists());
    }

    #[test]
    fn test_build_index_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let index_path = temp_dir.path().join("index.bin");

        let result = build_index(temp_dir.path(), &index_path);
        assert!(result.is_ok());
        assert!(index_path.exists());
    }

    #[test]
    fn test_load_index_nonexistent_file() {
        let result = load_index(Path::new("/nonexistent/index.bin"));
        assert!(result.is_err());
    }

    #[test]
    fn test_build_and_load_index_roundtrip() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file1.txt"), "content1").unwrap();
        fs::create_dir(temp_dir.path().join("subdir")).unwrap();
        fs::write(temp_dir.path().join("subdir/file2.txt"), "content2").unwrap();

        let index_path = temp_dir.path().join("index.bin");
        build_index(temp_dir.path(), &index_path).unwrap();

        let loaded = load_index(&index_path).unwrap();

        // Verify we got the same entries
        assert_eq!(loaded.len(), 3);
        assert!(
            loaded
                .iter()
                .any(|e| e.name == "file1.txt" && e.kind == EntryKind::File)
        );
        assert!(
            loaded
                .iter()
                .any(|e| e.name == "subdir" && e.kind == EntryKind::Directory)
        );
        assert!(
            loaded
                .iter()
                .any(|e| e.name == "file2.txt" && e.kind == EntryKind::File)
        );
    }

    #[test]
    fn test_build_index_to_invalid_path() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("file.txt"), "content").unwrap();

        // Try to write to a path with non-existent parent directory
        let invalid_path = temp_dir.path().join("nonexistent/parent/index.bin");
        let result = build_index(temp_dir.path(), &invalid_path);

        assert!(result.is_err());
    }

    #[test]
    fn test_load_index_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let index_path = temp_dir.path().join("index.bin");

        build_index(temp_dir.path(), &index_path).unwrap();
        let loaded = load_index(&index_path).unwrap();

        assert!(loaded.is_empty());
    }
}
