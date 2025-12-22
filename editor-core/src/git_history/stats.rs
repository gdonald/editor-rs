use crate::{EditorError, Result};
use chrono::Datelike;
use git2::Oid;
use std::collections::HashMap;
use std::path::Path;

use super::repository::GitHistoryManager;
use super::types::{FileStats, HistoryStats};

impl GitHistoryManager {
    pub fn get_date_range(&self, project_path: &Path) -> Result<Option<(i64, i64)>> {
        let commits = self.list_commits(project_path)?;

        if commits.is_empty() {
            return Ok(None);
        }

        let mut oldest = commits[0].timestamp;
        let mut newest = commits[0].timestamp;

        for commit in &commits {
            if commit.timestamp < oldest {
                oldest = commit.timestamp;
            }
            if commit.timestamp > newest {
                newest = commit.timestamp;
            }
        }

        Ok(Some((oldest, newest)))
    }

    pub fn get_per_file_stats(&self, project_path: &Path) -> Result<Vec<FileStats>> {
        let repo = self.open_repository(project_path)?;
        let commits = self.list_commits(project_path)?;

        let mut file_commit_counts: HashMap<String, usize> = HashMap::new();
        let mut file_sizes: HashMap<String, u64> = HashMap::new();

        for commit_info in &commits {
            let files_changed = self.get_files_changed(project_path, &commit_info.id)?;

            for file_change in files_changed {
                *file_commit_counts
                    .entry(file_change.path.clone())
                    .or_insert(0) += 1;
            }

            let oid =
                Oid::from_str(&commit_info.id).map_err(|e| EditorError::Git(e.to_string()))?;
            let commit = repo
                .find_commit(oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let tree = commit.tree().map_err(|e| EditorError::Git(e.to_string()))?;

            tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
                if entry.kind() == Some(git2::ObjectType::Blob) {
                    if let Some(name) = entry.name() {
                        let path = if root.is_empty() {
                            name.to_string()
                        } else {
                            format!("{}/{}", root, name)
                        };

                        if let Ok(object) = entry.to_object(&repo) {
                            if let Some(blob) = object.as_blob() {
                                let size = blob.content().len() as u64;
                                file_sizes
                                    .entry(path)
                                    .and_modify(|s| *s = (*s).max(size))
                                    .or_insert(size);
                            }
                        }
                    }
                }
                git2::TreeWalkResult::Ok
            })
            .map_err(|e| EditorError::Git(e.to_string()))?;
        }

        let threshold_bytes = self.large_file_config().threshold_mb * 1024 * 1024;

        let mut stats: Vec<FileStats> = file_commit_counts
            .into_iter()
            .map(|(path, commit_count)| {
                let total_size = *file_sizes.get(&path).unwrap_or(&0);
                FileStats {
                    path: path.clone(),
                    commit_count,
                    total_size,
                    is_large: total_size > threshold_bytes,
                }
            })
            .collect();

        stats.sort_by(|a, b| b.commit_count.cmp(&a.commit_count));

        Ok(stats)
    }

    pub fn get_commits_per_day(&self, project_path: &Path) -> Result<HashMap<String, usize>> {
        let commits = self.list_commits(project_path)?;
        let mut commits_by_day: HashMap<String, usize> = HashMap::new();

        for commit in &commits {
            let date = chrono::DateTime::<chrono::Utc>::from_timestamp(commit.timestamp, 0)
                .unwrap_or_default();
            let day_key = date.format("%Y-%m-%d").to_string();
            *commits_by_day.entry(day_key).or_insert(0) += 1;
        }

        Ok(commits_by_day)
    }

    pub fn get_commits_per_week(&self, project_path: &Path) -> Result<HashMap<String, usize>> {
        let commits = self.list_commits(project_path)?;
        let mut commits_by_week: HashMap<String, usize> = HashMap::new();

        for commit in &commits {
            let date = chrono::DateTime::<chrono::Utc>::from_timestamp(commit.timestamp, 0)
                .unwrap_or_default();
            let week_key = format!("{}-W{:02}", date.iso_week().year(), date.iso_week().week());
            *commits_by_week.entry(week_key).or_insert(0) += 1;
        }

        Ok(commits_by_week)
    }

    pub fn get_commits_per_month(&self, project_path: &Path) -> Result<HashMap<String, usize>> {
        let commits = self.list_commits(project_path)?;
        let mut commits_by_month: HashMap<String, usize> = HashMap::new();

        for commit in &commits {
            let date = chrono::DateTime::<chrono::Utc>::from_timestamp(commit.timestamp, 0)
                .unwrap_or_default();
            let month_key = date.format("%Y-%m").to_string();
            *commits_by_month.entry(month_key).or_insert(0) += 1;
        }

        Ok(commits_by_month)
    }

    pub fn list_large_files(&self, project_path: &Path) -> Result<Vec<FileStats>> {
        let file_stats = self.get_per_file_stats(project_path)?;
        Ok(file_stats.into_iter().filter(|fs| fs.is_large).collect())
    }

    pub fn get_history_stats(&self, project_path: &Path) -> Result<HistoryStats> {
        let total_commits = self.get_commit_count(project_path)?;
        let repository_size = self.get_repo_size(project_path)?;
        let date_range = self.get_date_range(project_path)?;
        let file_stats = self.get_per_file_stats(project_path)?;

        let large_file_count = file_stats.iter().filter(|fs| fs.is_large).count();
        let total_large_file_size: u64 = file_stats
            .iter()
            .filter(|fs| fs.is_large)
            .map(|fs| fs.total_size)
            .sum();

        Ok(HistoryStats {
            total_commits,
            repository_size,
            date_range,
            file_stats,
            large_file_count,
            total_large_file_size,
        })
    }
}
