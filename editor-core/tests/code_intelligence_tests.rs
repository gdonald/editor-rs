use editor_core::{Command, EditorState};

#[test]
fn test_auto_close_parentheses() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "()");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_square_brackets() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('['))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "[]");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_curly_braces() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('{'))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "{}");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_double_quotes() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('"'))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "\"\"");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_single_quotes() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('\''))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "''");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_no_close_when_char_after() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "(a");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_with_whitespace_after() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "() ");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_at_end_of_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('f')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "foo()");
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_auto_close_nested_brackets() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('['))
        .unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('{'))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "([{}])");
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_auto_close_multi_cursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('b')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('c')).unwrap();

    editor.execute_command(Command::MoveToEndOfLine).unwrap();
    editor
        .execute_command(Command::AddCursor(editor_core::CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::AddCursor(editor_core::CursorPosition::new(1, 1)))
        .unwrap();

    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "a()\nb()\nc()");
    let cursors = editor.cursors();
    assert_eq!(cursors.len(), 3);
}

#[test]
fn test_auto_close_non_bracket_char() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::InsertCharWithAutoClose('x'))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "x");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_auto_close_with_closing_bracket_after() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar(')')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor
        .execute_command(Command::InsertCharWithAutoClose('('))
        .unwrap();

    assert_eq!(editor.current_buffer().content(), "())");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_toggle_line_comment() {
    let mut editor = EditorState::new();
    // Simulate some code
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('b')).unwrap();

    // reset cursor
    editor.execute_command(Command::MoveToStartOfFile).unwrap();

    // Toggle comment on first line
    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "// a\nb");

    // Toggle comment again (uncomment)
    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "a\nb");
}

#[test]
fn test_toggle_line_comment_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('b')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveToEndOfFile).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "// a\n// b");

    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "a\nb");
}

#[test]
fn test_toggle_line_comment_different_indentation() {
    // This tests that we simply prepend at 0 for now based on implementation
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar('a')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "//  a");

    editor.execute_command(Command::ToggleLineComment).unwrap();
    assert_eq!(editor.current_buffer().content(), " a");
}

#[test]
fn test_toggle_block_comment() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveToEndOfLine).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    editor.execute_command(Command::ToggleBlockComment).unwrap();
    assert_eq!(editor.current_buffer().content(), "/*a*/");
}
