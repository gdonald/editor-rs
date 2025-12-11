use editor_core::{Command, CursorPosition, EditorState};

#[test]
fn test_selection_start_creates_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.anchor, CursorPosition::new(0, 0));
    assert_eq!(selection.cursor, CursorPosition::new(0, 0));
}

#[test]
fn test_selection_end_extends_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    assert!(editor.has_selection());
    let selection = editor.selection().unwrap();
    assert_eq!(selection.anchor, CursorPosition::new(0, 0));
    assert_eq!(selection.cursor, CursorPosition::new(0, 2));
}

#[test]
fn test_copy_empty_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    let result = editor.execute_command(Command::Copy);
    assert!(result.is_ok());
}

#[test]
fn test_copy_selected_text() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    let result = editor.execute_command(Command::Copy);
    assert!(result.is_ok());
}

#[test]
fn test_cut_removes_selected_text() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    editor.execute_command(Command::Cut).unwrap();

    assert!(!editor.has_selection());
    assert_eq!(editor.buffer().content(), "llo");
}

#[test]
fn test_cut_empty_selection() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    let result = editor.execute_command(Command::Cut);
    assert!(result.is_ok());
    assert_eq!(editor.buffer().content(), "Hello");
}

#[test]
fn test_paste_inserts_text() {
    editor_core::ClipboardManager::clear_test_clipboard();
    let mut editor1 = EditorState::new();
    editor1.execute_command(Command::InsertChar('H')).unwrap();
    editor1.execute_command(Command::InsertChar('e')).unwrap();
    editor1.execute_command(Command::InsertChar('l')).unwrap();
    editor1.execute_command(Command::InsertChar('l')).unwrap();
    editor1.execute_command(Command::InsertChar('o')).unwrap();

    editor1.execute_command(Command::MoveToStartOfLine).unwrap();
    editor1.execute_command(Command::SelectionStart).unwrap();
    editor1.execute_command(Command::MoveCursorRight).unwrap();
    editor1.execute_command(Command::MoveCursorRight).unwrap();
    editor1.execute_command(Command::SelectionEnd).unwrap();

    editor1.execute_command(Command::Copy).unwrap();

    let mut editor2 = EditorState::new();
    editor2.execute_command(Command::InsertChar('W')).unwrap();
    editor2.execute_command(Command::InsertChar('o')).unwrap();
    editor2.execute_command(Command::InsertChar('r')).unwrap();
    editor2.execute_command(Command::InsertChar('l')).unwrap();
    editor2.execute_command(Command::InsertChar('d')).unwrap();

    editor2.execute_command(Command::MoveToStartOfLine).unwrap();
    editor2.execute_command(Command::Paste).unwrap();

    assert_eq!(editor2.buffer().content(), "HeWorld");
}

#[test]
fn test_paste_replaces_selection() {
    editor_core::ClipboardManager::clear_test_clipboard();
    let mut editor1 = EditorState::new();
    editor1.execute_command(Command::InsertChar('H')).unwrap();
    editor1.execute_command(Command::InsertChar('e')).unwrap();

    editor1.execute_command(Command::MoveToStartOfLine).unwrap();
    editor1.execute_command(Command::SelectionStart).unwrap();
    editor1.execute_command(Command::MoveToEndOfLine).unwrap();
    editor1.execute_command(Command::SelectionEnd).unwrap();

    editor1.execute_command(Command::Copy).unwrap();

    let mut editor2 = EditorState::new();
    editor2.execute_command(Command::InsertChar('W')).unwrap();
    editor2.execute_command(Command::InsertChar('o')).unwrap();
    editor2.execute_command(Command::InsertChar('r')).unwrap();
    editor2.execute_command(Command::InsertChar('l')).unwrap();
    editor2.execute_command(Command::InsertChar('d')).unwrap();

    editor2.execute_command(Command::MoveToStartOfLine).unwrap();
    editor2.execute_command(Command::SelectionStart).unwrap();
    editor2.execute_command(Command::MoveCursorRight).unwrap();
    editor2.execute_command(Command::MoveCursorRight).unwrap();
    editor2.execute_command(Command::SelectionEnd).unwrap();

    editor2.execute_command(Command::Paste).unwrap();

    assert_eq!(editor2.buffer().content(), "Herld");
}

#[test]
fn test_selection_multiline() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('W')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveToEndOfFile).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    editor.execute_command(Command::Cut).unwrap();

    assert_eq!(editor.buffer().content(), "");
}

#[test]
fn test_block_selection_copy() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('D')).unwrap();
    editor.execute_command(Command::InsertChar('E')).unwrap();
    editor.execute_command(Command::InsertChar('F')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::ToggleBlockSelection)
        .unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorDown).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    let result = editor.execute_command(Command::Copy);
    assert!(result.is_ok());
}

#[test]
fn test_block_selection_cut() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('D')).unwrap();
    editor.execute_command(Command::InsertChar('E')).unwrap();
    editor.execute_command(Command::InsertChar('F')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::ToggleBlockSelection)
        .unwrap();
    editor.execute_command(Command::SelectionStart).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorRight).unwrap();
    editor.execute_command(Command::MoveCursorDown).unwrap();
    editor.execute_command(Command::SelectionEnd).unwrap();

    editor.execute_command(Command::Cut).unwrap();

    assert_eq!(editor.buffer().content(), "C\nF");
}

#[test]
fn test_selection_with_mouse() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor
        .execute_command(Command::MouseDragStart(CursorPosition::new(0, 0)))
        .unwrap();
    editor
        .execute_command(Command::MouseDrag(CursorPosition::new(0, 3)))
        .unwrap();
    editor
        .execute_command(Command::MouseDragEnd(CursorPosition::new(0, 3)))
        .unwrap();

    editor.execute_command(Command::Cut).unwrap();

    assert_eq!(editor.buffer().content(), "lo");
}
