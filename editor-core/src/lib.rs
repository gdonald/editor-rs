pub mod bookmark;
pub mod buffer;
pub mod clipboard;
pub mod command;
pub mod cursor;
pub mod diff_parser;
pub mod editor;
pub mod error;
pub mod git_history;
pub mod history;
pub mod history_browser;
pub mod selection;
pub mod session;
pub mod view;

pub use bookmark::{Bookmark, BookmarkManager, FileBookmarks};
pub use buffer::{Buffer, Encoding, LineEnding};
pub use clipboard::ClipboardManager;
pub use command::{CaseMode, Command};
pub use cursor::{CursorPosition, MultiCursor};
pub use diff_parser::{DiffLine, DiffLineType, SideBySideDiff};
pub use editor::{EditorState, VirtualViewport};
pub use error::{EditorError, Result};
pub use git_history::{
    create_signature, ChangeStatus, CleanupStats, CommitInfo, CommitResult, FileChange,
    FileSizeInfo, FileStats, GcConfig, GitHistoryManager, HistoryStats, IntegrityReport,
    LargeFileConfig, LargeFileStrategy, RetentionPolicy, TrackingMode,
};
pub use history::{Edit, History, HistoryEntry};
pub use history_browser::{DiffViewMode, HistoryBrowser};
pub use selection::{Selection, SelectionMode};
pub use session::{OpenFileState, Session, SessionManager};
pub use view::EditorView;
