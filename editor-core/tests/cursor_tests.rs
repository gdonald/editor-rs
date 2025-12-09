use editor_core::CursorPosition;

#[test]
fn test_cursor_new() {
    let cursor = CursorPosition::new(5, 10);
    assert_eq!(cursor.line, 5);
    assert_eq!(cursor.column, 10);
}

#[test]
fn test_cursor_zero() {
    let cursor = CursorPosition::zero();
    assert_eq!(cursor.line, 0);
    assert_eq!(cursor.column, 0);
}

#[test]
fn test_cursor_move_left() {
    let mut cursor = CursorPosition::new(0, 5);
    cursor.move_left().unwrap();
    assert_eq!(cursor.column, 4);
}

#[test]
fn test_cursor_move_left_at_start() {
    let mut cursor = CursorPosition::zero();
    let result = cursor.move_left();
    assert!(result.is_err());
}

#[test]
fn test_cursor_move_right() {
    let mut cursor = CursorPosition::new(0, 5);
    cursor.move_right(10).unwrap();
    assert_eq!(cursor.column, 6);
}

#[test]
fn test_cursor_move_right_at_end() {
    let mut cursor = CursorPosition::new(0, 10);
    let result = cursor.move_right(10);
    assert!(result.is_err());
}

#[test]
fn test_cursor_move_up() {
    let mut cursor = CursorPosition::new(5, 0);
    cursor.move_up().unwrap();
    assert_eq!(cursor.line, 4);
}

#[test]
fn test_cursor_move_up_at_top() {
    let mut cursor = CursorPosition::zero();
    let result = cursor.move_up();
    assert!(result.is_err());
}

#[test]
fn test_cursor_move_down() {
    let mut cursor = CursorPosition::new(0, 0);
    cursor.move_down(10).unwrap();
    assert_eq!(cursor.line, 1);
}

#[test]
fn test_cursor_move_down_at_bottom() {
    let mut cursor = CursorPosition::new(10, 0);
    let result = cursor.move_down(10);
    assert!(result.is_err());
}

#[test]
fn test_cursor_move_to_start_of_line() {
    let mut cursor = CursorPosition::new(5, 10);
    cursor.move_to_start_of_line();
    assert_eq!(cursor.column, 0);
    assert_eq!(cursor.line, 5);
}

#[test]
fn test_cursor_move_to_end_of_line() {
    let mut cursor = CursorPosition::new(5, 0);
    cursor.move_to_end_of_line(20);
    assert_eq!(cursor.column, 20);
    assert_eq!(cursor.line, 5);
}

#[test]
fn test_cursor_move_to_start_of_file() {
    let mut cursor = CursorPosition::new(10, 10);
    cursor.move_to_start_of_file();
    assert_eq!(cursor.line, 0);
    assert_eq!(cursor.column, 0);
}

#[test]
fn test_cursor_move_to_end_of_file() {
    let mut cursor = CursorPosition::zero();
    cursor.move_to_end_of_file(50, 30);
    assert_eq!(cursor.line, 50);
    assert_eq!(cursor.column, 30);
}

#[test]
fn test_cursor_equality() {
    let cursor1 = CursorPosition::new(5, 10);
    let cursor2 = CursorPosition::new(5, 10);
    let cursor3 = CursorPosition::new(5, 11);

    assert_eq!(cursor1, cursor2);
    assert_ne!(cursor1, cursor3);
}

#[test]
fn test_cursor_clone() {
    let cursor1 = CursorPosition::new(5, 10);
    let cursor2 = cursor1;
    assert_eq!(cursor1, cursor2);
}
