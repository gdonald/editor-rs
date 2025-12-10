use crate::error::{EditorError, Result};
use ropey::Rope;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEnding {
    Lf,
    Crlf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Encoding {
    Utf8,
}

impl LineEnding {
    pub fn as_str(&self) -> &str {
        match self {
            LineEnding::Lf => "\n",
            LineEnding::Crlf => "\r\n",
        }
    }

    fn detect(content: &str) -> Self {
        if content.contains("\r\n") {
            LineEnding::Crlf
        } else {
            LineEnding::Lf
        }
    }
}

#[derive(Debug, Clone)]
pub struct Buffer {
    rope: Rope,
    file_path: Option<PathBuf>,
    modified: bool,
    line_ending: LineEnding,
    encoding: Encoding,
    read_only: bool,
    is_binary: bool,
    last_saved: Option<SystemTime>,
    auto_save_enabled: bool,
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
            line_ending: LineEnding::Lf,
            encoding: Encoding::Utf8,
            read_only: false,
            is_binary: false,
            last_saved: None,
            auto_save_enabled: false,
        }
    }

    pub fn from_string(content: &str) -> Self {
        let line_ending = LineEnding::detect(content);
        let normalized = normalize_line_endings(content);
        Self {
            rope: Rope::from_str(&normalized),
            file_path: None,
            modified: false,
            line_ending,
            encoding: Encoding::Utf8,
            read_only: false,
            is_binary: false,
            last_saved: None,
            auto_save_enabled: false,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let metadata = std::fs::metadata(&path)?;
        let file_size = metadata.len();

        let read_only = metadata.permissions().readonly();

        let (content, line_ending) = if file_size > 10_000_000 {
            let mut file = std::fs::File::open(&path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::InvalidData {
                    EditorError::CorruptedFile(path.to_string_lossy().to_string())
                } else {
                    EditorError::Io(e)
                }
            })?;
            let rope = Rope::from_reader(&mut file).map_err(|_e| {
                EditorError::CorruptedFile(format!("{}: invalid UTF-8", path.display()))
            })?;
            let sample = rope.slice(0..rope.len_chars().min(8192)).to_string();
            let line_ending = LineEnding::detect(&sample);
            let content = rope.to_string();
            (content, line_ending)
        } else {
            let content = std::fs::read_to_string(&path).map_err(|e| {
                if e.kind() == std::io::ErrorKind::InvalidData {
                    EditorError::CorruptedFile(format!("{}: invalid UTF-8", path.display()))
                } else {
                    EditorError::Io(e)
                }
            })?;
            let line_ending = LineEnding::detect(&content);
            (content, line_ending)
        };

        let is_binary = Self::detect_binary(&content);
        if is_binary {
            return Err(EditorError::BinaryFile(path.to_string_lossy().to_string()));
        }

        let normalized = normalize_line_endings(&content);
        Ok(Self {
            rope: Rope::from_str(&normalized),
            file_path: Some(path),
            modified: false,
            line_ending,
            encoding: Encoding::Utf8,
            read_only,
            is_binary,
            last_saved: Some(SystemTime::now()),
            auto_save_enabled: false,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        self.check_read_only()?;
        if let Some(path) = &self.file_path {
            self.write_to_file(path)?;
            self.modified = false;
            self.last_saved = Some(SystemTime::now());
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "No file path set for buffer".to_string(),
            ))
        }
    }

    pub fn save_as(&mut self, path: PathBuf) -> Result<()> {
        self.write_to_file(&path)?;
        self.file_path = Some(path);
        self.modified = false;
        self.last_saved = Some(SystemTime::now());
        Ok(())
    }

    fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        use std::io::Write;

        let result = if self.rope.len_chars() > 10_000_000 {
            let file = std::fs::File::create(path)?;
            let mut writer = std::io::BufWriter::new(file);

            for chunk in self.rope.chunks() {
                match self.line_ending {
                    LineEnding::Lf => writer.write_all(chunk.as_bytes()),
                    LineEnding::Crlf => {
                        let converted = chunk.replace('\n', "\r\n");
                        writer.write_all(converted.as_bytes())
                    }
                }?;
            }
            writer.flush()
        } else {
            let content = self.content_with_line_endings();
            std::fs::write(path, content)
        };

        if let Err(e) = result {
            if e.kind() == std::io::ErrorKind::StorageFull || e.raw_os_error() == Some(28) {
                return Err(EditorError::DiskFull(format!(
                    "Failed to write {}: disk full",
                    path.display()
                )));
            }
            return Err(EditorError::Io(e));
        }

        Ok(())
    }

    pub fn insert_char(&mut self, line: usize, column: usize, ch: char) -> Result<()> {
        self.check_read_only()?;
        let char_idx = self.line_col_to_char_idx(line, column)?;
        self.rope.insert_char(char_idx, ch);
        self.modified = true;
        Ok(())
    }

    pub fn delete_char(&mut self, line: usize, column: usize) -> Result<()> {
        self.check_read_only()?;
        let char_idx = self.line_col_to_char_idx(line, column)?;
        if char_idx < self.rope.len_chars() {
            self.rope.remove(char_idx..char_idx + 1);
            self.modified = true;
            Ok(())
        } else {
            Err(EditorError::InvalidPosition { line, column })
        }
    }

    pub fn insert_str(&mut self, line: usize, column: usize, s: &str) -> Result<()> {
        self.check_read_only()?;
        let char_idx = self.line_col_to_char_idx(line, column)?;
        self.rope.insert(char_idx, s);
        self.modified = true;
        Ok(())
    }

    pub fn delete_range(
        &mut self,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Result<()> {
        self.check_read_only()?;
        let start_idx = self.line_col_to_char_idx(start_line, start_col)?;
        let end_idx = self.line_col_to_char_idx(end_line, end_col)?;
        if start_idx <= end_idx && end_idx <= self.rope.len_chars() {
            self.rope.remove(start_idx..end_idx);
            self.modified = true;
            Ok(())
        } else {
            Err(EditorError::InvalidPosition {
                line: end_line,
                column: end_col,
            })
        }
    }

    pub fn set_content(&mut self, content: String) -> Result<()> {
        self.check_read_only()?;
        self.rope = Rope::from_str(&content);
        self.modified = true;
        Ok(())
    }

    pub fn line_count(&self) -> usize {
        self.rope.len_lines()
    }

    pub fn line(&self, line_idx: usize) -> Result<String> {
        if line_idx < self.line_count() {
            let line = self.rope.line(line_idx);
            Ok(line.to_string())
        } else {
            Err(EditorError::InvalidPosition {
                line: line_idx,
                column: 0,
            })
        }
    }

    pub fn line_len(&self, line_idx: usize) -> Result<usize> {
        if line_idx < self.line_count() {
            let line = self.rope.line(line_idx);
            let len = line.len_chars();
            if len > 0 && line.char(len - 1) == '\n' {
                Ok(len - 1)
            } else {
                Ok(len)
            }
        } else {
            Err(EditorError::InvalidPosition {
                line: line_idx,
                column: 0,
            })
        }
    }

    pub fn is_modified(&self) -> bool {
        self.modified
    }

    pub fn file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn content(&self) -> String {
        self.rope.to_string()
    }

    pub fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) -> Result<()> {
        self.check_read_only()?;
        self.line_ending = line_ending;
        self.modified = true;
        Ok(())
    }

    pub fn encoding(&self) -> Encoding {
        self.encoding
    }

    fn content_with_line_endings(&self) -> String {
        match self.line_ending {
            LineEnding::Lf => self.rope.to_string(),
            LineEnding::Crlf => self.rope.to_string().replace('\n', "\r\n"),
        }
    }

    pub fn len_chars(&self) -> usize {
        self.rope.len_chars()
    }

    pub fn char_at(&self, idx: usize) -> Option<char> {
        self.rope.get_char(idx)
    }

    pub fn char_index(&self, line: usize, column: usize) -> Result<usize> {
        self.line_col_to_char_idx(line, column)
    }

    pub fn char_to_line_col(&self, idx: usize) -> Result<(usize, usize)> {
        if idx > self.rope.len_chars() {
            return Err(EditorError::InvalidPosition {
                line: self.line_count().saturating_sub(1),
                column: 0,
            });
        }

        let line = self.rope.char_to_line(idx);
        let line_start = self.rope.line_to_char(line);
        let column = idx - line_start;
        let line_len = self.line_len(line)?;

        if column > line_len {
            Err(EditorError::InvalidPosition { line, column })
        } else {
            Ok((line, column))
        }
    }

    fn line_col_to_char_idx(&self, line: usize, column: usize) -> Result<usize> {
        if line >= self.line_count() {
            return Err(EditorError::InvalidPosition { line, column });
        }

        let line_start_idx = self.rope.line_to_char(line);
        let line_len = self.line_len(line)?;

        if column > line_len {
            return Err(EditorError::InvalidPosition { line, column });
        }

        Ok(line_start_idx + column)
    }

    fn detect_binary(content: &str) -> bool {
        let check_len = content.len().min(8192);
        let bytes = &content.as_bytes()[..check_len];

        for &byte in bytes {
            if byte == 0 {
                return true;
            }
            if byte < 0x20 && byte != b'\t' && byte != b'\n' && byte != b'\r' {
                return true;
            }
        }
        false
    }

    pub fn is_read_only(&self) -> bool {
        self.read_only
    }

    pub fn is_binary(&self) -> bool {
        self.is_binary
    }

    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    fn check_read_only(&self) -> Result<()> {
        if self.read_only {
            let path = self
                .file_path
                .as_ref()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| "buffer".to_string());
            Err(EditorError::ReadOnlyFile(path))
        } else {
            Ok(())
        }
    }

    pub fn last_saved(&self) -> Option<SystemTime> {
        self.last_saved
    }

    pub fn auto_save_enabled(&self) -> bool {
        self.auto_save_enabled
    }

    pub fn set_auto_save(&mut self, enabled: bool) {
        self.auto_save_enabled = enabled;
    }

    pub fn create_backup(&self) -> Result<PathBuf> {
        let path = self.file_path.as_ref().ok_or_else(|| {
            EditorError::InvalidOperation("No file path set for buffer".to_string())
        })?;

        let backup_path = path.with_extension(format!(
            "{}.backup",
            path.extension().and_then(|e| e.to_str()).unwrap_or("txt")
        ));

        self.write_to_file(&backup_path)?;
        Ok(backup_path)
    }

    pub fn save_recovery_data(&self, recovery_dir: &PathBuf) -> Result<PathBuf> {
        use std::fs;

        fs::create_dir_all(recovery_dir)?;

        let recovery_name = if let Some(path) = &self.file_path {
            format!(
                "{}.recovery",
                path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("untitled")
            )
        } else {
            format!(
                "untitled_{}.recovery",
                SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            )
        };

        let recovery_path = recovery_dir.join(recovery_name);
        self.write_to_file(&recovery_path)?;
        Ok(recovery_path)
    }

    pub fn load_recovery_data(recovery_path: PathBuf) -> Result<Self> {
        Self::from_file(recovery_path)
    }

    pub fn check_external_modification(&self) -> Result<bool> {
        if let Some(path) = &self.file_path {
            if let Some(last_saved) = self.last_saved {
                let metadata = std::fs::metadata(path)?;
                let modified_time = metadata.modified()?;
                Ok(modified_time > last_saved)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    pub fn reload_from_disk(&mut self) -> Result<()> {
        if let Some(path) = self.file_path.clone() {
            let metadata = std::fs::metadata(&path)?;
            let file_size = metadata.len();

            let (content, line_ending) = if file_size > 10_000_000 {
                let mut file = std::fs::File::open(&path)?;
                let rope = Rope::from_reader(&mut file)?;
                let sample = rope.slice(0..rope.len_chars().min(8192)).to_string();
                let line_ending = LineEnding::detect(&sample);
                let content = rope.to_string();
                (content, line_ending)
            } else {
                let content = std::fs::read_to_string(&path)?;
                let line_ending = LineEnding::detect(&content);
                (content, line_ending)
            };

            let normalized = normalize_line_endings(&content);
            self.rope = Rope::from_str(&normalized);
            self.line_ending = line_ending;
            self.modified = false;
            self.last_saved = Some(SystemTime::now());
            Ok(())
        } else {
            Err(EditorError::InvalidOperation(
                "No file path set for buffer".to_string(),
            ))
        }
    }

    pub fn has_unsaved_changes(&self) -> bool {
        self.modified
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_line_endings(content: &str) -> String {
    content.replace("\r\n", "\n")
}
