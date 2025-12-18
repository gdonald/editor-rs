use editor_core::{GitHistoryManager, RetentionPolicy};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

fn setup_test_repo() -> (TempDir, PathBuf, GitHistoryManager) {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().to_path_buf();
    let history_manager = GitHistoryManager::new().unwrap();

    (temp_dir, project_path, history_manager)
}

fn create_commit_at_project(
    history_manager: &GitHistoryManager,
    project_path: &Path,
    file_name: &str,
    content: &str,
) {
    let file_path = project_path.join(file_name);
    fs::write(&file_path, content).unwrap();
    history_manager
        .auto_commit_on_save(project_path, &file_path)
        .unwrap();
}

#[test]
fn test_cleanup_with_forever_retention_deletes_nothing() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let commits_before = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits_before.len(), 3);

    let manager_forever = manager.with_retention_policy(RetentionPolicy::Forever);
    let stats = manager_forever.cleanup_old_commits(&project_path).unwrap();

    assert_eq!(stats.commits_before, 3);
    assert_eq!(stats.commits_after, 3);

    let commits_after = manager_forever.list_commits(&project_path).unwrap();
    assert_eq!(commits_after.len(), 3);
}

#[test]
fn test_cleanup_with_commits_retention_deletes_old_commits() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");
    create_commit_at_project(&manager, &project_path, "test.txt", "content4");
    create_commit_at_project(&manager, &project_path, "test.txt", "content5");

    let commits_before = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits_before.len(), 5);

    let manager_keep_3 = manager.with_retention_policy(RetentionPolicy::Commits(3));
    let stats = manager_keep_3.cleanup_old_commits(&project_path).unwrap();

    assert_eq!(stats.commits_before, 5);
    assert_eq!(stats.commits_after, 3);

    let commits_after = manager_keep_3.list_commits(&project_path).unwrap();
    assert_eq!(commits_after.len(), 3);
}

#[test]
fn test_cleanup_stats_includes_size_information() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let manager_forever = manager.with_retention_policy(RetentionPolicy::Forever);
    let stats = manager_forever.cleanup_old_commits(&project_path).unwrap();

    assert!(stats.size_before > 0);
    assert!(stats.size_after > 0);
    assert_eq!(stats.size_before, stats.size_after);
}

#[test]
fn test_cleanup_with_no_commits_to_delete() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let commits_before = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits_before.len(), 2);

    let manager_keep_10 = manager.with_retention_policy(RetentionPolicy::Commits(10));
    let stats = manager_keep_10.cleanup_old_commits(&project_path).unwrap();

    assert_eq!(stats.commits_before, 2);
    assert_eq!(stats.commits_after, 2);

    let commits_after = manager_keep_10.list_commits(&project_path).unwrap();
    assert_eq!(commits_after.len(), 2);
}

#[test]
fn test_cleanup_with_size_based_retention() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let commits_before = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits_before.len(), 3);

    let manager_zero_size = manager.with_retention_policy(RetentionPolicy::Size(0));
    let stats = manager_zero_size
        .cleanup_old_commits(&project_path)
        .unwrap();

    assert_eq!(stats.commits_before, 3);
}

#[test]
fn test_cleanup_preserves_repository_structure() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "file1.txt", "content1");
    create_commit_at_project(&manager, &project_path, "file2.txt", "content2");
    create_commit_at_project(&manager, &project_path, "file1.txt", "content3");

    let manager_keep_1 = manager.with_retention_policy(RetentionPolicy::Commits(1));
    manager_keep_1.cleanup_old_commits(&project_path).unwrap();

    let repo = manager_keep_1.open_repository(&project_path).unwrap();
    assert!(repo.head().is_ok());
}

#[test]
fn test_cleanup_after_cleanup_stats_are_consistent() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let manager_keep_2 = manager.with_retention_policy(RetentionPolicy::Commits(2));
    let stats = manager_keep_2.cleanup_old_commits(&project_path).unwrap();

    let actual_commits = manager_keep_2.list_commits(&project_path).unwrap();
    assert_eq!(stats.commits_after, actual_commits.len());
}

#[test]
fn test_auto_cleanup_with_disabled_flag() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let mut manager_keep_1 = manager.with_retention_policy(RetentionPolicy::Commits(1));
    manager_keep_1.set_auto_cleanup_enabled(false);

    let result = manager_keep_1
        .auto_cleanup_if_needed(&project_path)
        .unwrap();
    assert!(result.is_none());

    let commits = manager_keep_1.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 3);
}

#[test]
fn test_auto_cleanup_with_enabled_flag() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let mut manager_keep_1 = manager.with_retention_policy(RetentionPolicy::Commits(1));
    manager_keep_1.set_auto_cleanup_enabled(true);

    let result = manager_keep_1
        .auto_cleanup_if_needed(&project_path)
        .unwrap();
    assert!(result.is_some());

    let stats = result.unwrap();
    assert_eq!(stats.commits_before, 3);
    assert_eq!(stats.commits_after, 1);

    let commits = manager_keep_1.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_auto_cleanup_with_forever_retention_returns_none() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let mut manager_forever = manager.with_retention_policy(RetentionPolicy::Forever);
    manager_forever.set_auto_cleanup_enabled(true);

    let result = manager_forever
        .auto_cleanup_if_needed(&project_path)
        .unwrap();
    assert!(result.is_none());

    let commits = manager_forever.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_auto_cleanup_with_no_commits_to_delete_returns_none() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let mut manager_keep_10 = manager.with_retention_policy(RetentionPolicy::Commits(10));
    manager_keep_10.set_auto_cleanup_enabled(true);

    let result = manager_keep_10
        .auto_cleanup_if_needed(&project_path)
        .unwrap();
    assert!(result.is_none());

    let commits = manager_keep_10.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_auto_cleanup_enabled_getter_and_setter() {
    let (_temp_dir, _project_path, manager) = setup_test_repo();

    assert!(!manager.auto_cleanup_enabled());

    let mut manager_mut = manager;
    manager_mut.set_auto_cleanup_enabled(true);
    assert!(manager_mut.auto_cleanup_enabled());

    manager_mut.set_auto_cleanup_enabled(false);
    assert!(!manager_mut.auto_cleanup_enabled());
}
