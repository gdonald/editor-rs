use editor_core::{Command, CursorPosition, EditorState};

#[test]
fn test_editor_new() {
    let editor = EditorState::new();
    assert_eq!(editor.cursor(), &CursorPosition::zero());
    assert_eq!(editor.buffer().line_count(), 1);
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_editor_insert_char() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();

    assert_eq!(editor.buffer().content(), "Hi");
    assert_eq!(editor.cursor().column, 2);
}

#[test]
fn test_editor_delete_char() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor.execute_command(Command::DeleteChar).unwrap();

    assert_eq!(editor.buffer().content(), "H");
}

#[test]
fn test_editor_backspace() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.buffer().content(), "H");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_backspace_at_start() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.buffer().content(), "");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}

#[test]
fn test_editor_new_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('W')).unwrap();

    assert_eq!(editor.buffer().content(), "H\nW");
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_backspace_across_lines() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('W')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.buffer().content(), "HW");
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_delete_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('1')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('2')).unwrap();
    editor.execute_command(Command::MoveCursorUp).unwrap();
    editor.execute_command(Command::DeleteLine).unwrap();

    assert_eq!(editor.buffer().content(), "L2");
}

#[test]
fn test_editor_move_cursor_up() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    assert_eq!(editor.cursor().line, 1);

    editor.execute_command(Command::MoveCursorUp).unwrap();
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_move_cursor_down() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::MoveCursorUp).unwrap();

    assert_eq!(editor.cursor().line, 0);

    editor.execute_command(Command::MoveCursorDown).unwrap();
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_editor_move_cursor_left() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    assert_eq!(editor.cursor().column, 2);

    editor.execute_command(Command::MoveCursorLeft).unwrap();
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_move_cursor_right() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();

    assert_eq!(editor.cursor().column, 1);

    editor.execute_command(Command::MoveCursorRight).unwrap();
    assert_eq!(editor.cursor().column, 2);
}

#[test]
fn test_editor_move_cursor_left_wraps_to_previous_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 0);

    editor.execute_command(Command::MoveCursorLeft).unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_move_cursor_right_wraps_to_next_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::MoveCursorUp).unwrap();
    editor.execute_command(Command::MoveToEndOfLine).unwrap();

    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 1);

    editor.execute_command(Command::MoveCursorRight).unwrap();
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_editor_move_to_start_of_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_editor_move_to_end_of_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    editor.execute_command(Command::MoveToEndOfLine).unwrap();
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_editor_move_to_start_of_file() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    assert_eq!(editor.cursor(), &CursorPosition::zero());
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_editor_move_to_end_of_file() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::MoveToStartOfFile).unwrap();

    editor.execute_command(Command::MoveToEndOfFile).unwrap();
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_page_up() {
    let mut editor = EditorState::new();
    for i in 0..30 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::PageUp).unwrap();
    assert_eq!(editor.cursor().line, 9);
}

#[test]
fn test_editor_page_down() {
    let mut editor = EditorState::new();
    for i in 0..30 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfFile).unwrap();

    editor.execute_command(Command::PageDown).unwrap();
    assert_eq!(editor.cursor().line, 20);
}

#[test]
fn test_editor_goto_line() {
    let mut editor = EditorState::new();
    for i in 0..10 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::GotoLine(5)).unwrap();
    assert_eq!(editor.cursor().line, 5);
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_editor_goto_invalid_line() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::GotoLine(100));
    assert!(result.is_err());
}

#[test]
fn test_editor_adjust_viewport() {
    let mut editor = EditorState::new();
    for i in 0..50 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 0);

    editor.execute_command(Command::GotoLine(30)).unwrap();
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 11);
}

#[test]
fn test_editor_status_message() {
    let mut editor = EditorState::new();
    editor.set_status_message("Test message".to_string());
    assert_eq!(editor.status_message(), "Test message");
}

#[test]
fn test_editor_cursor_position_clamps_on_vertical_movement() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar('n')).unwrap();
    editor.execute_command(Command::InsertChar('g')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('X')).unwrap();

    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 1);

    editor.execute_command(Command::MoveCursorUp).unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_new_buffer() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('T')).unwrap();
    editor.execute_command(Command::New).unwrap();

    assert_eq!(editor.buffer().content(), "");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}
