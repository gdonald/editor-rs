use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

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
