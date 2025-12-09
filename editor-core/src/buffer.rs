use crate::error::{EditorError, Result};
use ropey::Rope;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_string(content: &str) -> Self {
        Self {
            rope: Rope::from_str(content),
            file_path: None,
            modified: false,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&path)?;
        Ok(Self {
            rope: Rope::from_str(&content),
            file_path: Some(path),
            modified: false,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.file_path {
            std::fs::write(path, self.content())?;
            self.modified = false;
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "No file path set for buffer".to_string(),
            ))
        }
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<()> {
        std::fs::write(&path, self.content())?;
        self.file_path = Some(path);
        self.modified = false;
        Ok(())
    }

    pub fn insert_char(&mut self, line: usize, column: usize, ch: char) -> Result<()> {
        let char_idx = self.line_col_to_char_idx(line, column)?;
        self.rope.insert_char(char_idx, ch);
        self.modified = true;
        Ok(())
    }

    pub fn delete_char(&mut self, line: usize, column: usize) -> Result<()> {
        let char_idx = self.line_col_to_char_idx(line, column)?;
        if char_idx < self.rope.len_chars() {
            self.rope.remove(char_idx..char_idx + 1);
            self.modified = true;
            Ok(())
        } else {
            Err(EditorError::InvalidPosition { line, column })
        }
    }

    pub fn insert_str(&mut self, line: usize, column: usize, s: &str) -> Result<()> {
        let char_idx = self.line_col_to_char_idx(line, column)?;
        self.rope.insert(char_idx, s);
        self.modified = true;
        Ok(())
    }

    pub fn delete_range(
        &mut self,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Result<()> {
        let start_idx = self.line_col_to_char_idx(start_line, start_col)?;
        let end_idx = self.line_col_to_char_idx(end_line, end_col)?;
        if start_idx <= end_idx && end_idx <= self.rope.len_chars() {
            self.rope.remove(start_idx..end_idx);
            self.modified = true;
            Ok(())
        } else {
            Err(EditorError::InvalidPosition {
                line: end_line,
                column: end_col,
            })
        }
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, line_idx: usize) -> Result<String> {
        if line_idx < self.line_count() {
            let line = self.rope.line(line_idx);
            Ok(line.to_string())
        } else {
            Err(EditorError::InvalidPosition {
                line: line_idx,
                column: 0,
            })
        }
    }

    pub fn line_len(&self, line_idx: usize) -> Result<usize> {
        if line_idx < self.line_count() {
            let line = self.rope.line(line_idx);
            let len = line.len_chars();
            if len > 0 && line.char(len - 1) == '\n' {
                Ok(len - 1)
            } else {
                Ok(len)
            }
        } else {
            Err(EditorError::InvalidPosition {
                line: line_idx,
                column: 0,
            })
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn content(&self) -> String {
        self.rope.to_string()
    }

    fn line_col_to_char_idx(&self, line: usize, column: usize) -> Result<usize> {
        if line >= self.line_count() {
            return Err(EditorError::InvalidPosition { line, column });
        }

        let line_start_idx = self.rope.line_to_char(line);
        let line_len = self.line_len(line)?;

        if column > line_len {
            return Err(EditorError::InvalidPosition { line, column });
        }

        Ok(line_start_idx + column)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}
