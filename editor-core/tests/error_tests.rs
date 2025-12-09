use editor_core::EditorError;
use std::error::Error;
use std::io;

#[test]
fn test_error_display_io() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = EditorError::Io(io_err);
    let display = format!("{}", err);
    assert!(display.contains("I/O error"));
}

#[test]
fn test_error_display_invalid_position() {
    let err = EditorError::InvalidPosition {
        line: 10,
        column: 5,
    };
    let display = format!("{}", err);
    assert_eq!(display, "Invalid position: line 10, column 5");
}

#[test]
fn test_error_display_invalid_operation() {
    let err = EditorError::InvalidOperation("Cannot do this".to_string());
    let display = format!("{}", err);
    assert_eq!(display, "Invalid operation: Cannot do this");
}

#[test]
fn test_error_display_file_not_found() {
    let err = EditorError::FileNotFound("/path/to/file".to_string());
    let display = format!("{}", err);
    assert_eq!(display, "File not found: /path/to/file");
}

#[test]
fn test_error_display_encoding_error() {
    let err = EditorError::EncodingError("Invalid UTF-8".to_string());
    let display = format!("{}", err);
    assert_eq!(display, "Encoding error: Invalid UTF-8");
}

#[test]
fn test_error_source_io() {
    let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let err = EditorError::Io(io_err);
    assert!(err.source().is_some());
}

#[test]
fn test_error_source_other() {
    let err = EditorError::InvalidPosition { line: 0, column: 0 };
    assert!(err.source().is_none());
}

#[test]
fn test_error_from_io() {
    let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "access denied");
    let err: EditorError = io_err.into();
    assert!(matches!(err, EditorError::Io(_)));
}
