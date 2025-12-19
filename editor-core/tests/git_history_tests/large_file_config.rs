use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_large_file_config_default() {
    let config = LargeFileConfig::default();

    assert_eq!(config.threshold_mb, 50);
    assert_eq!(config.strategy, LargeFileStrategy::Warn);
    assert!(!config.exclude_from_history);
}

#[test]
fn test_large_file_config_custom() {
    let config = LargeFileConfig {
        threshold_mb: 100,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: true,
    };

    assert_eq!(config.threshold_mb, 100);
    assert_eq!(config.strategy, LargeFileStrategy::Skip);
    assert!(config.exclude_from_history);
}

#[test]
fn test_large_file_strategy_variants() {
    let warn = LargeFileStrategy::Warn;
    let skip = LargeFileStrategy::Skip;
    let error = LargeFileStrategy::Error;
    let lfs = LargeFileStrategy::Lfs;

    assert_eq!(warn, LargeFileStrategy::Warn);
    assert_eq!(skip, LargeFileStrategy::Skip);
    assert_eq!(error, LargeFileStrategy::Error);
    assert_eq!(lfs, LargeFileStrategy::Lfs);

    assert_ne!(warn, skip);
    assert_ne!(warn, error);
    assert_ne!(warn, lfs);
}

#[test]
fn test_git_history_manager_with_large_file_config() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let config = LargeFileConfig {
        threshold_mb: 75,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: true,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config.clone());

    assert_eq!(manager.large_file_config().threshold_mb, 75);
    assert_eq!(
        manager.large_file_config().strategy,
        LargeFileStrategy::Error
    );
    assert!(manager.large_file_config().exclude_from_history);
}

#[test]
fn test_git_history_manager_default_large_file_config() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    assert_eq!(manager.large_file_config().threshold_mb, 50);
    assert_eq!(
        manager.large_file_config().strategy,
        LargeFileStrategy::Warn
    );
    assert!(!manager.large_file_config().exclude_from_history);
}

#[test]
fn test_large_file_config_clone() {
    let config1 = LargeFileConfig {
        threshold_mb: 200,
        strategy: LargeFileStrategy::Lfs,
        exclude_from_history: false,
    };

    let config2 = config1.clone();

    assert_eq!(config1, config2);
    assert_eq!(config2.threshold_mb, 200);
    assert_eq!(config2.strategy, LargeFileStrategy::Lfs);
    assert!(!config2.exclude_from_history);
}

#[test]
fn test_large_file_config_extreme_threshold_values() {
    let config_zero = LargeFileConfig {
        threshold_mb: 0,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    assert_eq!(config_zero.threshold_mb, 0);

    let config_max = LargeFileConfig {
        threshold_mb: u64::MAX,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    assert_eq!(config_max.threshold_mb, u64::MAX);
}

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
