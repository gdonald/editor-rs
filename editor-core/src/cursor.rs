use crate::error::{EditorError, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPosition {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultiCursor {
    positions: Vec<CursorPosition>,
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

impl MultiCursor {
    pub fn new() -> Self {
        Self {
            positions: vec![CursorPosition::zero()],
        }
    }

    pub fn positions(&self) -> &[CursorPosition] {
        &self.positions
    }

    pub fn primary(&self) -> &CursorPosition {
        &self.positions[0]
    }

    pub fn add_cursor(&mut self, position: CursorPosition) {
        self.positions.push(position);
        self.merge_overlaps();
    }

    pub fn remove_cursor(&mut self, index: usize) {
        if index < self.positions.len() {
            self.positions.remove(index);
        }

        if self.positions.is_empty() {
            self.positions.push(CursorPosition::zero());
        }
    }

    pub fn set_positions(&mut self, positions: Vec<CursorPosition>) {
        if positions.is_empty() {
            self.positions = vec![CursorPosition::zero()];
        } else {
            self.positions = positions;
            self.merge_overlaps();
        }
    }

    pub fn reset_to(&mut self, position: CursorPosition) {
        self.positions = vec![position];
    }

    pub fn merge_overlaps(&mut self) {
        self.positions
            .sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        self.positions.dedup();
    }
}

impl Default for MultiCursor {
    fn default() -> Self {
        Self::new()
    }
}
