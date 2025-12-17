use crate::{EditorError, Result};
use git2::{Signature, Time};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommitInfo {
    pub id: String,
    pub author_name: String,
    pub author_email: String,
    pub timestamp: i64,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileChange {
    pub path: String,
    pub status: ChangeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeStatus {
    Added,
    Deleted,
    Modified,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GcConfig {
    pub enabled: bool,
    pub commits_threshold: usize,
    pub size_threshold_mb: u64,
    pub aggressive: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileStats {
    pub path: String,
    pub commit_count: usize,
    pub total_size: u64,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            commits_threshold: 1000,
            size_threshold_mb: 100,
            aggressive: false,
        }
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
