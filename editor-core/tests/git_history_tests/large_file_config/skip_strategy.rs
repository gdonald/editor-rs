use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

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
