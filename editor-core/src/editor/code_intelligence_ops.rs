use super::state::EditorState;
use crate::error::Result;

impl EditorState {
    pub(super) fn toggle_line_comment(&mut self) -> Result<()> {
        // TODO: Detect language based on file extension
        let comment_prefix = "// ";
        // let comment_len = comment_prefix.len();

        let (start_line, end_line) = if let Some(selection) = &self.selection {
            let start = selection.start().line;
            let end = selection.end().line;
            // If the selection ends at the start of a line, don't include that line
            let end = if selection.end().column == 0 && start != end {
                end - 1
            } else {
                end
            };
            (start, end)
        } else {
            let line = self.cursors.primary().line;
            (line, line)
        };

        // Determine if we should uncomment by checking if lines are already commented
        // We consider "commented" if the line starts with `//` (ignoring whitespace and skipping empty lines)
        let mut lines_to_uncomment = true;
        let mut has_non_empty_lines = false;

        for line_idx in start_line..=end_line {
            let line = self.buffer.line(line_idx)?;
            let trimmed = line.trim_start();
            if trimmed.is_empty() {
                continue;
            }
            has_non_empty_lines = true;

            if !trimmed.starts_with("//") {
                lines_to_uncomment = false;
                break;
            }
        }

        // If all lines are empty, we can choose to comment them (or do nothing).
        // Let's assume we comment.
        if !has_non_empty_lines {
            lines_to_uncomment = false;
        }

        if lines_to_uncomment {
            // Remove comments
            for line_idx in start_line..=end_line {
                let line_content = self.buffer.line(line_idx)?;
                if let Some(first_char_idx) = line_content.find("//") {
                    // Check if it's the start (ignoring whitespace)
                    let prefix = &line_content[..first_char_idx];
                    if prefix.trim().is_empty() {
                        // Found comment at start
                        // Determine length to remove: "//" or "// "
                        let rest = &line_content[first_char_idx + 2..];
                        let remove_len = if rest.starts_with(' ') { 3 } else { 2 };

                        let (l, c) = self
                            .buffer
                            .char_to_line_col(self.buffer.char_index(line_idx, first_char_idx)?)?;
                        self.buffer.delete_range(l, c, l, c + remove_len)?;
                    }
                }
            }
        } else {
            // Add comments
            for line_idx in start_line..=end_line {
                let line_content = self.buffer.line(line_idx)?;
                if line_content.trim().is_empty() && start_line != end_line {
                    continue; // Skip empty lines in block selection
                }

                // Find insertion point (start of content)
                // Or just insert at 0?
                // Usually VSCode inserts at the indentation level of the first line,
                // or just at 0. Let's insert at 0 for simplicity or existing indentation?
                // Let's try insertion at 0 for now.
                self.buffer.insert_str(line_idx, 0, comment_prefix)?;
            }
        }

        Ok(())
    }

    pub(super) fn toggle_block_comment(&mut self) -> Result<()> {
        // TODO: Language detection
        let start_marker = "/*";
        let end_marker = "*/";

        if let Some(selection) = self.selection {
            // Copy is sufficient
            if selection.is_empty() {
                return Ok(());
            }

            let text = self.get_selected_text()?;
            // naive check: is text wrapped?
            let trimmed = text.trim();
            if trimmed.starts_with(start_marker) && trimmed.ends_with(end_marker) {
                // Remove markers
                // This is tricky because `get_selected_text` returns a string, finding indices back in buffer is hard if we modified it.
                // Better: Check start and end positions.
                // This is complex for a first pass.
                // Let's just implement ADDING block comment wrapping implementation for now.

                // If we want to toggle, we really need to check if the selection *bounds* are markers.
                // Let's keep it simple: Just wrap for now. Smart toggling is Phase 4.5 refinement.

                let start = selection.start();
                let end = selection.end();

                // Insert end first to not mess up start index?
                // Actually, inserting at start shifts end.

                self.buffer.insert_str(end.line, end.column, end_marker)?;
                self.buffer
                    .insert_str(start.line, start.column, start_marker)?;

                // Selection needs update? logic usually keeps markers inside or updates.
                // We lose selection tracking if we don't update `self.selection` but `insert_str` updates buffer.
                // Editor update loop might handle visual update.
            } else {
                let start = selection.start();
                let end = selection.end();
                // Same as above
                self.buffer.insert_str(end.line, end.column, end_marker)?;
                self.buffer
                    .insert_str(start.line, start.column, start_marker)?;
            }
        } else {
            // No selection, maybe wrap current word? Or do nothing.
        }
        Ok(())
    }

    pub(super) fn fold_code(&mut self) -> Result<()> {
        // Placeholder
        Ok(())
    }

    pub(super) fn unfold_code(&mut self) -> Result<()> {
        // Placeholder
        Ok(())
    }
}
