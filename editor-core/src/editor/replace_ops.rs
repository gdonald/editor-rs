use super::state::EditorState;
use crate::cursor::CursorPosition;
use crate::error::Result;

impl EditorState {
    pub(super) fn replace_next(&mut self, find: String, replace: String) -> Result<()> {
        if find.is_empty() {
            return Ok(());
        }

        self.add_to_replace_history(&find, &replace);
        self.last_search_query = Some(find.clone());

        let has_match = self.has_active_match(&find)? || self.find_next_occurrence(&find)?;

        if has_match {
            self.replace_current_match(&replace)?;
            self.find_next_after_replace(&find)?;
        }

        Ok(())
    }

    pub(super) fn replace_all(&mut self, find: String, replace: String) -> Result<()> {
        if find.is_empty() {
            return Ok(());
        }

        self.add_to_replace_history(&find, &replace);

        let opts = self.search_options;
        let matches = self.buffer.find_all_advanced(
            &find,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        );

        if matches.is_empty() {
            return Ok(());
        }

        for &match_idx in matches.iter().rev() {
            let match_len = self.get_match_length(&find, match_idx)?;
            let (start_line, start_col) = self.buffer.char_to_line_col(match_idx)?;
            let (end_line, end_col) = self.buffer.char_to_line_col(match_idx + match_len)?;

            self.buffer
                .delete_range(start_line, start_col, end_line, end_col)?;
            self.buffer.insert_str(start_line, start_col, &replace)?;
        }

        self.selection = None;

        Ok(())
    }

    pub(super) fn replace_in_selection(&mut self, find: String, replace: String) -> Result<()> {
        if find.is_empty() {
            return Ok(());
        }

        let selection = match &self.selection {
            Some(sel) => *sel,
            None => return Ok(()),
        };

        self.add_to_replace_history(&find, &replace);

        let start = selection.start();
        let end = selection.end();

        let start_idx = self.buffer.char_index(start.line, start.column)?;
        let end_idx = self.buffer.char_index(end.line, end.column)?;

        let opts = self.search_options;
        let matches = self.buffer.find_in_range(
            &find,
            start_idx,
            end_idx,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        );

        if matches.is_empty() {
            return Ok(());
        }

        for &match_idx in matches.iter().rev() {
            let match_len = self.get_match_length(&find, match_idx)?;
            let (start_line, start_col) = self.buffer.char_to_line_col(match_idx)?;
            let (end_line, end_col) = self.buffer.char_to_line_col(match_idx + match_len)?;

            self.buffer
                .delete_range(start_line, start_col, end_line, end_col)?;
            self.buffer.insert_str(start_line, start_col, &replace)?;
        }

        self.selection = None;

        Ok(())
    }

    fn add_to_replace_history(&mut self, find: &str, replace: &str) {
        let entry = (find.to_string(), replace.to_string());
        if self.replace_history.last() != Some(&entry) {
            self.replace_history.push(entry);
        }
    }

    fn has_active_match(&self, query: &str) -> Result<bool> {
        if let Some(sel) = &self.selection {
            let start = sel.start();
            let end = sel.end();
            let start_idx = self.buffer.char_index(start.line, start.column)?;
            let end_idx = self.buffer.char_index(end.line, end.column)?;

            let selected_len = end_idx - start_idx;
            let expected_len = self.get_match_length(query, start_idx)?;

            if selected_len != expected_len {
                return Ok(false);
            }

            let opts = self.search_options;
            if let Some(match_idx) = self.buffer.find_next_advanced(
                query,
                start_idx,
                opts.case_sensitive,
                opts.use_regex,
                opts.whole_word,
            ) {
                return Ok(match_idx == start_idx);
            }
        }

        Ok(false)
    }

    fn replace_current_match(&mut self, replacement: &str) -> Result<()> {
        if let Some(sel) = &self.selection {
            let start = sel.start();
            let end = sel.end();
            let start_idx = self.buffer.char_index(start.line, start.column)?;

            self.buffer
                .delete_range(start.line, start.column, end.line, end.column)?;
            self.buffer
                .insert_str(start.line, start.column, replacement)?;

            let replacement_len = replacement.chars().count();
            let (new_end_line, new_end_col) =
                self.buffer.char_to_line_col(start_idx + replacement_len)?;

            self.cursors
                .reset_to(CursorPosition::new(new_end_line, new_end_col));
            self.selection = None;
        }

        Ok(())
    }

    fn find_next_after_replace(&mut self, query: &str) -> Result<()> {
        let start_pos = self.cursors.primary();
        let start_idx = self.buffer.char_index(start_pos.line, start_pos.column)?;

        let opts = self.search_options;
        if let Some(idx) = self.buffer.find_next_advanced(
            query,
            start_idx,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        ) {
            let match_len = self.get_match_length(query, idx)?;
            self.move_to_match(idx, match_len)?;
        } else if let Some(idx) = self.buffer.find_next_advanced(
            query,
            0,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        ) {
            let match_len = self.get_match_length(query, idx)?;
            self.move_to_match(idx, match_len)?;
        }

        Ok(())
    }

    fn find_next_occurrence(&mut self, query: &str) -> Result<bool> {
        let start_pos = self.cursors.primary();
        let start_idx = self.buffer.char_index(start_pos.line, start_pos.column)?;

        let opts = self.search_options;
        if let Some(idx) = self.buffer.find_next_advanced(
            query,
            start_idx,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        ) {
            let match_len = self.get_match_length(query, idx)?;
            self.move_to_match(idx, match_len)?;
            return Ok(true);
        }

        if let Some(idx) = self.buffer.find_next_advanced(
            query,
            0,
            opts.case_sensitive,
            opts.use_regex,
            opts.whole_word,
        ) {
            let match_len = self.get_match_length(query, idx)?;
            self.move_to_match(idx, match_len)?;
            return Ok(true);
        }

        Ok(false)
    }
}
