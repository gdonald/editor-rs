use super::state::EditorState;
use crate::command::CaseMode;
use crate::cursor::CursorPosition;
use crate::error::Result;
use crate::history::{Edit, HistoryEntry};
use std::collections::HashMap;

impl EditorState {
    pub(super) fn insert_char(&mut self, ch: char) -> Result<()> {
        if ch == '\n' {
            return self.new_line();
        }

        let content_before = self.buffer().content();
        let cursor_before = self.cursors.positions().to_vec();
        let selection_before = self.selection;

        self.map_cursors_descending(|state, mut pos| {
            let line_len = state.buffer_mut().line_len(pos.line)?;
            if state.overwrite_mode && pos.column < line_len {
                state.buffer_mut().delete_char(pos.line, pos.column)?;
            }

            state.buffer_mut().insert_char(pos.line, pos.column, ch)?;
            pos.column += 1;
            Ok(pos)
        })?;

        let content_after = self.buffer().content();
        let cursor_after = self.cursors.positions().to_vec();
        let selection_after = self.selection;

        if content_before != content_after {
            let edit = Edit::Replace {
                position: CursorPosition::new(0, 0),
                old_text: content_before,
                new_text: content_after,
            };

            let entry = HistoryEntry::new(
                vec![edit],
                cursor_before,
                cursor_after,
                selection_before,
                selection_after,
            )
            .with_grouped(true);

            self.history.push(entry);
        }

        Ok(())
    }

    pub(super) fn insert_char_with_auto_close(&mut self, ch: char) -> Result<()> {
        if ch == '\n' {
            return self.new_line();
        }

        let closing_char = match ch {
            '(' => Some(')'),
            '[' => Some(']'),
            '{' => Some('}'),
            '"' => Some('"'),
            '\'' => Some('\''),
            _ => None,
        };

        if let Some(close) = closing_char {
            self.map_cursors_descending(|state, mut pos| {
                let line_len = state.buffer_mut().line_len(pos.line)?;
                let should_auto_close = if pos.column < line_len {
                    let char_idx = state.buffer_mut().char_index(pos.line, pos.column)?;
                    match state.buffer_mut().char_at(char_idx) {
                        Some(c) => c.is_whitespace() || c == ')' || c == ']' || c == '}',
                        None => true,
                    }
                } else {
                    true
                };

                if should_auto_close {
                    state.buffer_mut().insert_char(pos.line, pos.column, ch)?;
                    state
                        .buffer_mut()
                        .insert_char(pos.line, pos.column + 1, close)?;
                    pos.column += 1;
                } else {
                    state.buffer_mut().insert_char(pos.line, pos.column, ch)?;
                    pos.column += 1;
                }

                Ok(pos)
            })
        } else {
            self.insert_char(ch)
        }
    }

