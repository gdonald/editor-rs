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
    overwrite_mode: bool,
    soft_wrap_width: Option<usize>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursor: CursorPosition::zero(),
            viewport_top: 0,
            status_message: String::new(),
            overwrite_mode: false,
            soft_wrap_width: None,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let buffer = Buffer::from_file(path)?;
        Ok(Self {
            buffer,
            cursor: CursorPosition::zero(),
            viewport_top: 0,
            status_message: String::new(),
            overwrite_mode: false,
            soft_wrap_width: None,
        })
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        match command {
            Command::InsertChar(ch) => self.insert_char(ch),
            Command::DeleteChar => self.delete_char(),
            Command::Backspace => self.backspace(),
            Command::NewLine => self.new_line(),
            Command::DeleteLine => self.delete_line(),
            Command::Indent => self.indent_line(),
            Command::Dedent => self.dedent_line(),

            Command::MoveCursorUp => self.move_cursor_up(),
            Command::MoveCursorDown => self.move_cursor_down(),
            Command::MoveCursorLeft => self.move_cursor_left(),
            Command::MoveCursorRight => self.move_cursor_right(),
            Command::MoveToStartOfLine => self.move_to_start_of_line(),
            Command::MoveToEndOfLine => self.move_to_end_of_line(),
            Command::MoveToStartOfFile => self.move_to_start_of_file(),
            Command::MoveToEndOfFile => self.move_to_end_of_file(),
            Command::MoveCursorWordLeft => self.move_cursor_word_left(),
            Command::MoveCursorWordRight => self.move_cursor_word_right(),
            Command::PageUp => self.page_up(20),
            Command::PageDown => self.page_down(20),

            Command::ToggleOverwriteMode => self.toggle_overwrite_mode(),
            Command::HardWrap(width) => self.hard_wrap(width),
            Command::SetSoftWrap(width) => self.set_soft_wrap(width),
            Command::TrimTrailingWhitespace => self.trim_trailing_whitespace(),

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
        if ch == '\n' {
            return self.new_line();
        }

        let line_len = self.buffer.line_len(self.cursor.line)?;
        if self.overwrite_mode && self.cursor.column < line_len {
            self.buffer
                .delete_char(self.cursor.line, self.cursor.column)?;
        }

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
        let indent = self.current_line_indentation()?;

        self.buffer
            .insert_char(self.cursor.line, self.cursor.column, '\n')?;
        self.cursor.line += 1;
        self.cursor.column = 0;

        if !indent.is_empty() {
            self.buffer
                .insert_str(self.cursor.line, self.cursor.column, &indent)?;
            self.cursor.column = indent.chars().count();
        }

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

    fn indent_line(&mut self) -> Result<()> {
        let indent = "    ";
        self.buffer.insert_str(self.cursor.line, 0, indent)?;
        self.cursor.column += indent.len();
        Ok(())
    }

    fn dedent_line(&mut self) -> Result<()> {
        let line = self.buffer.line(self.cursor.line)?;
        let trimmed = line.trim_end_matches('\n');
        let mut remove_count = 0;

        for ch in trimmed.chars() {
            match ch {
                ' ' if remove_count < 4 => remove_count += 1,
                '\t' => {
                    remove_count = 1;
                    break;
                }
                _ => break,
            }
        }

        if remove_count > 0 {
            self.buffer
                .delete_range(self.cursor.line, 0, self.cursor.line, remove_count)?;

            if self.cursor.column < remove_count {
                self.cursor.column = 0;
            } else {
                self.cursor.column -= remove_count;
            }
        }

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

    fn move_cursor_word_left(&mut self) -> Result<()> {
        if self.buffer.len_chars() == 0 {
            return Ok(());
        }

        let mut idx = self
            .buffer
            .char_index(self.cursor.line, self.cursor.column)?;

        if idx == 0 {
            return Ok(());
        }

        idx -= 1;

        while !is_word_char(self.buffer.char_at(idx).unwrap()) {
            if idx == 0 {
                let (line, column) = self.buffer.char_to_line_col(0)?;
                self.cursor.line = line;
                self.cursor.column = column;
                return Ok(());
            }
            idx -= 1;
        }

        while idx > 0 && is_word_char(self.buffer.char_at(idx - 1).unwrap()) {
            idx -= 1;
        }

        let (line, column) = self.buffer.char_to_line_col(idx)?;
        self.cursor.line = line;
        self.cursor.column = column;
        Ok(())
    }

    fn move_cursor_word_right(&mut self) -> Result<()> {
        if self.buffer.len_chars() == 0 {
            return Ok(());
        }

        let mut idx = self
            .buffer
            .char_index(self.cursor.line, self.cursor.column)?;
        let total = self.buffer.len_chars();

        if idx >= total {
            return Ok(());
        }

        if is_word_char(self.buffer.char_at(idx).unwrap()) {
            while idx < total && is_word_char(self.buffer.char_at(idx).unwrap()) {
                idx += 1;
            }
        }

        while idx < total && !is_word_char(self.buffer.char_at(idx).unwrap()) {
            idx += 1;
        }

        let (line, column) = self.buffer.char_to_line_col(idx)?;
        self.cursor.line = line;
        self.cursor.column = column;
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

    fn toggle_overwrite_mode(&mut self) -> Result<()> {
        self.overwrite_mode = !self.overwrite_mode;
        Ok(())
    }

    fn hard_wrap(&mut self, width: usize) -> Result<()> {
        if width == 0 {
            return Err(EditorError::InvalidOperation(
                "Wrap width must be greater than zero".to_string(),
            ));
        }

        let content = self.buffer.content();
        let ends_with_newline = content.ends_with('\n');
        let mut wrapped_lines = Vec::new();

        for line in content.split('\n') {
            let chunks = self.wrap_line_to_width(line, width);
            if chunks.is_empty() {
                wrapped_lines.push(String::new());
            } else {
                wrapped_lines.extend(chunks);
            }
        }

        let mut new_content = wrapped_lines.join("\n");
        if ends_with_newline {
            new_content.push('\n');
        }

        self.buffer.set_content(new_content);
        self.clamp_cursor_after_edit()?;
        Ok(())
    }

    fn set_soft_wrap(&mut self, width: usize) -> Result<()> {
        if width == 0 {
            self.soft_wrap_width = None;
        } else {
            self.soft_wrap_width = Some(width);
        }
        Ok(())
    }

    fn trim_trailing_whitespace(&mut self) -> Result<()> {
        let content = self.buffer.content();
        let ends_with_newline = content.ends_with('\n');

        let lines: Vec<String> = content
            .split('\n')
            .map(|line| line.trim_end_matches([' ', '\t']).to_string())
            .collect();

        let mut new_content = lines.join("\n");
        if ends_with_newline && !new_content.ends_with('\n') {
            new_content.push('\n');
        }

        self.buffer.set_content(new_content);
        self.clamp_cursor_after_edit()?;
        Ok(())
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

    pub fn overwrite_mode(&self) -> bool {
        self.overwrite_mode
    }

    pub fn soft_wrap_width(&self) -> Option<usize> {
        self.soft_wrap_width
    }

    pub fn soft_wrapped_lines(&self) -> Vec<String> {
        let content = self.buffer.content();
        let width = self.soft_wrap_width;

        let mut lines = Vec::new();
        for line in content.split('\n') {
            if let Some(wrap_width) = width {
                if wrap_width > 0 {
                    lines.extend(self.wrap_line_to_width(line, wrap_width));
                    continue;
                }
            }
            lines.push(line.to_string());
        }

        lines
    }

    fn current_line_indentation(&self) -> Result<String> {
        let line = self.buffer.line(self.cursor.line)?;
        let trimmed = line.trim_end_matches('\n');
        let mut indent = String::new();

        for ch in trimmed.chars() {
            if ch == ' ' || ch == '\t' {
                indent.push(ch);
            } else {
                break;
            }
        }

        Ok(indent)
    }

    fn wrap_line_to_width(&self, line: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![line.to_string()];
        }

        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut count = 0;

        for ch in line.chars() {
            current.push(ch);
            count += 1;

            if count == width {
                chunks.push(current);
                current = String::new();
                count = 0;
            }
        }

        if !current.is_empty() || line.is_empty() {
            chunks.push(current);
        }

        chunks
    }

    fn clamp_cursor_after_edit(&mut self) -> Result<()> {
        if self.cursor.line >= self.buffer.line_count() {
            self.cursor.line = self.buffer.line_count().saturating_sub(1);
        }

        let line_len = self.buffer.line_len(self.cursor.line)?;
        if self.cursor.column > line_len {
            self.cursor.column = line_len;
        }

        Ok(())
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}
