use crate::error::{EditorError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

impl CursorPosition {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }

    pub fn zero() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn move_left(&mut self) -> Result<()> {
        if self.column > 0 {
            self.column -= 1;
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "Cannot move left from column 0".to_string(),
            ))
        }
    }

    pub fn move_right(&mut self, max_column: usize) -> Result<()> {
        if self.column < max_column {
            self.column += 1;
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "Cannot move right beyond line end".to_string(),
            ))
        }
    }

    pub fn move_up(&mut self) -> Result<()> {
        if self.line > 0 {
            self.line -= 1;
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "Cannot move up from line 0".to_string(),
            ))
        }
    }

    pub fn move_down(&mut self, max_line: usize) -> Result<()> {
        if self.line < max_line {
            self.line += 1;
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "Cannot move down beyond last line".to_string(),
            ))
        }
    }

    pub fn move_to_start_of_line(&mut self) {
        self.column = 0;
    }

    pub fn move_to_end_of_line(&mut self, line_length: usize) {
        self.column = line_length;
    }

    pub fn move_to_start_of_file(&mut self) {
        self.line = 0;
        self.column = 0;
    }

    pub fn move_to_end_of_file(&mut self, last_line: usize, last_line_length: usize) {
        self.line = last_line;
        self.column = last_line_length;
    }
}
