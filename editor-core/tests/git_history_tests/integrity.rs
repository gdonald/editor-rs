use editor_core::{GitHistoryManager, IntegrityReport};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_verify_repository_integrity_valid_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let report = manager.verify_repository_integrity(&project_path).unwrap();

    assert!(report.is_valid);
    assert!(report.errors.is_empty());
}

#[test]
fn test_verify_repository_integrity_nonexistent_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let report = manager.verify_repository_integrity(&project_path).unwrap();

    assert!(!report.is_valid);
    assert!(!report.errors.is_empty());
    assert!(report.errors[0].contains("does not exist"));
}

#[test]
fn test_verify_repository_integrity_empty_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    manager.open_repository(&project_path).unwrap();

    let report = manager.verify_repository_integrity(&project_path).unwrap();

    assert!(report.is_valid);
    assert!(report.warnings.iter().any(|w| w.contains("empty")));
}

#[test]
fn test_create_backup_valid_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let backup_name = manager.create_backup(&project_path).unwrap();

    assert!(backup_name.starts_with("backup_"));

    let backups = manager.list_backups(&project_path).unwrap();
    assert_eq!(backups.len(), 1);
    assert_eq!(backups[0], backup_name);
}

#[test]
fn test_create_backup_nonexistent_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.create_backup(&project_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
}

#[test]
fn test_list_backups_empty() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let backups = manager.list_backups(&project_path).unwrap();
    assert!(backups.is_empty());
}

#[test]
fn test_list_backups_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let backup1 = manager.create_backup(&project_path).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let backup2 = manager.create_backup(&project_path).unwrap();

    let backups = manager.list_backups(&project_path).unwrap();
    assert_eq!(backups.len(), 2);
    assert_eq!(backups[0], backup2);
    assert_eq!(backups[1], backup1);
}

#[test]
fn test_delete_backup() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let backup_name = manager.create_backup(&project_path).unwrap();

    let backups = manager.list_backups(&project_path).unwrap();
    assert_eq!(backups.len(), 1);

    manager.delete_backup(&project_path, &backup_name).unwrap();

    let backups = manager.list_backups(&project_path).unwrap();
    assert!(backups.is_empty());
}

#[test]
fn test_delete_backup_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let result = manager.delete_backup(&project_path, "backup_nonexistent");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
}

#[test]
fn test_repair_repository_valid_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let result = manager.repair_repository(&project_path);
    assert!(result.is_ok());

    let backups = manager.list_backups(&project_path).unwrap();
    assert_eq!(backups.len(), 1);
}

#[test]
fn test_repair_repository_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.repair_repository(&project_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
}

#[test]
fn test_integrity_report_default() {
    let report = IntegrityReport::default();

    assert!(report.is_valid);
    assert!(report.errors.is_empty());
    assert!(report.warnings.is_empty());
}

#[test]
fn test_integrity_report_add_error() {
    let mut report = IntegrityReport::new();

    report.add_error("Test error".to_string());

    assert!(!report.is_valid);
    assert_eq!(report.errors.len(), 1);
    assert_eq!(report.errors[0], "Test error");
}

#[test]
fn test_integrity_report_add_warning() {
    let mut report = IntegrityReport::new();

    report.add_warning("Test warning".to_string());

    assert!(report.is_valid);
    assert_eq!(report.warnings.len(), 1);
    assert_eq!(report.warnings[0], "Test warning");
}

#[test]
fn test_integrity_report_multiple_errors() {
    let mut report = IntegrityReport::new();

    report.add_error("Error 1".to_string());
    report.add_error("Error 2".to_string());
    report.add_warning("Warning 1".to_string());

    assert!(!report.is_valid);
    assert_eq!(report.errors.len(), 2);
    assert_eq!(report.warnings.len(), 1);
}

#[test]
fn test_backup_preserves_repository_content() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let commits_before = manager.list_commits(&project_path).unwrap();

    manager.create_backup(&project_path).unwrap();

    let commits_after = manager.list_commits(&project_path).unwrap();

    assert_eq!(commits_before.len(), commits_after.len());
    assert_eq!(commits_before[0].id, commits_after[0].id);
}

#[test]
fn test_verify_integrity_multiple_commits() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "v1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    fs::write(&file1, "v2").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    fs::write(&file1, "v3").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let report = manager.verify_repository_integrity(&project_path).unwrap();

    assert!(report.is_valid);
    assert!(report.errors.is_empty());
}

#[test]
fn test_backup_and_repair_workflow() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let commits_before = manager.list_commits(&project_path).unwrap();

    manager.repair_repository(&project_path).unwrap();

    let commits_after = manager.list_commits(&project_path).unwrap();

    assert_eq!(commits_before.len(), commits_after.len());
    assert_eq!(commits_before[0].id, commits_after[0].id);

    let backups = manager.list_backups(&project_path).unwrap();
    assert_eq!(backups.len(), 1);
}

#[test]
fn test_list_backups_sorted_by_timestamp() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let backup1 = manager.create_backup(&project_path).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let backup2 = manager.create_backup(&project_path).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(1));
    let backup3 = manager.create_backup(&project_path).unwrap();

    let backups = manager.list_backups(&project_path).unwrap();

    assert_eq!(backups.len(), 3);
    assert_eq!(backups[0], backup3);
    assert_eq!(backups[1], backup2);
    assert_eq!(backups[2], backup1);
}
