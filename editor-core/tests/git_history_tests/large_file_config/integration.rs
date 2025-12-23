use editor_core::{Command, EditorState, GitHistoryManager, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_full_workflow_with_all_strategies() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "initial content").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    state
        .current_buffer_mut()
        .insert_str(0, 0, "prefix ")
        .unwrap();
    state.execute_command(Command::Save).unwrap();
    assert_eq!(state.status_message(), "File saved");

    let config_warn = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    state.set_large_file_config(config_warn);

    state
        .current_buffer_mut()
        .insert_str(0, 7, "more ")
        .unwrap();
    state.execute_command(Command::Save).unwrap();
    assert!(state.status_message().contains("File saved"));

    let config_skip = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };
    state.set_large_file_config(config_skip);

    state
        .current_buffer_mut()
        .insert_str(0, 12, "text ")
        .unwrap();
    state.execute_command(Command::Save).unwrap();
    assert!(state.status_message().contains("File saved"));
}

#[test]
fn test_full_workflow_large_file_progression() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("growing.txt");

    fs::write(&file_path, "small").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state
        .current_buffer_mut()
        .insert_str(0, 0, "prefix ")
        .unwrap();
    state.execute_command(Command::Save).unwrap();
    assert!(state.status_message().contains("File saved"));

    let large_content = "A".repeat(2 * 1024 * 1024);
    state
        .current_buffer_mut()
        .insert_str(0, 7, &large_content)
        .unwrap();
    state.execute_command(Command::Save).unwrap();
    assert!(state.status_message().contains("File saved"));
}

#[test]
fn test_full_workflow_skip_strategy_with_manager() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small content").unwrap();
    manager
        .auto_commit_on_save(&project_path, &small_file)
        .unwrap();

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();
    manager
        .auto_commit_on_save(&project_path, &large_file)
        .unwrap();

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_full_workflow_error_strategy_with_manager() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small").unwrap();
    let result = manager.auto_commit_on_save(&project_path, &small_file);
    assert!(result.is_ok());

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'L'; 2 * 1024 * 1024];
    fs::write(&large_file, large_content).unwrap();
    let result = manager.auto_commit_on_save(&project_path, &large_file);
    assert!(result.is_err());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_full_workflow_config_changes_mid_session() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config_warn = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let mut manager = GitHistoryManager::with_storage_root(storage_root.clone())
        .unwrap()
        .with_large_file_config(config_warn);

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'X'; 2 * 1024 * 1024];
    fs::write(&large_file, &large_content).unwrap();

    let result = manager.auto_commit_on_save(&project_path, &large_file);
    assert!(result.is_ok());

    let config_error = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };
    manager = manager.with_large_file_config(config_error);

    fs::write(&large_file, &large_content).unwrap();
    let result = manager.auto_commit_on_save(&project_path, &large_file);
    assert!(result.is_err());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_full_workflow_multiple_files_mixed_sizes() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let small1 = project_path.join("small1.txt");
    fs::write(&small1, "small 1").unwrap();

    let small2 = project_path.join("small2.txt");
    fs::write(&small2, "small 2").unwrap();

    let large1 = project_path.join("large1.txt");
    let large_content1 = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&large1, large_content1).unwrap();

    let large2 = project_path.join("large2.txt");
    let large_content2 = vec![b'B'; 3 * 1024 * 1024];
    fs::write(&large2, large_content2).unwrap();

    let files = vec![&small1, &large1, &small2, &large2];
    let result = manager.auto_commit_on_save_multiple(&project_path, &files);
    assert!(result.is_ok());

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_full_workflow_exclude_from_history_flag() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config_no_exclude = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root.clone())
        .unwrap()
        .with_large_file_config(config_no_exclude);

    let small_file = project_path.join("small.txt");
    fs::write(&small_file, "small content").unwrap();

    manager
        .auto_commit_on_save(&project_path, &small_file)
        .unwrap();

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);

    let config_exclude = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: true,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config_exclude);

    let large_file = project_path.join("large.txt");
    let large_content = vec![b'X'; 2 * 1024 * 1024];
    fs::write(&large_file, &large_content).unwrap();
    manager
        .auto_commit_on_save(&project_path, &large_file)
        .unwrap();

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 1);
}

#[test]
fn test_full_workflow_threshold_boundary_conditions() {
    let temp_dir = TempDir::new().unwrap();
    let storage_root = temp_dir.path().join("storage");
    let project_path = temp_dir.path().join("project");
    fs::create_dir(&project_path).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 2,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };

    let manager = GitHistoryManager::with_storage_root(storage_root)
        .unwrap()
        .with_large_file_config(config);

    let just_under = project_path.join("just_under.txt");
    let just_under_content = vec![b'U'; (2 * 1024 * 1024) - 1];
    fs::write(&just_under, just_under_content).unwrap();
    manager
        .auto_commit_on_save(&project_path, &just_under)
        .unwrap();

    let exactly_at = project_path.join("exactly_at.txt");
    let exactly_at_content = vec![b'E'; 2 * 1024 * 1024];
    fs::write(&exactly_at, exactly_at_content).unwrap();
    manager
        .auto_commit_on_save(&project_path, &exactly_at)
        .unwrap();

    let just_over = project_path.join("just_over.txt");
    let just_over_content = vec![b'O'; (2 * 1024 * 1024) + 1];
    fs::write(&just_over, just_over_content).unwrap();
    manager
        .auto_commit_on_save(&project_path, &just_over)
        .unwrap();

    let commits = manager.list_commits(&project_path).unwrap();
    assert_eq!(commits.len(), 2);
}

#[test]
fn test_full_workflow_auto_commit_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "content").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();
    state.set_auto_commit_enabled(false);

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    let large_content = "X".repeat(2 * 1024 * 1024);
    state
        .current_buffer_mut()
        .insert_str(0, 0, &large_content)
        .unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert_eq!(state.status_message(), "File saved");
}

#[test]
fn test_full_workflow_sequential_saves() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("sequential.txt");

    fs::write(&file_path, "v1").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 5,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    for i in 1..=5 {
        state
            .current_buffer_mut()
            .insert_str(0, 0, &format!("v{} ", i + 1))
            .unwrap();
        state.execute_command(Command::Save).unwrap();
        assert!(state.status_message().contains("File saved"));
    }
}
