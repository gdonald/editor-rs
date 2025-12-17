use crate::{EditorError, Result};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::repository::GitHistoryManager;

impl GitHistoryManager {
    pub fn run_gc(&self, project_path: &Path, aggressive: bool) -> Result<()> {
        let repo = self.open_repository(project_path)?;

        let repo_path = repo
            .path()
            .parent()
            .ok_or_else(|| EditorError::Git("Could not determine repository path".to_string()))?;

        let mut cmd = Command::new("git");
        cmd.current_dir(repo_path);
        cmd.arg("gc");

        if aggressive {
            cmd.arg("--aggressive");
        }

        cmd.arg("--quiet");

        let output = cmd
            .output()
            .map_err(|e| EditorError::Git(format!("Failed to execute git gc: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(EditorError::Git(format!("git gc failed: {}", stderr)));
        }

        Ok(())
    }

    pub fn get_commit_count(&self, project_path: &Path) -> Result<usize> {
        let commits = self.list_commits(project_path)?;
        Ok(commits.len())
    }

    pub fn get_repo_size(&self, project_path: &Path) -> Result<u64> {
        let repo = self.open_repository(project_path)?;
        let repo_path = repo.path();

        let size = Self::calculate_dir_size(repo_path)?;
        Ok(size)
    }

    fn calculate_dir_size(path: &Path) -> Result<u64> {
        let mut total_size = 0u64;

        if path.is_dir() {
            let entries = fs::read_dir(path).map_err(EditorError::Io)?;
            for entry in entries {
                let entry = entry.map_err(EditorError::Io)?;
                let entry_path = entry.path();
                if entry_path.is_dir() {
                    total_size += Self::calculate_dir_size(&entry_path)?;
                } else {
                    let metadata = entry.metadata().map_err(EditorError::Io)?;
                    total_size += metadata.len();
                }
            }
        }

        Ok(total_size)
    }

    pub fn should_run_gc(&self, project_path: &Path) -> Result<bool> {
        if !self.gc_config().enabled {
            return Ok(false);
        }

        let commit_count = self.get_commit_count(project_path)?;
        if commit_count >= self.gc_config().commits_threshold {
            return Ok(true);
        }

        let repo_size = self.get_repo_size(project_path)?;
        let size_mb = repo_size / (1024 * 1024);
        if size_mb >= self.gc_config().size_threshold_mb {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn auto_gc_if_needed(&self, project_path: &Path) -> Result<bool> {
        if self.should_run_gc(project_path)? {
            match self.run_gc(project_path, self.gc_config().aggressive) {
                Ok(()) => Ok(true),
                Err(e) => {
                    eprintln!("Warning: Automatic git gc failed: {}", e);
                    Ok(false)
                }
            }
        } else {
            Ok(false)
        }
    }
}
