use editor_core::{Buffer, EditorError};

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.is_modified(), false);
    assert_eq!(buffer.file_path(), None);
}

#[test]
fn test_buffer_from_str() {
    let buffer = Buffer::from_string("Hello\nWorld");
    assert_eq!(buffer.line_count(), 2);
    assert_eq!(buffer.line(0).unwrap(), "Hello\n");
    assert_eq!(buffer.line(1).unwrap(), "World");
}

#[test]
fn test_buffer_insert_char() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.insert_char(0, 5, '!').unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hello!");
    assert_eq!(buffer.is_modified(), true);
}

#[test]
fn test_buffer_delete_char() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.delete_char(0, 4).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hell");
    assert_eq!(buffer.is_modified(), true);
}

#[test]
fn test_buffer_insert_str() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.insert_str(0, 5, " World").unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hello World");
    assert_eq!(buffer.is_modified(), true);
}

#[test]
fn test_buffer_delete_range() {
    let mut buffer = Buffer::from_string("Hello World");
    buffer.delete_range(0, 5, 0, 11).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hello");
    assert_eq!(buffer.is_modified(), true);
}

#[test]
fn test_buffer_multiline_operations() {
    let mut buffer = Buffer::from_string("Line 1\nLine 2\nLine 3");
    assert_eq!(buffer.line_count(), 3);

    buffer.insert_char(1, 0, 'X').unwrap();
    assert_eq!(buffer.line(1).unwrap(), "XLine 2\n");

    buffer.delete_char(1, 0).unwrap();
    assert_eq!(buffer.line(1).unwrap(), "Line 2\n");
}

#[test]
fn test_buffer_line_len() {
    let buffer = Buffer::from_string("Hello\nWorld");
    assert_eq!(buffer.line_len(0).unwrap(), 5);
    assert_eq!(buffer.line_len(1).unwrap(), 5);
}

#[test]
fn test_buffer_line_len_empty_line() {
    let buffer = Buffer::from_string("\n");
    assert_eq!(buffer.line_len(0).unwrap(), 0);
}

#[test]
fn test_buffer_invalid_position() {
    let buffer = Buffer::from_string("Hello");
    let result = buffer.line(10);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        EditorError::InvalidPosition { .. }
    ));
}

#[test]
fn test_buffer_to_string() {
    let buffer = Buffer::from_string("Hello\nWorld");
    assert_eq!(buffer.content(), "Hello\nWorld");
}

#[test]
fn test_buffer_empty() {
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert_eq!(buffer.line_len(0).unwrap(), 0);
    assert_eq!(buffer.content(), "");
}

#[test]
fn test_buffer_insert_at_invalid_position() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.insert_char(0, 100, 'x');
    assert!(result.is_err());
}

#[test]
fn test_buffer_delete_at_invalid_position() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.delete_char(0, 100);
    assert!(result.is_err());
}

#[test]
fn test_buffer_cross_line_delete_range() {
    let mut buffer = Buffer::from_string("Line 1\nLine 2\nLine 3");
    buffer.delete_range(0, 4, 2, 4).unwrap();
    assert_eq!(buffer.content(), "Line 3");
}
