mod commit;
mod diff;
mod gc;
mod repository;
mod restore;
mod stats;
mod types;

pub use repository::GitHistoryManager;
pub use types::{
    create_signature, ChangeStatus, CommitInfo, FileChange, FileStats, GcConfig, HistoryStats,
};
