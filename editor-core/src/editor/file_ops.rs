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

        if self.auto_commit_enabled {
            if let Some(project_path) = path.parent() {
                match self.git_history.auto_commit_on_save(project_path, &path) {
                    Ok(commit_result) => {
                        if !commit_result.skipped_files.is_empty() {
                            let file_name =
                                path.file_name().and_then(|n| n.to_str()).unwrap_or("file");
                            self.status_message = format!(
                                "File saved (large file {} skipped from history)",
                                file_name
                            );
                        } else {
                            self.status_message = "File saved".to_string();
                        }
                    }
                    Err(e) => {
                        self.status_message = format!("File saved (git history error: {})", e);
                    }
                }

                if let Ok(Some(stats)) = self.git_history.auto_cleanup_if_needed(project_path) {
                    self.cleanup_stats = Some(stats);
                }
            } else {
                self.status_message = "File saved".to_string();
            }
        } else {
            self.status_message = "File saved".to_string();
        }

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
            let file_count = saved_files.len();

            if self.auto_commit_enabled {
                if let Some(first_file) = saved_files.first() {
                    if let Some(project_path) = first_file.parent() {
                        let file_refs: Vec<&PathBuf> = saved_files.iter().collect();
                        match self
                            .git_history
                            .auto_commit_on_save_multiple(project_path, &file_refs)
                        {
                            Ok(commit_result) => {
                                if !commit_result.skipped_files.is_empty() {
                                    let skipped_count = commit_result.skipped_files.len();
                                    if file_count == 1 {
                                        self.status_message =
                                            "File saved (large file skipped from history)"
                                                .to_string();
                                    } else if skipped_count == file_count {
                                        self.status_message = format!(
                                            "{} files saved (all skipped from history: too large)",
                                            file_count
                                        );
                                    } else {
                                        self.status_message = format!(
                                            "{} files saved ({} large files skipped from history)",
                                            file_count, skipped_count
                                        );
                                    }
                                } else {
                                    self.status_message = if file_count == 1 {
                                        "File saved".to_string()
                                    } else {
                                        format!("{} files saved", file_count)
                                    };
                                }
                            }
                            Err(e) => {
                                self.status_message = if file_count == 1 {
                                    format!("File saved (git history error: {})", e)
                                } else {
                                    format!("{} files saved (git history error: {})", file_count, e)
                                };
                            }
                        }

                        if let Ok(Some(stats)) =
                            self.git_history.auto_cleanup_if_needed(project_path)
                        {
                            self.cleanup_stats = Some(stats);
                        }
                    } else {
                        self.status_message = if file_count == 1 {
                            "File saved".to_string()
                        } else {
                            format!("{} files saved", file_count)
                        };
                    }
                } else {
                    self.status_message = if file_count == 1 {
                        "File saved".to_string()
                    } else {
                        format!("{} files saved", file_count)
                    };
                }
            } else {
                self.status_message = if file_count == 1 {
                    "File saved".to_string()
                } else {
                    format!("{} files saved", file_count)
                };
            }
        } else {
            self.status_message = "No modified files to save".to_string();
        }

        Ok(())
    }
}
