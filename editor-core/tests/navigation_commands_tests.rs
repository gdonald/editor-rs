use editor_core::{Command, EditorState};

#[test]
fn test_jump_to_matching_bracket_parentheses_forward() {
    let mut editor = EditorState::new();
    for ch in "(test)".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 5);
}

#[test]
fn test_jump_to_matching_bracket_parentheses_backward() {
    let mut editor = EditorState::new();
    for ch in "(test)".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    for _ in 0..5 {
        editor.execute_command(Command::MoveCursorRight).unwrap();
    }

    assert_eq!(editor.cursor().column, 5);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_jump_to_matching_bracket_curly_braces_forward() {
    let mut editor = EditorState::new();
    for ch in "{ x }".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_jump_to_matching_bracket_curly_braces_backward() {
    let mut editor = EditorState::new();
    for ch in "{ x }".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    for _ in 0..4 {
        editor.execute_command(Command::MoveCursorRight).unwrap();
    }

    assert_eq!(editor.cursor().column, 4);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_jump_to_matching_bracket_square_brackets_forward() {
    let mut editor = EditorState::new();
    for ch in "[1, 2, 3]".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 8);
}

#[test]
fn test_jump_to_matching_bracket_square_brackets_backward() {
    let mut editor = EditorState::new();
    for ch in "[1, 2, 3]".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    for _ in 0..8 {
        editor.execute_command(Command::MoveCursorRight).unwrap();
    }

    assert_eq!(editor.cursor().column, 8);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 0);
}

#[test]
fn test_jump_to_matching_bracket_angle_brackets_forward() {
    let mut editor = EditorState::new();
    for ch in "<div></div>".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_jump_to_matching_bracket_nested() {
    let mut editor = EditorState::new();
    for ch in "((a))".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    assert_eq!(editor.cursor().column, 0);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 4);
}

#[test]
fn test_jump_to_matching_bracket_nested_inner() {
    let mut editor = EditorState::new();
    for ch in "((a))".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();

    assert_eq!(editor.cursor().column, 1);

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_jump_to_matching_bracket_no_match() {
    let mut editor = EditorState::new();
    for ch in "(abc".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();

    let original_col = editor.cursor().column;

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, original_col);
}

#[test]
fn test_jump_to_matching_bracket_not_on_bracket() {
    let mut editor = EditorState::new();
    for ch in "(abc)".chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();

    let original_col = editor.cursor().column;

    editor
        .execute_command(Command::JumpToMatchingBracket)
        .unwrap();
    assert_eq!(editor.cursor().column, original_col);
}

#[test]
fn test_scroll_offset_default() {
    let editor = EditorState::new();
    assert_eq!(editor.scroll_offset(), 5);
}

#[test]
fn test_scroll_offset_set() {
    let mut editor = EditorState::new();
    editor.set_scroll_offset(10);
    assert_eq!(editor.scroll_offset(), 10);
}

#[test]
fn test_adjust_viewport_with_scroll_offset() {
    let mut editor = EditorState::new();
    for i in 0..50 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.set_scroll_offset(3);
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 0);

    editor.execute_command(Command::GotoLine(10)).unwrap();
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 0);
}

#[test]
fn test_adjust_viewport_scroll_offset_near_bottom() {
    let mut editor = EditorState::new();
    for i in 0..50 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::GotoLine(40)).unwrap();
    editor.set_scroll_offset(5);
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 26);
}

#[test]
fn test_adjust_viewport_scroll_offset_zero() {
    let mut editor = EditorState::new();
    for i in 0..50 {
        if i > 0 {
            editor.execute_command(Command::NewLine).unwrap();
        }
        editor.execute_command(Command::InsertChar('A')).unwrap();
    }

    editor.execute_command(Command::GotoLine(25)).unwrap();
    editor.set_scroll_offset(0);
    editor.adjust_viewport(20);
    assert_eq!(editor.viewport_top(), 6);
}
