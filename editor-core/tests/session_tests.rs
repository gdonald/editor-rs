use editor_core::{CursorPosition, EditorState, OpenFileState, Session, SessionManager};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_session_new() {
    let session = Session::new();
    assert_eq!(session.open_files.len(), 0);
    assert_eq!(session.recent_files.len(), 0);
    assert_eq!(session.recent_files_limit, 20);
}

#[test]
fn test_session_default() {
    let session = Session::default();
    assert_eq!(session.open_files.len(), 0);
    assert_eq!(session.recent_files.len(), 0);
}

#[test]
fn test_add_open_file() {
    let mut session = Session::new();
    let path = PathBuf::from("/tmp/test.txt");
    let cursor = CursorPosition::new(5, 10);

    session.add_open_file(path.clone(), cursor, 0);

    assert_eq!(session.open_files.len(), 1);
    assert_eq!(session.open_files[0].path, path);
    assert_eq!(session.open_files[0].cursor_line, 5);
    assert_eq!(session.open_files[0].cursor_column, 10);
    assert_eq!(session.open_files[0].viewport_top, 0);
}

#[test]
fn test_add_open_file_updates_existing() {
    let mut session = Session::new();
    let path = PathBuf::from("/tmp/test.txt");

    session.add_open_file(path.clone(), CursorPosition::new(1, 1), 0);
    session.add_open_file(path.clone(), CursorPosition::new(5, 10), 3);

    assert_eq!(session.open_files.len(), 1);
    assert_eq!(session.open_files[0].cursor_line, 5);
    assert_eq!(session.open_files[0].cursor_column, 10);
    assert_eq!(session.open_files[0].viewport_top, 3);
}

#[test]
fn test_remove_open_file() {
    let mut session = Session::new();
    let path1 = PathBuf::from("/tmp/test1.txt");
    let path2 = PathBuf::from("/tmp/test2.txt");

    session.add_open_file(path1.clone(), CursorPosition::new(0, 0), 0);
    session.add_open_file(path2.clone(), CursorPosition::new(0, 0), 0);

    assert_eq!(session.open_files.len(), 2);

    session.remove_open_file(&path1);
    assert_eq!(session.open_files.len(), 1);
    assert_eq!(session.open_files[0].path, path2);
}

#[test]
fn test_set_active_file() {
    let mut session = Session::new();
    let path1 = PathBuf::from("/tmp/test1.txt");
    let path2 = PathBuf::from("/tmp/test2.txt");

    session.add_open_file(path1.clone(), CursorPosition::new(0, 0), 0);
    session.add_open_file(path2.clone(), CursorPosition::new(0, 0), 0);

    session.set_active_file(&path2);

    assert!(!session.open_files[0].active);
    assert!(session.open_files[1].active);
}

#[test]
fn test_update_file_state() {
    let mut session = Session::new();
    let path = PathBuf::from("/tmp/test.txt");

    session.add_open_file(path.clone(), CursorPosition::new(0, 0), 0);
    session.update_file_state(&path, CursorPosition::new(10, 20), 5);

    assert_eq!(session.open_files[0].cursor_line, 10);
    assert_eq!(session.open_files[0].cursor_column, 20);
    assert_eq!(session.open_files[0].viewport_top, 5);
}

#[test]
fn test_add_to_recent_files() {
    let mut session = Session::new();
    let path1 = PathBuf::from("/tmp/test1.txt");
    let path2 = PathBuf::from("/tmp/test2.txt");

    session.add_to_recent_files(path1.clone());
    session.add_to_recent_files(path2.clone());

    let recent = session.get_recent_files();
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0], path2);
    assert_eq!(recent[1], path1);
}

#[test]
fn test_recent_files_removes_duplicates() {
    let mut session = Session::new();
    let path = PathBuf::from("/tmp/test.txt");

    session.add_to_recent_files(path.clone());
    session.add_to_recent_files(PathBuf::from("/tmp/other.txt"));
    session.add_to_recent_files(path.clone());

    let recent = session.get_recent_files();
    assert_eq!(recent.len(), 2);
    assert_eq!(recent[0], path);
}

#[test]
fn test_recent_files_limit() {
    let mut session = Session::new();
    session.set_recent_files_limit(3);

    for i in 0..5 {
        session.add_to_recent_files(PathBuf::from(format!("/tmp/test{}.txt", i)));
    }

    let recent = session.get_recent_files();
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0], PathBuf::from("/tmp/test4.txt"));
    assert_eq!(recent[1], PathBuf::from("/tmp/test3.txt"));
    assert_eq!(recent[2], PathBuf::from("/tmp/test2.txt"));
}

#[test]
fn test_set_recent_files_limit_truncates() {
    let mut session = Session::new();

    for i in 0..10 {
        session.add_to_recent_files(PathBuf::from(format!("/tmp/test{}.txt", i)));
    }

    assert_eq!(session.get_recent_files().len(), 10);

    session.set_recent_files_limit(5);
    assert_eq!(session.get_recent_files().len(), 5);
}

