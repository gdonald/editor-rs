use crate::{CommitInfo, Result};
use std::path::Path;

use super::repository::GitHistoryManager;
use super::types::RetentionPolicy;

impl GitHistoryManager {
    pub fn should_retain_commit(&self, project_path: &Path, commit: &CommitInfo) -> Result<bool> {
        match self.retention_policy() {
            RetentionPolicy::Forever => Ok(true),
            RetentionPolicy::Days(days) => {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;

                let age_days = (now - commit.timestamp) / (24 * 60 * 60);
                Ok(age_days <= *days as i64)
            }
            RetentionPolicy::Commits(max_commits) => {
                let commits = self.list_commits(project_path)?;
                let commit_index = commits
                    .iter()
                    .position(|c| c.id == commit.id)
                    .unwrap_or(commits.len());
                Ok(commit_index < *max_commits)
            }
            RetentionPolicy::Size(max_size_bytes) => {
                let repo_size = self.get_repo_size(project_path)?;
                Ok(repo_size <= *max_size_bytes)
            }
        }
    }
}
