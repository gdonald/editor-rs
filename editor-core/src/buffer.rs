use crate::error::{EditorError, Result};
use ropey::Rope;
use std::path::PathBuf;

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
}

impl Buffer {
    pub fn new() -> Self {
        Self {
            rope: Rope::new(),
            file_path: None,
            modified: false,
            line_ending: LineEnding::Lf,
            encoding: Encoding::Utf8,
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
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
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
        Ok(Self {
            rope: Rope::from_str(&normalized),
            file_path: Some(path),
            modified: false,
            line_ending,
            encoding: Encoding::Utf8,
        })
    }

    pub fn save(&mut self) -> Result<()> {
        if let Some(path) = &self.file_path {
            self.write_to_file(path)?;
            self.modified = false;
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
        Ok(())
    }

    fn write_to_file(&self, path: &PathBuf) -> Result<()> {
        use std::io::Write;

        if self.rope.len_chars() > 10_000_000 {
            let file = std::fs::File::create(path)?;
            let mut writer = std::io::BufWriter::new(file);

            for chunk in self.rope.chunks() {
                match self.line_ending {
                    LineEnding::Lf => writer.write_all(chunk.as_bytes())?,
                    LineEnding::Crlf => {
                        let converted = chunk.replace('\n', "\r\n");
                        writer.write_all(converted.as_bytes())?;
                    }
                }
            }
            writer.flush()?;
        } else {
            let content = self.content_with_line_endings();
            std::fs::write(path, content)?;
        }
        Ok(())
    }

    pub fn insert_char(&mut self, line: usize, column: usize, ch: char) -> Result<()> {
        let char_idx = self.line_col_to_char_idx(line, column)?;
        self.rope.insert_char(char_idx, ch);
        self.modified = true;
        Ok(())
    }

    pub fn delete_char(&mut self, line: usize, column: usize) -> Result<()> {
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

    pub fn set_content(&mut self, content: String) {
        self.rope = Rope::from_str(&content);
        self.modified = true;
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

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
        self.modified = true;
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
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

fn normalize_line_endings(content: &str) -> String {
    content.replace("\r\n", "\n")
}
