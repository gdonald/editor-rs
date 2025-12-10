use std::fmt;
use std::io;

#[derive(Debug)]
pub enum EditorError {
    Io(io::Error),
    InvalidPosition { line: usize, column: usize },
    InvalidOperation(String),
    FileNotFound(String),
    EncodingError(String),
    ReadOnlyFile(String),
    BinaryFile(String),
}

impl fmt::Display for EditorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EditorError::Io(err) => write!(f, "I/O error: {}", err),
            EditorError::InvalidPosition { line, column } => {
                write!(f, "Invalid position: line {}, column {}", line, column)
            }
            EditorError::InvalidOperation(msg) => write!(f, "Invalid operation: {}", msg),
            EditorError::FileNotFound(path) => write!(f, "File not found: {}", path),
            EditorError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            EditorError::ReadOnlyFile(path) => write!(f, "Cannot modify read-only file: {}", path),
            EditorError::BinaryFile(path) => write!(f, "Cannot edit binary file: {}", path),
        }
    }
}

impl std::error::Error for EditorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            EditorError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for EditorError {
    fn from(err: io::Error) -> Self {
        EditorError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, EditorError>;
