use editor_core::{GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

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
