use super::state::{is_word_char, EditorState};
use crate::cursor::CursorPosition;
use crate::error::{EditorError, Result};

impl EditorState {
    pub(super) fn move_cursor_up(&mut self) -> Result<()> {
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

    pub(super) fn move_cursor_down(&mut self) -> Result<()> {
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

    pub(super) fn move_cursor_left(&mut self) -> Result<()> {
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

    pub(super) fn move_cursor_right(&mut self) -> Result<()> {
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

    pub(super) fn move_cursor_word_left(&mut self) -> Result<()> {
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

    pub(super) fn move_cursor_word_right(&mut self) -> Result<()> {
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

    pub(super) fn move_to_start_of_line(&mut self) -> Result<()> {
        self.map_cursors(|_, mut pos| {
            pos.column = 0;
            Ok(pos)
        })
    }

    pub(super) fn move_to_end_of_line(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            let line_len = state.buffer.line_len(pos.line)?;
            pos.column = line_len;
            Ok(pos)
        })
    }

    pub(super) fn move_to_start_of_file(&mut self) -> Result<()> {
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    pub(super) fn move_to_end_of_file(&mut self) -> Result<()> {
        if self.buffer.line_count() > 0 {
            let last_line = self.buffer.line_count() - 1;
            let last_len = self.buffer.line_len(last_line)?;
            self.cursors
                .reset_to(CursorPosition::new(last_line, last_len));
        }
        Ok(())
    }

    pub(super) fn page_up(&mut self, lines: usize) -> Result<()> {
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

    pub(super) fn page_down(&mut self, lines: usize) -> Result<()> {
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

    pub(super) fn goto_line(&mut self, line: usize) -> Result<()> {
        if line < self.buffer.line_count() {
            self.cursors
                .set_positions(vec![CursorPosition::new(line, 0)]);
            return Ok(());
        }
        Err(EditorError::InvalidPosition { line, column: 0 })
    }

    pub(super) fn add_cursor(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;
        self.cursors.add_cursor(position);
        Ok(())
    }

    pub(super) fn remove_cursor(&mut self, index: usize) -> Result<()> {
        self.cursors.remove_cursor(index);
        self.clamp_cursors_after_edit()
    }

    pub(super) fn clear_secondary_cursors(&mut self) -> Result<()> {
        let primary = *self.cursors.primary();
        self.cursors.reset_to(primary);
        Ok(())
    }

    pub fn adjust_viewport(&mut self, viewport_height: usize) {
        if viewport_height == 0 {
            return;
        }

        let primary = self.cursors.primary();
        let offset = self.scroll_offset.min(viewport_height / 2);

        if primary.line < self.viewport_top + offset {
            self.viewport_top = primary.line.saturating_sub(offset);
        } else if primary.line >= self.viewport_top + viewport_height - offset {
            self.viewport_top = primary.line.saturating_sub(viewport_height - offset - 1);
        }
    }

    pub(super) fn jump_to_matching_bracket(&mut self) -> Result<()> {
        self.map_cursors(|state, pos| {
            let char_idx = state.buffer.char_index(pos.line, pos.column)?;

            if char_idx >= state.buffer.len_chars() {
                return Ok(pos);
            }

            let current_char = state.buffer.char_at(char_idx);
            if current_char.is_none() {
                return Ok(pos);
            }
            let current_char = current_char.unwrap();

            let (opening, closing, direction) = match current_char {
                '(' => ('(', ')', 1),
                ')' => ('(', ')', -1),
                '[' => ('[', ']', 1),
                ']' => ('[', ']', -1),
                '{' => ('{', '}', 1),
                '}' => ('{', '}', -1),
                '<' => ('<', '>', 1),
                '>' => ('<', '>', -1),
                _ => return Ok(pos),
            };

            let mut depth = 0;
            let mut idx = char_idx as isize;
            let len = state.buffer.len_chars() as isize;

            loop {
                if direction == 1 {
                    if idx >= len {
                        break;
                    }
                } else if idx < 0 {
                    break;
                }

                if let Some(ch) = state.buffer.char_at(idx as usize) {
                    if ch == opening {
                        if direction == 1 {
                            depth += 1;
                        } else {
                            depth -= 1;
                        }
                    } else if ch == closing {
                        if direction == 1 {
                            depth -= 1;
                        } else {
                            depth += 1;
                        }
                    }

                    if depth == 0 && idx != char_idx as isize {
                        let (line, column) = state.buffer.char_to_line_col(idx as usize)?;
                        return Ok(CursorPosition::new(line, column));
                    }
                }

                idx += direction;
            }

            Ok(pos)
        })
    }
}
