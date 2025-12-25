use crate::buffer::Buffer;
use crate::error::Result;
use crate::history_browser::HistoryBrowser;

use super::state::EditorState;

impl EditorState {
    pub fn open_history_browser(&mut self) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation(
                "Cannot open history browser for unsaved buffer".to_string(),
            )
        })?;

        let commits = self.git_history.list_commits(file_path)?;

        self.history_browser = Some(HistoryBrowser::with_commits(commits));
        Ok(())
    }

    pub fn close_history_browser(&mut self) -> Result<()> {
        self.history_browser = None;
        Ok(())
    }

    pub fn get_history_diff(&self) -> Result<Option<String>> {
        use crate::error::EditorError;

        let browser = self.history_browser.as_ref().ok_or_else(|| {
            EditorError::InvalidOperation("History browser is not open".to_string())
        })?;

        let (from_commit, to_commit) = match browser.get_diff_commits() {
            Some(commits) => commits,
            None => return Ok(None),
        };

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation("No file path for current buffer".to_string())
        })?;

        let diff = self.git_history.get_diff_between_commits(
            file_path.parent().unwrap_or(file_path),
            &from_commit.id,
            &to_commit.id,
        )?;

        Ok(Some(diff))
    }

    pub(super) fn history_navigate_next(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.select_next();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_navigate_previous(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.select_previous();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_navigate_first(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.select_first();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_navigate_last(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.select_last();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_page_up(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.page_up();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_page_down(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.page_down();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_select_commit(&mut self, index: usize) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            if browser.select_commit(index) {
                Ok(())
            } else {
                Err(EditorError::InvalidOperation(format!(
                    "Invalid commit index: {}",
                    index
                )))
            }
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_toggle_file_list(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            if browser.selected_file().is_some() {
                browser.clear_selected_file();
            }
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_view_diff(&mut self) -> Result<()> {
        Ok(())
    }

    pub(super) fn history_toggle_side_by_side(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.toggle_side_by_side();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_search(&mut self, query: &str) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.set_search_query(Some(query.to_string()));
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_clear_search(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.clear_search();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_filter_by_file(&mut self, file_pattern: &str) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.set_file_filter(Some(file_pattern.to_string()));
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_clear_file_filter(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.clear_file_filter();
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_restore_commit(&mut self, commit_id: &str) -> Result<()> {
        use crate::error::EditorError;

        if self.buffer().is_modified() {
            return Err(EditorError::InvalidOperation(
                "Cannot restore: buffer has unsaved changes. Save or discard changes first."
                    .to_string(),
            ));
        }

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation("Cannot restore: buffer has no file path".to_string())
        })?;

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        self.git_history.restore_commit(project_path, commit_id)?;

        let buffer = Buffer::from_file(file_path.to_path_buf())?;
        self.buffers[self.current_buffer_index] = buffer;

        self.set_status_message(format!("Restored from commit {}", commit_id));
        Ok(())
    }

    pub(super) fn history_restore_file(&mut self, commit_id: &str, file_path: &str) -> Result<()> {
        use crate::error::EditorError;

        if self.buffer().is_modified() {
            return Err(EditorError::InvalidOperation(
                "Cannot restore: buffer has unsaved changes. Save or discard changes first."
                    .to_string(),
            ));
        }

        let current_file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation("Cannot restore: buffer has no file path".to_string())
        })?;

        let project_path = current_file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        let content =
            self.git_history
                .get_file_content_at_commit(project_path, file_path, commit_id)?;

        let target_path = project_path.join(file_path);
        std::fs::write(&target_path, content).map_err(EditorError::Io)?;

        if &target_path == current_file_path {
            let buffer = Buffer::from_file(target_path)?;
            self.buffers[self.current_buffer_index] = buffer;
        }

        self.set_status_message(format!(
            "Restored file {} from commit {}",
            file_path, commit_id
        ));
        Ok(())
    }

    pub(super) fn history_preview_restore(&mut self, commit_id: &str) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation("Cannot preview: buffer has no file path".to_string())
        })?;

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        let current_file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file name".to_string()))?;

        let content = self.git_history.get_file_content_at_commit(
            project_path,
            current_file_name,
            commit_id,
        )?;

        self.set_status_message(format!(
            "Preview of commit {}: {} bytes",
            commit_id,
            content.len()
        ));
        Ok(())
    }

    pub(super) fn history_set_base_commit(&mut self, index: usize) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            if browser.set_base_commit(Some(index)) {
                self.set_status_message(format!("Set base commit to index {}", index));
                Ok(())
            } else {
                Err(EditorError::InvalidOperation(format!(
                    "Invalid commit index: {}",
                    index
                )))
            }
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_clear_base_commit(&mut self) -> Result<()> {
        use crate::error::EditorError;

        if let Some(browser) = &mut self.history_browser {
            browser.set_base_commit(None);
            self.set_status_message("Cleared base commit".to_string());
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "History browser is not open".to_string(),
            ))
        }
    }

    pub(super) fn history_add_annotation(
        &mut self,
        commit_id: &str,
        annotation: String,
    ) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self
            .buffer()
            .file_path()
            .ok_or_else(|| {
                EditorError::InvalidOperation(
                    "Cannot add annotation: buffer has no file path".to_string(),
                )
            })?
            .to_path_buf();

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        self.git_history
            .add_annotation(project_path, commit_id, annotation)?;

        if let Some(browser) = &mut self.history_browser {
            let commits = self.git_history.list_commits(project_path)?;
            browser.set_commits(commits);
        }

        self.set_status_message(format!("Added annotation to commit {}", commit_id));
        Ok(())
    }

    pub(super) fn history_remove_annotation(&mut self, commit_id: &str) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self
            .buffer()
            .file_path()
            .ok_or_else(|| {
                EditorError::InvalidOperation(
                    "Cannot remove annotation: buffer has no file path".to_string(),
                )
            })?
            .to_path_buf();

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        self.git_history
            .remove_annotation(project_path, commit_id)?;

        if let Some(browser) = &mut self.history_browser {
            let commits = self.git_history.list_commits(project_path)?;
            browser.set_commits(commits);
        }

        self.set_status_message(format!("Removed annotation from commit {}", commit_id));
        Ok(())
    }

    pub(super) fn show_history_stats(&mut self) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation(
                "Cannot show history stats for unsaved buffer".to_string(),
            )
        })?;

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        let stats = self.git_history.get_history_stats(project_path)?;
        self.history_stats = Some(stats);
        Ok(())
    }

    pub(super) fn cleanup_history(&mut self) -> Result<()> {
        use crate::error::EditorError;

        let file_path = self.buffer().file_path().ok_or_else(|| {
            EditorError::InvalidOperation("Cannot cleanup history for unsaved buffer".to_string())
        })?;

        let project_path = file_path
            .parent()
            .ok_or_else(|| EditorError::InvalidOperation("Invalid file path".to_string()))?;

        let stats = self.git_history.cleanup_old_commits(project_path)?;
        self.cleanup_stats = Some(stats);
        Ok(())
    }
}
