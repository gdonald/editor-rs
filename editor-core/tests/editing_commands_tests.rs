use editor_core::{CaseMode, Command, CursorPosition, EditorState};

#[test]
fn test_duplicate_line_single_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('i')).unwrap();
    editor.execute_command(Command::DuplicateLine).unwrap();

    assert_eq!(editor.buffer().content(), "Hi\nHi");
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_duplicate_line_multiple_lines() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::DuplicateLine).unwrap();

    assert_eq!(editor.buffer().content(), "A\nA\nB\nC");
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_move_lines_up() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveLinesUp).unwrap();
    assert_eq!(editor.buffer().content(), "A\nC\nB");
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_move_lines_up_at_top() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::MoveLinesUp).unwrap();

    assert_eq!(editor.buffer().content(), "A\nB");
    assert_eq!(editor.cursor().line, 0);
}

#[test]
fn test_move_lines_down() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::MoveLinesDown).unwrap();

    assert_eq!(editor.buffer().content(), "B\nA\nC");
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_move_lines_down_at_bottom() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    editor.execute_command(Command::MoveLinesDown).unwrap();

    assert_eq!(editor.buffer().content(), "A\nB");
    assert_eq!(editor.cursor().line, 1);
}

#[test]
fn test_join_lines() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('W')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar('r')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('d')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::JoinLines).unwrap();

    assert_eq!(editor.buffer().content(), "Hello World");
}

#[test]
fn test_join_lines_with_leading_whitespace() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor.execute_command(Command::JoinLines).unwrap();

    assert_eq!(editor.buffer().content(), "A B");
}

#[test]
fn test_join_lines_at_last_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::JoinLines).unwrap();

    assert_eq!(editor.buffer().content(), "A");
}

#[test]
fn test_sort_lines_alphabetical() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 0)))
        .unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(2, 0)))
        .unwrap();
    editor
        .execute_command(Command::SortLines { numerical: false })
        .unwrap();

    assert_eq!(editor.buffer().content(), "A\nB\nC");
}

#[test]
fn test_sort_lines_numerical() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('1')).unwrap();
    editor.execute_command(Command::InsertChar('0')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('2')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('5')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 0)))
        .unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(2, 0)))
        .unwrap();
    editor
        .execute_command(Command::SortLines { numerical: true })
        .unwrap();

    assert_eq!(editor.buffer().content(), "2\n5\n10");
}

#[test]
fn test_sort_lines_single_line() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor
        .execute_command(Command::SortLines { numerical: false })
        .unwrap();

    assert_eq!(editor.buffer().content(), "A");
}

#[test]
fn test_change_case_upper() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::ChangeCase {
            mode: CaseMode::Upper,
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "HELLO");
}

#[test]
fn test_change_case_lower() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('E')).unwrap();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('L')).unwrap();
    editor.execute_command(Command::InsertChar('O')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::ChangeCase {
            mode: CaseMode::Lower,
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "hello");
}

#[test]
fn test_change_case_title() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('h')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar(' ')).unwrap();
    editor.execute_command(Command::InsertChar('w')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();
    editor.execute_command(Command::InsertChar('r')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('d')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::ChangeCase {
            mode: CaseMode::Title,
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "Hello World");
}

#[test]
fn test_transpose_characters_middle() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('H')).unwrap();
    editor.execute_command(Command::InsertChar('e')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('l')).unwrap();
    editor.execute_command(Command::InsertChar('o')).unwrap();

    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor.execute_command(Command::MoveCursorLeft).unwrap();
    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "Hlelo");
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_transpose_characters_at_start() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfLine).unwrap();
    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "BAC");
    assert_eq!(editor.cursor().column, 1);
}

#[test]
fn test_transpose_characters_at_end() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "ACB");
}

#[test]
fn test_transpose_characters_single_char() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();

    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "A");
}

#[test]
fn test_transpose_characters_empty_line() {
    let mut editor = EditorState::new();
    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "");
}

#[test]
fn test_duplicate_line_with_multicursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 0)))
        .unwrap();
    editor.execute_command(Command::DuplicateLine).unwrap();

    assert_eq!(editor.buffer().content(), "A\nA\nB\nB\nC");
}

#[test]
fn test_join_lines_with_multicursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('D')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 1)))
        .unwrap();
    editor.execute_command(Command::JoinLines).unwrap();

    assert_eq!(editor.buffer().content(), "A B C\nD");
}

#[test]
fn test_change_case_with_multicursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('a')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('b')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('c')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 0)))
        .unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(2, 0)))
        .unwrap();
    editor
        .execute_command(Command::ChangeCase {
            mode: CaseMode::Upper,
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "A\nB\nC");
}

#[test]
fn test_transpose_characters_with_multicursor() {
    let mut editor = EditorState::new();
    editor.execute_command(Command::InsertChar('A')).unwrap();
    editor.execute_command(Command::InsertChar('B')).unwrap();
    editor.execute_command(Command::NewLine).unwrap();
    editor.execute_command(Command::InsertChar('C')).unwrap();
    editor.execute_command(Command::InsertChar('D')).unwrap();

    editor.execute_command(Command::MoveToStartOfFile).unwrap();
    editor
        .execute_command(Command::AddCursor(CursorPosition::new(1, 0)))
        .unwrap();
    editor
        .execute_command(Command::TransposeCharacters)
        .unwrap();

    assert_eq!(editor.buffer().content(), "BA\nDC");
}
