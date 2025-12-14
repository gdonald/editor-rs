use crate::{EditorError, Result};
use git2::{Repository, Signature, Time};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

pub struct GitHistoryManager {
    storage_root: PathBuf,
}

impl GitHistoryManager {
    pub fn new() -> Result<Self> {
        let storage_root = Self::default_storage_root()?;
        Ok(Self { storage_root })
    }

    pub fn with_storage_root(storage_root: PathBuf) -> Result<Self> {
        Ok(Self { storage_root })
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

    pub fn auto_commit_on_save(&self, project_path: &Path, file_path: &Path) -> Result<()> {
        let path_buf = file_path.to_path_buf();
        self.auto_commit_on_save_multiple(project_path, &[&path_buf])
    }

    pub fn auto_commit_on_save_multiple(
        &self,
        project_path: &Path,
        file_paths: &[&PathBuf],
    ) -> Result<()> {
        if file_paths.is_empty() {
            return Ok(());
        }

        let repo = self.open_repository(project_path)?;
        let canonical_project = project_path.canonicalize().map_err(EditorError::Io)?;

        let mut index = repo.index().map_err(|e| EditorError::Git(e.to_string()))?;
        let mut relative_paths = Vec::new();

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
            return Ok(());
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
        let message = if relative_paths.len() == 1 {
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

        Ok(())
    }
}

impl Default for GitHistoryManager {
    fn default() -> Self {
        Self::new().expect("Failed to create default GitHistoryManager")
    }
}

pub fn create_signature() -> Result<Signature<'static>> {
    let now = Time::new(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        0,
    );

    Signature::new("editor-rs", "editor-rs@localhost", &now)
        .map_err(|e| EditorError::Git(e.to_string()))
}
