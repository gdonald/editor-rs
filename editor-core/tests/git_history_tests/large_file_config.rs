use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
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
