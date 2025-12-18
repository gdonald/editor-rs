use crate::{CommitInfo, EditorError, Result};
use git2::{Oid, Repository};
use std::path::Path;
use std::process::Command;

use super::repository::GitHistoryManager;

pub struct CleanupStats {
    pub commits_before: usize,
    pub commits_after: usize,
    pub size_before: u64,
    pub size_after: u64,
}

impl GitHistoryManager {
    pub fn auto_cleanup_if_needed(&self, project_path: &Path) -> Result<Option<CleanupStats>> {
        if !self.auto_cleanup_enabled() {
            return Ok(None);
        }

        if self.retention_policy() == &crate::git_history::RetentionPolicy::Forever {
            return Ok(None);
        }

        let stats = self.cleanup_old_commits(project_path)?;

        if stats.commits_before == stats.commits_after {
            return Ok(None);
        }

        Ok(Some(stats))
    }

    pub fn cleanup_old_commits(&self, project_path: &Path) -> Result<CleanupStats> {
        let repo = self.open_repository(project_path)?;
        let commits_before_list = self.list_commits(project_path)?;
        let commits_before = commits_before_list.len();
        let size_before = self.get_repo_size(project_path)?;

        let commits_to_retain: Vec<CommitInfo> = commits_before_list
            .iter()
            .filter(|commit| {
                self.should_retain_commit(project_path, commit)
                    .unwrap_or(true)
            })
            .cloned()
            .collect();

        if commits_to_retain.len() == commits_before {
            return Ok(CleanupStats {
                commits_before,
                commits_after: commits_before,
                size_before,
                size_after: size_before,
            });
        }

        self.rebuild_history_with_commits(&repo, project_path, &commits_to_retain)?;

        self.expire_reflog_and_prune(project_path)?;

        let commits_after = self.list_commits(project_path)?.len();
        let size_after = self.get_repo_size(project_path)?;

        Ok(CleanupStats {
            commits_before,
            commits_after,
            size_before,
            size_after,
        })
    }

    fn rebuild_history_with_commits(
        &self,
        repo: &Repository,
        _project_path: &Path,
        commits_to_retain: &[CommitInfo],
    ) -> Result<()> {
        if commits_to_retain.is_empty() {
            return Ok(());
        }

        let newest_commit_id = &commits_to_retain[0].id;
        let oldest_commit_id = &commits_to_retain[commits_to_retain.len() - 1].id;

        let newest_oid =
            Oid::from_str(newest_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let oldest_oid =
            Oid::from_str(oldest_commit_id).map_err(|e| EditorError::Git(e.to_string()))?;

        repo.set_head_detached(newest_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let oldest_commit = repo
            .find_commit(oldest_oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        if oldest_commit.parent_count() > 0 {
            let graft_path = repo.path().join("info").join("grafts");
            std::fs::create_dir_all(graft_path.parent().unwrap()).map_err(EditorError::Io)?;

            let graft_content = format!("{}\n", oldest_oid);
            std::fs::write(&graft_path, graft_content).map_err(EditorError::Io)?;
        }

        let repo_path = repo
            .path()
            .parent()
            .ok_or_else(|| EditorError::Git("Could not determine repository path".to_string()))?;

        let mut cmd = Command::new("git");
        cmd.current_dir(repo_path);
        cmd.args(["reflog", "expire", "--expire=now", "--all"]);

        let output = cmd
            .output()
            .map_err(|e| EditorError::Git(format!("Failed to expire reflog: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: reflog expire had issues: {}", stderr);
        }

        Ok(())
    }

    fn expire_reflog_and_prune(&self, project_path: &Path) -> Result<()> {
        let repo = self.open_repository(project_path)?;
        let repo_path = repo
            .path()
            .parent()
            .ok_or_else(|| EditorError::Git("Could not determine repository path".to_string()))?;

        let mut cmd = Command::new("git");
        cmd.current_dir(repo_path);
        cmd.args(["gc", "--prune=now", "--aggressive"]);

        let output = cmd
            .output()
            .map_err(|e| EditorError::Git(format!("Failed to run gc: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EditorError::Git(format!("git gc failed: {}", stderr)));
        }

        Ok(())
    }
}
