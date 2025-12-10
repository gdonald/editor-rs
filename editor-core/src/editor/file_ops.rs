use super::state::EditorState;
use crate::buffer::Buffer;
use crate::cursor::CursorPosition;
use crate::error::Result;
use std::path::PathBuf;

impl EditorState {
    pub(super) fn open_file(&mut self, path: PathBuf) -> Result<()> {
        self.buffer = Buffer::from_file(path)?;
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    pub(super) fn save(&mut self) -> Result<()> {
        self.buffer.save()?;
        self.status_message = "File saved".to_string();
        Ok(())
    }

    pub(super) fn save_as(&mut self, path: PathBuf) -> Result<()> {
        self.buffer.save_as(path)?;
        self.status_message = "File saved".to_string();
        Ok(())
    }

    pub(super) fn new_buffer(&mut self) -> Result<()> {
        self.buffer = Buffer::new();
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }
}
