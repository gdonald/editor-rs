use super::state::EditorState;
use crate::error::{EditorError, Result};

pub struct VirtualViewport {
    pub start_line: usize,
    pub end_line: usize,
    pub visible_lines: Vec<String>,
}

impl EditorState {
    pub fn get_virtual_viewport(&self, viewport_height: usize) -> VirtualViewport {
        let total_lines = self.buffer().line_count();
        let start_line = self.viewport_top;
        let end_line = (start_line + viewport_height).min(total_lines);

        let mut visible_lines = Vec::with_capacity(viewport_height);
        for line_idx in start_line..end_line {
            if let Ok(line) = self.buffer().line(line_idx) {
                visible_lines.push(line);
            }
        }

        VirtualViewport {
            start_line,
            end_line,
            visible_lines,
        }
    }

    pub fn adjust_viewport_to_cursor(&mut self, viewport_height: usize) {
        let cursor_line = self.cursor().line;
        let offset = self.scroll_offset;

        if cursor_line < self.viewport_top + offset {
            self.viewport_top = cursor_line.saturating_sub(offset);
        } else if cursor_line >= self.viewport_top + viewport_height.saturating_sub(offset) {
            self.viewport_top = cursor_line
                .saturating_sub(viewport_height.saturating_sub(offset))
                .saturating_add(1);
        }
    }

    pub(super) fn toggle_overwrite_mode(&mut self) -> Result<()> {
        self.overwrite_mode = !self.overwrite_mode;
        Ok(())
    }

    pub(super) fn hard_wrap(&mut self, width: usize) -> Result<()> {
        if width == 0 {
            return Err(EditorError::InvalidOperation(
                "Wrap width must be greater than zero".to_string(),
            ));
        }

        let content = self.buffer().content();
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

        self.buffer_mut().set_content(new_content)?;
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    pub(super) fn set_soft_wrap(&mut self, width: usize) -> Result<()> {
        if width == 0 {
            self.soft_wrap_width = None;
        } else {
            self.soft_wrap_width = Some(width);
        }
        Ok(())
    }

    pub fn soft_wrapped_lines(&self) -> Vec<String> {
        let content = self.buffer().content();
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
}
