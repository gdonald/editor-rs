use editor_core::GitHistoryManager;
use std::fs;
use tempfile::TempDir;

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
