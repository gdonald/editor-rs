use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

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
