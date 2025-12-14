use editor_core::{create_signature, ChangeStatus, GitHistoryManager};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_project_hash_consistency() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    let hash1 = GitHistoryManager::project_hash(project_path).unwrap();
    let hash2 = GitHistoryManager::project_hash(project_path).unwrap();

    assert_eq!(hash1, hash2);
    assert_eq!(hash1.len(), 64);
}

#[test]
fn test_project_hash_different_paths() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();

    let hash1 = GitHistoryManager::project_hash(temp_dir1.path()).unwrap();
    let hash2 = GitHistoryManager::project_hash(temp_dir2.path()).unwrap();

    assert_ne!(hash1, hash2);
}

#[test]
fn test_init_repository() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root.clone()).unwrap();
    let repo = manager.init_repository(project_dir.path()).unwrap();

    assert!(repo.path().exists());

    let repo_path_canonical = repo.path().canonicalize().unwrap();
    let storage_root_canonical = storage_root.canonicalize().unwrap();
    assert!(repo_path_canonical.starts_with(&storage_root_canonical));
}

#[test]
fn test_open_repository_creates_if_missing() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let repo = manager.open_repository(project_dir.path()).unwrap();

    assert!(repo.path().exists());
}

#[test]
fn test_open_existing_repository() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let repo1 = manager.init_repository(project_dir.path()).unwrap();
    let repo1_path = repo1.path().to_path_buf();
    drop(repo1);

    let repo2 = manager.open_repository(project_dir.path()).unwrap();
    assert_eq!(repo1_path, repo2.path());
}

#[test]
fn test_project_metadata() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let repo = manager.init_repository(project_dir.path()).unwrap();

    let retrieved_path = manager.get_project_path(&repo).unwrap();
    assert!(retrieved_path.is_some());

    let canonical_project = project_dir.path().canonicalize().unwrap();
    assert_eq!(retrieved_path.unwrap(), canonical_project);
}

#[test]
fn test_create_signature() {
    let sig = create_signature().unwrap();
    assert_eq!(sig.name().unwrap(), "editor-rs");
    assert_eq!(sig.email().unwrap(), "editor-rs@localhost");
}

#[test]
fn test_repo_path_uses_hash() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root.clone()).unwrap();
    let repo_path = manager.repo_path(project_dir.path()).unwrap();

    let hash = GitHistoryManager::project_hash(project_dir.path()).unwrap();
    let expected_path = storage_root.join(hash);

    assert_eq!(repo_path, expected_path);
}

#[test]
fn test_auto_commit_on_save_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Hello, World!").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();

    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    assert!(commit.message().unwrap().contains("Auto-save: test.txt"));
}

#[test]
fn test_auto_commit_creates_multiple_commits() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");

    fs::write(&file_path, "Version 1").unwrap();
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    fs::write(&file_path, "Version 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let commits: Vec<_> = revwalk.collect();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_auto_commit_nested_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let nested_dir = project_dir.path().join("src").join("nested");
    fs::create_dir_all(&nested_dir).unwrap();

    let file_path = nested_dir.join("module.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    assert!(
        commit.message().unwrap().contains("src/nested/module.rs")
            || commit.message().unwrap().contains("src\\nested\\module.rs")
    );
}

#[test]
fn test_auto_commit_file_outside_project_skipped() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();
    let other_dir = TempDir::new().unwrap();

    let file_path = other_dir.path().join("test.txt");
    fs::write(&file_path, "Hello").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let result = manager.auto_commit_on_save(project_dir.path(), &file_path);

    assert!(result.is_ok());
}

#[test]
fn test_auto_commit_doesnt_affect_user_repo() {
    let user_repo_dir = TempDir::new().unwrap();
    git2::Repository::init(user_repo_dir.path()).unwrap();

    let file_path = user_repo_dir.path().join("test.txt");
    fs::write(&file_path, "Hello").unwrap();

    let storage_root = TempDir::new().unwrap();
    let manager = GitHistoryManager::with_storage_root(storage_root.path().to_path_buf()).unwrap();

    manager
        .auto_commit_on_save(user_repo_dir.path(), &file_path)
        .unwrap();

    let user_repo = git2::Repository::open(user_repo_dir.path()).unwrap();
    let is_head_unborn = user_repo.head().is_err() || user_repo.is_empty().unwrap();
    assert!(is_head_unborn);
}

#[test]
fn test_auto_commit_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file1_path = project_dir.path().join("file1.txt");
    let file2_path = project_dir.path().join("file2.txt");
    let file3_path = project_dir.path().join("file3.txt");

    fs::write(&file1_path, "File 1 content").unwrap();
    fs::write(&file2_path, "File 2 content").unwrap();
    fs::write(&file3_path, "File 3 content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let file_paths = vec![&file1_path, &file2_path, &file3_path];

    manager
        .auto_commit_on_save_multiple(project_dir.path(), &file_paths)
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    let message = commit.message().unwrap();
    assert!(message.contains("3 files"));
    assert!(message.contains("file1.txt"));
    assert!(message.contains("file2.txt"));
    assert!(message.contains("file3.txt"));
}

