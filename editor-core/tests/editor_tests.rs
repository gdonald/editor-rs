use editor_core::{Command, CursorPosition, EditorState};
use std::path::PathBuf;

#[test]
fn test_editor_new() {
    let editor = EditorState::new();
    assert_eq!(editor.cursor(), &CursorPosition::zero());
    assert_eq!(editor.current_buffer().line_count(), 1);
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_editor_insert_char() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();

    assert_eq!(editor.current_buffer().content(), "Hi");
    assert_eq!(editor.cursor().column, 2);
}

#[test]
fn test_editor_delete_char() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor.execute_command(Command::DeleteChar).unwrap();

    assert_eq!(editor.current_buffer().content(), "H");
}

#[test]
fn test_editor_backspace() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.current_buffer().content(), "H");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_backspace_at_start() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::Backspace).unwrap();

    assert_eq!(editor.current_buffer().content(), "");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}

#[test]
fn test_editor_new_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('W')).unwrap();

    assert_eq!(editor.current_buffer().content(), "H\nW");
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

    assert_eq!(editor.current_buffer().content(), "HW");
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

    assert_eq!(editor.current_buffer().content(), "L2");
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
fn test_editor_move_cursor_word_right() {
    let mut editor = EditorState::new();
    for ch in "hello  world".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    editor
        .execute_command(Command::MoveCursorWordRight)
        .unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 7);

    editor
        .execute_command(Command::MoveCursorWordRight)
        .unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 12);
}

#[test]
fn test_editor_move_cursor_word_right_across_lines() {
    let mut editor = EditorState::new();
    for ch in "hello".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::NewLine).unwrap();
    for ch in "world".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfFile).unwrap();

    editor
        .execute_command(Command::MoveCursorWordRight)
        .unwrap();
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::MoveCursorWordRight)
        .unwrap();
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 5);
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
fn test_editor_move_cursor_word_left() {
    let mut editor = EditorState::new();
    for ch in "hello  world".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToEndOfLine).unwrap();

    editor.execute_command(Command::MoveCursorWordLeft).unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 7);

    editor.execute_command(Command::MoveCursorWordLeft).unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_editor_move_cursor_word_left_across_lines() {
    let mut editor = EditorState::new();
    for ch in "first".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::NewLine).unwrap();
    for ch in "second".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::GotoLine(1)).unwrap();

    editor.execute_command(Command::MoveCursorWordLeft).unwrap();
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 0);
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
    assert_eq!(editor.viewport_top(), 16);
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

    assert_eq!(editor.current_buffer().content(), "");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}

#[test]
fn test_editor_from_file() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    fs::write(&path, "Test file content").unwrap();

    let editor = EditorState::from_file(path).unwrap();
    assert_eq!(editor.current_buffer().content(), "Test file content");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}

#[test]
fn test_editor_from_file_nonexistent() {
    let path = PathBuf::from("/nonexistent/file.txt");
    let result = EditorState::from_file(path);
    assert!(result.is_err());
}

#[test]
fn test_editor_open_file() {
    use std::fs;
    use tempfile::NamedTempFile;

    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('O')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('d')).unwrap();

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    fs::write(&path, "New file content").unwrap();

    editor.execute_command(Command::Open(path)).unwrap();
    assert_eq!(editor.current_buffer().content(), "New file content");
    assert_eq!(editor.cursor(), &CursorPosition::zero());
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_editor_save() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    fs::write(&path, "").unwrap();

    let mut editor = EditorState::from_file(path.clone()).unwrap();
    editor.execute_command(Command::InsertChar('T')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('s')).unwrap();
    editor.execute_command(Command::InsertChar('t')).unwrap();

    editor.execute_command(Command::Save).unwrap();
    assert_eq!(editor.status_message(), "File saved");

    let content = fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Test");
}

#[test]
fn test_editor_save_as() {
    use tempfile::NamedTempFile;

    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('D')).unwrap();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::InsertChar('t')).unwrap();
    editor.execute_command(Command::InsertChar('a')).unwrap();

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();

    editor
        .execute_command(Command::SaveAs(path.clone()))
        .unwrap();
    assert_eq!(editor.status_message(), "File saved");

    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Data");
}

#[test]
fn test_editor_close_command() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::Close);
    assert!(result.is_ok());
}

#[test]
fn test_editor_default() {
    let editor = EditorState::default();
    assert_eq!(editor.cursor(), &CursorPosition::zero());
    assert_eq!(editor.current_buffer().line_count(), 1);
}

#[test]
fn test_editor_buffer_accessor() {
    let editor = EditorState::new();
    let buffer = editor.current_buffer();
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_editor_viewport_top_accessor() {
    let editor = EditorState::new();
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_editor_delete_line_last_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('1')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('2')).unwrap();

    editor.execute_command(Command::DeleteLine).unwrap();
    assert_eq!(editor.current_buffer().content(), "L1\n");
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_editor_page_up_at_top() {
    let mut editor = EditorState::new();
    for i in 0..10 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::PageUp).unwrap();
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_page_down_at_bottom() {
    let mut editor = EditorState::new();
    for i in 0..10 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }
    editor.execute_command(Command::PageDown).unwrap();
    assert_eq!(editor.cursor().line, 9);
}

