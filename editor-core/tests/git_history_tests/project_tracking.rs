use editor_core::{GitHistoryManager, TrackingMode};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_tracking_mode_single_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_single_file());
    assert!(!mode.is_project());
    assert_eq!(mode.path(), temp_dir.path().canonicalize().unwrap());
}

#[test]
fn test_detect_tracking_mode_project_with_git() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let git_dir = project_path.join(".git");
    fs::create_dir(&git_dir).unwrap();

    let file_path = project_path.join("test.txt");
    fs::write(&file_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert!(!mode.is_single_file());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}

#[test]
fn test_detect_tracking_mode_project_with_cargo_toml() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let cargo_toml = project_path.join("Cargo.toml");
    fs::write(&cargo_toml, "[package]").unwrap();

    let file_path = project_path.join("main.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}

#[test]
fn test_detect_tracking_mode_project_with_package_json() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let package_json = project_path.join("package.json");
    fs::write(&package_json, "{}").unwrap();

    let file_path = project_path.join("index.js");
    fs::write(&file_path, "console.log('hello');").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}

#[test]
fn test_detect_tracking_mode_nested_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let cargo_toml = project_path.join("Cargo.toml");
    fs::write(&cargo_toml, "[package]").unwrap();

    let src_dir = project_path.join("src");
    fs::create_dir(&src_dir).unwrap();

    let file_path = src_dir.join("lib.rs");
    fs::write(&file_path, "pub fn test() {}").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}

#[test]
fn test_get_tracking_path() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let cargo_toml = project_path.join("Cargo.toml");
    fs::write(&cargo_toml, "[package]").unwrap();

    let file_path = project_path.join("main.rs");
    fs::write(&file_path, "fn main() {}").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let tracking_path = manager.get_tracking_path(&file_path).unwrap();

    assert_eq!(tracking_path, project_path.canonicalize().unwrap());
}

#[test]
fn test_handle_file_move_within_project() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let cargo_toml = project_path.join("Cargo.toml");
    fs::write(&cargo_toml, "[package]").unwrap();

    let old_path = project_path.join("old.rs");
    fs::write(&old_path, "content").unwrap();

    let new_path = project_path.join("new.rs");
    fs::write(&new_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.handle_file_move(&old_path, &new_path).unwrap();

    assert!(result.is_none());
}

#[test]
fn test_handle_file_move_between_projects() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let project1 = temp_dir.path().join("project1");
    fs::create_dir(&project1).unwrap();
    fs::write(project1.join("Cargo.toml"), "[package]").unwrap();

    let project2 = temp_dir.path().join("project2");
    fs::create_dir(&project2).unwrap();
    fs::write(project2.join("Cargo.toml"), "[package]").unwrap();

    let old_path = project1.join("file.rs");
    fs::write(&old_path, "content").unwrap();

    let new_path = project2.join("file.rs");
    fs::write(&new_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.handle_file_move(&old_path, &new_path).unwrap();

    assert!(result.is_some());
    let (old_project, new_project) = result.unwrap();
    assert_eq!(old_project, project1.canonicalize().unwrap());
    assert_eq!(new_project, project2.canonicalize().unwrap());
}

#[test]
fn test_handle_project_rename() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let old_project = temp_dir.path().join("old_project");
    fs::create_dir(&old_project).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file = old_project.join("test.txt");
    fs::write(&file, "content").unwrap();
    manager.auto_commit_on_save(&old_project, &file).unwrap();

    let new_project = temp_dir.path().join("new_project");
    fs::create_dir(&new_project).unwrap();

    manager
        .handle_project_rename(&old_project, &new_project)
        .unwrap();

    let commits = manager.list_commits(&new_project).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_handle_project_rename_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let old_project = temp_dir.path().join("old_project");
    fs::create_dir(&old_project).unwrap();

    let new_project = temp_dir.path().join("new_project");
    fs::create_dir(&new_project).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager.handle_project_rename(&old_project, &new_project);
    assert!(result.is_ok());
}

#[test]
fn test_list_tracked_projects_empty() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let projects = manager.list_tracked_projects().unwrap();
    assert!(projects.is_empty());
}

#[test]
fn test_list_tracked_projects() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let project1 = temp_dir.path().join("project1");
    fs::create_dir(&project1).unwrap();

    let project2 = temp_dir.path().join("project2");
    fs::create_dir(&project2).unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let file1 = project1.join("test.txt");
    fs::write(&file1, "content1").unwrap();
    manager.auto_commit_on_save(&project1, &file1).unwrap();

    let file2 = project2.join("test.txt");
    fs::write(&file2, "content2").unwrap();
    manager.auto_commit_on_save(&project2, &file2).unwrap();

    let projects = manager.list_tracked_projects().unwrap();
    assert_eq!(projects.len(), 2);
    assert!(projects.contains(&project1.canonicalize().unwrap()));
    assert!(projects.contains(&project2.canonicalize().unwrap()));
}

#[test]
fn test_is_file_in_project_true() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let file_path = project_path.join("test.txt");
    fs::write(&file_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager
        .is_file_in_project(&file_path, &project_path)
        .unwrap();
    assert!(result);
}

#[test]
fn test_is_file_in_project_false() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");

    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let other_path = temp_dir.path().join("other");
    fs::create_dir(&other_path).unwrap();

    let file_path = other_path.join("test.txt");
    fs::write(&file_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager
        .is_file_in_project(&file_path, &project_path)
        .unwrap();
    assert!(!result);
}

#[test]
fn test_is_file_in_project_nested() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let src_dir = project_path.join("src");
    fs::create_dir(&src_dir).unwrap();

    let file_path = src_dir.join("lib.rs");
    fs::write(&file_path, "content").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let result = manager
        .is_file_in_project(&file_path, &project_path)
        .unwrap();
    assert!(result);
}

#[test]
fn test_tracking_mode_path_accessor() {
    let temp_dir = TempDir::new().unwrap();
    let project_path = temp_dir.path().to_path_buf();

    let mode_project = TrackingMode::Project(project_path.clone());
    assert_eq!(mode_project.path(), project_path);

    let mode_file = TrackingMode::SingleFile(project_path.clone());
    assert_eq!(mode_file.path(), project_path);
}

#[test]
fn test_detect_tracking_mode_project_with_pyproject_toml() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let pyproject_toml = project_path.join("pyproject.toml");
    fs::write(&pyproject_toml, "[project]").unwrap();

    let file_path = project_path.join("main.py");
    fs::write(&file_path, "print('hello')").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}

#[test]
fn test_detect_tracking_mode_project_with_go_mod() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let go_mod = project_path.join("go.mod");
    fs::write(&go_mod, "module example").unwrap();

    let file_path = project_path.join("main.go");
    fs::write(&file_path, "package main").unwrap();

    let manager = GitHistoryManager::with_storage_root(storage_root).unwrap();

    let mode = manager.detect_tracking_mode(&file_path).unwrap();

    assert!(mode.is_project());
    assert_eq!(mode.path(), project_path.canonicalize().unwrap());
}