#[test]
fn test_auto_commit_multiple_files_nested() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let src_dir = project_dir.path().join("src");
    let tests_dir = project_dir.path().join("tests");
    fs::create_dir_all(&src_dir).unwrap();
    fs::create_dir_all(&tests_dir).unwrap();

    let file1_path = src_dir.join("main.rs");
    let file2_path = src_dir.join("lib.rs");
    let file3_path = tests_dir.join("integration.rs");

    fs::write(&file1_path, "fn main() {}").unwrap();
    fs::write(&file2_path, "pub fn lib() {}").unwrap();
    fs::write(&file3_path, "#[test] fn test() {}").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let file_paths = vec![&file1_path, &file2_path, &file3_path];

    manager
        .auto_commit_on_save_multiple(project_dir.path(), &file_paths)
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    let message = commit.message().unwrap();
    assert!(message.contains("3 files"));
    assert!(message.contains("src") && message.contains("main.rs"));
    assert!(message.contains("src") && message.contains("lib.rs"));
    assert!(message.contains("tests") && message.contains("integration.rs"));
}

#[test]
fn test_auto_commit_empty_file_list() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let file_paths: Vec<&std::path::PathBuf> = vec![];

    let result = manager.auto_commit_on_save_multiple(project_dir.path(), &file_paths);
    assert!(result.is_ok());
}

#[test]
fn test_auto_commit_single_file_via_multiple() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("single.txt");
    fs::write(&file_path, "Single file").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    manager
        .auto_commit_on_save_multiple(project_dir.path(), &[&file_path])
        .unwrap();

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    let message = commit.message().unwrap();
    assert!(message.contains("Auto-save: single.txt"));
    assert!(!message.contains("files"));
}

#[test]
fn test_corrupted_repo_recovery() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root.clone()).unwrap();

    let repo_path = manager.repo_path(project_dir.path()).unwrap();
    fs::create_dir_all(&repo_path).unwrap();
    fs::write(repo_path.join("corrupt_file"), "not a git repo").unwrap();

    let result = manager.open_repository(project_dir.path());
    assert!(result.is_ok());

    let repo = result.unwrap();
    assert!(repo.path().exists());
}

#[test]
fn test_auto_commit_with_mixed_valid_invalid_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();
    let other_dir = TempDir::new().unwrap();

    let valid_file = project_dir.path().join("valid.txt");
    let invalid_file = other_dir.path().join("invalid.txt");

    fs::write(&valid_file, "Valid content").unwrap();
    fs::write(&invalid_file, "Invalid content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let file_paths = vec![&valid_file, &invalid_file];

    let result = manager.auto_commit_on_save_multiple(project_dir.path(), &file_paths);
    assert!(result.is_ok());

    let repo = manager.open_repository(project_dir.path()).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();

    let message = commit.message().unwrap();
    assert!(message.contains("valid.txt"));
    assert!(!message.contains("invalid.txt"));
}

#[test]
fn test_list_commits_empty_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager.init_repository(project_dir.path()).unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 0);
}

#[test]
fn test_list_commits_single_commit() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Hello, World!").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 1);

    let commit = &commits[0];
    assert_eq!(commit.author_name, "editor-rs");
    assert_eq!(commit.author_email, "editor-rs@localhost");
    assert!(commit.message.contains("Auto-save: test.txt"));
    assert!(commit.timestamp > 0);
    assert!(!commit.id.is_empty());
}

#[test]
fn test_list_commits_multiple_commits() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");

    fs::write(&file_path, "Version 1").unwrap();
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    fs::write(&file_path, "Version 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    fs::write(&file_path, "Version 3").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 3);

    for commit in commits.iter().take(3) {
        assert_eq!(commit.author_name, "editor-rs");
        assert!(!commit.id.is_empty());
    }
}

#[test]
fn test_get_commit_details() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 1);

    let commit_id = &commits[0].id;
    let details = manager
        .get_commit_details(project_dir.path(), commit_id)
        .unwrap();

    assert_eq!(details.id, *commit_id);
    assert_eq!(details.author_name, "editor-rs");
    assert_eq!(details.author_email, "editor-rs@localhost");
    assert!(details.message.contains("Auto-save: test.txt"));
    assert!(details.timestamp > 0);
}

#[test]
fn test_get_commit_details_invalid_id() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager.init_repository(project_dir.path()).unwrap();

    let result = manager.get_commit_details(project_dir.path(), "invalid_id");
    assert!(result.is_err());
}

#[test]
fn test_get_files_changed_single_file_added() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    let commit_id = &commits[0].id;

    let files = manager
        .get_files_changed(project_dir.path(), commit_id)
        .unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "test.txt");
    assert_eq!(files[0].status, ChangeStatus::Added);
}

