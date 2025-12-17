use crate::{EditorError, Result};
use git2::{Diff, DiffOptions, Oid};
use std::path::Path;

use super::repository::GitHistoryManager;

impl GitHistoryManager {
    pub fn get_diff_between_commits(
        &self,
        project_path: &Path,
        from_commit_id: &str,
        to_commit_id: &str,
    ) -> Result<String> {
        let repo = self.open_repository(project_path)?;

        let from_oid =
            Oid::from_str(from_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let to_oid = Oid::from_str(to_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;

        let from_commit = repo
            .find_commit(from_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;
        let to_commit = repo
            .find_commit(to_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let from_tree = from_commit
            .tree()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        let to_tree = to_commit
            .tree()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let diff = repo
            .diff_tree_to_tree(Some(&from_tree), Some(&to_tree), None)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        self.format_diff(&diff)
    }

    pub fn get_file_diff_between_commits(
        &self,
        project_path: &Path,
        file_path: &str,
        from_commit_id: &str,
        to_commit_id: &str,
    ) -> Result<String> {
        let repo = self.open_repository(project_path)?;

        let from_oid =
            Oid::from_str(from_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let to_oid = Oid::from_str(to_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;

        let from_commit = repo
            .find_commit(from_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;
        let to_commit = repo
            .find_commit(to_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let from_tree = from_commit
            .tree()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        let to_tree = to_commit
            .tree()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let mut opts = DiffOptions::new();
        opts.pathspec(file_path);

        let diff = repo
            .diff_tree_to_tree(Some(&from_tree), Some(&to_tree), Some(&mut opts))
            .map_err(|e| EditorError::Git(e.to_string()))?;

        self.format_diff(&diff)
    }

    pub(crate) fn format_diff(&self, diff: &Diff) -> Result<String> {
        let mut diff_text = String::new();

        diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
            let origin = line.origin();
            let content = std::str::from_utf8(line.content()).unwrap_or("");

            match origin {
                '+' | '-' | ' ' => {
                    diff_text.push(origin);
                    diff_text.push_str(content);
                }
                _ => {
                    diff_text.push_str(content);
                }
            }

            true
        })
        .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(diff_text)
    }
}
