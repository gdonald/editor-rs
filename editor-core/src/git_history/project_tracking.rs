use crate::{EditorError, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::repository::GitHistoryManager;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackingMode {
    Project(PathBuf),
    SingleFile(PathBuf),
}

impl TrackingMode {
    pub fn is_project(&self) -> bool {
        matches!(self, TrackingMode::Project(_))
    }

    pub fn is_single_file(&self) -> bool {
        matches!(self, TrackingMode::SingleFile(_))
    }

    pub fn path(&self) -> &Path {
        match self {
            TrackingMode::Project(path) => path,
            TrackingMode::SingleFile(path) => path,
        }
    }
}

impl GitHistoryManager {
    pub fn detect_tracking_mode(&self, file_path: &Path) -> Result<TrackingMode> {
        let file_path = file_path.canonicalize().map_err(EditorError::Io)?;

        if let Some(project_path) = self.find_project_root(&file_path)? {
            Ok(TrackingMode::Project(project_path))
        } else {
            let file_dir = if file_path.is_dir() {
                file_path
            } else {
                file_path
                    .parent()
                    .ok_or_else(|| {
                        EditorError::InvalidOperation("File has no parent directory".to_string())
                    })?
                    .to_path_buf()
            };
            Ok(TrackingMode::SingleFile(file_dir))
        }
    }

    fn find_project_root(&self, file_path: &Path) -> Result<Option<PathBuf>> {
        let mut current = if file_path.is_dir() {
            file_path
        } else {
            file_path.parent().ok_or_else(|| {
                EditorError::InvalidOperation("File has no parent directory".to_string())
            })?
        };

        loop {
            if self.is_project_root(current)? {
                return Ok(Some(current.to_path_buf()));
            }

            match current.parent() {
                Some(parent) => current = parent,
                None => return Ok(None),
            }
        }
    }

    fn is_project_root(&self, path: &Path) -> Result<bool> {
        if path.join(".git").exists() {
            return Ok(true);
        }

        if path.join("Cargo.toml").exists() {
            return Ok(true);
        }

        if path.join("package.json").exists() {
            return Ok(true);
        }

        if path.join("pyproject.toml").exists() {
            return Ok(true);
        }

        if path.join("go.mod").exists() {
            return Ok(true);
        }

        if path.join("pom.xml").exists() {
            return Ok(true);
        }

        if path.join("build.gradle").exists() || path.join("build.gradle.kts").exists() {
            return Ok(true);
        }

        Ok(false)
    }

    pub fn get_tracking_path(&self, file_path: &Path) -> Result<PathBuf> {
        let mode = self.detect_tracking_mode(file_path)?;
        Ok(mode.path().to_path_buf())
    }

    pub fn handle_file_move(
        &self,
        old_path: &Path,
        new_path: &Path,
    ) -> Result<Option<(PathBuf, PathBuf)>> {
        let old_mode = self.detect_tracking_mode(old_path)?;
        let new_mode = self.detect_tracking_mode(new_path)?;

        if old_mode.path() != new_mode.path() {
            Ok(Some((
                old_mode.path().to_path_buf(),
                new_mode.path().to_path_buf(),
            )))
        } else {
            Ok(None)
        }
    }

    pub fn handle_project_rename(
        &self,
        old_project_path: &Path,
        new_project_path: &Path,
    ) -> Result<()> {
        let old_repo_path = self.repo_path(old_project_path)?;
        let new_repo_path = self.repo_path(new_project_path)?;

        if !old_repo_path.exists() {
            return Ok(());
        }

        if new_repo_path.exists() {
            return Err(EditorError::InvalidOperation(format!(
                "Repository for new project path {} already exists",
                new_project_path.display()
            )));
        }

        fs::create_dir_all(new_repo_path.parent().unwrap()).map_err(EditorError::Io)?;
        fs::rename(&old_repo_path, &new_repo_path).map_err(EditorError::Io)?;

        let repo =
            git2::Repository::open(&new_repo_path).map_err(|e| EditorError::Git(e.to_string()))?;
        self.write_project_metadata(&repo, new_project_path)?;

        Ok(())
    }

    pub fn list_tracked_projects(&self) -> Result<Vec<PathBuf>> {
        let storage_root = self.storage_root();

        if !storage_root.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        for entry in fs::read_dir(storage_root).map_err(EditorError::Io)? {
            let entry = entry.map_err(EditorError::Io)?;
            let path = entry.path();

            if path.is_dir() {
                if let Ok(repo) = git2::Repository::open(&path) {
                    if let Ok(Some(project_path)) = self.get_project_path(&repo) {
                        projects.push(project_path);
                    }
                }
            }
        }

        projects.sort();
        Ok(projects)
    }

    pub fn is_file_in_project(&self, file_path: &Path, project_path: &Path) -> Result<bool> {
        let file_path = file_path.canonicalize().map_err(EditorError::Io)?;
        let project_path = project_path.canonicalize().map_err(EditorError::Io)?;

        Ok(file_path.starts_with(&project_path))
    }
}
