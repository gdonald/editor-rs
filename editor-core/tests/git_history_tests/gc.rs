use editor_core::{GcConfig, GitHistoryManager};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_run_gc() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Test content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let result = manager.run_gc(project_dir.path(), false);
    assert!(result.is_ok());
}

#[test]
fn test_run_gc_aggressive() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Test content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let result = manager.run_gc(project_dir.path(), true);
    assert!(result.is_ok());
}

#[test]
fn test_get_commit_count() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    for i in 1..=5 {
        fs::write(&file_path, format!("Content {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
    }

    let count = manager.get_commit_count(project_dir.path()).unwrap();
    assert_eq!(count, 5);
}

#[test]
fn test_get_repo_size() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Test content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let size = manager.get_repo_size(project_dir.path()).unwrap();
    assert!(size > 0);
}

#[test]
fn test_gc_config_default() {
    let config = GcConfig::default();
    assert!(config.enabled);
    assert_eq!(config.commits_threshold, 1000);
    assert_eq!(config.size_threshold_mb, 100);
    assert!(!config.aggressive);
}

#[test]
fn test_with_gc_config() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let custom_config = GcConfig {
        enabled: false,
        commits_threshold: 500,
        size_threshold_mb: 50,
        aggressive: true,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_gc_config(custom_config.clone());

    assert_eq!(manager.gc_config(), &custom_config);
}

#[test]
fn test_should_run_gc_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = GcConfig {
        enabled: false,
        commits_threshold: 1,
        size_threshold_mb: 1,
        aggressive: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_gc_config(config);

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Test content").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let should_run = manager.should_run_gc(project_dir.path()).unwrap();
    assert!(!should_run);
}

#[test]
fn test_should_run_gc_commit_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = GcConfig {
        enabled: true,
        commits_threshold: 3,
        size_threshold_mb: 999999,
        aggressive: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_gc_config(config);

    let file_path = project_dir.path().join("test.txt");

    for i in 1..=2 {
        fs::write(&file_path, format!("Content {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
    }

    let should_run = manager.should_run_gc(project_dir.path()).unwrap();
    assert!(!should_run);

    fs::write(&file_path, "Content 3").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let should_run = manager.should_run_gc(project_dir.path()).unwrap();
    assert!(should_run);
}

#[test]
fn test_auto_gc_if_needed() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = GcConfig {
        enabled: true,
        commits_threshold: 2,
        size_threshold_mb: 999999,
        aggressive: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_gc_config(config);

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let gc_ran = manager.auto_gc_if_needed(project_dir.path()).unwrap();
    assert!(!gc_ran);

    fs::write(&file_path, "Content 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let gc_ran = manager.auto_gc_if_needed(project_dir.path()).unwrap();
    assert!(gc_ran);
}

#[test]
fn test_auto_gc_if_needed_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = GcConfig {
        enabled: false,
        commits_threshold: 1,
        size_threshold_mb: 1,
        aggressive: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_gc_config(config);

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Test content").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let gc_ran = manager.auto_gc_if_needed(project_dir.path()).unwrap();
    assert!(!gc_ran);
}
