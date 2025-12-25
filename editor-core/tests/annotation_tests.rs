use editor_core::GitHistoryManager;
use std::fs;
use tempfile::TempDir;

fn create_test_repo() -> (TempDir, GitHistoryManager) {
    let temp_dir = TempDir::new().unwrap();
    let git_history = GitHistoryManager::new().unwrap();

    let test_file = temp_dir.path().join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    git_history
        .auto_commit_on_save(temp_dir.path(), &test_file)
        .unwrap();

    (temp_dir, git_history)
}

#[test]
fn test_add_annotation() {
    let (temp_dir, git_history) = create_test_repo();
    let project_path = temp_dir.path();

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(commits.len(), 1);
    let commit_id = &commits[0].id;

    assert!(commits[0].annotation.is_none());

    git_history
        .add_annotation(project_path, commit_id, "Test annotation".to_string())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(commits[0].annotation, Some("Test annotation".to_string()));
}

#[test]
fn test_remove_annotation() {
    let (temp_dir, git_history) = create_test_repo();
    let project_path = temp_dir.path();

    let commits = git_history.list_commits(project_path).unwrap();
    let commit_id = &commits[0].id;

    git_history
        .add_annotation(project_path, commit_id, "Test annotation".to_string())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(commits[0].annotation, Some("Test annotation".to_string()));

    git_history
        .remove_annotation(project_path, commit_id)
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert!(commits[0].annotation.is_none());
}

#[test]
fn test_update_annotation() {
    let (temp_dir, git_history) = create_test_repo();
    let project_path = temp_dir.path();

    let commits = git_history.list_commits(project_path).unwrap();
    let commit_id = &commits[0].id;

    git_history
        .add_annotation(project_path, commit_id, "First annotation".to_string())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(commits[0].annotation, Some("First annotation".to_string()));

    git_history
        .add_annotation(project_path, commit_id, "Updated annotation".to_string())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(
        commits[0].annotation,
        Some("Updated annotation".to_string())
    );
}

#[test]
fn test_get_annotation() {
    let (temp_dir, git_history) = create_test_repo();
    let project_path = temp_dir.path();

    let commits = git_history.list_commits(project_path).unwrap();
    let commit_id = &commits[0].id;

    let annotation = git_history.get_annotation(project_path, commit_id).unwrap();
    assert!(annotation.is_none());

    git_history
        .add_annotation(project_path, commit_id, "Test annotation".to_string())
        .unwrap();

    let annotation = git_history.get_annotation(project_path, commit_id).unwrap();
    assert_eq!(annotation, Some("Test annotation".to_string()));
}

#[test]
fn test_annotations_persist_across_sessions() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path();

    let test_file = project_path.join("test.txt");
    fs::write(&test_file, "initial content").unwrap();

    let git_history1 = GitHistoryManager::new().unwrap();
    git_history1
        .auto_commit_on_save(temp_dir.path(), &test_file)
        .unwrap();

    let commits = git_history1.list_commits(project_path).unwrap();
    let commit_id = commits[0].id.clone();

    git_history1
        .add_annotation(
            project_path,
            &commit_id,
            "Persistent annotation".to_string(),
        )
        .unwrap();

    drop(git_history1);

    let git_history2 = GitHistoryManager::new().unwrap();
    let commits = git_history2.list_commits(project_path).unwrap();
    assert_eq!(
        commits[0].annotation,
        Some("Persistent annotation".to_string())
    );
}

#[test]
fn test_multiple_annotations() {
    let temp_dir = TempDir::new().unwrap();
    let git_history = GitHistoryManager::new().unwrap();
    let project_path = temp_dir.path();

    let test_file = project_path.join("test.txt");

    for i in 1..=3 {
        fs::write(&test_file, format!("content {}", i)).unwrap();
        git_history
            .auto_commit_on_save(temp_dir.path(), &test_file)
            .unwrap();
    }

    let commits = git_history.list_commits(project_path).unwrap();
    assert_eq!(commits.len(), 3);

    for (i, commit) in commits.iter().enumerate() {
        git_history
            .add_annotation(
                project_path,
                &commit.id,
                format!("Annotation for commit {}", i + 1),
            )
            .unwrap();
    }

    let commits = git_history.list_commits(project_path).unwrap();
    for (i, commit) in commits.iter().enumerate() {
        assert_eq!(
            commit.annotation,
            Some(format!("Annotation for commit {}", i + 1))
        );
    }
}

#[test]
fn test_empty_annotation_removes() {
    let (temp_dir, git_history) = create_test_repo();
    let project_path = temp_dir.path();

    let commits = git_history.list_commits(project_path).unwrap();
    let commit_id = &commits[0].id;

    git_history
        .add_annotation(project_path, commit_id, "Test annotation".to_string())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert!(commits[0].annotation.is_some());

    git_history
        .add_annotation(project_path, commit_id, String::new())
        .unwrap();

    let commits = git_history.list_commits(project_path).unwrap();
    assert!(commits[0].annotation.is_none());
}

#[test]
fn test_annotation_count() {
    let temp_dir = TempDir::new().unwrap();
    let git_history = GitHistoryManager::new().unwrap();
    let project_path = temp_dir.path();

    let test_file = project_path.join("test.txt");

    for i in 1..=5 {
        fs::write(&test_file, format!("content {}", i)).unwrap();
        git_history
            .auto_commit_on_save(temp_dir.path(), &test_file)
            .unwrap();
    }

    let commits = git_history.list_commits(project_path).unwrap();

    git_history
        .add_annotation(project_path, &commits[0].id, "Note 1".to_string())
        .unwrap();
    git_history
        .add_annotation(project_path, &commits[2].id, "Note 2".to_string())
        .unwrap();
    git_history
        .add_annotation(project_path, &commits[4].id, "Note 3".to_string())
        .unwrap();

    let annotations = git_history.load_annotations(project_path).unwrap();
    assert_eq!(annotations.count(), 3);
}
