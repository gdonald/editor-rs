use editor_core::{Buffer, EditorError, Encoding, LineEnding};
use std::path::PathBuf;

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.line_count(), 1);
    assert!(!buffer.is_modified());
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
    assert!(buffer.is_modified());
}

#[test]
fn test_buffer_delete_char() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.delete_char(0, 4).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hell");
    assert!(buffer.is_modified());
}

#[test]
fn test_buffer_insert_str() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.insert_str(0, 5, " World").unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hello World");
    assert!(buffer.is_modified());
}

#[test]
fn test_buffer_delete_range() {
    let mut buffer = Buffer::from_string("Hello World");
    buffer.delete_range(0, 5, 0, 11).unwrap();
    assert_eq!(buffer.line(0).unwrap(), "Hello");
    assert!(buffer.is_modified());
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

#[test]
fn test_buffer_from_file() {
    use std::fs;
    use tempfile::NamedTempFile;

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();
    fs::write(&path, "Test content").unwrap();

    let buffer = Buffer::from_file(path.clone()).unwrap();
    assert_eq!(buffer.content(), "Test content");
    assert_eq!(buffer.file_path(), Some(&path));
    assert!(!buffer.is_modified());
}

#[test]
fn test_buffer_from_file_nonexistent() {
    let path = PathBuf::from("/nonexistent/file.txt");
    let result = Buffer::from_file(path);
    assert!(result.is_err());
}

#[test]
fn test_buffer_save() {
    use tempfile::NamedTempFile;

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();

    let mut buffer = Buffer::from_file(path.clone()).unwrap();
    buffer.insert_str(0, 0, "New content").unwrap();
    assert!(buffer.is_modified());

    buffer.save().unwrap();
    assert!(!buffer.is_modified());

    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, "New content");
}

#[test]
fn test_buffer_save_no_path() {
    let mut buffer = Buffer::new();
    buffer.insert_str(0, 0, "Some content").unwrap();

    let result = buffer.save();
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        EditorError::InvalidOperation(_)
    ));
}

#[test]
fn test_buffer_save_as() {
    use tempfile::NamedTempFile;

    let mut buffer = Buffer::from_string("Original content");

    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().to_path_buf();

    buffer.save_as(path.clone()).unwrap();
    assert_eq!(buffer.file_path(), Some(&path));
    assert!(!buffer.is_modified());

    let content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(content, "Original content");
}

#[test]
fn test_buffer_delete_char_at_end() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.delete_char(0, 5);
    assert!(result.is_err());
}

#[test]
fn test_buffer_delete_range_invalid() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.delete_range(0, 10, 0, 15);
    assert!(result.is_err());
}

#[test]
fn test_buffer_line_invalid() {
    let buffer = Buffer::from_string("Hello");
    let result = buffer.line_len(10);
    assert!(result.is_err());
}

#[test]
fn test_buffer_insert_char_error_path() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.insert_char(10, 0, 'x');
    assert!(result.is_err());
}

#[test]
fn test_buffer_insert_str_error_path() {
    let mut buffer = Buffer::from_string("Hello");
    let result = buffer.insert_str(10, 0, "test");
    assert!(result.is_err());
}

#[test]
fn test_buffer_delete_char_boundary() {
    let mut buffer = Buffer::from_string("Hello");
    buffer.delete_char(0, 4).unwrap();
    assert_eq!(buffer.content(), "Hell");
}

#[test]
fn test_buffer_delete_range_start_equals_end() {
    let mut buffer = Buffer::from_string("Hello World");
    buffer.delete_range(0, 5, 0, 5).unwrap();
    assert_eq!(buffer.content(), "Hello World");
}

#[test]
fn test_buffer_default() {
    let buffer = Buffer::default();
    assert_eq!(buffer.line_count(), 1);
    assert!(!buffer.is_modified());
}

#[test]
fn test_line_ending_detection_lf() {
    let buffer = Buffer::from_string("Hello\nWorld\n");
    assert_eq!(buffer.line_ending(), LineEnding::Lf);
}

