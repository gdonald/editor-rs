use crate::cursor::CursorPosition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    pub position: CursorPosition,
    pub name: Option<String>,
}

impl Bookmark {
    pub fn new(position: CursorPosition) -> Self {
        Self {
            position,
            name: None,
        }
    }

    pub fn with_name(position: CursorPosition, name: String) -> Self {
        Self {
            position,
            name: Some(name),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
    named_bookmarks: HashMap<String, usize>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
            named_bookmarks: HashMap::new(),
        }
    }

    pub fn add_bookmark(&mut self, bookmark: Bookmark) -> usize {
        let index = self.bookmarks.len();
        if let Some(ref name) = bookmark.name {
            self.named_bookmarks.insert(name.clone(), index);
        }
        self.bookmarks.push(bookmark);
        index
    }

    pub fn remove_bookmark(&mut self, index: usize) -> Option<Bookmark> {
        if index < self.bookmarks.len() {
            let bookmark = self.bookmarks.remove(index);

            if let Some(ref name) = bookmark.name {
                self.named_bookmarks.remove(name);
            }

            for (_, idx) in self.named_bookmarks.iter_mut() {
                if *idx > index {
                    *idx -= 1;
                }
            }

            Some(bookmark)
        } else {
            None
        }
    }

    pub fn get_bookmark(&self, index: usize) -> Option<&Bookmark> {
        self.bookmarks.get(index)
    }

    pub fn get_bookmark_by_name(&self, name: &str) -> Option<&Bookmark> {
        self.named_bookmarks
            .get(name)
            .and_then(|&index| self.bookmarks.get(index))
    }

    pub fn bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn find_bookmark_at_position(&self, position: CursorPosition) -> Option<usize> {
        self.bookmarks.iter().position(|b| b.position == position)
    }

    pub fn toggle_bookmark(&mut self, position: CursorPosition) -> bool {
        if let Some(index) = self.find_bookmark_at_position(position) {
            self.remove_bookmark(index);
            false
        } else {
            self.add_bookmark(Bookmark::new(position));
            true
        }
    }

    pub fn clear_all(&mut self) {
        self.bookmarks.clear();
        self.named_bookmarks.clear();
    }

    pub fn next_bookmark(&self, from: CursorPosition) -> Option<&Bookmark> {
        self.bookmarks.iter().find(|b| {
            b.position.line > from.line
                || (b.position.line == from.line && b.position.column > from.column)
        })
    }

    pub fn previous_bookmark(&self, from: CursorPosition) -> Option<&Bookmark> {
        self.bookmarks.iter().rev().find(|b| {
            b.position.line < from.line
                || (b.position.line == from.line && b.position.column < from.column)
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileBookmarks {
    pub file_path: PathBuf,
    pub bookmarks: Vec<Bookmark>,
}

impl FileBookmarks {
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            bookmarks: Vec::new(),
        }
    }

    pub fn from_manager(file_path: PathBuf, manager: &BookmarkManager) -> Self {
        Self {
            file_path,
            bookmarks: manager.bookmarks.clone(),
        }
    }

    pub fn to_manager(&self) -> BookmarkManager {
        let mut manager = BookmarkManager::new();
        for bookmark in &self.bookmarks {
            manager.add_bookmark(bookmark.clone());
        }
        manager
    }
}
