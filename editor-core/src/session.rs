use crate::bookmark::FileBookmarks;
use crate::cursor::CursorPosition;
use crate::error::{EditorError, Result};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

const DEFAULT_RECENT_FILES_LIMIT: usize = 20;
const SESSION_DIR: &str = ".config/editor-rs";
const SESSION_FILE_PREFIX: &str = "session";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenFileState {
    pub path: PathBuf,
    pub cursor_line: usize,
    pub cursor_column: usize,
    pub viewport_top: usize,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub open_files: Vec<OpenFileState>,
    pub recent_files: VecDeque<PathBuf>,
    pub recent_files_limit: usize,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub bookmarks: Vec<FileBookmarks>,
}

impl Session {
    pub fn new() -> Self {
        let now = SystemTime::now();
        Self {
            open_files: Vec::new(),
            recent_files: VecDeque::new(),
            recent_files_limit: DEFAULT_RECENT_FILES_LIMIT,
            created_at: now,
            last_accessed: now,
            bookmarks: Vec::new(),
        }
    }

    pub fn add_open_file(&mut self, path: PathBuf, cursor: CursorPosition, viewport_top: usize) {
        if let Some(existing) = self.open_files.iter_mut().find(|f| f.path == path) {
            existing.cursor_line = cursor.line;
            existing.cursor_column = cursor.column;
            existing.viewport_top = viewport_top;
        } else {
            self.open_files.push(OpenFileState {
                path,
                cursor_line: cursor.line,
                cursor_column: cursor.column,
                viewport_top,
                active: false,
            });
        }
        self.last_accessed = SystemTime::now();
    }

    pub fn remove_open_file(&mut self, path: &Path) {
        self.open_files.retain(|f| f.path != path);
        self.last_accessed = SystemTime::now();
    }

    pub fn set_active_file(&mut self, path: &Path) {
        for file in &mut self.open_files {
            file.active = file.path == path;
        }
        self.last_accessed = SystemTime::now();
    }

    pub fn update_file_state(&mut self, path: &Path, cursor: CursorPosition, viewport_top: usize) {
        if let Some(file) = self.open_files.iter_mut().find(|f| f.path == path) {
            file.cursor_line = cursor.line;
            file.cursor_column = cursor.column;
            file.viewport_top = viewport_top;
            self.last_accessed = SystemTime::now();
        }
    }

    pub fn add_to_recent_files(&mut self, path: PathBuf) {
        self.recent_files.retain(|p| p != &path);
        self.recent_files.push_front(path);

        while self.recent_files.len() > self.recent_files_limit {
            self.recent_files.pop_back();
        }
        self.last_accessed = SystemTime::now();
    }

    pub fn get_recent_files(&self) -> Vec<PathBuf> {
        self.recent_files.iter().cloned().collect()
    }

    pub fn set_recent_files_limit(&mut self, limit: usize) {
        self.recent_files_limit = limit;
        while self.recent_files.len() > limit {
            self.recent_files.pop_back();
        }
    }

    pub fn get_open_files(&self) -> &[OpenFileState] {
        &self.open_files
    }

    pub fn get_active_file(&self) -> Option<&OpenFileState> {
        self.open_files.iter().find(|f| f.active)
    }

    pub fn save_bookmarks(&mut self, file_path: PathBuf, bookmarks: FileBookmarks) {
        self.bookmarks.retain(|fb| fb.file_path != file_path);
        if !bookmarks.bookmarks.is_empty() {
            self.bookmarks.push(bookmarks);
        }
        self.last_accessed = SystemTime::now();
    }

    pub fn load_bookmarks(&self, file_path: &Path) -> Option<&FileBookmarks> {
        self.bookmarks.iter().find(|fb| fb.file_path == file_path)
    }

    pub fn remove_bookmarks(&mut self, file_path: &Path) {
        self.bookmarks.retain(|fb| fb.file_path != file_path);
        self.last_accessed = SystemTime::now();
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let toml_string =
            toml::to_string_pretty(self).map_err(|e| EditorError::Io(std::io::Error::other(e)))?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, toml_string)?;
        Ok(())
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let mut session: Session =
            toml::from_str(&content).map_err(|e| EditorError::Io(std::io::Error::other(e)))?;

        session.open_files.retain(|f| f.path.exists());
        session.recent_files.retain(|p| p.exists());

        session.last_accessed = SystemTime::now();
        Ok(session)
    }
}

impl Default for Session {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SessionManager {
    session_path: PathBuf,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        let session_path = Self::get_session_path()?;
        Ok(Self { session_path })
    }

    pub fn with_custom_path(path: PathBuf) -> Self {
        Self { session_path: path }
    }

    fn get_session_path() -> Result<PathBuf> {
        let home_dir = std::env::var("HOME").map_err(|_| {
            EditorError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME environment variable not set",
            ))
        })?;

        let pid = std::process::id();
        let session_file = format!("{}-{}.toml", SESSION_FILE_PREFIX, pid);

        Ok(PathBuf::from(home_dir).join(SESSION_DIR).join(session_file))
    }

    pub fn session_path(&self) -> &Path {
        &self.session_path
    }

    pub fn save_session(&self, session: &Session) -> Result<()> {
        session.save_to_file(&self.session_path)
    }

    pub fn load_session(&self) -> Result<Session> {
        if self.session_path.exists() {
            Session::load_from_file(&self.session_path)
        } else {
            Ok(Session::new())
        }
    }

    pub fn delete_session(&self) -> Result<()> {
        if self.session_path.exists() {
            fs::remove_file(&self.session_path)?;
        }
        Ok(())
    }

    pub fn cleanup_stale_sessions(max_age_secs: u64) -> Result<()> {
        let home_dir = std::env::var("HOME").map_err(|_| {
            EditorError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "HOME environment variable not set",
            ))
        })?;

        let session_dir = PathBuf::from(home_dir).join(SESSION_DIR);
        if !session_dir.exists() {
            return Ok(());
        }

        let now = SystemTime::now();
        for entry in fs::read_dir(session_dir)? {
            let entry = entry?;
            let path = entry.path();

            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    if name_str.starts_with(SESSION_FILE_PREFIX) && name_str.ends_with(".toml") {
                        if let Ok(metadata) = entry.metadata() {
                            if let Ok(modified) = metadata.modified() {
                                if let Ok(duration) = now.duration_since(modified) {
                                    if duration.as_secs() > max_age_secs {
                                        let _ = fs::remove_file(&path);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create SessionManager")
    }
}