#[test]
fn test_get_active_file() {
    let mut session = Session::new();
    let path1 = PathBuf::from("/tmp/test1.txt");
    let path2 = PathBuf::from("/tmp/test2.txt");

    session.add_open_file(path1.clone(), CursorPosition::new(0, 0), 0);
    session.add_open_file(path2.clone(), CursorPosition::new(0, 0), 0);
    session.set_active_file(&path2);

    let active = session.get_active_file();
    assert!(active.is_some());
    assert_eq!(active.unwrap().path, path2);
}

#[test]
fn test_get_active_file_none() {
    let session = Session::new();
    assert!(session.get_active_file().is_none());
}

#[test]
fn test_session_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("session.toml");

    let mut session = Session::new();
    session.add_open_file(
        temp_dir.path().join("test.txt"),
        CursorPosition::new(5, 10),
        2,
    );
    session.add_to_recent_files(temp_dir.path().join("recent.txt"));

    fs::write(temp_dir.path().join("test.txt"), "test content").unwrap();
    fs::write(temp_dir.path().join("recent.txt"), "recent content").unwrap();

    session.save_to_file(&session_path).unwrap();
    assert!(session_path.exists());

    let loaded = Session::load_from_file(&session_path).unwrap();
    assert_eq!(loaded.open_files.len(), 1);
    assert_eq!(loaded.recent_files.len(), 1);
}

#[test]
fn test_session_load_filters_nonexistent_files() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("session.toml");

    let mut session = Session::new();
    session.add_open_file(
        temp_dir.path().join("exists.txt"),
        CursorPosition::new(0, 0),
        0,
    );
    session.add_open_file(
        temp_dir.path().join("missing.txt"),
        CursorPosition::new(0, 0),
        0,
    );
    session.add_to_recent_files(temp_dir.path().join("exists.txt"));
    session.add_to_recent_files(temp_dir.path().join("missing.txt"));

    fs::write(temp_dir.path().join("exists.txt"), "content").unwrap();

    session.save_to_file(&session_path).unwrap();

    let loaded = Session::load_from_file(&session_path).unwrap();
    assert_eq!(loaded.open_files.len(), 1);
    assert_eq!(loaded.open_files[0].path.file_name().unwrap(), "exists.txt");
    assert_eq!(loaded.recent_files.len(), 1);
}

#[test]
fn test_session_manager_new() {
    let manager = SessionManager::new().unwrap();
    assert!(manager
        .session_path()
        .to_str()
        .unwrap()
        .contains("session-"));
    assert!(manager.session_path().to_str().unwrap().ends_with(".toml"));
}

#[test]
fn test_session_manager_with_custom_path() {
    let custom_path = PathBuf::from("/tmp/custom-session.toml");
    let manager = SessionManager::with_custom_path(custom_path.clone());
    assert_eq!(manager.session_path(), custom_path);
}

#[test]
fn test_session_manager_save_and_load() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("session.toml");
    let manager = SessionManager::with_custom_path(session_path.clone());

    let mut session = Session::new();
    session.add_to_recent_files(PathBuf::from("/tmp/test.txt"));

    manager.save_session(&session).unwrap();
    assert!(session_path.exists());

    let loaded = manager.load_session().unwrap();
    assert_eq!(loaded.recent_files.len(), 0);
}

#[test]
fn test_session_manager_load_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("nonexistent.toml");
    let manager = SessionManager::with_custom_path(session_path);

    let session = manager.load_session().unwrap();
    assert_eq!(session.open_files.len(), 0);
    assert_eq!(session.recent_files.len(), 0);
}

#[test]
fn test_session_manager_delete_session() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("session.toml");
    let manager = SessionManager::with_custom_path(session_path.clone());

    let session = Session::new();
    manager.save_session(&session).unwrap();
    assert!(session_path.exists());

    manager.delete_session().unwrap();
    assert!(!session_path.exists());
}

#[test]
fn test_session_manager_delete_nonexistent() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("nonexistent.toml");
    let manager = SessionManager::with_custom_path(session_path);

    let result = manager.delete_session();
    assert!(result.is_ok());
}

#[test]
fn test_session_manager_cleanup_stale_sessions() {
    let temp_dir = TempDir::new().unwrap();
    std::env::set_var("HOME", temp_dir.path());

    let session_dir = temp_dir.path().join(".config/editor-rs");
    fs::create_dir_all(&session_dir).unwrap();

    let old_session = session_dir.join("session-12345.toml");
    fs::write(&old_session, "").unwrap();

    let result = SessionManager::cleanup_stale_sessions(0);
    assert!(result.is_ok());
}

#[test]
fn test_open_file_state_equality() {
    let state1 = OpenFileState {
        path: PathBuf::from("/tmp/test.txt"),
        cursor_line: 5,
        cursor_column: 10,
        viewport_top: 2,
        active: true,
    };

    let state2 = OpenFileState {
        path: PathBuf::from("/tmp/test.txt"),
        cursor_line: 5,
        cursor_column: 10,
        viewport_top: 2,
        active: true,
    };

    assert_eq!(state1, state2);
}

