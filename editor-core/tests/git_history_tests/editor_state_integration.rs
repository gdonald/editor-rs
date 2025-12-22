use editor_core::{Command, EditorState, LargeFileConfig, LargeFileStrategy};
use std::fs;
use tempfile::TempDir;

#[test]
fn test_editor_state_default_large_file_config() {
    let state = EditorState::new();

    assert_eq!(state.large_file_config().threshold_mb, 50);
    assert_eq!(state.large_file_config().strategy, LargeFileStrategy::Warn);
    assert!(!state.large_file_config().exclude_from_history);
}

#[test]
fn test_editor_state_set_large_file_config() {
    let mut state = EditorState::new();

    let config = LargeFileConfig {
        threshold_mb: 100,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: true,
    };

    state.set_large_file_config(config.clone());

    assert_eq!(state.large_file_config().threshold_mb, 100);
    assert_eq!(state.large_file_config().strategy, LargeFileStrategy::Skip);
    assert!(state.large_file_config().exclude_from_history);
}

#[test]
fn test_editor_state_save_with_warn_strategy() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "Hello, world!").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Warn,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state
        .current_buffer_mut()
        .insert_str(0, 0, "Modified ")
        .unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert!(state.status_message().contains("File saved"));
}

#[test]
fn test_editor_state_save_large_file_with_skip_strategy() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");

    let large_content = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&file_path, large_content).unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state.current_buffer_mut().insert_str(0, 0, "X").unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert!(state.status_message().contains("skipped from history"));
}

#[test]
fn test_editor_state_save_large_file_with_error_strategy() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large.txt");

    let large_content = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&file_path, large_content).unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state.current_buffer_mut().insert_str(0, 0, "X").unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert!(state.status_message().contains("git history error"));
}

#[test]
fn test_editor_state_save_small_file_under_threshold() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("small.txt");

    fs::write(&file_path, "small content").unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    let config = LargeFileConfig {
        threshold_mb: 10,
        strategy: LargeFileStrategy::Skip,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state
        .current_buffer_mut()
        .insert_str(0, 0, "Modified ")
        .unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert_eq!(state.status_message(), "File saved");
}

#[test]
fn test_editor_state_save_with_auto_commit_disabled() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    let large_content = vec![b'A'; 2 * 1024 * 1024];
    fs::write(&file_path, large_content).unwrap();

    let mut state = EditorState::from_file(file_path.clone()).unwrap();

    state.set_auto_commit_enabled(false);

    let config = LargeFileConfig {
        threshold_mb: 1,
        strategy: LargeFileStrategy::Error,
        exclude_from_history: false,
    };
    state.set_large_file_config(config);

    state.current_buffer_mut().insert_str(0, 0, "X").unwrap();

    let result = state.execute_command(Command::Save);
    assert!(result.is_ok());
    assert_eq!(state.status_message(), "File saved");
}

#[test]
fn test_editor_state_from_file_inherits_config() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");

    fs::write(&file_path, "test").unwrap();

    let state = EditorState::from_file(file_path.clone()).unwrap();

    assert_eq!(state.large_file_config().threshold_mb, 50);
    assert_eq!(state.large_file_config().strategy, LargeFileStrategy::Warn);
}
