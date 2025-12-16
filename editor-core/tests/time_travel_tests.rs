use editor_core::error::Result;
use editor_core::git_history::GitHistoryManager;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_restore_commit() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let file_path = project_path.join("test.txt");
    fs::write(&file_path, "Version 1").unwrap();

    git_history.auto_commit_on_save(project_path, &file_path)?;

    fs::write(&file_path, "Version 2").unwrap();
    git_history.auto_commit_on_save(project_path, &file_path)?;

    fs::write(&file_path, "Version 3").unwrap();
    git_history.auto_commit_on_save(project_path, &file_path)?;

    let commits = git_history.list_commits(project_path)?;
    assert_eq!(commits.len(), 3);

    let second_commit_id = &commits[1].id;

    git_history.restore_commit(project_path, second_commit_id)?;

    let repo_path = git_history.repo_path(project_path)?;
    let repo_file_path = repo_path.join("test.txt");
    let content = fs::read_to_string(&repo_file_path).unwrap();
    assert_eq!(content, "Version 2");

    Ok(())
}

#[test]
fn test_restore_specific_file() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let file1_path = project_path.join("file1.txt");
    let file2_path = project_path.join("file2.txt");

    fs::write(&file1_path, "File1 V1").unwrap();
    fs::write(&file2_path, "File2 V1").unwrap();

    git_history.auto_commit_on_save_multiple(
        project_path,
        &[&file1_path.to_path_buf(), &file2_path.to_path_buf()],
    )?;

    fs::write(&file1_path, "File1 V2").unwrap();
    fs::write(&file2_path, "File2 V2").unwrap();

    git_history.auto_commit_on_save_multiple(
        project_path,
        &[&file1_path.to_path_buf(), &file2_path.to_path_buf()],
    )?;

    let commits = git_history.list_commits(project_path)?;
    assert_eq!(commits.len(), 2);

    let first_commit_id = &commits[1].id;

    let content = git_history.restore_file(project_path, "file1.txt", first_commit_id)?;
    assert_eq!(String::from_utf8(content).unwrap(), "File1 V1");

    let content = git_history.restore_file(project_path, "file2.txt", first_commit_id)?;
    assert_eq!(String::from_utf8(content).unwrap(), "File2 V1");

    Ok(())
}

#[test]
fn test_get_file_content_at_commit() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let file_path = project_path.join("test.txt");
    fs::write(&file_path, "Initial content").unwrap();

    git_history.auto_commit_on_save(project_path, &file_path)?;

    fs::write(&file_path, "Updated content").unwrap();
    git_history.auto_commit_on_save(project_path, &file_path)?;

    let commits = git_history.list_commits(project_path)?;
    assert_eq!(commits.len(), 2);

    let first_commit_id = &commits[1].id;
    let content =
        git_history.get_file_content_at_commit(project_path, "test.txt", first_commit_id)?;
    assert_eq!(content, "Initial content");

    let second_commit_id = &commits[0].id;
    let content =
        git_history.get_file_content_at_commit(project_path, "test.txt", second_commit_id)?;
    assert_eq!(content, "Updated content");

    Ok(())
}

#[test]
fn test_restore_nonexistent_file() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let file_path = project_path.join("test.txt");
    fs::write(&file_path, "Content").unwrap();

    git_history.auto_commit_on_save(project_path, &file_path)?;

    let commits = git_history.list_commits(project_path)?;
    let commit_id = &commits[0].id;

    let result = git_history.restore_file(project_path, "nonexistent.txt", commit_id);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_restore_multiple_files_in_commit() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let file1_path = project_path.join("file1.txt");
    let file2_path = project_path.join("file2.txt");
    let file3_path = project_path.join("file3.txt");

    fs::write(&file1_path, "File1 V1").unwrap();
    fs::write(&file2_path, "File2 V1").unwrap();
    fs::write(&file3_path, "File3 V1").unwrap();

    git_history.auto_commit_on_save_multiple(
        project_path,
        &[
            &file1_path.to_path_buf(),
            &file2_path.to_path_buf(),
            &file3_path.to_path_buf(),
        ],
    )?;

    fs::write(&file1_path, "File1 V2").unwrap();
    fs::write(&file2_path, "File2 V2").unwrap();
    fs::write(&file3_path, "File3 V2").unwrap();

    git_history.auto_commit_on_save_multiple(
        project_path,
        &[
            &file1_path.to_path_buf(),
            &file2_path.to_path_buf(),
            &file3_path.to_path_buf(),
        ],
    )?;

    let commits = git_history.list_commits(project_path)?;
    let first_commit_id = &commits[1].id;

    git_history.restore_commit(project_path, first_commit_id)?;

    let repo_path = git_history.repo_path(project_path)?;

    let content = fs::read_to_string(repo_path.join("file1.txt")).unwrap();
    assert_eq!(content, "File1 V1");

    let content = fs::read_to_string(repo_path.join("file2.txt")).unwrap();
    assert_eq!(content, "File2 V1");

    let content = fs::read_to_string(repo_path.join("file3.txt")).unwrap();
    assert_eq!(content, "File3 V1");

    Ok(())
}

#[test]
fn test_restore_with_nested_directories() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();
    let storage_dir = TempDir::new().unwrap();

    let git_history = GitHistoryManager::with_storage_root(storage_dir.path().to_path_buf())?;

    let nested_dir = project_path.join("subdir");
    fs::create_dir_all(&nested_dir).unwrap();

    let file_path = nested_dir.join("nested.txt");
    fs::write(&file_path, "Nested V1").unwrap();

    git_history.auto_commit_on_save(project_path, &file_path)?;

    fs::write(&file_path, "Nested V2").unwrap();
    git_history.auto_commit_on_save(project_path, &file_path)?;

    let commits = git_history.list_commits(project_path)?;
    let first_commit_id = &commits[1].id;

    let content = git_history.get_file_content_at_commit(
        project_path,
        "subdir/nested.txt",
        first_commit_id,
    )?;
    assert_eq!(content, "Nested V1");

    Ok(())
}
