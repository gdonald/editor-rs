use editor_core::{Command, CursorPosition, EditorState};

#[test]
fn test_mouse_click_positions_cursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::MouseClick(CursorPosition::new(0, 2)))
        .unwrap();

    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 2);
    assert!(!editor.has_selection());
}

#[test]
fn test_mouse_click_clears_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();

    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 0)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(0, 2)))
        .unwrap();

    assert!(editor.has_selection());

    editor
        .execute_command(Command::MouseClick(CursorPosition::new(0, 1)))
        .unwrap();

    assert!(!editor.has_selection());
}

#[test]
fn test_mouse_drag_creates_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(0, 4)))
        .unwrap();
    editor
        .execute_command(Command::MouseDragEnd(CursorPosition::new(0, 4)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.anchor, CursorPosition::new(0, 1));
    assert_eq!(selection.cursor, CursorPosition::new(0, 4));
    assert_eq!(selection.start(), CursorPosition::new(0, 1));
    assert_eq!(selection.end(), CursorPosition::new(0, 4));
}

#[test]
fn test_mouse_drag_multiline_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::InsertChar('n')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('t')).unwrap();
    editor.execute_command(Command::InsertChar('w')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(1, 2)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.anchor, CursorPosition::new(0, 1));
    assert_eq!(selection.cursor, CursorPosition::new(1, 2));
}

#[test]
fn test_mouse_double_click_selects_word() {
    let mut editor = EditorState::new();
    let word = "hello";
    for ch in word.chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    let word2 = "world";
    for ch in word2.chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor
        .execute_command(Command::MouseDoubleClick(CursorPosition::new(0, 2)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.start(), CursorPosition::new(0, 0));
    assert_eq!(selection.end(), CursorPosition::new(0, 5));
}

#[test]
fn test_mouse_double_click_second_word() {
    let mut editor = EditorState::new();
    let word = "hello";
    for ch in word.chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    let word2 = "world";
    for ch in word2.chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor
        .execute_command(Command::MouseDoubleClick(CursorPosition::new(0, 7)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.start(), CursorPosition::new(0, 6));
    assert_eq!(selection.end(), CursorPosition::new(0, 11));
}

#[test]
fn test_mouse_double_click_on_whitespace_does_nothing() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar('b')).unwrap();
    editor.execute_command(Command::InsertChar('y')).unwrap();

    editor
        .execute_command(Command::MouseDoubleClick(CursorPosition::new(0, 3)))
        .unwrap();

    assert!(!editor.has_selection());
}

#[test]
fn test_mouse_triple_click_selects_line() {
    let mut editor = EditorState::new();
    let line = "hello world this is a test";
    for ch in line.chars() {
        editor.execute_command(Command::InsertChar(ch)).unwrap();
    }

    editor
        .execute_command(Command::MouseTripleClick(CursorPosition::new(0, 10)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.start(), CursorPosition::new(0, 0));
    assert_eq!(selection.end(), CursorPosition::new(0, 26));
}

#[test]
fn test_mouse_triple_click_multiline() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('f')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::InsertChar('r')).unwrap();
    editor.execute_command(Command::InsertChar('s')).unwrap();
    editor.execute_command(Command::InsertChar('t')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('s')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('c')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar('n')).unwrap();
    editor.execute_command(Command::InsertChar('d')).unwrap();

    editor
        .execute_command(Command::MouseTripleClick(CursorPosition::new(1, 2)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.start(), CursorPosition::new(1, 0));
    assert_eq!(selection.end(), CursorPosition::new(1, 6));
}

#[test]
fn test_toggle_block_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::ToggleBlockSelection)
        .unwrap();
    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 0)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(2, 1)))
        .unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert!(selection.is_block());
    assert_eq!(selection.anchor, CursorPosition::new(0, 0));
    assert_eq!(selection.cursor, CursorPosition::new(2, 1));
}

#[test]
fn test_block_selection_contains_check() {
    let mut editor = EditorState::new();
    for _ in 0..5 {
        editor.execute_command(Command::InsertChar('a')).unwrap();
    }
    editor.execute_command(Command::NewLine).unwrap();
    for _ in 0..5 {
        editor.execute_command(Command::InsertChar('b')).unwrap();
    }
    editor.execute_command(Command::NewLine).unwrap();
    for _ in 0..5 {
        editor.execute_command(Command::InsertChar('c')).unwrap();
    }

    editor
        .execute_command(Command::ToggleBlockSelection)
        .unwrap();
    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(2, 3)))
        .unwrap();

    let selection = editor.selection().unwrap();
    assert!(selection.is_block());

    assert!(selection.contains(CursorPosition::new(0, 1)));
    assert!(selection.contains(CursorPosition::new(0, 2)));
    assert!(selection.contains(CursorPosition::new(1, 1)));
    assert!(selection.contains(CursorPosition::new(1, 2)));
    assert!(selection.contains(CursorPosition::new(2, 1)));
    assert!(selection.contains(CursorPosition::new(2, 2)));

    assert!(!selection.contains(CursorPosition::new(0, 0)));
    assert!(!selection.contains(CursorPosition::new(0, 4)));
    assert!(!selection.contains(CursorPosition::new(1, 0)));
    assert!(!selection.contains(CursorPosition::new(1, 4)));
}

#[test]
fn test_normal_selection_contains_check() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 1)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(1, 1)))
        .unwrap();

    let selection = editor.selection().unwrap();
    assert!(!selection.is_block());

    assert!(selection.contains(CursorPosition::new(0, 1)));
    assert!(selection.contains(CursorPosition::new(0, 2)));
    assert!(selection.contains(CursorPosition::new(1, 0)));
    assert!(selection.contains(CursorPosition::new(1, 1)));

    assert!(!selection.contains(CursorPosition::new(0, 0)));
    assert!(!selection.contains(CursorPosition::new(1, 2)));
}
