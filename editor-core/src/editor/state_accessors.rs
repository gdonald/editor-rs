use crate::buffer::Buffer;
use crate::cursor::CursorPosition;
use crate::git_history::{CleanupStats, HistoryStats, LargeFileConfig};
use crate::history_browser::HistoryBrowser;
use std::path::Path;

use super::mode::EditorMode;
use super::search_types::SearchOptions;
use super::state::EditorState;

impl EditorState {
    pub(super) fn buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer_index]
    }

    pub(super) fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
    }

    pub fn current_buffer(&self) -> &Buffer {
        &self.buffers[self.current_buffer_index]
    }

    pub fn current_buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffers[self.current_buffer_index]
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
        self.buffer().file_path().map(|p| p.as_path())
    }

    pub fn auto_commit_enabled(&self) -> bool {
        self.auto_commit_enabled
    }

    pub fn set_auto_commit_enabled(&mut self, enabled: bool) {
        self.auto_commit_enabled = enabled;
    }

    pub fn large_file_config(&self) -> &LargeFileConfig {
        &self.large_file_config
    }

    pub fn set_large_file_config(&mut self, config: LargeFileConfig) {
        self.large_file_config = config.clone();
        self.git_history.set_large_file_config(config);
    }

    pub fn selection(&self) -> Option<&crate::selection::Selection> {
        self.selection.as_ref()
    }

    pub fn has_selection(&self) -> bool {
        self.selection.is_some()
    }

    pub fn bookmarks(&self) -> &crate::bookmark::BookmarkManager {
        &self.bookmarks
    }

    pub fn bookmarks_mut(&mut self) -> &mut crate::bookmark::BookmarkManager {
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

    pub fn replace_history(&self) -> &[(String, String)] {
        &self.replace_history
    }

    pub fn is_history_browser_open(&self) -> bool {
        self.history_browser.is_some()
    }

    pub fn history_browser(&self) -> Option<&HistoryBrowser> {
        self.history_browser.as_ref()
    }

    pub fn history_browser_mut(&mut self) -> Option<&mut HistoryBrowser> {
        self.history_browser.as_mut()
    }

    pub fn is_history_stats_open(&self) -> bool {
        self.history_stats.is_some()
    }

    pub fn history_stats(&self) -> Option<&HistoryStats> {
        self.history_stats.as_ref()
    }

    pub fn close_history_stats(&mut self) {
        self.history_stats = None;
    }

    pub fn is_cleanup_stats_open(&self) -> bool {
        self.cleanup_stats.is_some()
    }

    pub fn cleanup_stats(&self) -> Option<&CleanupStats> {
        self.cleanup_stats.as_ref()
    }

    pub fn close_cleanup_stats(&mut self) {
        self.cleanup_stats = None;
    }
}