    pub(super) fn delete_char(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, pos| {
            state.buffer_mut().delete_char(pos.line, pos.column)?;
            Ok(pos)
        })
    }

    pub(super) fn backspace(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, mut pos| {
            if pos.column > 0 {
                pos.column -= 1;
                state.buffer_mut().delete_char(pos.line, pos.column)?;
                return Ok(pos);
            }

            if pos.line > 0 {
                let prev_line_len = state.buffer_mut().line_len(pos.line - 1)?;
                state
                    .buffer_mut()
                    .delete_char(pos.line - 1, prev_line_len)?;
                pos.line -= 1;
                pos.column = prev_line_len;
            }

            Ok(pos)
        })
    }

    pub(super) fn new_line(&mut self) -> Result<()> {
        self.map_cursors_descending(|state, mut pos| {
            let indent = state.indentation_for_line(pos.line)?;

            state.buffer_mut().insert_char(pos.line, pos.column, '\n')?;
            pos.line += 1;
            pos.column = 0;

            if !indent.is_empty() {
                state
                    .buffer_mut()
                    .insert_str(pos.line, pos.column, &indent)?;
                pos.column = indent.chars().count();
            }

            Ok(pos)
        })
    }

    pub(super) fn delete_line(&mut self) -> Result<()> {
        if self.buffer().line_count() == 0 {
            return Ok(());
        }

        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();
        lines.reverse();

        for line in lines {
            let is_last_line = line == self.buffer().line_count().saturating_sub(1);

            if is_last_line {
                let line_len = self.buffer().line_len(line)?;
                self.buffer_mut().delete_range(line, 0, line, line_len)?;
            } else {
                self.buffer_mut().delete_range(line, 0, line + 1, 0)?;
            }
        }

        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for pos in self.cursors.positions() {
            let line = pos.line.min(self.buffer().line_count().saturating_sub(1));
            positions.push(CursorPosition::new(line, 0));
        }

        self.cursors.set_positions(positions);
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    pub(super) fn indent_line(&mut self) -> Result<()> {
        let indent = "    ";
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        for line in lines {
            self.buffer_mut().insert_str(line, 0, indent)?;
        }

        self.map_cursors(|state, mut pos| {
            pos.column += indent.len();
            let line_len = state.buffer_mut().line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }
            Ok(pos)
        })
    }

    pub(super) fn dedent_line(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        let mut removed_by_line = HashMap::new();

        for line_idx in &lines {
            let line = self.buffer().line(*line_idx)?;
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
                self.buffer_mut()
                    .delete_range(*line_idx, 0, *line_idx, remove_count)?;
                removed_by_line.insert(*line_idx, remove_count);
            }
        }

        self.map_cursors(|state, mut pos| {
            let line_len = state.buffer_mut().line_len(pos.line)?;

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

    pub(super) fn duplicate_line(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();
        lines.reverse();

        for line in lines {
            let line_content = self.buffer().line(line)?.trim_end_matches('\n').to_string();
            let insert_line = line + 1;
            if insert_line >= self.buffer().line_count() {
                let line_len = self.buffer().line_len(line)?;
                self.buffer_mut().insert_char(line, line_len, '\n')?;
                self.buffer_mut()
                    .insert_str(insert_line, 0, &line_content)?;
            } else {
                self.buffer_mut()
                    .insert_str(insert_line, 0, &line_content)?;
                self.buffer_mut()
                    .insert_char(insert_line, line_content.chars().count(), '\n')?;
            }
        }

        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for pos in self.cursors.positions() {
            positions.push(CursorPosition::new(pos.line + 1, pos.column));
        }
        self.cursors.set_positions(positions);
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    pub(super) fn move_lines_up(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        if lines.is_empty() || lines[0] == 0 {
            return Ok(());
        }

        let total_lines = self.buffer().line_count();
        let mut all_lines: Vec<String> = Vec::new();
        for i in 0..total_lines {
            let line = self.buffer().line(i)?;
            all_lines.push(line.trim_end_matches('\n').to_string());
        }

        for line in lines.iter() {
            if *line > 0 && *line < all_lines.len() {
                all_lines.swap(line - 1, *line);
            }
        }

        let new_content = all_lines.join("\n");
        self.buffer_mut().set_content(new_content)?;

        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for pos in self.cursors.positions() {
            if pos.line > 0 {
                positions.push(CursorPosition::new(pos.line - 1, pos.column));
            } else {
                positions.push(*pos);
            }
        }
        self.cursors.set_positions(positions);
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    pub(super) fn move_lines_down(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();
        lines.reverse();

        let total_lines = self.buffer().line_count();
        let last_line = total_lines.saturating_sub(1);
        if lines.is_empty() || lines[0] == last_line {
            return Ok(());
        }

        let mut all_lines: Vec<String> = Vec::new();
        for i in 0..total_lines {
            let line = self.buffer().line(i)?;
            all_lines.push(line.trim_end_matches('\n').to_string());
        }

        for line in lines.iter() {
            if *line < all_lines.len().saturating_sub(1) {
                all_lines.swap(*line, line + 1);
            }
        }

        let new_content = all_lines.join("\n");
        self.buffer_mut().set_content(new_content)?;

        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for pos in self.cursors.positions() {
            let new_line = (pos.line + 1).min(last_line);
            positions.push(CursorPosition::new(new_line, pos.column));
        }
        self.cursors.set_positions(positions);
        self.clamp_cursors_after_edit()?;
        Ok(())
    }

    pub(super) fn join_lines(&mut self) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();
        lines.reverse();

        for line in lines.iter() {
            if *line >= self.buffer().line_count().saturating_sub(1) {
                continue;
            }

            let current_line = self
                .buffer()
                .line(*line)?
                .trim_end_matches('\n')
                .to_string();
            let next_line = self
                .buffer()
                .line(line + 1)?
                .trim_end_matches('\n')
                .to_string();
            let trimmed_next = next_line.trim_start();

            let joined = if trimmed_next.is_empty() {
                current_line
            } else {
                format!("{} {}", current_line, trimmed_next)
            };

            let current_len = self.buffer().line_len(*line)?;
            let next_len = self.buffer().line_len(line + 1)?;

            self.buffer_mut()
                .delete_range(*line, 0, *line, current_len)?;
            self.buffer_mut().delete_char(*line, 0)?;
            self.buffer_mut().delete_range(*line, 0, *line, next_len)?;
            self.buffer_mut().insert_str(*line, 0, &joined)?;
        }

        Ok(())
    }

    pub(super) fn sort_lines(&mut self, numerical: bool) -> Result<()> {
        let mut lines: Vec<usize> = self.cursors.positions().iter().map(|p| p.line).collect();
        lines.sort_unstable();
        lines.dedup();

        if lines.len() < 2 {
            return Ok(());
        }

        let start_line = lines[0];
        let end_line = lines[lines.len() - 1];

        let mut line_contents: Vec<String> = Vec::new();
        for line_idx in start_line..=end_line {
            let line = self.buffer().line(line_idx)?;
            line_contents.push(line.trim_end_matches('\n').to_string());
        }

        if numerical {
            line_contents.sort_by(|a, b| {
                let a_num = a.trim().parse::<f64>().ok();
                let b_num = b.trim().parse::<f64>().ok();
                match (a_num, b_num) {
                    (Some(av), Some(bv)) => {
                        av.partial_cmp(&bv).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    (Some(_), None) => std::cmp::Ordering::Less,
                    (None, Some(_)) => std::cmp::Ordering::Greater,
                    (None, None) => a.cmp(b),
                }
            });
        } else {
            line_contents.sort();
        }

        for (i, line_idx) in (start_line..=end_line).enumerate() {
            let line_len = self.buffer().line_len(line_idx)?;
            self.buffer_mut()
                .delete_range(line_idx, 0, line_idx, line_len)?;
            self.buffer_mut()
                .insert_str(line_idx, 0, &line_contents[i])?;
        }

        Ok(())
    }

    pub(super) fn change_case(&mut self, mode: CaseMode) -> Result<()> {
        self.map_cursors(|state, pos| {
            let line_content = state
                .buffer()
                .line(pos.line)?
                .trim_end_matches('\n')
                .to_string();
            let transformed = match mode {
                CaseMode::Upper => line_content.to_uppercase(),
                CaseMode::Lower => line_content.to_lowercase(),
                CaseMode::Title => {
                    let mut result = String::new();
                    let mut capitalize_next = true;
                    for ch in line_content.chars() {
                        if ch.is_whitespace() {
                            result.push(ch);
                            capitalize_next = true;
                        } else if capitalize_next {
                            result.push_str(&ch.to_uppercase().to_string());
                            capitalize_next = false;
                        } else {
                            result.push_str(&ch.to_lowercase().to_string());
                        }
                    }
                    result
                }
            };

            let line_len = state.buffer_mut().line_len(pos.line)?;
            state
                .buffer_mut()
                .delete_range(pos.line, 0, pos.line, line_len)?;
            state.buffer_mut().insert_str(pos.line, 0, &transformed)?;

            Ok(pos)
        })
    }

    pub(super) fn transpose_characters(&mut self) -> Result<()> {
        self.map_cursors(|state, mut pos| {
            let line_content = state
                .buffer()
                .line(pos.line)?
                .trim_end_matches('\n')
                .to_string();
            let line_len = line_content.chars().count();

            if line_len < 2 {
                return Ok(pos);
            }

            let mut chars: Vec<char> = line_content.chars().collect();

            if pos.column == 0 || pos.column == 1 {
                chars.swap(0, 1);
                pos.column = 1;
            } else if pos.column >= line_len {
                if line_len >= 2 {
                    chars.swap(line_len - 2, line_len - 1);
                }
            } else {
                chars.swap(pos.column - 2, pos.column - 1);
            }

            let new_line: String = chars.into_iter().collect();
            let old_len = state.buffer_mut().line_len(pos.line)?;
            state
                .buffer_mut()
                .delete_range(pos.line, 0, pos.line, old_len)?;
            state.buffer_mut().insert_str(pos.line, 0, &new_line)?;

            Ok(pos)
        })
    }

    pub(super) fn trim_trailing_whitespace(&mut self) -> Result<()> {
        let content = self.buffer().content();
        let ends_with_newline = content.ends_with('\n');

        let lines: Vec<String> = content
            .split('\n')
            .map(|line| line.trim_end_matches([' ', '\t']).to_string())
            .collect();

        let mut new_content = lines.join("\n");
        if ends_with_newline && !new_content.ends_with('\n') {
            new_content.push('\n');
        }

        self.buffer_mut().set_content(new_content)?;
        self.clamp_cursors_after_edit()?;
        Ok(())
    }
}
