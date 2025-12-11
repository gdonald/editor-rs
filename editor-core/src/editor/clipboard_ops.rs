use super::state::EditorState;
use crate::cursor::CursorPosition;
use crate::error::Result;
use crate::selection::{Selection, SelectionMode};

impl EditorState {
    pub(super) fn selection_start(&mut self) -> Result<()> {
        let cursor_pos = *self.cursors.primary();
        let mode = if self.block_selection_mode {
            SelectionMode::Block
        } else {
            SelectionMode::Normal
        };
        self.selection = Some(if mode == SelectionMode::Block {
            Selection::new_block(cursor_pos, cursor_pos)
        } else {
            Selection::new(cursor_pos, cursor_pos)
        });
        Ok(())
    }

    pub(super) fn selection_end(&mut self) -> Result<()> {
        if let Some(selection) = &self.selection {
            let anchor = selection.anchor;
            let cursor_pos = *self.cursors.primary();
            let mode = selection.mode;
            self.selection = Some(if mode == SelectionMode::Block {
                Selection::new_block(anchor, cursor_pos)
            } else {
                Selection::new(anchor, cursor_pos)
            });
        }
        Ok(())
    }

    pub(super) fn get_selected_text(&self) -> Result<String> {
        if let Some(selection) = &self.selection {
            if selection.is_empty() {
                return Ok(String::new());
            }

            match selection.mode {
                SelectionMode::Normal => {
                    let start = selection.start();
                    let end = selection.end();
                    self.get_text_range(start, end)
                }
                SelectionMode::Block => {
                    let start = selection.start();
                    let end = selection.end();
                    let min_col = selection.anchor.column.min(selection.cursor.column);
                    let max_col = selection.anchor.column.max(selection.cursor.column);

                    let mut result = String::new();
                    for line in start.line..=end.line {
                        let line_len = self.buffer.line_len(line)?;
                        let start_col = min_col.min(line_len);
                        let end_col = max_col.min(line_len);

                        let start_pos = CursorPosition::new(line, start_col);
                        let end_pos = CursorPosition::new(line, end_col);

                        if !result.is_empty() {
                            result.push('\n');
                        }
                        result.push_str(&self.get_text_range(start_pos, end_pos)?);
                    }
                    Ok(result)
                }
            }
        } else {
            Ok(String::new())
        }
    }

    fn get_text_range(&self, start: CursorPosition, end: CursorPosition) -> Result<String> {
        let start_idx = self.buffer.char_index(start.line, start.column)?;
        let end_idx = self.buffer.char_index(end.line, end.column)?;

        let mut result = String::new();
        for i in start_idx..end_idx {
            if let Some(ch) = self.buffer.char_at(i) {
                result.push(ch);
            }
        }
        Ok(result)
    }

    pub(super) fn copy(&mut self) -> Result<()> {
        let text = self.get_selected_text()?;
        if !text.is_empty() {
            self.clipboard.set_text(&text)?;
        }
        Ok(())
    }

    pub(super) fn cut(&mut self) -> Result<()> {
        if let Some(selection) = &self.selection {
            if !selection.is_empty() {
                let text = self.get_selected_text()?;
                self.clipboard.set_text(&text)?;

                match selection.mode {
                    SelectionMode::Normal => {
                        let start = selection.start();
                        let end = selection.end();
                        self.delete_range(start, end)?;
                    }
                    SelectionMode::Block => {
                        let start = selection.start();
                        let end = selection.end();
                        let min_col = selection.anchor.column.min(selection.cursor.column);
                        let max_col = selection.anchor.column.max(selection.cursor.column);

                        for line in (start.line..=end.line).rev() {
                            let line_len = self.buffer.line_len(line)?;
                            let start_col = min_col.min(line_len);
                            let end_col = max_col.min(line_len);

                            if start_col < end_col {
                                let start_pos = CursorPosition::new(line, start_col);
                                let end_pos = CursorPosition::new(line, end_col);
                                self.delete_range(start_pos, end_pos)?;
                            }
                        }
                    }
                }

                self.selection = None;
            }
        }
        Ok(())
    }

    pub(super) fn paste(&mut self) -> Result<()> {
        let text = self.clipboard.get_text()?;
        if !text.is_empty() {
            if self.selection.is_some() {
                self.cut()?;
            }
            let cursor_pos = *self.cursors.primary();
            self.insert_text_at(cursor_pos, &text)?;
        }
        Ok(())
    }

    fn delete_range(&mut self, start: CursorPosition, end: CursorPosition) -> Result<()> {
        let start_idx = self.buffer.char_index(start.line, start.column)?;
        let end_idx = self.buffer.char_index(end.line, end.column)?;

        for _ in start_idx..end_idx {
            let (line, col) = self.buffer.char_to_line_col(start_idx)?;
            self.buffer.delete_char(line, col)?;
        }

        self.cursors.reset_to(start);
        Ok(())
    }

    fn insert_text_at(&mut self, position: CursorPosition, text: &str) -> Result<()> {
        self.buffer
            .insert_str(position.line, position.column, text)?;

        let char_idx = self.buffer.char_index(position.line, position.column)?;
        let new_char_idx = char_idx + text.chars().count();
        let (new_line, new_col) = self.buffer.char_to_line_col(new_char_idx)?;
        self.cursors
            .reset_to(CursorPosition::new(new_line, new_col));
        Ok(())
    }
}
