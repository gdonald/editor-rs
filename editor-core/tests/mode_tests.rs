use editor_core::{Command, EditorState};

#[test]
fn test_toggle_read_only() {
    let mut editor = EditorState::new();
    assert!(!editor.buffer().is_read_only());

    editor.execute_command(Command::ToggleReadOnly).unwrap();
    assert!(editor.buffer().is_read_only());

    editor.execute_command(Command::ToggleReadOnly).unwrap();
    assert!(!editor.buffer().is_read_only());
}

#[test]
fn test_read_only_edit_commands_fail() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::ToggleReadOnly).unwrap();

    // InsertChar
    let result = editor.execute_command(Command::InsertChar('a'));
    assert!(result.is_err());

    // DeleteChar
    // Note: buffer is empty initially, so delete might fail anyway on invalid position?
    // Let's insert first, then read only.
    editor.execute_command(Command::ToggleReadOnly).unwrap(); // off
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor.execute_command(Command::ToggleReadOnly).unwrap(); // on

    let result = editor.execute_command(Command::DeleteChar);
    assert!(result.is_err());

    // Backspace
    let result = editor.execute_command(Command::Backspace);
    assert!(result.is_err()); // Either ReadOnly error or just error

    // Paste
    // Paste does check read only inside insert_str equivalent?
    // `state::paste` -> `clipboard::get_text` -> `insert_text_at` -> `buffer::insert_str`
    // `buffer::insert_str` checks read only.
    let _result = editor.execute_command(Command::Paste);
    // Paste might succeed if clipboard is empty?
    // Mock clipboard is probably empty.
    // Let's mock clipboard content first.
    editor_core::ClipboardManager::enable_mock_clipboard();
    // But setting text needs read/write access? No, clipboard access is independent.
    // Just need to set it.
    // But `Command::Copy` works in read only mode? Yes.

    // Let's toggle off, set content, select, copy, toggle on, then paste.
    editor.execute_command(Command::ToggleReadOnly).unwrap(); // off
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();
    editor.execute_command(Command::Copy).unwrap(); // Copy 'a'
    editor.execute_command(Command::ToggleReadOnly).unwrap(); // on

    let result = editor.execute_command(Command::Paste);
    assert!(result.is_err());
}

#[test]
fn test_read_only_navigation_works() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::ToggleReadOnly).unwrap();

    let result = editor.execute_command(Command::MoveCursorLeft);
    assert!(result.is_ok());

    let result = editor.execute_command(Command::MoveCursorRight);
    assert!(result.is_ok());
}