#[test]
fn test_editor_state_capture_file_state() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Line 1\nLine 2\nLine 3\n").unwrap();

    let mut editor = EditorState::from_file(file_path.clone()).unwrap();
    editor
        .execute_command(editor_core::Command::GotoLine(1))
        .unwrap();

    let state = editor.capture_file_state();
    assert!(state.is_some());

    let state = state.unwrap();
    assert_eq!(state.path, file_path);
    assert_eq!(state.cursor_line, 1);
    assert_eq!(state.cursor_column, 0);
    assert!(state.active);
}

#[test]
fn test_editor_state_capture_file_state_no_file() {
    let editor = EditorState::new();
    let state = editor.capture_file_state();
    assert!(state.is_none());
}

#[test]
fn test_editor_state_restore_from_file_state() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\n").unwrap();

    let file_state = OpenFileState {
        path: file_path.clone(),
        cursor_line: 2,
        cursor_column: 3,
        viewport_top: 1,
        active: true,
    };

    let mut editor = EditorState::new();
    editor.restore_from_file_state(&file_state).unwrap();

    assert_eq!(editor.cursor().line, 2);
    assert_eq!(editor.cursor().column, 3);
    assert_eq!(editor.viewport_top(), 1);
    assert_eq!(editor.file_path().unwrap(), file_path);
}

#[test]
fn test_editor_state_restore_clamps_invalid_cursor() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Short\n").unwrap();

    let file_state = OpenFileState {
        path: file_path.clone(),
        cursor_line: 0,
        cursor_column: 100,
        viewport_top: 0,
        active: true,
    };

    let mut editor = EditorState::new();
    editor.restore_from_file_state(&file_state).unwrap();

    assert_eq!(editor.cursor().line, 0);
    assert!(editor.cursor().column <= 5);
}

#[test]
fn test_editor_state_restore_clamps_invalid_viewport() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Line 1\nLine 2\n").unwrap();

    let file_state = OpenFileState {
        path: file_path.clone(),
        cursor_line: 0,
        cursor_column: 0,
        viewport_top: 100,
        active: true,
    };

    let mut editor = EditorState::new();
    editor.restore_from_file_state(&file_state).unwrap();

    assert!(editor.viewport_top() < 100);
}

#[test]
fn test_editor_state_save_session_state() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "Line 1\nLine 2\nLine 3\n").unwrap();

    let mut editor = EditorState::from_file(file_path.clone()).unwrap();
    editor
        .execute_command(editor_core::Command::GotoLine(1))
        .unwrap();

    let mut session = Session::new();
    editor.save_session_state(&mut session);

    assert_eq!(session.open_files.len(), 1);
    assert_eq!(session.open_files[0].path, file_path);
    assert_eq!(session.open_files[0].cursor_line, 1);
    assert!(session.open_files[0].active);

    let recent = session.get_recent_files();
    assert_eq!(recent.len(), 1);
    assert_eq!(recent[0], file_path);
}

#[test]
fn test_editor_state_save_session_state_no_file() {
    let editor = EditorState::new();
    let mut session = Session::new();

    editor.save_session_state(&mut session);

    assert_eq!(session.open_files.len(), 0);
    assert_eq!(session.recent_files.len(), 0);
}

#[test]
fn test_session_round_trip_with_multiple_files() {
    let temp_dir = TempDir::new().unwrap();
    let session_path = temp_dir.path().join("session.toml");

    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    fs::write(&file1, "Content 1").unwrap();
    fs::write(&file2, "Content 2").unwrap();

    let mut session = Session::new();
    session.add_open_file(file1.clone(), CursorPosition::new(0, 0), 0);
    session.add_open_file(file2.clone(), CursorPosition::new(1, 5), 2);
    session.set_active_file(&file2);
    session.add_to_recent_files(file1.clone());
    session.add_to_recent_files(file2.clone());

    session.save_to_file(&session_path).unwrap();

    let loaded = Session::load_from_file(&session_path).unwrap();

    assert_eq!(loaded.open_files.len(), 2);
    assert_eq!(loaded.recent_files.len(), 2);

    let active = loaded.get_active_file().unwrap();
    assert_eq!(active.path, file2);
    assert_eq!(active.cursor_line, 1);
    assert_eq!(active.cursor_column, 5);
    assert_eq!(active.viewport_top, 2);
}

#[test]
fn test_session_timestamps_update() {
    let mut session = Session::new();
    let created = session.created_at;

    std::thread::sleep(std::time::Duration::from_millis(10));

    session.add_open_file(PathBuf::from("/tmp/test.txt"), CursorPosition::new(0, 0), 0);

    assert!(session.last_accessed > created);
}

#[test]
fn test_editor_state_file_path() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "content").unwrap();

    let editor = EditorState::from_file(file_path.clone()).unwrap();
    assert_eq!(editor.file_path(), Some(file_path.as_path()));
}

#[test]
fn test_editor_state_file_path_none() {
    let editor = EditorState::new();
    assert_eq!(editor.file_path(), None);
}