#[test]
fn test_editor_move_to_end_of_file_empty_buffer() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::MoveToEndOfFile).unwrap();
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_delete_line_empty_buffer() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::DeleteLine).unwrap();
    assert_eq!(editor.current_buffer().content(), "");
}

#[test]
fn test_editor_move_cursor_up_at_top() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::MoveCursorUp).unwrap();
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_move_cursor_down_at_bottom() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::MoveCursorDown).unwrap();
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_editor_move_cursor_left_at_start() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    assert_eq!(editor.cursor(), &CursorPosition::zero());
}

#[test]
fn test_editor_move_cursor_right_at_end() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_editor_indent_and_dedent_line() {
    let mut editor = EditorState::new();
    for ch in "text".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::Indent).unwrap();
    assert_eq!(editor.current_buffer().content(), "    text");
    assert_eq!(editor.cursor().column, 8);

    editor.execute_command(Command::Dedent).unwrap();
    assert_eq!(editor.current_buffer().content(), "text");
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_editor_auto_indent_new_line() {
    let mut editor = EditorState::new();
    for ch in "    line".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::NewLine).unwrap();
    assert_eq!(editor.current_buffer().content(), "    line\n    ");
    assert_eq!(editor.cursor().line, 1);
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_editor_overwrite_mode() {
    let mut editor = EditorState::new();
    for ch in "hello".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::ToggleOverwriteMode)
        .unwrap();
    editor.execute_command(Command::InsertChar('Y')).unwrap();
    assert_eq!(editor.current_buffer().content(), "Yello");
    assert!(editor.overwrite_mode());

    editor.execute_command(Command::MoveToEndOfLine).unwrap();
    editor.execute_command(Command::InsertChar('!')).unwrap();
    assert_eq!(editor.current_buffer().content(), "Yello!");
}

#[test]
fn test_editor_hard_wrap() {
    let mut editor = EditorState::new();
    for ch in "abcdefghij".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::HardWrap(4)).unwrap();
    assert_eq!(editor.current_buffer().content(), "abcd\nefgh\nij");
}

#[test]
fn test_editor_soft_wrap_lines() {
    let mut editor = EditorState::new();
    for ch in "wraptext".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor.execute_command(Command::SetSoftWrap(3)).unwrap();
    let lines = editor.soft_wrapped_lines();
    assert_eq!(
        lines,
        vec!["wra".to_string(), "pte".to_string(), "xt".to_string()]
    );
    assert_eq!(editor.current_buffer().content(), "wraptext");
}

#[test]
fn test_editor_trim_trailing_whitespace() {
    let mut editor = EditorState::new();
    for ch in "abc  \nline\t\n".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor
        .execute_command(Command::TrimTrailingWhitespace)
        .unwrap();
    assert_eq!(editor.current_buffer().content(), "abc\nline\n");
    assert_eq!(editor.cursor().line, 2);
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_auto_commit_enabled_by_default() {
    let editor = EditorState::new();
    assert!(editor.auto_commit_enabled());
}

#[test]
fn test_set_auto_commit_enabled() {
    let mut editor = EditorState::new();
    assert!(editor.auto_commit_enabled());

    editor.set_auto_commit_enabled(false);
    assert!(!editor.auto_commit_enabled());

    editor.set_auto_commit_enabled(true);
    assert!(editor.auto_commit_enabled());
}

#[test]
fn test_history_browser_not_open_by_default() {
    let editor = EditorState::new();
    assert!(!editor.is_history_browser_open());
    assert!(editor.history_browser().is_none());
}

#[test]
fn test_open_history_browser_unsaved_buffer() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::OpenHistoryBrowser);
    assert!(result.is_err());
}

#[test]
fn test_open_history_browser_no_history() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    fs::write(&path, "test content").unwrap();

    let mut editor = EditorState::from_file(path).unwrap();
    let result = editor.execute_command(Command::OpenHistoryBrowser);
    assert!(result.is_err());
}

#[test]
fn test_close_history_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::CloseHistoryBrowser);
    assert!(result.is_ok());
    assert!(!editor.is_history_browser_open());
}

#[test]
fn test_history_browser_accessors() {
    let mut editor = EditorState::new();
    assert!(editor.history_browser().is_none());
    assert!(editor.history_browser_mut().is_none());
}

#[test]
fn test_history_navigate_next_without_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::HistoryNavigateNext);
    assert!(result.is_err());
}

#[test]
fn test_history_navigate_previous_without_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::HistoryNavigatePrevious);
    assert!(result.is_err());
}

#[test]
fn test_history_select_commit_without_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::HistorySelectCommit(0));
    assert!(result.is_err());
}

#[test]
fn test_history_toggle_file_list_without_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::HistoryToggleFileList);
    assert!(result.is_err());
}

#[test]
fn test_history_view_diff_without_browser() {
    let mut editor = EditorState::new();
    let result = editor.execute_command(Command::HistoryViewDiff);
    assert!(result.is_ok());
}
