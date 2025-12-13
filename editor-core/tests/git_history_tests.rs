use editor_core::{create_signature, GitHistoryManager};
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
