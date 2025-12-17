use editor_core::GitHistoryManager;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_get_date_range_empty() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let date_range = manager.get_date_range(project_dir.path()).unwrap();

    assert!(date_range.is_none());
}

#[test]
fn test_get_date_range() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    fs::write(&file_path, "Content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(1));

    fs::write(&file_path, "Content 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let date_range = manager.get_date_range(project_dir.path()).unwrap();
    assert!(date_range.is_some());

    let (oldest, newest) = date_range.unwrap();
    assert!(oldest <= newest);
    assert!(newest - oldest >= 1);
}

#[test]
fn test_get_per_file_stats() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file1_path = project_dir.path().join("file1.txt");
    let file2_path = project_dir.path().join("file2.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    fs::write(&file1_path, "Content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file1_path)
        .unwrap();

    fs::write(&file2_path, "Content 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file2_path)
        .unwrap();

    fs::write(&file1_path, "Updated content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file1_path)
        .unwrap();

    let stats = manager.get_per_file_stats(project_dir.path()).unwrap();

    assert_eq!(stats.len(), 2);

    let file1_stats = stats.iter().find(|s| s.path == "file1.txt").unwrap();
    assert_eq!(file1_stats.commit_count, 2);
    assert!(file1_stats.total_size > 0);

    let file2_stats = stats.iter().find(|s| s.path == "file2.txt").unwrap();
    assert_eq!(file2_stats.commit_count, 1);
    assert!(file2_stats.total_size > 0);
}

#[test]
fn test_get_commits_per_day() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    for i in 1..=3 {
        fs::write(&file_path, format!("Content {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
    }

    let commits_per_day = manager.get_commits_per_day(project_dir.path()).unwrap();

    assert!(!commits_per_day.is_empty());

    let total_commits: usize = commits_per_day.values().sum();
    assert_eq!(total_commits, 3);
}

#[test]
fn test_get_commits_per_week() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    for i in 1..=3 {
        fs::write(&file_path, format!("Content {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
    }

    let commits_per_week = manager.get_commits_per_week(project_dir.path()).unwrap();

    assert!(!commits_per_week.is_empty());

    let total_commits: usize = commits_per_week.values().sum();
    assert_eq!(total_commits, 3);
}

#[test]
fn test_get_commits_per_month() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    for i in 1..=3 {
        fs::write(&file_path, format!("Content {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
    }

    let commits_per_month = manager.get_commits_per_month(project_dir.path()).unwrap();

    assert!(!commits_per_month.is_empty());

    let total_commits: usize = commits_per_month.values().sum();
    assert_eq!(total_commits, 3);
}
