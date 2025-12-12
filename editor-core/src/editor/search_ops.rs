use super::state::EditorState;
use crate::error::Result;

impl EditorState {
    pub(super) fn search(&mut self, query: String) -> Result<()> {
        if query.is_empty() {
            return Ok(());
        }

        // Add to history if unique from last?
        if self.search_history.last() != Some(&query) {
            self.search_history.push(query.clone());
        }

        self.last_search_query = Some(query.clone());

        // Start search from current cursor position
        let start_pos = self.cursors.primary();
        let start_idx = self.buffer.char_index(start_pos.line, start_pos.column)?;

        // Use find_next from buffer
        // Note: find_next returns generic index. Need to convert to CursorPosition.
        // If match includes current char, should find_next return current position or next?
        // Usually "Find" jumps to next occurrence *after* cursor, or *at* cursor if currently on start of match?
        // Let's say if we are at start of "foo" and search "foo", we probably want to stay there or go to next?
        // Usually "Find" goes to next match. If I type "Find", and I'm at match, effectively "Find Next".
        // But if I just typed query, maybe I want the nearest match including current?

        // Simple heuristic: Search from current position. If found at current position, maybe that's fine.

        let case_sensitive = self.search_options.case_sensitive;
        if let Some(idx) = self.buffer.find_next(&query, start_idx, case_sensitive) {
            self.move_to_match(idx, query.len())?;
        }

        Ok(())
    }

    pub(super) fn next_match(&mut self) -> Result<()> {
        if let Some(query) = self.last_search_query.clone() {
            let start_pos = self.cursors.primary();
            let mut start_idx = self.buffer.char_index(start_pos.line, start_pos.column)?;
            let case_sensitive = self.search_options.case_sensitive;

            // If we are currently sitting on a match, we don't want to find the same match again.
            // We should advance by 1 char? Or length of match?
            // Advancing by 1 char is safe to find overlapping matches?
            // Or usually non-overlapping.
            // Let's advance by 1 char to ensure we find "next" one.
            start_idx += 1;

            if let Some(idx) = self.buffer.find_next(&query, start_idx, case_sensitive) {
                self.move_to_match(idx, query.len())?;
            } else {
                // Wrap around?
                // Roadmap doesn't explicitly splitting 'wrap' but "BASIC search" implies basic navigation.
                // Let's implement wrap.
                if let Some(idx) = self.buffer.find_next(&query, 0, case_sensitive) {
                    self.move_to_match(idx, query.len())?;
                }
            }
        }
        Ok(())
    }

    pub(super) fn previous_match(&mut self) -> Result<()> {
        if let Some(query) = self.last_search_query.clone() {
            let start_pos = self.cursors.primary();
            let original_idx = self.buffer.char_index(start_pos.line, start_pos.column)?;
            let mut search_idx = original_idx;
            let case_sensitive = self.search_options.case_sensitive;

            // Search backwards
            // If we find a match that ENDS at original_idx, we should verify it's the one we want to skip?
            // Yes, if we are at the end of a match, we consider that "current".

            loop {
                if search_idx == 0 {
                    break;
                }

                if let Some(idx) = self
                    .buffer
                    .find_previous(&query, search_idx - 1, case_sensitive)
                {
                    if idx + query.len() == original_idx {
                        // This is the match we are currently at.
                        // Skip it and search before it.
                        search_idx = idx;
                        continue;
                    }

                    self.move_to_match(idx, query.len())?;
                    return Ok(());
                } else {
                    break;
                }
            }

            // Wrap around (search from end)
            let len = self.buffer.len_chars();

            if let Some(idx) = self.buffer.find_previous(&query, len, case_sensitive) {
                // Check if it is same as original (single match case)
                if idx + query.len() == original_idx {
                    // Only one match in file.
                    // Do nothing? Or re-select it?
                    // Let's re-select it to be consistent.
                    self.move_to_match(idx, query.len())?;
                } else {
                    self.move_to_match(idx, query.len())?;
                }
            }
        }
        Ok(())
    }

    fn move_to_match(&mut self, char_idx: usize, len: usize) -> Result<()> {
        let (line, col) = self.buffer.char_to_line_col(char_idx)?;
        // We might want to select the match?
        // Roadmap says "Implement search result highlighting" later.
        // For basic search, let's just move cursor to start of match.
        // Or maybe select it so it is visible?
        // Let's just set cursor for now.

        // Actually, setting selection is standard behavior.
        // "Selection" support exists.

        let (end_line, end_col) = self.buffer.char_to_line_col(char_idx + len)?;

        // Move primary cursor
        use crate::cursor::CursorPosition;
        let start = CursorPosition::new(line, col);
        let end = CursorPosition::new(end_line, end_col);

        // We clear secondary cursors and move primary to end of match
        self.cursors.reset_to(end);

        // Let's select the match.
        use crate::selection::Selection;
        self.selection = Some(Selection::new(start, end));

        Ok(())
    }
}
