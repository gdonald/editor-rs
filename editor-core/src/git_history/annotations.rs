use crate::{EditorError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use super::repository::GitHistoryManager;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CommitAnnotations {
    annotations: HashMap<String, String>,
}

impl CommitAnnotations {
    pub fn new() -> Self {
        Self {
            annotations: HashMap::new(),
        }
    }

    pub fn get(&self, commit_id: &str) -> Option<&str> {
        self.annotations.get(commit_id).map(|s| s.as_str())
    }

    pub fn set(&mut self, commit_id: String, annotation: String) {
        if annotation.is_empty() {
            self.annotations.remove(&commit_id);
        } else {
            self.annotations.insert(commit_id, annotation);
        }
    }

    pub fn remove(&mut self, commit_id: &str) -> Option<String> {
        self.annotations.remove(commit_id)
    }

    pub fn has_annotation(&self, commit_id: &str) -> bool {
        self.annotations.contains_key(commit_id)
    }

    pub fn count(&self) -> usize {
        self.annotations.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.annotations.iter()
    }
}

impl GitHistoryManager {
    fn get_annotations_path(&self, project_path: &Path) -> Result<PathBuf> {
        let repo = self.open_repository(project_path)?;
        let repo_path = repo
            .path()
            .parent()
            .ok_or_else(|| EditorError::Git("Invalid repository path".to_string()))?;
        Ok(repo_path.join("annotations.json"))
    }

    pub fn load_annotations(&self, project_path: &Path) -> Result<CommitAnnotations> {
        let annotations_path = self.get_annotations_path(project_path)?;

        if !annotations_path.exists() {
            return Ok(CommitAnnotations::new());
        }

        let contents = fs::read_to_string(&annotations_path)?;

        serde_json::from_str(&contents)
            .map_err(|e| EditorError::Parse(format!("Failed to parse annotations: {}", e)))
    }

    pub fn save_annotations(
        &self,
        project_path: &Path,
        annotations: &CommitAnnotations,
    ) -> Result<()> {
        let annotations_path = self.get_annotations_path(project_path)?;

        let contents = serde_json::to_string_pretty(annotations)
            .map_err(|e| EditorError::Parse(format!("Failed to serialize annotations: {}", e)))?;

        fs::write(&annotations_path, contents)?;

        Ok(())
    }

    pub fn add_annotation(
        &self,
        project_path: &Path,
        commit_id: &str,
        annotation: String,
    ) -> Result<()> {
        let mut annotations = self.load_annotations(project_path)?;
        annotations.set(commit_id.to_string(), annotation);
        self.save_annotations(project_path, &annotations)
    }

    pub fn remove_annotation(&self, project_path: &Path, commit_id: &str) -> Result<()> {
        let mut annotations = self.load_annotations(project_path)?;
        annotations.remove(commit_id);
        self.save_annotations(project_path, &annotations)
    }

    pub fn get_annotation(&self, project_path: &Path, commit_id: &str) -> Result<Option<String>> {
        let annotations = self.load_annotations(project_path)?;
        Ok(annotations.get(commit_id).map(|s| s.to_string()))
    }
}
