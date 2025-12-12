use crate::bookmark::BookmarkManager;
use crate::buffer::Buffer;
use crate::clipboard::ClipboardManager;
use crate::command::Command;
use crate::cursor::{CursorPosition, MultiCursor};
use crate::error::Result;
use crate::selection::Selection;
use std::path::{Path, PathBuf};

use super::mode::EditorMode;

pub struct EditorState {
    pub(super) buffer: Buffer,
    pub(super) cursors: MultiCursor,
    pub(super) viewport_top: usize,
    pub(super) scroll_offset: usize,
    pub(super) status_message: String,
    pub(super) overwrite_mode: bool,
    pub(super) soft_wrap_width: Option<usize>,
    pub(super) selection: Option<Selection>,
    pub(super) block_selection_mode: bool,
    pub(super) bookmarks: BookmarkManager,
    pub(super) clipboard: ClipboardManager,
    pub(super) mode: EditorMode,
    pub(super) last_search_query: Option<String>,
    pub(super) search_options: SearchOptions,
    pub(super) search_history: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SearchOptions {
    pub case_sensitive: bool,
    pub use_regex: bool,
    pub whole_word: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_sensitive: true, // Default to true as per roadmap? "Case-sensitive (default usually)"
            use_regex: false,
            whole_word: false,
        }
    }
}

