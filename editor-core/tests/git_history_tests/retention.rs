use editor_core::{GitHistoryManager, RetentionPolicy};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;
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
fn test_retention_policy_default_is_forever() {
    let policy = RetentionPolicy::default();
    assert_eq!(policy, RetentionPolicy::Forever);
}

#[test]
fn test_git_history_manager_default_retention_policy() {
    let manager = GitHistoryManager::new().unwrap();
    assert_eq!(manager.retention_policy(), &RetentionPolicy::Forever);
}

#[test]
fn test_with_retention_policy_builder() {
    let manager = GitHistoryManager::new()
        .unwrap()
        .with_retention_policy(RetentionPolicy::Days(30));
    assert_eq!(manager.retention_policy(), &RetentionPolicy::Days(30));
}

#[test]
fn test_retention_policy_forever() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 3);

    let manager_forever = manager.with_retention_policy(RetentionPolicy::Forever);

    for commit in &commits {
        assert!(manager_forever
            .should_retain_commit(&project_path, commit)
            .unwrap());
    }
}

#[test]
fn test_retention_policy_days() {
    let (_temp_dir, project_path, _manager) = setup_test_repo();
    let manager = GitHistoryManager::new().unwrap();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");

    thread::sleep(Duration::from_millis(100));

    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);

    let manager_days = manager.with_retention_policy(RetentionPolicy::Days(365));

    for commit in &commits {
        assert!(manager_days
            .should_retain_commit(&project_path, commit)
            .unwrap());
    }

    let manager_one_day = GitHistoryManager::new()
        .unwrap()
        .with_retention_policy(RetentionPolicy::Days(1));
    for commit in &commits {
        assert!(manager_one_day
            .should_retain_commit(&project_path, commit)
            .unwrap());
    }
}

#[test]
fn test_retention_policy_commits() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");
    create_commit_at_project(&manager, &project_path, "test.txt", "content3");
    create_commit_at_project(&manager, &project_path, "test.txt", "content4");
    create_commit_at_project(&manager, &project_path, "test.txt", "content5");

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 5);

    let manager_commits = manager.with_retention_policy(RetentionPolicy::Commits(3));

    let retained_commits: Vec<_> = commits
        .iter()
        .filter(|c| {
            manager_commits
                .should_retain_commit(&project_path, c)
                .unwrap()
        })
        .collect();

    assert_eq!(retained_commits.len(), 3);

    let not_retained_commits: Vec<_> = commits
        .iter()
        .filter(|c| {
            !manager_commits
                .should_retain_commit(&project_path, c)
                .unwrap()
        })
        .collect();

    assert_eq!(not_retained_commits.len(), 2);
}

#[test]
fn test_retention_policy_size() {
    let (_temp_dir, project_path, manager) = setup_test_repo();

    create_commit_at_project(&manager, &project_path, "test.txt", "content1");
    create_commit_at_project(&manager, &project_path, "test.txt", "content2");

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);

    let repo_size = manager.get_repo_size(&project_path).unwrap();

    let manager_large_size = manager.with_retention_policy(RetentionPolicy::Size(repo_size + 1000));

    for commit in &commits {
        assert!(manager_large_size
            .should_retain_commit(&project_path, commit)
            .unwrap());
    }

    let manager_small_size = GitHistoryManager::new()
        .unwrap()
        .with_retention_policy(RetentionPolicy::Size(0));

    for commit in &commits {
        assert!(!manager_small_size
            .should_retain_commit(&project_path, commit)
            .unwrap());
    }
}

#[test]
fn test_retention_policy_clone_and_eq() {
    let policy_forever = RetentionPolicy::Forever;
    let cloned = policy_forever.clone();
    assert_eq!(policy_forever, cloned);

    let policy_days = RetentionPolicy::Days(30);
    let cloned = policy_days.clone();
    assert_eq!(policy_days, cloned);

    let policy_commits = RetentionPolicy::Commits(100);
    let cloned = policy_commits.clone();
    assert_eq!(policy_commits, cloned);

    let policy_size = RetentionPolicy::Size(1024 * 1024 * 100);
    let cloned = policy_size.clone();
    assert_eq!(policy_size, cloned);
}
