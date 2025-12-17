use crate::{EditorError, Result};
use git2::Oid;
use std::fs;
use std::path::Path;

use super::repository::GitHistoryManager;

impl GitHistoryManager {
    pub fn restore_commit(&self, project_path: &Path, commit_id: &str) -> Result<()> {
        let repo = self.open_repository(project_path)?;
        let oid = Oid::from_str(commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let tree = commit.tree().map_err(|e| EditorError::Git(e.to_string()))?;

        let workdir = repo
            .workdir()
            .ok_or_else(|| EditorError::Git("Repository has no working directory".to_string()))?;

        tree.walk(git2::TreeWalkMode::PreOrder, |root, entry| {
            if let Some(name) = entry.name() {
                let entry_path = Path::new(root).join(name);
                let full_path = workdir.join(&entry_path);

                if entry.kind() == Some(git2::ObjectType::Blob) {
                    if let Ok(object) = entry.to_object(&repo) {
                        if let Some(blob) = object.as_blob() {
                            if let Some(parent) = full_path.parent() {
                                let _ = fs::create_dir_all(parent);
                            }
                            let _ = fs::write(&full_path, blob.content());
                        }
                    }
                }
            }
            git2::TreeWalkResult::Ok
        })
        .map_err(|e| EditorError::Git(e.to_string()))?;

        Ok(())
    }

    pub fn restore_file(
        &self,
        project_path: &Path,
        file_path: &str,
        commit_id: &str,
    ) -> Result<Vec<u8>> {
        let repo = self.open_repository(project_path)?;
        let oid = Oid::from_str(commit_id).map_err(|e| EditorError::Git(e.to_string()))?;
        let commit = repo
            .find_commit(oid)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let tree = commit.tree().map_err(|e| EditorError::Git(e.to_string()))?;

        let entry = tree
            .get_path(Path::new(file_path))
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let object = entry
            .to_object(&repo)
            .map_err(|e| EditorError::Git(e.to_string()))?;

        let blob = object
            .as_blob()
            .ok_or_else(|| EditorError::Git("Object is not a blob".to_string()))?;

        Ok(blob.content().to_vec())
    }

    pub fn get_file_content_at_commit(
        &self,
        project_path: &Path,
        file_path: &str,
        commit_id: &str,
    ) -> Result<String> {
        let content = self.restore_file(project_path, file_path, commit_id)?;
        String::from_utf8(content).map_err(|e| EditorError::Parse(e.to_string()))
    }
}
