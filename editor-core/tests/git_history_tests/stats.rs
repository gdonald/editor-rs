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
    assert!(!file1_stats.is_large);

    let file2_stats = stats.iter().find(|s| s.path == "file2.txt").unwrap();
    assert_eq!(file2_stats.commit_count, 1);
    assert!(file2_stats.total_size > 0);
    assert!(!file2_stats.is_large);
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

#[test]
fn test_get_history_stats() {
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

    fs::write(&file2_path, "Content 2 with more data").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file2_path)
        .unwrap();

    fs::write(&file1_path, "Updated content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file1_path)
        .unwrap();

    let stats = manager.get_history_stats(project_dir.path()).unwrap();

    assert_eq!(stats.total_commits, 3);

    assert!(stats.repository_size > 0);

    assert!(stats.date_range.is_some());
    let (oldest, newest) = stats.date_range.unwrap();
    assert!(oldest <= newest);

    assert_eq!(stats.file_stats.len(), 2);

    let file1_stats = stats
        .file_stats
        .iter()
        .find(|s| s.path == "file1.txt")
        .unwrap();
    assert_eq!(file1_stats.commit_count, 2);

    let file2_stats = stats
        .file_stats
        .iter()
        .find(|s| s.path == "file2.txt")
        .unwrap();
    assert_eq!(file2_stats.commit_count, 1);
}

#[test]
fn test_get_history_stats_empty() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let stats = manager.get_history_stats(project_dir.path()).unwrap();

    assert_eq!(stats.total_commits, 0);
    assert!(stats.repository_size > 0);
    assert!(stats.date_range.is_none());
    assert_eq!(stats.file_stats.len(), 0);
    assert_eq!(stats.large_file_count, 0);
    assert_eq!(stats.total_large_file_size, 0);
}

#[test]
fn test_get_per_file_stats_with_large_files() {
    use editor_core::LargeFileConfig;

    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = LargeFileConfig {
        threshold_mb: 0,
        strategy: editor_core::LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_dir.path().join("small.txt");
    let large_file = project_dir.path().join("large.txt");

    fs::write(&small_file, "").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &small_file)
        .unwrap();

    fs::write(&large_file, "Large content").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &large_file)
        .unwrap();

    let stats = manager.get_per_file_stats(project_dir.path()).unwrap();

    assert_eq!(stats.len(), 2);

    let small_stats = stats.iter().find(|s| s.path == "small.txt").unwrap();
    assert!(!small_stats.is_large);

    let large_stats = stats.iter().find(|s| s.path == "large.txt").unwrap();
    assert!(large_stats.is_large);
}

#[test]
fn test_list_large_files() {
    use editor_core::LargeFileConfig;

    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = LargeFileConfig {
        threshold_mb: 0,
        strategy: editor_core::LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_dir.path().join("small.txt");
    let large_file1 = project_dir.path().join("large1.txt");
    let large_file2 = project_dir.path().join("large2.txt");

    fs::write(&small_file, "").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &small_file)
        .unwrap();

    fs::write(&large_file1, "Large content 1").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &large_file1)
        .unwrap();

    fs::write(&large_file2, "Large content 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &large_file2)
        .unwrap();

    let large_files = manager.list_large_files(project_dir.path()).unwrap();

    assert_eq!(large_files.len(), 2);
    assert!(large_files.iter().all(|f| f.is_large));
    assert!(large_files.iter().any(|f| f.path == "large1.txt"));
    assert!(large_files.iter().any(|f| f.path == "large2.txt"));
}

#[test]
fn test_get_history_stats_with_large_files() {
    use editor_core::LargeFileConfig;

    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let config = LargeFileConfig {
        threshold_mb: 0,
        strategy: editor_core::LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_dir.path().join("small.txt");
    let large_file = project_dir.path().join("large.txt");

    fs::write(&small_file, "").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &small_file)
        .unwrap();

    fs::write(&large_file, "Large content").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &large_file)
        .unwrap();

    let stats = manager.get_history_stats(project_dir.path()).unwrap();

    assert_eq!(stats.total_commits, 2);
    assert_eq!(stats.large_file_count, 1);
    assert!(stats.total_large_file_size > 0);
    assert_eq!(stats.file_stats.len(), 2);
}

#[test]
fn test_list_large_files_empty() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let small_file = project_dir.path().join("small.txt");
    fs::write(&small_file, "Small content").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &small_file)
        .unwrap();

    let large_files = manager.list_large_files(project_dir.path()).unwrap();

    assert_eq!(large_files.len(), 0);
}
