use crate::{EditorError, Result};
use git2::Oid;
use std::fs;
use std::path::{Path, PathBuf};

use super::repository::GitHistoryManager;
use super::types::{create_signature, ChangeStatus, CommitInfo, FileChange, LargeFileStrategy};

pub struct FileSizeInfo {
    pub size_bytes: u64,
    pub exceeds_threshold: bool,
}

#[derive(Debug)]
pub struct CommitResult {
    pub skipped_files: Vec<PathBuf>,
}

fn log_large_file_warning(file_path: &Path, size_bytes: u64, threshold_mb: u64) {
    eprintln!(
        "Warning: Large file {} ({} bytes) exceeds threshold of {} MB",
        file_path.display(),
        size_bytes,
        threshold_mb
    );
}

impl GitHistoryManager {
    fn get_file_size(file_path: &Path) -> Result<u64> {
        let metadata = fs::metadata(file_path).map_err(EditorError::Io)?;
        Ok(metadata.len())
    }

    pub fn check_file_size(&self, file_path: &Path) -> Result<FileSizeInfo> {
        let size_bytes = Self::get_file_size(file_path)?;
        let threshold_bytes = self.large_file_config().threshold_mb * 1024 * 1024;
        let exceeds_threshold = size_bytes > threshold_bytes;

        Ok(FileSizeInfo {
            size_bytes,
            exceeds_threshold,
        })
    }

    pub fn is_large_file(&self, file_path: &Path) -> Result<bool> {
        let info = self.check_file_size(file_path)?;
        Ok(info.exceeds_threshold)
    }

    pub fn auto_commit_on_save(
        &self,
        project_path: &Path,
        file_path: &Path,
    ) -> Result<CommitResult> {
        let path_buf = file_path.to_path_buf();
        self.auto_commit_on_save_multiple(project_path, &[&path_buf])
    }

