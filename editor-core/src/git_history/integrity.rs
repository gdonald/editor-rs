use crate::{EditorError, Result};
use git2::Repository;
use std::fs;
use std::path::Path;

use super::repository::GitHistoryManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegrityReport {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl IntegrityReport {
    pub fn new() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.is_valid = false;
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

impl Default for IntegrityReport {
    fn default() -> Self {
        Self::new()
    }
}

impl GitHistoryManager {
    pub fn verify_repository_integrity(&self, project_path: &Path) -> Result<IntegrityReport> {
        let mut report = IntegrityReport::new();

        let repo_path = self.repo_path(project_path)?;
        if !repo_path.exists() {
            report.add_error(format!(
                "Repository does not exist at {}",
                repo_path.display()
            ));
            return Ok(report);
        }

        let repo = match Repository::open(&repo_path) {
            Ok(r) => r,
            Err(e) => {
                report.add_error(format!("Failed to open repository: {}", e));
                return Ok(report);
            }
        };

        if let Err(e) = self.check_repository_structure(&repo, &mut report) {
            report.add_error(format!("Repository structure check failed: {}", e));
        }

        if let Err(e) = self.check_head_reference(&repo, &mut report) {
            report.add_error(format!("HEAD reference check failed: {}", e));
        }

        if let Err(e) = self.check_commits(&repo, &mut report) {
            report.add_error(format!("Commit check failed: {}", e));
        }

        if let Err(e) = self.check_objects(&repo, &mut report) {
            report.add_error(format!("Object check failed: {}", e));
        }

        Ok(report)
    }

    fn check_repository_structure(
        &self,
        repo: &Repository,
        report: &mut IntegrityReport,
    ) -> Result<()> {
        let repo_path = repo.path();

        if !repo_path.join("HEAD").exists() {
            report.add_error("HEAD file is missing".to_string());
        }

        if !repo_path.join("objects").exists() {
            report.add_error("objects directory is missing".to_string());
        }

        if !repo_path.join("refs").exists() {
            report.add_error("refs directory is missing".to_string());
        }

        Ok(())
    }

    fn check_head_reference(&self, repo: &Repository, report: &mut IntegrityReport) -> Result<()> {
        match repo.head() {
            Ok(head) => {
                if !head.is_branch() && head.target().is_none() {
                    report.add_warning("HEAD is in detached state".to_string());
                }
            }
            Err(e) => {
                if e.code() == git2::ErrorCode::UnbornBranch {
                    report.add_warning("Repository is empty (no commits)".to_string());
                } else {
                    report.add_error(format!("Failed to read HEAD: {}", e));
                }
            }
        }

        Ok(())
    }

    fn check_commits(&self, repo: &Repository, report: &mut IntegrityReport) -> Result<()> {
        if repo
            .is_empty()
            .map_err(|e| EditorError::Git(e.to_string()))?
        {
            return Ok(());
        }

        let mut revwalk = repo
            .revwalk()
            .map_err(|e| EditorError::Git(e.to_string()))?;
        revwalk
            .push_head()
            .map_err(|e| EditorError::Git(e.to_string()))?;

        for oid in revwalk {
            let oid = oid.map_err(|e| EditorError::Git(e.to_string()))?;

            match repo.find_commit(oid) {
                Ok(commit) => {
                    if let Err(e) = commit.tree() {
                        report.add_error(format!("Commit {} has invalid tree: {}", oid, e));
                    }

                    if commit.message().is_none() {
                        report.add_warning(format!("Commit {} has no message", oid));
                    }
                }
                Err(e) => {
                    report.add_error(format!("Failed to read commit {}: {}", oid, e));
                }
            }
        }

        Ok(())
    }

    fn check_objects(&self, repo: &Repository, report: &mut IntegrityReport) -> Result<()> {
        let odb = repo.odb().map_err(|e| EditorError::Git(e.to_string()))?;

        odb.foreach(|oid| {
            if let Err(e) = odb.read(*oid) {
                report.add_error(format!("Object {} is corrupted: {}", oid, e));
            }
            true
        })
        .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(())
    }

    pub fn create_backup(&self, project_path: &Path) -> Result<String> {
        let repo_path = self.repo_path(project_path)?;

        if !repo_path.exists() {
            return Err(EditorError::InvalidOperation(
                "Repository does not exist".to_string(),
            ));
        }

        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let backup_name = format!("backup_{}", timestamp);
        let backup_path = repo_path.parent().unwrap().join(&backup_name);

        self.copy_directory(&repo_path, &backup_path)?;

        Ok(backup_name)
    }

    fn copy_directory(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst).map_err(EditorError::Io)?;

        for entry in fs::read_dir(src).map_err(EditorError::Io)? {
            let entry = entry.map_err(EditorError::Io)?;
            let file_type = entry.file_type().map_err(EditorError::Io)?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if file_type.is_dir() {
                self.copy_directory(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path).map_err(EditorError::Io)?;
            }
        }

        Ok(())
    }

    pub fn repair_repository(&self, project_path: &Path) -> Result<()> {
        let backup_name = self.create_backup(project_path)?;

        let repair_result = self.attempt_repair(project_path);

        if repair_result.is_err() {
            self.restore_from_backup(project_path, &backup_name)?;
            return repair_result;
        }

        Ok(())
    }

    fn attempt_repair(&self, project_path: &Path) -> Result<()> {
        let repo_path = self.repo_path(project_path)?;

        if !repo_path.exists() {
            return Err(EditorError::InvalidOperation(
                "Repository does not exist".to_string(),
            ));
        }

        let repo = Repository::open(&repo_path).map_err(|e| EditorError::Git(e.to_string()))?;

        repo.odb()
            .map_err(|e| EditorError::Git(e.to_string()))?
            .foreach(|_| true)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        if !repo
            .is_empty()
            .map_err(|e| EditorError::Git(e.to_string()))?
        {
            match repo.head() {
                Ok(_) => {}
                Err(e) => {
                    if e.code() != git2::ErrorCode::UnbornBranch {
                        return Err(EditorError::Git(format!("HEAD is corrupted: {}", e)));
                    }
                }
            }
        }

        Ok(())
    }

    fn restore_from_backup(&self, project_path: &Path, backup_name: &str) -> Result<()> {
        let repo_path = self.repo_path(project_path)?;
        let backup_path = repo_path.parent().unwrap().join(backup_name);

        if !backup_path.exists() {
            return Err(EditorError::InvalidOperation(format!(
                "Backup {} does not exist",
                backup_name
            )));
        }

        if repo_path.exists() {
            fs::remove_dir_all(&repo_path).map_err(EditorError::Io)?;
        }

        self.copy_directory(&backup_path, &repo_path)?;

        Ok(())
    }

    pub fn delete_backup(&self, project_path: &Path, backup_name: &str) -> Result<()> {
        let repo_path = self.repo_path(project_path)?;
        let backup_path = repo_path.parent().unwrap().join(backup_name);

        if !backup_path.exists() {
            return Err(EditorError::InvalidOperation(format!(
                "Backup {} does not exist",
                backup_name
            )));
        }

        fs::remove_dir_all(&backup_path).map_err(EditorError::Io)?;

        Ok(())
    }

    pub fn list_backups(&self, project_path: &Path) -> Result<Vec<String>> {
        let repo_path = self.repo_path(project_path)?;
        let parent_dir = repo_path.parent().unwrap();

        if !parent_dir.exists() {
            return Ok(Vec::new());
        }

        let mut backups = Vec::new();

        for entry in fs::read_dir(parent_dir).map_err(EditorError::Io)? {
            let entry = entry.map_err(EditorError::Io)?;
            let file_name = entry.file_name();
            let name = file_name.to_string_lossy();

            if name.starts_with("backup_") && entry.file_type().map_err(EditorError::Io)?.is_dir() {
                backups.push(name.to_string());
            }
        }

        backups.sort();
        backups.reverse();

        Ok(backups)
    }
}