#[test]
fn test_line_ending_detection_crlf() {
    let buffer = Buffer::from_string("Hello\r\nWorld\r\n");
    assert_eq!(buffer.line_ending(), LineEnding::Crlf);
}

#[test]
fn test_line_ending_normalization() {
    let buffer = Buffer::from_string("Hello\r\nWorld\r\n");
    assert_eq!(buffer.content(), "Hello\nWorld\n");
}

#[test]
fn test_line_ending_save_crlf() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_line_ending_crlf.txt");

    let mut buffer = Buffer::from_string("Hello\r\nWorld\r\n");
    buffer.save_as(path.clone()).unwrap();

    let saved_content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(saved_content, "Hello\r\nWorld\r\n");

    std::fs::remove_file(path).ok();
}

#[test]
fn test_line_ending_save_lf() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_line_ending_lf.txt");

    let mut buffer = Buffer::from_string("Hello\nWorld\n");
    buffer.save_as(path.clone()).unwrap();

    let saved_content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(saved_content, "Hello\nWorld\n");

    std::fs::remove_file(path).ok();
}

#[test]
fn test_line_ending_from_file_crlf() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_from_file_crlf.txt");
    std::fs::write(&path, "Hello\r\nWorld\r\n").unwrap();

    let buffer = Buffer::from_file(path.clone()).unwrap();
    assert_eq!(buffer.line_ending(), LineEnding::Crlf);
    assert_eq!(buffer.content(), "Hello\nWorld\n");

    std::fs::remove_file(path).ok();
}

#[test]
fn test_line_ending_from_file_lf() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_from_file_lf.txt");
    std::fs::write(&path, "Hello\nWorld\n").unwrap();

    let buffer = Buffer::from_file(path.clone()).unwrap();
    assert_eq!(buffer.line_ending(), LineEnding::Lf);
    assert_eq!(buffer.content(), "Hello\nWorld\n");

    std::fs::remove_file(path).ok();
}

#[test]
fn test_set_line_ending() {
    let mut buffer = Buffer::from_string("Hello\nWorld\n");
    assert_eq!(buffer.line_ending(), LineEnding::Lf);
    assert!(!buffer.is_modified());

    buffer.set_line_ending(LineEnding::Crlf);
    assert_eq!(buffer.line_ending(), LineEnding::Crlf);
    assert!(buffer.is_modified());
}

#[test]
fn test_line_ending_round_trip() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_round_trip.txt");
    std::fs::write(&path, "Line1\r\nLine2\r\nLine3\r\n").unwrap();

    let mut buffer = Buffer::from_file(path.clone()).unwrap();
    assert_eq!(buffer.line_ending(), LineEnding::Crlf);

    buffer.insert_str(1, 0, "NEW ").unwrap();
    buffer.save().unwrap();

    let saved_content = std::fs::read_to_string(&path).unwrap();
    assert_eq!(saved_content, "Line1\r\nNEW Line2\r\nLine3\r\n");

    std::fs::remove_file(path).ok();
}

#[test]
fn test_encoding_utf8() {
    let buffer = Buffer::from_string("Hello World");
    assert_eq!(buffer.encoding(), Encoding::Utf8);
}

#[test]
fn test_encoding_from_file() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_encoding.txt");
    std::fs::write(&path, "UTF-8 content").unwrap();

    let buffer = Buffer::from_file(path.clone()).unwrap();
    assert_eq!(buffer.encoding(), Encoding::Utf8);

    std::fs::remove_file(path).ok();
}

#[test]
fn test_encoding_invalid_utf8() {
    let dir = std::env::temp_dir();
    let path = dir.join("test_invalid_utf8.txt");
    std::fs::write(&path, &[0xFF, 0xFE, 0xFD]).unwrap();

    let result = Buffer::from_file(path.clone());
    assert!(result.is_err());

    std::fs::remove_file(path).ok();
}
