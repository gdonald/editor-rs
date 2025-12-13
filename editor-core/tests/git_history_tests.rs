use editor_core::{create_signature, GitHistoryManager};
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
fn test_auto_commit_file_outside_project_fails() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_dir = TempDir::new().unwrap();
    let other_dir = TempDir::new().unwrap();

    let file_path = other_dir.path().join("test.txt");
    fs::write(&file_path, "Hello").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();
    let result = manager.auto_commit_on_save(project_dir.path(), &file_path);

    assert!(result.is_err());
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
