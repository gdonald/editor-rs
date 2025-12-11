use crate::bookmark::Bookmark;
use crate::editor::state::EditorState;
use crate::error::{EditorError, Result};

impl EditorState {
    pub(super) fn toggle_bookmark(&mut self) -> Result<()> {
        let position = *self.cursor();
        self.validate_position(position)?;

        let added = self.bookmarks.toggle_bookmark(position);
        if added {
            self.set_status_message(format!(
                "Bookmark added at line {}:{}",
                position.line + 1,
                position.column
            ));
        } else {
            self.set_status_message(format!(
                "Bookmark removed from line {}:{}",
                position.line + 1,
                position.column
            ));
        }

        Ok(())
    }

    pub(super) fn add_named_bookmark(&mut self, name: String) -> Result<()> {
        let position = *self.cursor();
        self.validate_position(position)?;

        if let Some(existing) = self.bookmarks.find_bookmark_at_position(position) {
            self.bookmarks.remove_bookmark(existing);
        }

        let bookmark = Bookmark::with_name(position, name.clone());
        self.bookmarks.add_bookmark(bookmark);

        self.set_status_message(format!(
            "Named bookmark '{}' added at line {}:{}",
            name,
            position.line + 1,
            position.column
        ));

        Ok(())
    }

    pub(super) fn remove_bookmark(&mut self, index: usize) -> Result<()> {
        if let Some(bookmark) = self.bookmarks.remove_bookmark(index) {
            if let Some(name) = bookmark.name {
                self.set_status_message(format!("Removed bookmark '{}'", name));
            } else {
                self.set_status_message(format!(
                    "Removed bookmark at line {}:{}",
                    bookmark.position.line + 1,
                    bookmark.position.column
                ));
            }
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(format!(
                "No bookmark at index {}",
                index
            )))
        }
    }

    pub(super) fn jump_to_bookmark(&mut self, index: usize) -> Result<()> {
        if let Some(bookmark) = self.bookmarks.get_bookmark(index) {
            let position = bookmark.position;
            self.validate_position(position)?;
            self.cursors.reset_to(position);
            self.selection = None;

            if let Some(ref name) = bookmark.name {
                self.set_status_message(format!(
                    "Jumped to bookmark '{}' at line {}:{}",
                    name,
                    position.line + 1,
                    position.column
                ));
            } else {
                self.set_status_message(format!(
                    "Jumped to bookmark at line {}:{}",
                    position.line + 1,
                    position.column
                ));
            }

            Ok(())
        } else {
            Err(EditorError::InvalidOperation(format!(
                "No bookmark at index {}",
                index
            )))
        }
    }

    pub(super) fn jump_to_named_bookmark(&mut self, name: String) -> Result<()> {
        if let Some(bookmark) = self.bookmarks.get_bookmark_by_name(&name) {
            let position = bookmark.position;
            self.validate_position(position)?;
            self.cursors.reset_to(position);
            self.selection = None;

            self.set_status_message(format!(
                "Jumped to bookmark '{}' at line {}:{}",
                name,
                position.line + 1,
                position.column
            ));

            Ok(())
        } else {
            Err(EditorError::InvalidOperation(format!(
                "No bookmark named '{}'",
                name
            )))
        }
    }

    pub(super) fn next_bookmark(&mut self) -> Result<()> {
        let current_position = *self.cursor();

        if let Some(bookmark) = self.bookmarks.next_bookmark(current_position) {
            let position = bookmark.position;
            self.validate_position(position)?;
            self.cursors.reset_to(position);
            self.selection = None;

            if let Some(ref name) = bookmark.name {
                self.set_status_message(format!(
                    "Next bookmark '{}' at line {}:{}",
                    name,
                    position.line + 1,
                    position.column
                ));
            } else {
                self.set_status_message(format!(
                    "Next bookmark at line {}:{}",
                    position.line + 1,
                    position.column
                ));
            }

            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "No bookmarks after current position".to_string(),
            ))
        }
    }

    pub(super) fn previous_bookmark(&mut self) -> Result<()> {
        let current_position = *self.cursor();

        if let Some(bookmark) = self.bookmarks.previous_bookmark(current_position) {
            let position = bookmark.position;
            self.validate_position(position)?;
            self.cursors.reset_to(position);
            self.selection = None;

            if let Some(ref name) = bookmark.name {
                self.set_status_message(format!(
                    "Previous bookmark '{}' at line {}:{}",
                    name,
                    position.line + 1,
                    position.column
                ));
            } else {
                self.set_status_message(format!(
                    "Previous bookmark at line {}:{}",
                    position.line + 1,
                    position.column
                ));
            }

            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "No bookmarks before current position".to_string(),
            ))
        }
    }

    pub(super) fn clear_all_bookmarks(&mut self) -> Result<()> {
        let count = self.bookmarks.bookmarks().len();
        self.bookmarks.clear_all();
        self.set_status_message(format!("Cleared {} bookmarks", count));
        Ok(())
    }
}
