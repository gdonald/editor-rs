use editor_core::GitHistoryManager;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_export_history_creates_git_repository() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    fs::write(&file1, "content 1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let file2 = project_path.join("file2.txt");
    fs::write(&file2, "content 2").unwrap();
    manager.auto_commit_on_save(&project_path, &file2).unwrap();

    let result = manager.export_history(&project_path, &export_path);
    assert!(result.is_ok());

    assert!(export_path.exists());
    assert!(export_path.join(".git").exists());

    let exported_repo = git2::Repository::open(&export_path).unwrap();
    assert!(!exported_repo.is_bare());

    let mut revwalk = exported_repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    let commit_count = revwalk.count();
    assert_eq!(commit_count, 2);
}

#[test]
fn test_export_history_path_already_exists() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&export_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    fs::write(&file1, "content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let result = manager.export_history(&project_path, &export_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));
}

#[test]
fn test_export_history_preserves_commit_messages() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "v1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    fs::write(&file1, "v2").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    let exported_repo = git2::Repository::open(&export_path).unwrap();
    let mut revwalk = exported_repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    for oid in revwalk {
        let commit = exported_repo.find_commit(oid.unwrap()).unwrap();
        let message = commit.message().unwrap();
        assert!(message.contains("Auto-save"));
    }
}

#[test]
fn test_export_history_preserves_file_content() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    fs::write(&file1, "hello world").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    let exported_file = export_path.join("file1.txt");
    assert!(exported_file.exists());
    let content = fs::read_to_string(&exported_file).unwrap();
    assert_eq!(content, "hello world");
}

#[test]
fn test_import_history_from_exported_repo() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");
    let new_project_path = temp_dir.path().join("new_project");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&new_project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    fs::write(&file1, "content 1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let file2 = project_path.join("file2.txt");
    fs::write(&file2, "content 2").unwrap();
    manager.auto_commit_on_save(&project_path, &file2).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    let result = manager.import_history(&new_project_path, &export_path);
    assert!(result.is_ok());

    let commits = manager.list_commits(&new_project_path).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_import_history_path_does_not_exist() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let import_path = temp_dir.path().join("nonexistent");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.import_history(&project_path, &import_path);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
}

#[test]
fn test_import_history_preserves_commit_order() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");
    let new_project_path = temp_dir.path().join("new_project");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&new_project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("test.txt");
    fs::write(&file1, "v1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    fs::write(&file1, "v2").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    fs::write(&file1, "v3").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    manager
        .import_history(&new_project_path, &export_path)
        .unwrap();

    let original_commits = manager.list_commits(&project_path).unwrap();
    let imported_commits = manager.list_commits(&new_project_path).unwrap();

    assert_eq!(original_commits.len(), imported_commits.len());
    assert_eq!(original_commits.len(), 3);

    for (orig, imp) in original_commits.iter().zip(imported_commits.iter()) {
        assert_eq!(orig.message, imp.message);
    }
}

#[test]
fn test_export_import_round_trip() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");
    let new_project_path = temp_dir.path().join("new_project");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&new_project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    fs::write(&file1, "content 1").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    let file2 = project_path.join("file2.txt");
    fs::write(&file2, "content 2").unwrap();
    manager.auto_commit_on_save(&project_path, &file2).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();
    manager
        .import_history(&new_project_path, &export_path)
        .unwrap();

    let original_commits = manager.list_commits(&project_path).unwrap();
    let imported_commits = manager.list_commits(&new_project_path).unwrap();

    assert_eq!(original_commits.len(), imported_commits.len());

    for (orig, imp) in original_commits.iter().zip(imported_commits.iter()) {
        assert_eq!(orig.message, imp.message);
        assert_eq!(orig.author_name, imp.author_name);
        assert_eq!(orig.author_email, imp.author_email);
    }
}

#[test]
fn test_export_history_with_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project_path.join("file1.txt");
    let file2 = project_path.join("file2.txt");
    let file3 = project_path.join("file3.txt");

    fs::write(&file1, "content 1").unwrap();
    fs::write(&file2, "content 2").unwrap();
    fs::write(&file3, "content 3").unwrap();

    let files = vec![&file1, &file2, &file3];
    manager
        .auto_commit_on_save_multiple(&project_path, &files)
        .unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    assert!(export_path.join("file1.txt").exists());
    assert!(export_path.join("file2.txt").exists());
    assert!(export_path.join("file3.txt").exists());

    let content1 = fs::read_to_string(export_path.join("file1.txt")).unwrap();
    let content2 = fs::read_to_string(export_path.join("file2.txt")).unwrap();
    let content3 = fs::read_to_string(export_path.join("file3.txt")).unwrap();

    assert_eq!(content1, "content 1");
    assert_eq!(content2, "content 2");
    assert_eq!(content3, "content 3");
}

#[test]
fn test_export_history_with_nested_directories() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");

    fs::create_dir(&project_path).unwrap();
    let subdir = project_path.join("subdir");
    fs::create_dir(&subdir).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = subdir.join("nested.txt");
    fs::write(&file1, "nested content").unwrap();
    manager.auto_commit_on_save(&project_path, &file1).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    let exported_file = export_path.join("subdir").join("nested.txt");
    assert!(exported_file.exists());
    let content = fs::read_to_string(&exported_file).unwrap();
    assert_eq!(content, "nested content");
}

#[test]
fn test_import_history_appends_to_existing_history() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    let export_path = temp_dir.path().join("export");
    let new_project_path = temp_dir.path().join("new_project");

    fs::create_dir(&project_path).unwrap();
    fs::create_dir(&new_project_path).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = new_project_path.join("existing.txt");
    fs::write(&file1, "existing").unwrap();
    manager
        .auto_commit_on_save(&new_project_path, &file1)
        .unwrap();

    let file2 = project_path.join("import.txt");
    fs::write(&file2, "import content").unwrap();
    manager.auto_commit_on_save(&project_path, &file2).unwrap();

    manager.export_history(&project_path, &export_path).unwrap();

    manager
        .import_history(&new_project_path, &export_path)
        .unwrap();

    let commits = manager.list_commits(&new_project_path).unwrap();
    assert_eq!(commits.len(), 2);
}
