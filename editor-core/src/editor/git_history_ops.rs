use super::state::EditorState;
use crate::error::Result;

impl EditorState {
    pub fn perform_startup_cleanup(&mut self) -> Result<()> {
        if !self.auto_commit_enabled {
            return Ok(());
        }

        let current_file_path = match self.buffer().file_path() {
            Some(path) => path.to_path_buf(),
            None => return Ok(()),
        };

        if let Some(project_path) = current_file_path.parent() {
            if let Ok(Some(stats)) = self.git_history.auto_cleanup_if_needed(project_path) {
                self.cleanup_stats = Some(stats);
            }
        }

        Ok(())
    }
}
