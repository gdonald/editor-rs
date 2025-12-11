use editor_core::{Bookmark, BookmarkManager, CursorPosition, EditorState, FileBookmarks};
use std::path::PathBuf;

#[test]
fn test_bookmark_creation() {
    let pos = CursorPosition::new(5, 10);
    let bookmark = Bookmark::new(pos);

    assert_eq!(bookmark.position, pos);
    assert_eq!(bookmark.name, None);
}

#[test]
fn test_named_bookmark_creation() {
    let pos = CursorPosition::new(5, 10);
    let name = "test_bookmark".to_string();
    let bookmark = Bookmark::with_name(pos, name.clone());

    assert_eq!(bookmark.position, pos);
    assert_eq!(bookmark.name, Some(name));
}

#[test]
fn test_bookmark_manager_add_bookmark() {
    let mut manager = BookmarkManager::new();
    let pos = CursorPosition::new(5, 10);
    let bookmark = Bookmark::new(pos);

    let index = manager.add_bookmark(bookmark.clone());

    assert_eq!(index, 0);
    assert_eq!(manager.bookmarks().len(), 1);
    assert_eq!(manager.get_bookmark(0).unwrap().position, pos);
}

#[test]
fn test_bookmark_manager_add_named_bookmark() {
    let mut manager = BookmarkManager::new();
    let pos = CursorPosition::new(5, 10);
    let name = "test".to_string();
    let bookmark = Bookmark::with_name(pos, name.clone());

    manager.add_bookmark(bookmark);

    assert_eq!(manager.bookmarks().len(), 1);
    assert_eq!(manager.get_bookmark_by_name(&name).unwrap().position, pos);
}

#[test]
fn test_bookmark_manager_remove_bookmark() {
    let mut manager = BookmarkManager::new();
    let pos = CursorPosition::new(5, 10);
    let bookmark = Bookmark::new(pos);

    manager.add_bookmark(bookmark);
    assert_eq!(manager.bookmarks().len(), 1);

    let removed = manager.remove_bookmark(0);
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().position, pos);
    assert_eq!(manager.bookmarks().len(), 0);
}

#[test]
fn test_bookmark_manager_remove_nonexistent_bookmark() {
    let mut manager = BookmarkManager::new();

    let removed = manager.remove_bookmark(5);
    assert!(removed.is_none());
}

#[test]
fn test_bookmark_manager_remove_named_bookmark() {
    let mut manager = BookmarkManager::new();
    let pos = CursorPosition::new(5, 10);
    let name = "test".to_string();
    let bookmark = Bookmark::with_name(pos, name.clone());

    manager.add_bookmark(bookmark);

    let removed = manager.remove_bookmark(0);
    assert!(removed.is_some());
    assert!(manager.get_bookmark_by_name(&name).is_none());
}

#[test]
fn test_bookmark_manager_find_bookmark_at_position() {
    let mut manager = BookmarkManager::new();
    let pos1 = CursorPosition::new(5, 10);
    let pos2 = CursorPosition::new(10, 20);

    manager.add_bookmark(Bookmark::new(pos1));
    manager.add_bookmark(Bookmark::new(pos2));

    assert_eq!(manager.find_bookmark_at_position(pos1), Some(0));
    assert_eq!(manager.find_bookmark_at_position(pos2), Some(1));
    assert_eq!(
        manager.find_bookmark_at_position(CursorPosition::new(15, 5)),
        None
    );
}

#[test]
fn test_bookmark_manager_toggle_bookmark() {
    let mut manager = BookmarkManager::new();
    let pos = CursorPosition::new(5, 10);

    let added = manager.toggle_bookmark(pos);
    assert!(added);
    assert_eq!(manager.bookmarks().len(), 1);

    let removed = manager.toggle_bookmark(pos);
    assert!(!removed);
    assert_eq!(manager.bookmarks().len(), 0);
}

#[test]
fn test_bookmark_manager_clear_all() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark(Bookmark::new(CursorPosition::new(5, 10)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(10, 20)));
    manager.add_bookmark(Bookmark::with_name(
        CursorPosition::new(15, 5),
        "test".to_string(),
    ));

    assert_eq!(manager.bookmarks().len(), 3);

    manager.clear_all();

    assert_eq!(manager.bookmarks().len(), 0);
    assert!(manager.get_bookmark_by_name("test").is_none());
}

#[test]
fn test_bookmark_manager_next_bookmark() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark(Bookmark::new(CursorPosition::new(5, 10)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(10, 20)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(15, 5)));

    let current = CursorPosition::new(6, 0);
    let next = manager.next_bookmark(current);
    assert!(next.is_some());
    assert_eq!(next.unwrap().position, CursorPosition::new(10, 20));

    let current = CursorPosition::new(10, 15);
    let next = manager.next_bookmark(current);
    assert!(next.is_some());
    assert_eq!(next.unwrap().position, CursorPosition::new(10, 20));

    let current = CursorPosition::new(20, 0);
    let next = manager.next_bookmark(current);
    assert!(next.is_none());
}

