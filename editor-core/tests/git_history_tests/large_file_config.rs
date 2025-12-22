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

#[test]
fn test_auto_commit_with_warn_strategy_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'X'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &large_file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("Auto-save"));
}

#[test]
fn test_auto_commit_with_warn_strategy_small_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small content").unwrap();

    let result = manager.auto_commit_on_save(&project_path, &small_file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("Auto-save"));
}

#[test]
fn test_auto_commit_multiple_files_with_warn_strategy() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small").unwrap();

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let files = vec![&small_file, &large_file];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("2 files"));
}

#[test]
fn test_warn_strategy_does_not_block_commit() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let very_large_file = project_path.join("very_large.txt");
    let very_large_content = vec![b'Z'; 100 * 1024 * 1024];
    fs::write(&very_large_file, very_large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &very_large_file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_warn_strategy_with_exactly_threshold_size() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 5,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let exact_file = project_path.join("exact.txt");
    let exact_content = vec![b'E'; 5 * 1024 * 1024];
    fs::write(&exact_file, exact_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &exact_file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_skip_strategy_excludes_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &large_file);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 1);
    assert_eq!(
        commit_result.skipped_files[0].to_str().unwrap(),
        "large.txt"
    );

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_skip_strategy_commits_small_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small content").unwrap();

    let result = manager.auto_commit_on_save(&project_path, &small_file);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 0);

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("Auto-save"));
}

#[test]
fn test_skip_strategy_mixed_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small").unwrap();

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let files = vec![&small_file, &large_file];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 1);
    assert_eq!(
        commit_result.skipped_files[0].to_str().unwrap(),
        "large.txt"
    );

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("small.txt"));
    assert!(commits[0].message.contains("1 large file excluded"));
}

#[test]
fn test_skip_strategy_all_files_large() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file1 = project_path.join("large1.txt");
    let large_content1 = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&large_file1, large_content1).unwrap();

    let large_file2 = project_path.join("large2.txt");
    let large_content2 = vec![b'B'; 3 * 1024 * 1024];
    fs::write(&large_file2, large_content2).unwrap();

    let files = vec![&large_file1, &large_file2];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 2);

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_skip_strategy_commit_message_notation() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small1 = project_path.join("small1.txt");
    fs::write(&small1, "small1").unwrap();

    let small2 = project_path.join("small2.txt");
    fs::write(&small2, "small2").unwrap();

    let large1 = project_path.join("large1.txt");
    let large_content1 = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large1, large_content1).unwrap();

    let large2 = project_path.join("large2.txt");
    let large_content2 = vec![b'M'; 3 * 1024 * 1024];
    fs::write(&large2, large_content2).unwrap();

    let files = vec![&small1, &small2, &large1, &large2];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 2);

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("2 files"));
    assert!(commits[0].message.contains("2 large files excluded"));
    assert!(commits[0].message.contains("large1.txt"));
    assert!(commits[0].message.contains("large2.txt"));
}

#[test]
fn test_skip_strategy_singular_plural_message() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small").unwrap();

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let files = vec![&small_file, &large_file];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("1 large file excluded"));
    assert!(!commits[0].message.contains("1 large files excluded"));
}

#[test]
fn test_error_strategy_blocks_large_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &large_file);

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        editor_core::EditorError::FileTooLarge { path, size, limit } => {
            assert!(path.contains("large.txt"));
            assert_eq!(size, 2 * 1024 * 1024);
            assert_eq!(limit, 1024 * 1024);
        }
        _ => panic!("Expected FileTooLarge error, got: {:?}", err),
    }

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_error_strategy_allows_small_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small content").unwrap();

    let result = manager.auto_commit_on_save(&project_path, &small_file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("Auto-save"));
}

