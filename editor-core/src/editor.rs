use crate::buffer::Buffer;
use crate::command::Command;
use crate::cursor::CursorPosition;
use crate::error::{EditorError, Result};
use std::path::PathBuf;

pub struct EditorState {
    buffer: Buffer,
    cursor: CursorPosition,
    viewport_top: usize,
    status_message: String,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: CursorPosition::zero(),
            viewport_top: 0,
            status_message: String::new(),
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let buffer = Buffer::from_file(path)?;
        Ok(Self {
            buffer,
            cursor: CursorPosition::zero(),
            viewport_top: 0,
            status_message: String::new(),
        })
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::InsertChar(ch) => self.insert_char(ch),
            Command::DeleteChar => self.delete_char(),
            Command::Backspace => self.backspace(),
            Command::NewLine => self.new_line(),
            Command::DeleteLine => self.delete_line(),

            Command::MoveCursorUp => self.move_cursor_up(),
            Command::MoveCursorDown => self.move_cursor_down(),
            Command::MoveCursorLeft => self.move_cursor_left(),
            Command::MoveCursorRight => self.move_cursor_right(),
            Command::MoveToStartOfLine => self.move_to_start_of_line(),
            Command::MoveToEndOfLine => self.move_to_end_of_line(),
            Command::MoveToStartOfFile => self.move_to_start_of_file(),
            Command::MoveToEndOfFile => self.move_to_end_of_file(),
            Command::PageUp => self.page_up(20),
            Command::PageDown => self.page_down(20),

            Command::Open(path) => self.open_file(path),
            Command::Save => self.save(),
            Command::SaveAs(path) => self.save_as(path),
            Command::New => self.new_buffer(),
            Command::Close => Ok(()),

            Command::GotoLine(line) => self.goto_line(line),

            _ => Err(EditorError::InvalidOperation(
                "Command not yet implemented".to_string(),
            )),
        }
    }

    fn insert_char(&mut self, ch: char) -> Result<()> {
        self.buffer
            .insert_char(self.cursor.line, self.cursor.column, ch)?;
        self.cursor.column += 1;
        Ok(())
    }

    fn delete_char(&mut self) -> Result<()> {
        self.buffer
            .delete_char(self.cursor.line, self.cursor.column)
    }

    fn backspace(&mut self) -> Result<()> {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
            self.buffer
                .delete_char(self.cursor.line, self.cursor.column)
        } else if self.cursor.line > 0 {
            let prev_line_len = self.buffer.line_len(self.cursor.line - 1)?;
            self.buffer
                .delete_char(self.cursor.line - 1, prev_line_len)?;
            self.cursor.line -= 1;
            self.cursor.column = prev_line_len;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn new_line(&mut self) -> Result<()> {
        self.buffer
            .insert_char(self.cursor.line, self.cursor.column, '\n')?;
        self.cursor.line += 1;
        self.cursor.column = 0;
        Ok(())
    }

    fn delete_line(&mut self) -> Result<()> {
        if self.buffer.line_count() == 0 {
            return Ok(());
        }

        let is_last_line = self.cursor.line == self.buffer.line_count() - 1;

        if is_last_line {
            let line_len = self.buffer.line_len(self.cursor.line)?;
            self.buffer
                .delete_range(self.cursor.line, 0, self.cursor.line, line_len)?;
        } else {
            self.buffer
                .delete_range(self.cursor.line, 0, self.cursor.line + 1, 0)?;
        }

        self.cursor.column = 0;
        Ok(())
    }

    fn move_cursor_up(&mut self) -> Result<()> {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
            let line_len = self.buffer.line_len(self.cursor.line)?;
            if self.cursor.column > line_len {
                self.cursor.column = line_len;
            }
        }
        Ok(())
    }

    fn move_cursor_down(&mut self) -> Result<()> {
        if self.cursor.line < self.buffer.line_count() - 1 {
            self.cursor.line += 1;
            let line_len = self.buffer.line_len(self.cursor.line)?;
            if self.cursor.column > line_len {
                self.cursor.column = line_len;
            }
        }
        Ok(())
    }

    fn move_cursor_left(&mut self) -> Result<()> {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.buffer.line_len(self.cursor.line)?;
        }
        Ok(())
    }

    fn move_cursor_right(&mut self) -> Result<()> {
        let line_len = self.buffer.line_len(self.cursor.line)?;
        if self.cursor.column < line_len {
            self.cursor.column += 1;
        } else if self.cursor.line < self.buffer.line_count() - 1 {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
        Ok(())
    }

    fn move_to_start_of_line(&mut self) -> Result<()> {
        self.cursor.column = 0;
        Ok(())
    }

    fn move_to_end_of_line(&mut self) -> Result<()> {
        let line_len = self.buffer.line_len(self.cursor.line)?;
        self.cursor.column = line_len;
        Ok(())
    }

    fn move_to_start_of_file(&mut self) -> Result<()> {
        self.cursor.line = 0;
        self.cursor.column = 0;
        self.viewport_top = 0;
        Ok(())
    }

    fn move_to_end_of_file(&mut self) -> Result<()> {
        if self.buffer.line_count() > 0 {
            self.cursor.line = self.buffer.line_count() - 1;
            self.cursor.column = self.buffer.line_len(self.cursor.line)?;
        }
        Ok(())
    }

    fn page_up(&mut self, lines: usize) -> Result<()> {
        if self.cursor.line >= lines {
            self.cursor.line -= lines;
        } else {
            self.cursor.line = 0;
        }

        let line_len = self.buffer.line_len(self.cursor.line)?;
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }

        Ok(())
    }

    fn page_down(&mut self, lines: usize) -> Result<()> {
        let max_line = if self.buffer.line_count() > 0 {
            self.buffer.line_count() - 1
        } else {
            0
        };

        if self.cursor.line + lines <= max_line {
            self.cursor.line += lines;
        } else {
            self.cursor.line = max_line;
        }

        let line_len = self.buffer.line_len(self.cursor.line)?;
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }

        Ok(())
    }

    fn goto_line(&mut self, line: usize) -> Result<()> {
        if line < self.buffer.line_count() {
            self.cursor.line = line;
            self.cursor.column = 0;
            Ok(())
        } else {
            Err(EditorError::InvalidPosition { line, column: 0 })
        }
    }

    fn open_file(&mut self, path: PathBuf) -> Result<()> {
        self.buffer = Buffer::from_file(path)?;
        self.cursor = CursorPosition::zero();
        self.viewport_top = 0;
        Ok(())
    }

    fn save(&mut self) -> Result<()> {
        self.buffer.save()?;
        self.status_message = "File saved".to_string();
        Ok(())
    }

    fn save_as(&mut self, path: PathBuf) -> Result<()> {
        self.buffer.save_as(path)?;
        self.status_message = "File saved".to_string();
        Ok(())
    }

    fn new_buffer(&mut self) -> Result<()> {
        self.buffer = Buffer::new();
        self.cursor = CursorPosition::zero();
        self.viewport_top = 0;
        Ok(())
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn cursor(&self) -> &CursorPosition {
        &self.cursor
    }

    pub fn viewport_top(&self) -> usize {
        self.viewport_top
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn adjust_viewport(&mut self, viewport_height: usize) {
        if self.cursor.line < self.viewport_top {
            self.viewport_top = self.cursor.line;
        } else if self.cursor.line >= self.viewport_top + viewport_height {
            self.viewport_top = self.cursor.line - viewport_height + 1;
        }
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