#[test]
fn test_bookmark_manager_previous_bookmark() {
    let mut manager = BookmarkManager::new();

    manager.add_bookmark(Bookmark::new(CursorPosition::new(5, 10)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(10, 20)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(15, 5)));

    let current = CursorPosition::new(12, 0);
    let prev = manager.previous_bookmark(current);
    assert!(prev.is_some());
    assert_eq!(prev.unwrap().position, CursorPosition::new(10, 20));

    let current = CursorPosition::new(10, 15);
    let prev = manager.previous_bookmark(current);
    assert!(prev.is_some());
    assert_eq!(prev.unwrap().position, CursorPosition::new(5, 10));

    let current = CursorPosition::new(3, 0);
    let prev = manager.previous_bookmark(current);
    assert!(prev.is_none());
}

#[test]
fn test_file_bookmarks_creation() {
    let path = PathBuf::from("/test/file.rs");
    let file_bookmarks = FileBookmarks::new(path.clone());

    assert_eq!(file_bookmarks.file_path, path);
    assert_eq!(file_bookmarks.bookmarks.len(), 0);
}

#[test]
fn test_file_bookmarks_from_manager() {
    let mut manager = BookmarkManager::new();
    manager.add_bookmark(Bookmark::new(CursorPosition::new(5, 10)));
    manager.add_bookmark(Bookmark::new(CursorPosition::new(10, 20)));

    let path = PathBuf::from("/test/file.rs");
    let file_bookmarks = FileBookmarks::from_manager(path.clone(), &manager);

    assert_eq!(file_bookmarks.file_path, path);
    assert_eq!(file_bookmarks.bookmarks.len(), 2);
}

#[test]
fn test_file_bookmarks_to_manager() {
    let path = PathBuf::from("/test/file.rs");
    let mut file_bookmarks = FileBookmarks::new(path);

    file_bookmarks
        .bookmarks
        .push(Bookmark::new(CursorPosition::new(5, 10)));
    file_bookmarks
        .bookmarks
        .push(Bookmark::new(CursorPosition::new(10, 20)));

    let manager = file_bookmarks.to_manager();

    assert_eq!(manager.bookmarks().len(), 2);
    assert_eq!(
        manager.get_bookmark(0).unwrap().position,
        CursorPosition::new(5, 10)
    );
    assert_eq!(
        manager.get_bookmark(1).unwrap().position,
        CursorPosition::new(10, 20)
    );
}

#[test]
fn test_editor_state_toggle_bookmark() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 1);
    assert_eq!(
        state.bookmarks().get_bookmark(0).unwrap().position,
        CursorPosition::new(1, 1)
    );

    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 0);
}

#[test]
fn test_editor_state_add_named_bookmark() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test".to_string()))
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 1);
    assert_eq!(
        state
            .bookmarks()
            .get_bookmark_by_name("test")
            .unwrap()
            .position,
        CursorPosition::new(0, 1)
    );
}

#[test]
fn test_editor_state_replace_named_bookmark_at_same_position() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test1".to_string()))
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 1);

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test2".to_string()))
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 1);
    assert_eq!(
        state
            .bookmarks()
            .get_bookmark_by_name("test2")
            .unwrap()
            .position,
        CursorPosition::new(0, 1)
    );
    assert!(state.bookmarks().get_bookmark_by_name("test1").is_none());
}

#[test]
fn test_editor_state_remove_bookmark() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 1);

    state
        .execute_command(editor_core::Command::RemoveBookmark(0))
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 0);
}

#[test]
fn test_editor_state_remove_nonexistent_bookmark() {
    let mut state = EditorState::new();

    let result = state.execute_command(editor_core::Command::RemoveBookmark(5));

    assert!(result.is_err());
}

#[test]
fn test_editor_state_jump_to_bookmark() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('c'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    assert_eq!(state.cursor().line, 2);

    state
        .execute_command(editor_core::Command::JumpToBookmark(0))
        .unwrap();

    assert_eq!(state.cursor().line, 0);
    assert_eq!(state.cursor().column, 0);
}

#[test]
fn test_editor_state_jump_to_named_bookmark() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("start".to_string()))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    assert_eq!(state.cursor().line, 1);

    state
        .execute_command(editor_core::Command::JumpToNamedBookmark(
            "start".to_string(),
        ))
        .unwrap();

    assert_eq!(state.cursor().line, 0);
    assert_eq!(state.cursor().column, 0);
}

#[test]
fn test_editor_state_jump_to_nonexistent_bookmark() {
    let mut state = EditorState::new();

    let result = state.execute_command(editor_core::Command::JumpToBookmark(5));

    assert!(result.is_err());
}