    pub fn auto_commit_on_save_multiple(
        &self,
        project_path: &Path,
        file_paths: &[&PathBuf],
    ) -> Result<CommitResult> {
        if file_paths.is_empty() {
            return Ok(CommitResult {
                skipped_files: Vec::new(),
            });
        }

        let repo = self.open_repository(project_path)?;
        let canonical_project = project_path.canonicalize().map_err(EditorError::Io)?;

        let mut index = repo.index().map_err(|e| EditorError::Git(e.to_string()))?;
        let mut relative_paths = Vec::new();
        let mut skipped_files = Vec::new();

        for file_path in file_paths {
            let canonical_file = match file_path.canonicalize() {
                Ok(path) => path,
                Err(e) => {
                    eprintln!(
                        "Warning: Could not canonicalize file path {}: {}",
                        file_path.display(),
                        e
                    );
                    continue;
                }
            };

            let relative_path = match canonical_file.strip_prefix(&canonical_project) {
                Ok(path) => path,
                Err(_) => {
                    eprintln!(
                        "Warning: File {} is outside project directory {}, skipping auto-commit",
                        canonical_file.display(),
                        canonical_project.display()
                    );
                    continue;
                }
            };

            let size_info = self.check_file_size(&canonical_file)?;
            if size_info.exceeds_threshold {
                match self.large_file_config().strategy {
                    LargeFileStrategy::Warn => {
                        log_large_file_warning(
                            &canonical_file,
                            size_info.size_bytes,
                            self.large_file_config().threshold_mb,
                        );
                    }
                    LargeFileStrategy::Skip => {
                        eprintln!(
                            "Skipping large file {} ({} bytes exceeds {} MB threshold)",
                            canonical_file.display(),
                            size_info.size_bytes,
                            self.large_file_config().threshold_mb
                        );
                        skipped_files.push(relative_path.to_path_buf());
                        continue;
                    }
                    LargeFileStrategy::Error => {
                        return Err(EditorError::FileTooLarge {
                            path: canonical_file.to_string_lossy().to_string(),
                            size: size_info.size_bytes,
                            limit: self.large_file_config().threshold_mb * 1024 * 1024,
                        });
                    }
                    _ => {}
                }
            }

            let repo_file_path = repo
                .workdir()
                .ok_or_else(|| EditorError::Git("Repository has no working directory".to_string()))?
                .join(relative_path);

            if let Some(parent) = repo_file_path.parent() {
                fs::create_dir_all(parent).map_err(EditorError::Io)?;
            }

            fs::copy(&canonical_file, &repo_file_path).map_err(EditorError::Io)?;

            index
                .add_path(relative_path)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            relative_paths.push(relative_path.to_path_buf());
        }

        if relative_paths.is_empty() {
            return Ok(CommitResult { skipped_files });
        }

        index.write().map_err(|e| EditorError::Git(e.to_string()))?;

        let signature = create_signature()?;
        let tree_id = index
            .write_tree()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        let tree = repo
            .find_tree(tree_id)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let mut message = if relative_paths.len() == 1 {
            format!(
                "Auto-save: {} at {}",
                relative_paths[0].display(),
                timestamp
            )
        } else {
            let files_list = relative_paths
                .iter()
                .map(|p| format!("  - {}", p.display()))
                .collect::<Vec<_>>()
                .join("\n");
            format!(
                "Auto-save: {} files at {}\n\n{}",
                relative_paths.len(),
                timestamp,
                files_list
            )
        };

        if !skipped_files.is_empty() {
            let skipped_count = skipped_files.len();
            let skipped_list = skipped_files
                .iter()
                .map(|p| format!("  - {}", p.display()))
                .collect::<Vec<_>>()
                .join("\n");
            message.push_str(&format!(
                "\n\n({} large file{} excluded)\n{}",
                skipped_count,
                if skipped_count == 1 { "" } else { "s" },
                skipped_list
            ));
        }

        let parent_commit = repo.head().ok().and_then(|head| head.peel_to_commit().ok());

        let parents: Vec<_> = parent_commit.iter().collect();

        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &message,
            &tree,
            &parents,
        )
        .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(CommitResult { skipped_files })
    }

    pub fn list_commits(&self, project_path: &Path) -> Result<Vec<CommitInfo>> {
        let repo = self.open_repository(project_path)?;

        let mut revwalk = repo
            .revwalk()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        if revwalk.push_head().is_err() {
            return Ok(Vec::new());
        }

        let annotations = self.load_annotations(project_path).unwrap_or_default();

        let mut commits = Vec::new();

        for oid_result in revwalk {
            let oid = oid_result.map_err(|e| EditorError::Git(e.to_string()))?;
            let commit = repo
                .find_commit(oid)
                .map_err(|e| EditorError::Git(e.to_string()))?;

            let commit_id = oid.to_string();
            let annotation = annotations.get(&commit_id).map(|s| s.to_string());

            commits.push(CommitInfo {
                id: commit_id,
                author_name: commit.author().name().unwrap_or("Unknown").to_string(),
                author_email: commit
                    .author()
                    .email()
                    .unwrap_or("unknown@localhost")
                    .to_string(),
                timestamp: commit.time().seconds(),
                message: commit.message().unwrap_or("").to_string(),
                annotation,
            });
        }

        Ok(commits)
    }

    pub fn get_commit_details(&self, project_path: &Path, commit_id: &str) -> Result<CommitInfo> {
        let repo = self.open_repository(project_path)?;
        let oid = Oid::from_str(commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let author = commit.author();
        let author_name = author.name().unwrap_or("Unknown").to_string();
        let author_email = author.email().unwrap_or("unknown@localhost").to_string();
        let timestamp = commit.time().seconds();
        let message = commit.message().unwrap_or("").to_string();

        let annotation = self.get_annotation(project_path, commit_id).unwrap_or(None);

        Ok(CommitInfo {
            id: oid.to_string(),
            author_name,
            author_email,
            timestamp,
            message,
            annotation,
        })
    }

    pub fn get_files_changed(
        &self,
        project_path: &Path,
        commit_id: &str,
    ) -> Result<Vec<FileChange>> {
        let repo = self.open_repository(project_path)?;
        let oid = Oid::from_str(commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let tree = commit.tree().map_err(|e| EditorError::Git(e.to_string()))?;

        let parent_tree = if commit.parent_count() > 0 {
            Some(
                commit
                    .parent(0)
                    .map_err(|e| EditorError::Git(e.to_string()))?
                    .tree()
                    .map_err(|e| EditorError::Git(e.to_string()))?,
            )
        } else {
            None
        };

        let diff = repo
            .diff_tree_to_tree(parent_tree.as_ref(), Some(&tree), None)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let mut files = Vec::new();

        diff.foreach(
            &mut |delta, _| {
                let status = match delta.status() {
                    git2::Delta::Added => ChangeStatus::Added,
                    git2::Delta::Deleted => ChangeStatus::Deleted,
                    git2::Delta::Modified => ChangeStatus::Modified,
                    _ => return true,
                };

                let path = delta
                    .new_file()
                    .path()
                    .or_else(|| delta.old_file().path())
                    .and_then(|p| p.to_str())
                    .unwrap_or("")
                    .to_string();

                files.push(FileChange { path, status });
                true
            },
            None,
            None,
            None,
        )
        .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(files)
    }
}
