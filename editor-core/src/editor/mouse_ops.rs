use super::state::{is_word_char, EditorState};
use crate::cursor::CursorPosition;
use crate::error::Result;
use crate::selection::{Selection, SelectionMode};

impl EditorState {
    pub(super) fn mouse_click(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;
        self.cursors.reset_to(position);
        self.selection = None;
        Ok(())
    }

    pub(super) fn mouse_drag_start(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;
        self.cursors.reset_to(position);
        let mode = if self.block_selection_mode {
            SelectionMode::Block
        } else {
            SelectionMode::Normal
        };
        self.selection = Some(if mode == SelectionMode::Block {
            Selection::new_block(position, position)
        } else {
            Selection::new(position, position)
        });
        Ok(())
    }

    pub(super) fn mouse_drag(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;
        if let Some(selection) = &self.selection {
            let anchor = selection.anchor;
            let mode = selection.mode;
            self.cursors.reset_to(position);
            self.selection = Some(if mode == SelectionMode::Block {
                Selection::new_block(anchor, position)
            } else {
                Selection::new(anchor, position)
            });
        } else {
            self.mouse_drag_start(position)?;
        }
        Ok(())
    }

    pub(super) fn mouse_drag_end(&mut self, position: CursorPosition) -> Result<()> {
        self.mouse_drag(position)?;
        Ok(())
    }

    pub(super) fn mouse_double_click(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;

        let char_idx = self.buffer().char_index(position.line, position.column)?;
        let total_chars = self.buffer().len_chars();

        if total_chars == 0 {
            return Ok(());
        }

        let current_char = self.buffer().char_at(char_idx);
        if current_char.is_none() || !is_word_char(current_char.unwrap()) {
            return Ok(());
        }

        let mut start_idx = char_idx;
        while start_idx > 0 {
            if let Some(ch) = self.buffer().char_at(start_idx - 1) {
                if !is_word_char(ch) {
                    break;
                }
                start_idx -= 1;
            } else {
                break;
            }
        }

        let mut end_idx = char_idx;
        while end_idx < total_chars {
            if let Some(ch) = self.buffer().char_at(end_idx) {
                if !is_word_char(ch) {
                    break;
                }
                end_idx += 1;
            } else {
                break;
            }
        }

        let (start_line, start_col) = self.buffer().char_to_line_col(start_idx)?;
        let (end_line, end_col) = self.buffer().char_to_line_col(end_idx)?;

        let start_pos = CursorPosition::new(start_line, start_col);
        let end_pos = CursorPosition::new(end_line, end_col);

        self.cursors.reset_to(end_pos);
        self.selection = Some(Selection::new(start_pos, end_pos));

        Ok(())
    }

    pub(super) fn mouse_triple_click(&mut self, position: CursorPosition) -> Result<()> {
        self.validate_position(position)?;

        let line_len = self.buffer().line_len(position.line)?;
        let start_pos = CursorPosition::new(position.line, 0);
        let end_pos = CursorPosition::new(position.line, line_len);

        self.cursors.reset_to(end_pos);
        self.selection = Some(Selection::new(start_pos, end_pos));

        Ok(())
    }

    pub(super) fn toggle_block_selection(&mut self) -> Result<()> {
        self.block_selection_mode = !self.block_selection_mode;
        if let Some(selection) = &mut self.selection {
            let new_mode = if self.block_selection_mode {
                SelectionMode::Block
            } else {
                SelectionMode::Normal
            };
            *selection = if new_mode == SelectionMode::Block {
                Selection::new_block(selection.anchor, selection.cursor)
            } else {
                Selection::new(selection.anchor, selection.cursor)
            };
        }
        Ok(())
    }
}
