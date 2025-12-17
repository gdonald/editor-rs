use crate::{EditorError, Result};
use git2::Repository;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use super::types::{GcConfig, RetentionPolicy};

pub struct GitHistoryManager {
    storage_root: PathBuf,
    gc_config: GcConfig,
    retention_policy: RetentionPolicy,
}

impl GitHistoryManager {
    pub fn new() -> Result<Self> {
        let storage_root = Self::default_storage_root()?;
        Ok(Self {
            storage_root,
            gc_config: GcConfig::default(),
            retention_policy: RetentionPolicy::default(),
        })
    }

    pub fn with_storage_root(storage_root: PathBuf) -> Result<Self> {
        Ok(Self {
            storage_root,
            gc_config: GcConfig::default(),
            retention_policy: RetentionPolicy::default(),
        })
    }

    pub fn with_gc_config(mut self, gc_config: GcConfig) -> Self {
        self.gc_config = gc_config;
        self
    }

    pub fn with_retention_policy(mut self, retention_policy: RetentionPolicy) -> Self {
        self.retention_policy = retention_policy;
        self
    }

    pub fn gc_config(&self) -> &GcConfig {
        &self.gc_config
    }

    pub fn retention_policy(&self) -> &RetentionPolicy {
        &self.retention_policy
    }

    fn default_storage_root() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| {
            EditorError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Could not determine home directory",
            ))
        })?;
        Ok(home.join(".editor-rs").join("history"))
    }

    pub fn project_hash(project_path: &Path) -> Result<String> {
        let canonical_path = project_path.canonicalize().map_err(EditorError::Io)?;

        let path_str = canonical_path.to_string_lossy();
        let mut hasher = Sha256::new();
        hasher.update(path_str.as_bytes());
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }

    pub fn repo_path(&self, project_path: &Path) -> Result<PathBuf> {
        let hash = Self::project_hash(project_path)?;
        Ok(self.storage_root.join(hash))
    }

    pub fn init_repository(&self, project_path: &Path) -> Result<Repository> {
        let repo_path = self.repo_path(project_path)?;

        fs::create_dir_all(&repo_path).map_err(EditorError::Io)?;

        let repo = Repository::init(&repo_path).map_err(|e| EditorError::Git(e.to_string()))?;

        self.write_project_metadata(&repo, project_path)?;

        Ok(repo)
    }

    pub fn open_repository(&self, project_path: &Path) -> Result<Repository> {
        let repo_path = self.repo_path(project_path)?;

        if !repo_path.exists() {
            return self.init_repository(project_path);
        }

        match Repository::open(&repo_path) {
            Ok(repo) => Ok(repo),
            Err(e) => {
                eprintln!(
                    "Warning: Git repository at {} appears corrupted ({}), reinitializing...",
                    repo_path.display(),
                    e
                );
                if let Err(remove_err) = fs::remove_dir_all(&repo_path) {
                    eprintln!(
                        "Warning: Failed to remove corrupted repository: {}",
                        remove_err
                    );
                }
                self.init_repository(project_path)
            }
        }
    }

    fn write_project_metadata(&self, repo: &Repository, project_path: &Path) -> Result<()> {
        let metadata_path = repo.path().join("project_metadata.toml");
        let canonical_path = project_path.canonicalize().map_err(EditorError::Io)?;

        let metadata = format!(
            "original_path = \"{}\"\n",
            canonical_path.to_string_lossy().replace('\\', "\\\\")
        );

        fs::write(&metadata_path, metadata).map_err(EditorError::Io)?;

        Ok(())
    }

    pub fn get_project_path(&self, repo: &Repository) -> Result<Option<PathBuf>> {
        let metadata_path = repo.path().join("project_metadata.toml");

        if !metadata_path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&metadata_path).map_err(EditorError::Io)?;

        let value: toml::Value =
            toml::from_str(&content).map_err(|e| EditorError::Parse(e.to_string()))?;

        if let Some(path_str) = value.get("original_path").and_then(|v| v.as_str()) {
            Ok(Some(PathBuf::from(path_str)))
        } else {
            Ok(None)
        }
    }

    pub fn storage_root(&self) -> &Path {
        &self.storage_root
    }
}

impl Default for GitHistoryManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default GitHistoryManager")
    }
}
