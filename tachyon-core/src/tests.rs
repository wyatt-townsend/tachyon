mod index_tests {
    use crate::index::builder::walk_directory;
    use crate::index::entry::EntryKind;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;

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
}
