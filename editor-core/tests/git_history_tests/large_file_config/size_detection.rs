use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_check_file_size_small_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("small.txt");

    fs::write(&file_path, "Hello, world!").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 13);
    assert!(!size_info.exceeds_threshold);
}

#[test]
fn test_check_file_size_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("large.txt");

    let large_content = vec![b'A'; 60 * 1024 * 1024];
    fs::write(&file_path, large_content).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 60 * 1024 * 1024);
    assert!(size_info.exceeds_threshold);
}

#[test]
fn test_check_file_size_exactly_at_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("exact.txt");

    let exact_content = vec![b'A'; 50 * 1024 * 1024];
    fs::write(&file_path, exact_content).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 50 * 1024 * 1024);
    assert!(!size_info.exceeds_threshold);
}

#[test]
fn test_check_file_size_one_byte_over_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("one_over.txt");

    let content = vec![b'A'; 50 * 1024 * 1024 + 1];
    fs::write(&file_path, content).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 50 * 1024 * 1024 + 1);
    assert!(size_info.exceeds_threshold);
}

#[test]
fn test_check_file_size_custom_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("custom.txt");

    let content = vec![b'A'; 15 * 1024 * 1024];
    fs::write(&file_path, content).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 15 * 1024 * 1024);
    assert!(size_info.exceeds_threshold);
}

#[test]
fn test_is_large_file_predicate() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let small_file = temp_dir.path().join("small.txt");
    fs::write(&small_file, "small").unwrap();

    let large_file = temp_dir.path().join("large.txt");
    let large_content = vec![b'A'; 60 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    assert!(!manager.is_large_file(&small_file).unwrap());
    assert!(manager.is_large_file(&large_file).unwrap());
}

#[test]
fn test_check_file_size_nonexistent_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("nonexistent.txt");

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.check_file_size(&file_path);
    assert!(result.is_err());
}

#[test]
fn test_check_file_size_empty_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("empty.txt");

    fs::write(&file_path, "").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 0);
    assert!(!size_info.exceeds_threshold);
}

#[test]
fn test_check_file_size_with_zero_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("any.txt");

    fs::write(&file_path, "x").unwrap();

    let config = LargeFileConfig {
        threshold_mb: 0,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let size_info = manager.check_file_size(&file_path).unwrap();
    assert_eq!(size_info.size_bytes, 1);
    assert!(size_info.exceeds_threshold);
}
