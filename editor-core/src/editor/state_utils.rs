use crate::cursor::CursorPosition;
use crate::error::Result;

use super::state::EditorState;

impl EditorState {
    pub(super) fn map_cursors<F>(&mut self, mut f: F) -> Result<()>
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

    pub(super) fn map_cursors_descending<F>(&mut self, mut f: F) -> Result<()>
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

    pub(super) fn validate_position(&self, position: CursorPosition) -> Result<()> {
        use crate::error::EditorError;

        if position.line >= self.buffer().line_count() {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        let line_len = self.buffer().line_len(position.line)?;
        if position.column > line_len {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        Ok(())
    }

    pub(super) fn indentation_for_line(&self, line_idx: usize) -> Result<String> {
        let line = self.buffer().line(line_idx)?;
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

    pub(super) fn wrap_line_to_width(&self, line: &str, width: usize) -> Vec<String> {
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

    pub(super) fn clamp_cursors_after_edit(&mut self) -> Result<()> {
        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for mut pos in self.cursors.positions().to_vec() {
            let line_count = self.buffer().line_count();
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

            let line_len = self.buffer().line_len(pos.line)?;
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

pub(super) fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}
