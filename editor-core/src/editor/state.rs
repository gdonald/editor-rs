use crate::bookmark::BookmarkManager;
use crate::buffer::Buffer;
use crate::clipboard::ClipboardManager;
use crate::command::Command;
use crate::cursor::MultiCursor;
use crate::error::Result;
use crate::git_history::{CleanupStats, GitHistoryManager, HistoryStats, LargeFileConfig};
use crate::history::History;
use crate::history_browser::HistoryBrowser;
use crate::selection::Selection;
use std::path::PathBuf;

use super::mode::EditorMode;
use super::search_types::SearchOptions;

pub struct EditorState {
    pub(super) buffers: Vec<Buffer>,
    pub(super) current_buffer_index: usize,
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
    pub(super) replace_history: Vec<(String, String)>,
    pub(super) history: History,
    pub(super) git_history: GitHistoryManager,
    pub(super) auto_commit_enabled: bool,
    pub(super) large_file_config: LargeFileConfig,
    pub(super) history_browser: Option<HistoryBrowser>,
    pub(super) history_stats: Option<HistoryStats>,
    pub(super) cleanup_stats: Option<CleanupStats>,
}

impl EditorState {
    pub fn new() -> Self {
        let large_file_config = LargeFileConfig::default();
        let git_history =
            GitHistoryManager::default().with_large_file_config(large_file_config.clone());
        Self {
            buffers: vec![Buffer::new()],
            current_buffer_index: 0,
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
            replace_history: Vec::new(),
            history: History::new(),
            git_history,
            auto_commit_enabled: true,
            large_file_config,
            history_browser: None,
            history_stats: None,
            cleanup_stats: None,
        }
    }

    pub fn from_file(path: PathBuf) -> Result<Self> {
        let buffer = Buffer::from_file(path)?;
        let large_file_config = LargeFileConfig::default();
        let git_history =
            GitHistoryManager::default().with_large_file_config(large_file_config.clone());
        Ok(Self {
            buffers: vec![buffer],
            current_buffer_index: 0,
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
            replace_history: Vec::new(),
            history: History::new(),
            git_history,
            auto_commit_enabled: true,
            large_file_config,
            history_browser: None,
            history_stats: None,
            cleanup_stats: None,
        })
    }

    pub fn execute_command(&mut self, command: Command) -> Result<()> {
        use crate::error::EditorError;
        if self.buffer().is_read_only() && command.is_editing_command() {
            return Err(EditorError::ReadOnlyFile(
                self.buffer()
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

            Command::ReplaceNext { find, replace } => self.replace_next(find, replace),
            Command::ReplaceAll { find, replace } => self.replace_all(find, replace),
            Command::ReplaceInSelection { find, replace } => {
                self.replace_in_selection(find, replace)
            }

            Command::Undo => self.undo(),
            Command::Redo => self.redo(),

            Command::OpenHistoryBrowser => self.open_history_browser(),
            Command::CloseHistoryBrowser => self.close_history_browser(),
            Command::HistoryNavigateNext => self.history_navigate_next(),
            Command::HistoryNavigatePrevious => self.history_navigate_previous(),
            Command::HistoryNavigateFirst => self.history_navigate_first(),
            Command::HistoryNavigateLast => self.history_navigate_last(),
            Command::HistoryPageUp => self.history_page_up(),
            Command::HistoryPageDown => self.history_page_down(),
            Command::HistorySelectCommit(index) => self.history_select_commit(index),
            Command::HistoryToggleFileList => self.history_toggle_file_list(),
            Command::HistoryViewDiff => self.history_view_diff(),
            Command::HistoryToggleSideBySide => self.history_toggle_side_by_side(),
            Command::HistorySearch(query) => self.history_search(&query),
            Command::HistoryClearSearch => self.history_clear_search(),
            Command::HistoryFilterByFile(file_pattern) => {
                self.history_filter_by_file(&file_pattern)
            }
            Command::HistoryClearFileFilter => self.history_clear_file_filter(),
            Command::HistoryRestoreCommit(commit_id) => self.history_restore_commit(&commit_id),
            Command::HistoryRestoreFile {
                commit_id,
                file_path,
            } => self.history_restore_file(&commit_id, &file_path),
            Command::HistoryPreviewRestore(commit_id) => self.history_preview_restore(&commit_id),
            Command::HistorySetBaseCommit(index) => self.history_set_base_commit(index),
            Command::HistoryClearBaseCommit => self.history_clear_base_commit(),
            Command::HistoryAddAnnotation {
                commit_id,
                annotation,
            } => self.history_add_annotation(&commit_id, annotation.clone()),
            Command::HistoryRemoveAnnotation(commit_id) => {
                self.history_remove_annotation(&commit_id)
            }
            Command::ShowHistoryStats => self.show_history_stats(),
            Command::CleanupHistory => self.cleanup_history(),

            _ => Err(EditorError::InvalidOperation(
                "Command not yet implemented".to_string(),
            )),
        }
    }

    pub fn toggle_read_only(&mut self) -> Result<()> {
        let current = self.buffer().is_read_only();
        self.buffer_mut().set_read_only(!current);
        Ok(())
    }
}

impl Default for EditorState {
    fn default() -> Self {
        Self::new()
    }
}
