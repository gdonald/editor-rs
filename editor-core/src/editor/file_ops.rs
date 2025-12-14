use super::state::EditorState;
use crate::buffer::Buffer;
use crate::cursor::CursorPosition;
use crate::error::Result;
use std::path::PathBuf;

impl EditorState {
    pub(super) fn open_file(&mut self, path: PathBuf) -> Result<()> {
        let buffer = Buffer::from_file(path)?;
        self.buffers.push(buffer);
        self.current_buffer_index = self.buffers.len() - 1;
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    pub(super) fn save(&mut self) -> Result<()> {
        self.save_all()
    }

    pub(super) fn save_as(&mut self, path: PathBuf) -> Result<()> {
        self.buffer_mut().save_as(path.clone())?;

        if let Some(project_path) = path.parent() {
            let _ = self.git_history.auto_commit_on_save(project_path, &path);
        }

        self.status_message = "File saved".to_string();
        Ok(())
    }

    pub(super) fn new_buffer(&mut self) -> Result<()> {
        self.buffers.push(Buffer::new());
        self.current_buffer_index = self.buffers.len() - 1;
        self.cursors.reset_to(CursorPosition::zero());
        self.viewport_top = 0;
        Ok(())
    }

    pub(super) fn save_all(&mut self) -> Result<()> {
        let mut saved_files = Vec::new();

        for buffer in &mut self.buffers {
            if buffer.is_modified() && buffer.file_path().is_some() {
                buffer.save()?;
                if let Some(file_path) = buffer.file_path() {
                    saved_files.push(file_path.clone());
                }
            }
        }

        if !saved_files.is_empty() {
            if let Some(first_file) = saved_files.first() {
                if let Some(project_path) = first_file.parent() {
                    let file_refs: Vec<&PathBuf> = saved_files.iter().collect();
                    let _ = self
                        .git_history
                        .auto_commit_on_save_multiple(project_path, &file_refs);
                }
            }

            let file_count = saved_files.len();
            self.status_message = if file_count == 1 {
                "File saved".to_string()
            } else {
                format!("{} files saved", file_count)
            };
        } else {
            self.status_message = "No modified files to save".to_string();
        }

        Ok(())
    }
}
