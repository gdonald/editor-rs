use crate::buffer::Buffer;
use crate::command::Command;
use crate::cursor::{CursorPosition, MultiCursor};
use crate::error::{EditorError, Result};
use std::collections::HashMap;
use std::path::PathBuf;

pub struct EditorState {
    buffer: Buffer,
    cursors: MultiCursor,
    viewport_top: usize,
    status_message: String,
    overwrite_mode: bool,
    soft_wrap_width: Option<usize>,
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursors: MultiCursor::new(),
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
            cursors: MultiCursor::new(),
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
            Command::AddCursor(position) => self.add_cursor(position),
            Command::RemoveCursor(index) => self.remove_cursor(index),
            Command::ClearSecondaryCursors => self.clear_secondary_cursors(),

            _ => Err(EditorError::InvalidOperation(
                "Command not yet implemented".to_string(),
            )),
        }
    }

    fn map_cursors<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut EditorState, CursorPosition) -> Result<CursorPosition>,
    {
        let positions = self.cursors.positions().to_vec();
        let mut updated = Vec::with_capacity(positions.len());
        for pos in positions {
            updated.push(f(self, pos)?);
        }
        self.cursors.set_positions(updated);
        Ok(())
    }

    fn map_cursors_descending<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut EditorState, CursorPosition) -> Result<CursorPosition>,
    {
        let mut positions = self.cursors.positions().to_vec();
        positions.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        positions.reverse();

        let mut updated = Vec::with_capacity(positions.len());
        for pos in positions {
            updated.push(f(self, pos)?);
        }

        self.cursors.set_positions(updated);
        Ok(())
    }

    fn insert_char(&mut self, ch: char) -> Result<()> {
        if ch == '\n' {
            return self.new_line();
        }

        self.map_cursors_descending(|state, mut pos| {
            let line_len = state.buffer.line_len(pos.line)?;
            if state.overwrite_mode && pos.column < line_len {
                state.buffer.delete_char(pos.line, pos.column)?;
            }

            state.buffer.insert_char(pos.line, pos.column, ch)?;
            pos.column += 1;
            Ok(pos)
        })
    }

    fn delete_char(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, pos| {
            state.buffer.delete_char(pos.line, pos.column)?;
            Ok(pos)
        })
    }

    fn backspace(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, mut pos| {
            if pos.column > 0 {
                pos.column -= 1;
                state.buffer.delete_char(pos.line, pos.column)?;
                return Ok(pos);
            }

            if pos.line > 0 {
                let prev_line_len = state.buffer.line_len(pos.line - 1)?;
                state.buffer.delete_char(pos.line - 1, prev_line_len)?;
                pos.line -= 1;
                pos.column = prev_line_len;
            }

            Ok(pos)
        })
    }

    fn new_line(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, mut pos| {
            let indent = state.indentation_for_line(pos.line)?;

            state.buffer.insert_char(pos.line, pos.column, '\n')?;
            pos.line += 1;
            pos.column = 0;

            if !indent.is_empty() {
                state.buffer.insert_str(pos.line, pos.column, &indent)?;
                pos.column = indent.chars().count();
            }

            Ok(pos)
        })
    }

    fn delete_line(&mut self) -> Result<()> {
        if self.buffer.line_count() == 0 {
            return Ok(());
        }

        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();
        lines.reverse();

        for line in lines {
            let is_last_line = line == self.buffer.line_count().saturating_sub(1);

            if is_last_line {
                let line_len = self.buffer.line_len(line)?;
                self.buffer.delete_range(line, 0, line, line_len)?;
            } else {
                self.buffer.delete_range(line, 0, line + 1, 0)?;
            }
        }

        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for pos in self.cursors.positions() {
            let line = pos.line.min(self.buffer.line_count().saturating_sub(1));
            positions.push(CursorPosition::new(line, 0));
        }

        self.cursors.set_positions(positions);
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    fn indent_line(&mut self) -> Result<()> {
        let indent = "    ";
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        for line in lines {
            self.buffer.insert_str(line, 0, indent)?;
        }

        self.map_cursors(|state, mut pos| {
            pos.column += indent.len();
            let line_len = state.buffer.line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }
            Ok(pos)
        })
    }

    fn dedent_line(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        let mut removed_by_line = HashMap::new();

        for line_idx in &lines {
            let line = self.buffer.line(*line_idx)?;
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
                    .delete_range(*line_idx, 0, *line_idx, remove_count)?;
                removed_by_line.insert(*line_idx, remove_count);
            }
        }

        self.map_cursors(|state, mut pos| {
            let line_len = state.buffer.line_len(pos.line)?;

            if let Some(amount) = removed_by_line.get(&pos.line) {
                if pos.column < *amount {
                    pos.column = 0;
                } else {
                    pos.column -= *amount;
                }
            }

            if pos.column > line_len {
                pos.column = line_len;
            }

            Ok(pos)
        })
    }

    fn move_cursor_up(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if pos.line > 0 {
                pos.line -= 1;
                let line_len = state.buffer.line_len(pos.line)?;
                if pos.column > line_len {
                    pos.column = line_len;
                }
            }
            Ok(pos)
        })
    }

    fn move_cursor_down(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if pos.line + 1 < state.buffer.line_count() {
                pos.line += 1;
                let line_len = state.buffer.line_len(pos.line)?;
                if pos.column > line_len {
                    pos.column = line_len;
                }
            }
            Ok(pos)
        })
    }

    fn move_cursor_left(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if pos.column > 0 {
                pos.column -= 1;
            } else if pos.line > 0 {
                pos.line -= 1;
                pos.column = state.buffer.line_len(pos.line)?;
            }
            Ok(pos)
        })
    }

    fn move_cursor_right(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            let line_len = state.buffer.line_len(pos.line)?;
            if pos.column < line_len {
                pos.column += 1;
            } else if pos.line + 1 < state.buffer.line_count() {
                pos.line += 1;
                pos.column = 0;
            }
            Ok(pos)
        })
    }

    fn move_cursor_word_left(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if state.buffer.len_chars() == 0 {
                return Ok(pos);
            }

            let mut idx = state.buffer.char_index(pos.line, pos.column)?;

            if idx == 0 {
                pos.line = 0;
                pos.column = 0;
                return Ok(pos);
            }

            idx -= 1;

            while !is_word_char(state.buffer.char_at(idx).unwrap()) {
                if idx == 0 {
                    let (line, column) = state.buffer.char_to_line_col(0)?;
                    pos.line = line;
                    pos.column = column;
                    return Ok(pos);
                }
                idx -= 1;
            }

            while idx > 0 && is_word_char(state.buffer.char_at(idx - 1).unwrap()) {
                idx -= 1;
            }

            let (line, column) = state.buffer.char_to_line_col(idx)?;
            pos.line = line;
            pos.column = column;
            Ok(pos)
        })
    }

    fn move_cursor_word_right(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if state.buffer.len_chars() == 0 {
                return Ok(pos);
            }

            let mut idx = state.buffer.char_index(pos.line, pos.column)?;
            let total = state.buffer.len_chars();

            if idx >= total {
                return Ok(pos);
            }

            if is_word_char(state.buffer.char_at(idx).unwrap()) {
                while idx < total && is_word_char(state.buffer.char_at(idx).unwrap()) {
                    idx += 1;
                }
            }

            while idx < total && !is_word_char(state.buffer.char_at(idx).unwrap()) {
                idx += 1;
            }

            let (line, column) = state.buffer.char_to_line_col(idx)?;
            pos.line = line;
            pos.column = column;
            Ok(pos)
        })
    }

    fn move_to_start_of_line(&mut self) -> Result<()> {
        self.map_cursors(|_, mut pos| {
            pos.column = 0;
            Ok(pos)
        })
    }

    fn move_to_end_of_line(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            let line_len = state.buffer.line_len(pos.line)?;
            pos.column = line_len;
            Ok(pos)
        })
    }

    fn move_to_start_of_file(&mut self) -> Result<()> {
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    fn move_to_end_of_file(&mut self) -> Result<()> {
        if self.buffer.line_count() > 0 {
            let last_line = self.buffer.line_count() - 1;
            let last_len = self.buffer.line_len(last_line)?;
            self.cursors
                .reset_to(CursorPosition::new(last_line, last_len));
        }
        Ok(())
    }

    fn page_up(&mut self, lines: usize) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            if pos.line >= lines {
                pos.line -= lines;
            } else {
                pos.line = 0;
            }

            let line_len = state.buffer.line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }

            Ok(pos)
        })
    }

    fn page_down(&mut self, lines: usize) -> Result<()> {
        let max_line = if self.buffer.line_count() > 0 {
            self.buffer.line_count() - 1
        } else {
            0
        };

        self.map_cursors(|state, mut pos| {
            if pos.line + lines <= max_line {
                pos.line += lines;
            } else {
                pos.line = max_line;
            }

            let line_len = state.buffer.line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }

            Ok(pos)
        })
    }

    fn goto_line(&mut self, line: usize) -> Result<()> {
        if line < self.buffer.line_count() {
            self.cursors
                .set_positions(vec![CursorPosition::new(line, 0)]);
            return Ok(());
        }
        Err(EditorError::InvalidPosition { line, column: 0 })
    }

    fn add_cursor(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;
        self.cursors.add_cursor(position);
        Ok(())
    }

    fn remove_cursor(&mut self, index: usize) -> Result<()> {
        self.cursors.remove_cursor(index);
        self.clamp_cursors_after_edit()
    }

    fn clear_secondary_cursors(&mut self) -> Result<()> {
        let primary = *self.cursors.primary();
        self.cursors.reset_to(primary);
        Ok(())
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
        self.clamp_cursors_after_edit()?;
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
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    fn open_file(&mut self, path: PathBuf) -> Result<()> {
        self.buffer = Buffer::from_file(path)?;
        self.cursors.reset_to(CursorPosition::zero());
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
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn cursor(&self) -> &CursorPosition {
        self.cursors.primary()
    }

    pub fn cursors(&self) -> &[CursorPosition] {
        self.cursors.positions()
    }

    pub fn cursor_count(&self) -> usize {
        self.cursors.positions().len()
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
        if viewport_height == 0 {
            return;
        }

        let primary = self.cursors.primary();
        if primary.line < self.viewport_top {
            self.viewport_top = primary.line;
        } else if primary.line >= self.viewport_top + viewport_height {
            self.viewport_top = primary.line.saturating_sub(viewport_height - 1);
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

    fn validate_position(&self, position: CursorPosition) -> Result<()> {
        if position.line >= self.buffer.line_count() {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        let line_len = self.buffer.line_len(position.line)?;
        if position.column > line_len {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        Ok(())
    }

    fn indentation_for_line(&self, line_idx: usize) -> Result<String> {
        let line = self.buffer.line(line_idx)?;
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

    fn clamp_cursors_after_edit(&mut self) -> Result<()> {
        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for mut pos in self.cursors.positions().to_vec() {
            let line_count = self.buffer.line_count();
            if line_count == 0 {
                pos.line = 0;
                pos.column = 0;
                positions.push(pos);
                continue;
            }

            let last_line = line_count - 1;
            if pos.line > last_line {
                pos.line = last_line;
            }

            let line_len = self.buffer.line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }

            positions.push(pos);
        }

        if positions.is_empty() {
            positions.push(CursorPosition::zero());
        }

        self.cursors.set_positions(positions);
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
