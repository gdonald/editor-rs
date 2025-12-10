use super::state::EditorState;
use crate::error::{EditorError, Result};

impl EditorState {
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

        self.buffer.set_content(new_content)?;
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
}
