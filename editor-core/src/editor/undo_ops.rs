use super::state::EditorState;
use crate::error::Result;
use crate::history::Edit;

impl EditorState {
    pub fn undo(&mut self) -> Result<()> {
        let entry = self.history.undo()?;

        for edit in entry.edits.iter().rev() {
            let inverted = edit.invert();
            self.apply_edit(&inverted)?;
        }

        self.cursors.set_positions(entry.cursor_before.clone());
        self.selection = entry.selection_before;

        self.set_status_message("Undo".to_string());
        Ok(())
    }

    pub fn redo(&mut self) -> Result<()> {
        let entry = self.history.redo()?;

        for edit in &entry.edits {
            self.apply_edit(edit)?;
        }

        self.cursors.set_positions(entry.cursor_after.clone());
        self.selection = entry.selection_after;

        self.set_status_message("Redo".to_string());
        Ok(())
    }

    fn apply_edit(&mut self, edit: &Edit) -> Result<()> {
        match edit {
            Edit::Insert { position, text } => {
                self.buffer
                    .insert_str(position.line, position.column, text)?;
            }
            Edit::Delete { position, text } => {
                let end_line = position.line;
                let end_column = position.column + text.chars().count();
                self.buffer
                    .delete_range(position.line, position.column, end_line, end_column)?;
            }
            Edit::Replace {
                position,
                old_text,
                new_text,
            } => {
                let end_line = position.line;
                let end_column = position.column + old_text.chars().count();
                self.buffer
                    .delete_range(position.line, position.column, end_line, end_column)?;
                self.buffer
                    .insert_str(position.line, position.column, new_text)?;
            }
        }
        Ok(())
    }

    pub fn can_undo(&self) -> bool {
        self.history.can_undo()
    }

    pub fn can_redo(&self) -> bool {
        self.history.can_redo()
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn history_memory_usage(&self) -> usize {
        self.history.memory_usage()
    }

    pub fn undo_stack_len(&self) -> usize {
        self.history.undo_stack_len()
    }

    pub fn redo_stack_len(&self) -> usize {
        self.history.redo_stack_len()
    }
}
