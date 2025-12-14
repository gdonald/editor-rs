use editor_core::{Command, CursorPosition, EditorState};

#[test]
fn add_and_remove_cursors() {
    let mut editor = EditorState::new();
    for ch in "abcd".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    editor
        .execute_command(Command::AddCursor(CursorPosition::new(0, 2)))
        .unwrap();
    assert_eq!(editor.cursor_count(), 2);

    editor.execute_command(Command::RemoveCursor(1)).unwrap();
    assert_eq!(editor.cursor_count(), 1);
    assert_eq!(editor.cursor(), &CursorPosition::new(0, 0));
}

#[test]
fn multi_cursor_insert_inserts_at_all_positions() {
    let mut editor = EditorState::new();
    for ch in "abcd".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(0, 2)))
        .unwrap();

    editor.execute_command(Command::InsertChar('X')).unwrap();

    assert_eq!(editor.current_buffer().content(), "XabXcd");
    let positions: Vec<_> = editor.cursors().to_vec();
    assert_eq!(
        positions,
        vec![CursorPosition::new(0, 1), CursorPosition::new(0, 3)]
    );
}

#[test]
fn multi_cursor_backspace_updates_each_cursor() {
    let mut editor = EditorState::new();
    for ch in "ab\ncd".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::GotoLine(0)).unwrap();
    editor.execute_command(Command::MoveToEndOfLine).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 2)))
        .unwrap();

    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.current_buffer().content(), "a\nc");
    let positions: Vec<_> = editor.cursors().to_vec();
    assert_eq!(
        positions,
        vec![CursorPosition::new(0, 1), CursorPosition::new(1, 1)]
    );
}

#[test]
fn multi_cursor_commands_deduplicate_and_clear() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::zero()))
        .unwrap();
    assert_eq!(editor.cursor_count(), 1);

    editor
        .execute_command(Command::AddCursor(CursorPosition::new(0, 0)))
        .unwrap();
    assert_eq!(editor.cursor_count(), 1);

    editor
        .execute_command(Command::AddCursor(CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::ClearSecondaryCursors)
        .unwrap();
    assert_eq!(editor.cursor_count(), 1);
}

#[test]
fn add_cursor_rejects_invalid_positions() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::AddCursor(CursorPosition::new(5, 0)));
    assert!(result.is_err());
}