impl EditorState {
    pub fn new() -> Self {
        Self {
            buffer: Buffer::new(),
            cursors: MultiCursor::new(),
            viewport_top: 0,
            scroll_offset: 5,
            status_message: String::new(),
            overwrite_mode: false,
            soft_wrap_width: None,
            selection: None,
            block_selection_mode: false,
            bookmarks: BookmarkManager::new(),
            clipboard: ClipboardManager,
            mode: EditorMode::default(),
            last_search_query: None,
            search_options: SearchOptions::default(),
            search_history: Vec::new(),
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let buffer = Buffer::from_file(path)?;
        Ok(Self {
            buffer,
            cursors: MultiCursor::new(),
            viewport_top: 0,
            scroll_offset: 5,
            status_message: String::new(),
            overwrite_mode: false,
            soft_wrap_width: None,
            selection: None,
            block_selection_mode: false,
            bookmarks: BookmarkManager::new(),
            clipboard: ClipboardManager,
            mode: EditorMode::default(),
            last_search_query: None,
            search_options: SearchOptions::default(),
            search_history: Vec::new(),
        })
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        use crate::error::EditorError;
        if self.buffer.is_read_only() && command.is_editing_command() {
            return Err(EditorError::ReadOnlyFile(
                self.buffer
                    .file_path()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "buffer".to_string()),
            ));
        }

        match command {
            Command::InsertChar(ch) => self.insert_char(ch),
            Command::DeleteChar => self.delete_char(),
            Command::Backspace => self.backspace(),
            Command::NewLine => self.new_line(),
            Command::DeleteLine => self.delete_line(),
            Command::DuplicateLine => self.duplicate_line(),
            Command::MoveLinesUp => self.move_lines_up(),
            Command::MoveLinesDown => self.move_lines_down(),
            Command::JoinLines => self.join_lines(),
            Command::SortLines { numerical } => self.sort_lines(numerical),
            Command::ChangeCase { mode } => self.change_case(mode),
            Command::TransposeCharacters => self.transpose_characters(),
            Command::Indent => self.indent_line(),
            Command::Dedent => self.dedent_line(),

            Command::MoveCursorUp => self.move_cursor_up(),
            Command::MoveCursorDown => self.move_cursor_down(),
            Command::MoveCursorLeft => self.move_cursor_left(),
            Command::MoveCursorRight => self.move_cursor_right(),
            Command::MoveToStartOfLine => self.move_to_start_of_line(),
            Command::MoveToEndOfLine => self.move_to_end_of_line(),
            Command::MoveToStartOfFile => self.move_to_start_of_file(),
            Command::MoveToEndOfFile => self.move_to_end_of_file(),
            Command::MoveCursorWordLeft => self.move_cursor_word_left(),
            Command::MoveCursorWordRight => self.move_cursor_word_right(),
            Command::PageUp => self.page_up(20),
            Command::PageDown => self.page_down(20),

            Command::ToggleOverwriteMode => self.toggle_overwrite_mode(),
            Command::HardWrap(width) => self.hard_wrap(width),
            Command::SetSoftWrap(width) => self.set_soft_wrap(width),
            Command::TrimTrailingWhitespace => self.trim_trailing_whitespace(),

            Command::Open(path) => self.open_file(path),
            Command::Save => self.save(),
            Command::SaveAs(path) => self.save_as(path),
            Command::New => self.new_buffer(),
            Command::Close => Ok(()),

            Command::GotoLine(line) => self.goto_line(line),
            Command::JumpToMatchingBracket => self.jump_to_matching_bracket(),
            Command::InsertCharWithAutoClose(ch) => self.insert_char_with_auto_close(ch),
            Command::AddCursor(position) => self.add_cursor(position),
            Command::RemoveCursor(index) => self.remove_cursor(index),
            Command::ClearSecondaryCursors => self.clear_secondary_cursors(),

            Command::MouseClick(position) => self.mouse_click(position),
            Command::MouseDragStart(position) => self.mouse_drag_start(position),
            Command::MouseDrag(position) => self.mouse_drag(position),
            Command::MouseDragEnd(position) => self.mouse_drag_end(position),
            Command::MouseDoubleClick(position) => self.mouse_double_click(position),
            Command::MouseTripleClick(position) => self.mouse_triple_click(position),
            Command::ToggleBlockSelection => self.toggle_block_selection(),

            Command::ToggleBookmark => self.toggle_bookmark(),
            Command::AddNamedBookmark(name) => self.add_named_bookmark(name),
            Command::RemoveBookmark(index) => self.remove_bookmark(index),
            Command::JumpToBookmark(index) => self.jump_to_bookmark(index),
            Command::JumpToNamedBookmark(name) => self.jump_to_named_bookmark(name),
            Command::NextBookmark => self.next_bookmark(),
            Command::PreviousBookmark => self.previous_bookmark(),
            Command::ClearAllBookmarks => self.clear_all_bookmarks(),

            Command::SelectionStart => self.selection_start(),
            Command::SelectionEnd => self.selection_end(),
            Command::Copy => self.copy(),
            Command::Cut => self.cut(),
            Command::Paste => self.paste(),
            Command::ToggleLineComment => self.toggle_line_comment(),
            Command::ToggleBlockComment => self.toggle_block_comment(),
            Command::FoldCode => self.fold_code(),
            Command::UnfoldCode => self.unfold_code(),

            Command::ToggleReadOnly => self.toggle_read_only(),

            Command::Search(query) => self.search(query),
            Command::NextMatch => self.next_match(),
            Command::PreviousMatch => self.previous_match(),

            _ => Err(EditorError::InvalidOperation(
                "Command not yet implemented".to_string(),
            )),
        }
    }

    pub fn toggle_read_only(&mut self) -> Result<()> {
        let current = self.buffer.is_read_only();
        self.buffer.set_read_only(!current);
        Ok(())
    }