#[test]
fn test_error_strategy_blocks_mixed_files_with_large() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small").unwrap();

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let files = vec![&small_file, &large_file];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        editor_core::EditorError::FileTooLarge { path, size, limit } => {
            assert!(path.contains("large.txt"));
            assert_eq!(size, 2 * 1024 * 1024);
            assert_eq!(limit, 1024 * 1024);
        }
        _ => panic!("Expected FileTooLarge error, got: {:?}", err),
    }

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_error_strategy_blocks_at_exact_threshold_plus_one() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 5,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let file = project_path.join("exact_plus_one.txt");
    let content = vec![b'X'; 5 * 1024 * 1024 + 1];
    fs::write(&file, content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &file);

    assert!(result.is_err());
    match result.unwrap_err() {
        editor_core::EditorError::FileTooLarge { size, limit, .. } => {
            assert_eq!(size, 5 * 1024 * 1024 + 1);
            assert_eq!(limit, 5 * 1024 * 1024);
        }
        err => panic!("Expected FileTooLarge error, got: {:?}", err),
    }

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_error_strategy_allows_at_exact_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 5,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let file = project_path.join("exact.txt");
    let content = vec![b'X'; 5 * 1024 * 1024];
    fs::write(&file, content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &file);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_error_strategy_descriptive_error_message() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 2,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file = project_path.join("massive.bin");
    let large_content = vec![b'M'; 10 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &large_file);

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        editor_core::EditorError::FileTooLarge { path, size, limit } => {
            assert!(path.contains("massive.bin"));
            assert_eq!(size, 10 * 1024 * 1024);
            assert_eq!(limit, 2 * 1024 * 1024);
        }
        _ => panic!("Expected FileTooLarge error with descriptive details"),
    }
}

#[test]
fn test_error_strategy_multiple_large_files_blocks_on_first() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let large_file1 = project_path.join("large1.txt");
    let large_content1 = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&large_file1, large_content1).unwrap();

    let large_file2 = project_path.join("large2.txt");
    let large_content2 = vec![b'B'; 3 * 1024 * 1024];
    fs::write(&large_file2, large_content2).unwrap();

    let files = vec![&large_file1, &large_file2];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_err());
    let err = result.unwrap_err();
    match err {
        editor_core::EditorError::FileTooLarge { .. } => {}
        _ => panic!("Expected FileTooLarge error"),
    }

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_edge_case_file_size_changes_between_check_and_commit() {
    use std::io::Write;

    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let file_path = project_path.join("changing.txt");
    fs::write(&file_path, "small").unwrap();

    let result = manager.auto_commit_on_save(&project_path, &file_path);
    assert!(result.is_ok());

    let large_content = vec![b'X'; 2 * 1024 * 1024];
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();
    file.write_all(&large_content).unwrap();
    drop(file);

    let result = manager.auto_commit_on_save(&project_path, &file_path);
    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_edge_case_file_deleted_before_commit() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file_path = project_path.join("to_delete.txt");
    fs::write(&file_path, "content").unwrap();

    fs::remove_file(&file_path).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &file_path);
    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_edge_case_threshold_very_large_no_files_large() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1_000_000,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let huge_file = project_path.join("huge.txt");
    let huge_content = vec![b'H'; 100 * 1024 * 1024];
    fs::write(&huge_file, huge_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &huge_file);
    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_edge_case_file_deleted_in_middle_of_multi_file_commit() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    let file2 = project_path.join("file2_to_delete.txt");
    let file3 = project_path.join("file3.txt");

    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();
    fs::write(&file3, "content3").unwrap();

    fs::remove_file(&file2).unwrap();

    let files = vec![&file1, &file2, &file3];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);

    let commit = &commits[0];
    assert!(commit.message.contains("file1.txt"));
    assert!(!commit.message.contains("file2_to_delete.txt"));
    assert!(commit.message.contains("file3.txt"));
}

#[test]
fn test_edge_case_all_files_deleted_before_commit() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    let file2 = project_path.join("file2.txt");

    fs::write(&file1, "content1").unwrap();
    fs::write(&file2, "content2").unwrap();

    fs::remove_file(&file1).unwrap();
    fs::remove_file(&file2).unwrap();

    let files = vec![&file1, &file2];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_edge_case_mixed_normal_large_and_deleted_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let normal_file = project_path.join("normal.txt");
    let large_file = project_path.join("large.txt");
    let deleted_file = project_path.join("deleted.txt");

    fs::write(&normal_file, "normal content").unwrap();
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();
    fs::write(&deleted_file, "to be deleted").unwrap();

    fs::remove_file(&deleted_file).unwrap();

    let files = vec![&normal_file, &large_file, &deleted_file];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);

    assert!(result.is_ok());
    let commit_result = result.unwrap();
    assert_eq!(commit_result.skipped_files.len(), 1);
    assert_eq!(
        commit_result.skipped_files[0].to_str().unwrap(),
        "large.txt"
    );

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
    assert!(commits[0].message.contains("normal.txt"));
    assert!(!commits[0].message.contains("deleted.txt"));
    assert!(commits[0].message.contains("1 large file excluded"));
}
