use super::state::EditorState;
use crate::buffer::Buffer;
use crate::cursor::CursorPosition;
use crate::error::Result;
use crate::session::{OpenFileState, Session};

impl EditorState {
    pub fn capture_file_state(&self) -> Option<OpenFileState> {
        self.buffer().file_path().map(|path| OpenFileState {
            path: path.to_path_buf(),
            cursor_line: self.cursors.primary().line,
            cursor_column: self.cursors.primary().column,
            viewport_top: self.viewport_top,
            active: true,
        })
    }

    pub fn restore_from_file_state(&mut self, file_state: &OpenFileState) -> Result<()> {
        let buffer = Buffer::from_file(file_state.path.clone())?;
        self.buffers.push(buffer);
        self.current_buffer_index = self.buffers.len() - 1;

        let cursor = CursorPosition::new(file_state.cursor_line, file_state.cursor_column);
        self.cursors.reset_to(cursor);
        self.viewport_top = file_state.viewport_top;

        self.clamp_cursors_after_edit()?;

        let line_count = self.buffer().line_count();
        if self.viewport_top >= line_count {
            self.viewport_top = line_count.saturating_sub(1);
        }

        Ok(())
    }

    pub fn save_session_state(&self, session: &mut Session) {
        if let Some(file_state) = self.capture_file_state() {
            session.add_open_file(
                file_state.path.clone(),
                CursorPosition::new(file_state.cursor_line, file_state.cursor_column),
                file_state.viewport_top,
            );
            session.set_active_file(&file_state.path);
            session.add_to_recent_files(file_state.path);
        }
    }
}
