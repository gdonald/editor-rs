use crate::cursor::CursorPosition;
use crate::error::{EditorError, Result};
use crate::selection::Selection;

#[derive(Debug, Clone)]
pub enum Edit {
    Insert {
        position: CursorPosition,
        text: String,
    },
    Delete {
        position: CursorPosition,
        text: String,
    },
    Replace {
        position: CursorPosition,
        old_text: String,
        new_text: String,
    },
}

impl Edit {
    pub fn invert(&self) -> Edit {
        match self {
            Edit::Insert { position, text } => Edit::Delete {
                position: *position,
                text: text.clone(),
            },
            Edit::Delete { position, text } => Edit::Insert {
                position: *position,
                text: text.clone(),
            },
            Edit::Replace {
                position,
                old_text,
                new_text,
            } => Edit::Replace {
                position: *position,
                old_text: new_text.clone(),
                new_text: old_text.clone(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub edits: Vec<Edit>,
    pub cursor_before: Vec<CursorPosition>,
    pub cursor_after: Vec<CursorPosition>,
    pub selection_before: Option<Selection>,
    pub selection_after: Option<Selection>,
    pub grouped: bool,
}

impl HistoryEntry {
    pub fn new(
        edits: Vec<Edit>,
        cursor_before: Vec<CursorPosition>,
        cursor_after: Vec<CursorPosition>,
        selection_before: Option<Selection>,
        selection_after: Option<Selection>,
    ) -> Self {
        Self {
            edits,
            cursor_before,
            cursor_after,
            selection_before,
            selection_after,
            grouped: false,
        }
    }

    pub fn with_grouped(mut self, grouped: bool) -> Self {
        self.grouped = grouped;
        self
    }
}

pub struct History {
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
    max_entries: usize,
    group_timeout_ms: u64,
    last_edit_time: Option<std::time::Instant>,
}

impl History {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            max_entries: 1000,
            group_timeout_ms: 500,
            last_edit_time: None,
        }
    }

    pub fn with_max_entries(mut self, max_entries: usize) -> Self {
        self.max_entries = max_entries;
        self
    }

    pub fn with_group_timeout(mut self, timeout_ms: u64) -> Self {
        self.group_timeout_ms = timeout_ms;
        self
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        let now = std::time::Instant::now();
        let should_group = if let Some(last_time) = self.last_edit_time {
            now.duration_since(last_time).as_millis() < self.group_timeout_ms as u128
                && entry.grouped
        } else {
            false
        };

        if should_group && !self.undo_stack.is_empty() {
            if let Some(last_entry) = self.undo_stack.last_mut() {
                if last_entry.grouped {
                    last_entry.edits.extend(entry.edits);
                    last_entry.cursor_after = entry.cursor_after;
                    last_entry.selection_after = entry.selection_after;
                } else {
                    self.undo_stack.push(entry);
                }
            }
        } else {
            self.undo_stack.push(entry);
        }

        if self.undo_stack.len() > self.max_entries {
            self.undo_stack.remove(0);
        }

        self.redo_stack.clear();
        self.last_edit_time = Some(now);
    }

    pub fn undo(&mut self) -> Result<HistoryEntry> {
        if let Some(entry) = self.undo_stack.pop() {
            self.redo_stack.push(entry.clone());
            Ok(entry)
        } else {
            Err(EditorError::InvalidOperation("Nothing to undo".to_string()))
        }
    }

    pub fn redo(&mut self) -> Result<HistoryEntry> {
        if let Some(entry) = self.redo_stack.pop() {
            self.undo_stack.push(entry.clone());
            Ok(entry)
        } else {
            Err(EditorError::InvalidOperation("Nothing to redo".to_string()))
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    pub fn clear(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.last_edit_time = None;
    }

    pub fn undo_stack_len(&self) -> usize {
        self.undo_stack.len()
    }

    pub fn redo_stack_len(&self) -> usize {
        self.redo_stack.len()
    }

    pub fn memory_usage(&self) -> usize {
        let undo_size: usize = self
            .undo_stack
            .iter()
            .map(|entry| {
                entry
                    .edits
                    .iter()
                    .map(|edit| match edit {
                        Edit::Insert { text, .. } | Edit::Delete { text, .. } => text.len(),
                        Edit::Replace {
                            old_text, new_text, ..
                        } => old_text.len() + new_text.len(),
                    })
                    .sum::<usize>()
            })
            .sum();

        let redo_size: usize = self
            .redo_stack
            .iter()
            .map(|entry| {
                entry
                    .edits
                    .iter()
                    .map(|edit| match edit {
                        Edit::Insert { text, .. } | Edit::Delete { text, .. } => text.len(),
                        Edit::Replace {
                            old_text, new_text, ..
                        } => old_text.len() + new_text.len(),
                    })
                    .sum::<usize>()
            })
            .sum();

        undo_size + redo_size
    }
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}