    pub(super) fn map_cursors<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut EditorState, CursorPosition) -> Result<CursorPosition>,
    {
        let positions = self.cursors.positions().to_vec();
        let mut updated = Vec::with_capacity(positions.len());
        for pos in positions {
            updated.push(f(self, pos)?);
        }
        self.cursors.set_positions(updated);
        Ok(())
    }

    pub(super) fn map_cursors_descending<F>(&mut self, mut f: F) -> Result<()>
    where
        F: FnMut(&mut EditorState, CursorPosition) -> Result<CursorPosition>,
    {
        let mut positions = self.cursors.positions().to_vec();
        positions.sort_by(|a, b| a.line.cmp(&b.line).then(a.column.cmp(&b.column)));
        positions.reverse();

        let mut updated = Vec::with_capacity(positions.len());
        for pos in positions {
            updated.push(f(self, pos)?);
        }

        self.cursors.set_positions(updated);
        Ok(())
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    pub fn cursor(&self) -> &CursorPosition {
        self.cursors.primary()
    }

    pub fn cursors(&self) -> &[CursorPosition] {
        self.cursors.positions()
    }

    pub fn cursor_count(&self) -> usize {
        self.cursors.positions().len()
    }

    pub fn viewport_top(&self) -> usize {
        self.viewport_top
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }

    pub fn set_status_message(&mut self, message: String) {
        self.status_message = message;
    }

    pub fn overwrite_mode(&self) -> bool {
        self.overwrite_mode
    }

    pub fn soft_wrap_width(&self) -> Option<usize> {
        self.soft_wrap_width
    }

    pub fn scroll_offset(&self) -> usize {
        self.scroll_offset
    }

    pub fn set_scroll_offset(&mut self, offset: usize) {
        self.scroll_offset = offset;
    }

    pub fn file_path(&self) -> Option<&Path> {
        self.buffer.file_path().map(|p| p.as_path())
    }

    pub fn selection(&self) -> Option<&Selection> {
        self.selection.as_ref()
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    pub fn bookmarks(&self) -> &BookmarkManager {
        &self.bookmarks
    }

    pub fn bookmarks_mut(&mut self) -> &mut BookmarkManager {
        &mut self.bookmarks
    }

    pub fn mode(&self) -> EditorMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn search_options(&self) -> SearchOptions {
        self.search_options
    }

    pub fn set_search_options(&mut self, options: SearchOptions) {
        self.search_options = options;
    }

    pub fn search_history(&self) -> &[String] {
        &self.search_history
    }

    pub(super) fn validate_position(&self, position: CursorPosition) -> Result<()> {
        use crate::error::EditorError;

        if position.line >= self.buffer.line_count() {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        let line_len = self.buffer.line_len(position.line)?;
        if position.column > line_len {
            return Err(EditorError::InvalidPosition {
                line: position.line,
                column: position.column,
            });
        }

        Ok(())
    }

    pub(super) fn indentation_for_line(&self, line_idx: usize) -> Result<String> {
        let line = self.buffer.line(line_idx)?;
        let trimmed = line.trim_end_matches('\n');
        let mut indent = String::new();

        for ch in trimmed.chars() {
            if ch == ' ' || ch == '\t' {
                indent.push(ch);
            } else {
                break;
            }
        }

        Ok(indent)
    }

    pub(super) fn wrap_line_to_width(&self, line: &str, width: usize) -> Vec<String> {
        if width == 0 {
            return vec![line.to_string()];
        }

        let mut chunks = Vec::new();
        let mut current = String::new();
        let mut count = 0;

        for ch in line.chars() {
            current.push(ch);
            count += 1;

            if count == width {
                chunks.push(current);
                current = String::new();
                count = 0;
            }
        }

        if !current.is_empty() || line.is_empty() {
            chunks.push(current);
        }

        chunks
    }

    pub(super) fn clamp_cursors_after_edit(&mut self) -> Result<()> {
        let mut positions = Vec::with_capacity(self.cursors.positions().len());
        for mut pos in self.cursors.positions().to_vec() {
            let line_count = self.buffer.line_count();
            if line_count == 0 {
                pos.line = 0;
                pos.column = 0;
                positions.push(pos);
                continue;
            }

            let last_line = line_count - 1;
            if pos.line > last_line {
                pos.line = last_line;
            }

            let line_len = self.buffer.line_len(pos.line)?;
            if pos.column > line_len {
                pos.column = line_len;
            }

            positions.push(pos);
        }

        if positions.is_empty() {
            positions.push(CursorPosition::zero());
        }

        self.cursors.set_positions(positions);
        Ok(())
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}

pub(super) fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_'
}