#[test]
fn test_editor_state_jump_to_nonexistent_named_bookmark() {
    let mut state = EditorState::new();

    let result = state.execute_command(editor_core::Command::JumpToNamedBookmark(
        "nonexistent".to_string(),
    ));

    assert!(result.is_err());
}

#[test]
fn test_editor_state_next_bookmark() {
    let mut state = EditorState::new();
    for i in 0..10 {
        state
            .execute_command(editor_core::Command::InsertChar((b'a' + i as u8) as char))
            .unwrap();
        state
            .execute_command(editor_core::Command::NewLine)
            .unwrap();
    }

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::NextBookmark)
        .unwrap();

    assert_eq!(state.cursor().line, 1);

    state
        .execute_command(editor_core::Command::NextBookmark)
        .unwrap();

    assert_eq!(state.cursor().line, 3);
}

#[test]
fn test_editor_state_next_bookmark_no_more() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    let result = state.execute_command(editor_core::Command::NextBookmark);

    assert!(result.is_err());
}

#[test]
fn test_editor_state_previous_bookmark() {
    let mut state = EditorState::new();
    for i in 0..10 {
        state
            .execute_command(editor_core::Command::InsertChar((b'a' + i as u8) as char))
            .unwrap();
        state
            .execute_command(editor_core::Command::NewLine)
            .unwrap();
    }

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::PreviousBookmark)
        .unwrap();

    assert_eq!(state.cursor().line, 3);

    state
        .execute_command(editor_core::Command::PreviousBookmark)
        .unwrap();

    assert_eq!(state.cursor().line, 1);
}

#[test]
fn test_editor_state_previous_bookmark_no_more() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();

    let result = state.execute_command(editor_core::Command::PreviousBookmark);

    assert!(result.is_err());
}

#[test]
fn test_editor_state_clear_all_bookmarks() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 2);

    state
        .execute_command(editor_core::Command::ClearAllBookmarks)
        .unwrap();

    assert_eq!(state.bookmarks().bookmarks().len(), 0);
}

#[test]
fn test_bookmark_status_messages() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();
    assert!(state.status_message().contains("Bookmark added"));

    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();
    assert!(state.status_message().contains("Bookmark removed"));
}

#[test]
fn test_named_bookmark_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test".to_string()))
        .unwrap();
    assert!(state
        .status_message()
        .contains("Named bookmark 'test' added"));
}

#[test]
fn test_remove_bookmark_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::RemoveBookmark(0))
        .unwrap();
    assert!(state.status_message().contains("Removed bookmark"));
}

#[test]
fn test_remove_named_bookmark_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test".to_string()))
        .unwrap();

    state
        .execute_command(editor_core::Command::RemoveBookmark(0))
        .unwrap();
    assert!(state.status_message().contains("Removed bookmark 'test'"));
}

#[test]
fn test_jump_to_bookmark_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::JumpToBookmark(0))
        .unwrap();
    assert!(state.status_message().contains("Jumped to bookmark"));
}

#[test]
fn test_jump_to_named_bookmark_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();

    state
        .execute_command(editor_core::Command::AddNamedBookmark("test".to_string()))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::JumpToNamedBookmark(
            "test".to_string(),
        ))
        .unwrap();
    assert!(state.status_message().contains("Jumped to bookmark 'test'"));
}

#[test]
fn test_next_bookmark_status_message() {
    let mut state = EditorState::new();
    for i in 0..5 {
        state
            .execute_command(editor_core::Command::InsertChar((b'a' + i as u8) as char))
            .unwrap();
        state
            .execute_command(editor_core::Command::NewLine)
            .unwrap();
    }

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::NextBookmark)
        .unwrap();
    assert!(state.status_message().contains("Next bookmark"));
}

#[test]
fn test_previous_bookmark_status_message() {
    let mut state = EditorState::new();
    for i in 0..5 {
        state
            .execute_command(editor_core::Command::InsertChar((b'a' + i as u8) as char))
            .unwrap();
        state
            .execute_command(editor_core::Command::NewLine)
            .unwrap();
    }

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::MoveCursorDown)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();

    state
        .execute_command(editor_core::Command::PreviousBookmark)
        .unwrap();
    assert!(state.status_message().contains("Previous bookmark"));
}

#[test]
fn test_clear_all_bookmarks_status_message() {
    let mut state = EditorState::new();
    state
        .execute_command(editor_core::Command::InsertChar('a'))
        .unwrap();
    state
        .execute_command(editor_core::Command::NewLine)
        .unwrap();
    state
        .execute_command(editor_core::Command::InsertChar('b'))
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToStartOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::MoveToEndOfFile)
        .unwrap();
    state
        .execute_command(editor_core::Command::ToggleBookmark)
        .unwrap();

    state
        .execute_command(editor_core::Command::ClearAllBookmarks)
        .unwrap();
    assert!(state.status_message().contains("Cleared 2 bookmarks"));
}
