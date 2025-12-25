mod annotations;
mod cleanup;
mod commit;
mod diff;
mod export_import;
mod gc;
mod graph;
mod integrity;
mod project_tracking;
mod repository;
mod restore;
mod retention;
mod stats;
mod types;

pub use annotations::CommitAnnotations;
pub use cleanup::CleanupStats;
pub use commit::{CommitResult, FileSizeInfo};
pub use graph::{format_graph_line, generate_commit_graph, CommitGraphNode};
pub use integrity::IntegrityReport;
pub use project_tracking::TrackingMode;
pub use repository::GitHistoryManager;
pub use types::{
    create_signature, ChangeStatus, CommitInfo, FileChange, FileStats, GcConfig, HistoryStats,
    LargeFileConfig, LargeFileStrategy, RetentionPolicy,
};