#[test]
fn test_get_files_changed_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file1 = project_dir.path().join("file1.txt");
    let file2 = project_dir.path().join("file2.txt");
    let file3 = project_dir.path().join("file3.txt");

    fs::write(&file1, "Content 1").unwrap();
    fs::write(&file2, "Content 2").unwrap();
    fs::write(&file3, "Content 3").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save_multiple(project_dir.path(), &[&file1, &file2, &file3])
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    let commit_id = &commits[0].id;

    let files = manager
        .get_files_changed(project_dir.path(), commit_id)
        .unwrap();
    assert_eq!(files.len(), 3);

    let file_names: Vec<&str> = files.iter().map(|f| f.path.as_str()).collect();
    assert!(file_names.contains(&"file1.txt"));
    assert!(file_names.contains(&"file2.txt"));
    assert!(file_names.contains(&"file3.txt"));

    for file in &files {
        assert_eq!(file.status, ChangeStatus::Added);
    }
}

#[test]
fn test_get_files_changed_modified_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Version 1").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    fs::write(&file_path, "Version 2").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    let latest_commit_id = &commits[0].id;

    let files = manager
        .get_files_changed(project_dir.path(), latest_commit_id)
        .unwrap();
    assert_eq!(files.len(), 1);
    assert_eq!(files[0].path, "test.txt");
    assert_eq!(files[0].status, ChangeStatus::Modified);
}

#[test]
fn test_get_diff_between_commits() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Line 1\nLine 2\n").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    fs::write(&file_path, "Line 1\nLine 2 Modified\nLine 3\n").unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 2);

    let from_commit = &commits[1].id;
    let to_commit = &commits[0].id;

    let diff = manager
        .get_diff_between_commits(project_dir.path(), from_commit, to_commit)
        .unwrap();

    assert!(diff.contains("test.txt"));
    assert!(diff.contains("-Line 2"));
    assert!(diff.contains("+Line 2 Modified"));
    assert!(diff.contains("+Line 3"));
}

#[test]
fn test_get_file_diff_between_commits() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file1 = project_dir.path().join("file1.txt");
    let file2 = project_dir.path().join("file2.txt");

    fs::write(&file1, "File 1 Version 1\n").unwrap();
    fs::write(&file2, "File 2 Version 1\n").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save_multiple(project_dir.path(), &[&file1, &file2])
        .unwrap();

    fs::write(&file1, "File 1 Version 2\n").unwrap();
    fs::write(&file2, "File 2 Version 2\n").unwrap();
    manager
        .auto_commit_on_save_multiple(project_dir.path(), &[&file1, &file2])
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    let from_commit = &commits[1].id;
    let to_commit = &commits[0].id;

    let diff1 = manager
        .get_file_diff_between_commits(project_dir.path(), "file1.txt", from_commit, to_commit)
        .unwrap();

    assert!(diff1.contains("file1.txt"));
    assert!(diff1.contains("-File 1 Version 1"));
    assert!(diff1.contains("+File 1 Version 2"));
    assert!(!diff1.contains("file2.txt"));

    let diff2 = manager
        .get_file_diff_between_commits(project_dir.path(), "file2.txt", from_commit, to_commit)
        .unwrap();

    assert!(diff2.contains("file2.txt"));
    assert!(diff2.contains("-File 2 Version 1"));
    assert!(diff2.contains("+File 2 Version 2"));
    assert!(!diff2.contains("file1.txt"));
}

#[test]
fn test_get_diff_between_commits_no_changes() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    fs::write(&file_path, "Content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    manager
        .auto_commit_on_save(project_dir.path(), &file_path)
        .unwrap();

    let commits = manager.list_commits(project_dir.path()).unwrap();
    let commit_id = &commits[0].id;

    let diff = manager
        .get_diff_between_commits(project_dir.path(), commit_id, commit_id)
        .unwrap();
    assert!(diff.is_empty() || diff.trim().is_empty());
}

#[test]
fn test_list_commits_order() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();

    let file_path = project_dir.path().join("test.txt");
    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    for i in 1..=5 {
        fs::write(&file_path, format!("Version {}", i)).unwrap();
        manager
            .auto_commit_on_save(project_dir.path(), &file_path)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    let commits = manager.list_commits(project_dir.path()).unwrap();
    assert_eq!(commits.len(), 5);

    for i in 0..commits.len() - 1 {
        assert!(commits[i].timestamp >= commits[i + 1].timestamp);
    }
}

#[test]
fn test_auto_commit_all_files_outside_project() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();
    let other_dir = TempDir::new().unwrap();

    let file1 = other_dir.path().join("file1.txt");
    let file2 = other_dir.path().join("file2.txt");

    fs::write(&file1, "Content 1").unwrap();
    fs::write(&file2, "Content 2").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let file_paths = vec![&file1, &file2];

    let result = manager.auto_commit_on_save_multiple(project_dir.path(), &file_paths);
    assert!(result.is_ok());
}
