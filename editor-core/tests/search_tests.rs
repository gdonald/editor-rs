use editor_core::{Command, EditorState};

#[test]
fn test_search_finds_first_match() {
    let mut editor = EditorState::new();
    let content = "hello world\nhello universe\nhello galaxy";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    // Search for "hello"
    editor
        .execute_command(Command::Search("hello".to_string()))
        .unwrap();

    // Should be at first match (line 0, col 0) -- wait, "hello" starts at 0.
    // Cursor should be at end of match effectively? Or selecting it.
    // My implementation selects the match and puts cursor at end.
    // "hello" is length 5. selection: 0,0 -> 0,5. Primary cursor at 0,5.

    let cursor = editor.cursor();
    assert_eq!(cursor.line, 0);
    assert_eq!(cursor.column, 5);

    // Selection check
    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().line, 0);
    assert_eq!(sel.start().column, 0);
    assert_eq!(sel.end().line, 0);
    assert_eq!(sel.end().column, 5);
}

#[test]
fn test_search_next_match() {
    let mut editor = EditorState::new();
    let content = "foo bar foo baz foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();
    // 1st foo: 0..3. Cursor at 3.
    assert_eq!(editor.cursor().column, 3);

    editor.execute_command(Command::NextMatch).unwrap();
    // 2nd foo: 8..11. Cursor at 11.
    assert_eq!(editor.cursor().column, 11);

    editor.execute_command(Command::NextMatch).unwrap();
    // 3rd foo: 16..19. Cursor at 19.
    assert_eq!(editor.cursor().column, 19);

    editor.execute_command(Command::NextMatch).unwrap();
    // Wrap around to 1st foo: 0..3. Cursor at 3.
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_search_previous_match() {
    let mut editor = EditorState::new();
    let content = "foo bar foo baz foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    // Start at beginning
    editor.execute_command(Command::MoveToStartOfFile).unwrap();

    // Set query without searching first?
    // Usually 'search' sets the query.
    // Let's use search first, which goes to 1st match.
    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();
    assert_eq!(editor.cursor().column, 3);

    // Previous from 1st match should wrap to last (3rd)
    editor.execute_command(Command::PreviousMatch).unwrap();
    // 3rd foo: 16..19
    assert_eq!(editor.cursor().column, 19);

    editor.execute_command(Command::PreviousMatch).unwrap();
    // 2nd foo: 8..11
    assert_eq!(editor.cursor().column, 11);

    editor.execute_command(Command::PreviousMatch).unwrap();
    // 1st foo: 0..3
    assert_eq!(editor.cursor().column, 3);
}

#[test]
fn test_search_no_match() {
    let mut editor = EditorState::new();
    let content = "hello world";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    // Search for non-existent
    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();

    // Cursor shouldn't move (still at 0,0 initially?)
    // Actually set_content keeps cursor at 0,0? `set_content` keeps `cursors`?
    // Usually cursors are clamped. 0,0 is valid.
    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 0);
    assert!(editor.selection().is_none());
}

#[test]
fn test_search_case_insensitive() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("Hello World".to_string())
        .unwrap();

    // Default is case-sensitive. "hello" should not match "Hello".
    editor
        .execute_command(Command::Search("hello".to_string()))
        .unwrap();
    assert_eq!(editor.cursor().column, 0); // No move, default 0,0

    // Enable case-insensitive
    use editor_core::editor::SearchOptions;
    let options = SearchOptions {
        case_sensitive: false,
    };
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search("hello".to_string()))
        .unwrap();
    // Should match "Hello". Length 5.
    assert_eq!(editor.cursor().column, 5);
}

#[test]
fn test_search_find_all() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo bar foo baz".to_string())
        .unwrap();

    // Default case sensitive
    let matches = editor.buffer().find_all("foo", true);
    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0], 0);
    assert_eq!(matches[1], 8);
}

#[test]
fn test_search_history() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo bar".to_string())
        .unwrap();

    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();
    editor
        .execute_command(Command::Search("bar".to_string()))
        .unwrap();

    // We can't access history publicly unless we expose it.
    // I added `search_history` field but no accessor.
    // I should add `search_history()` accessor to `EditorState`.
    let history = editor.search_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], "foo");
    assert_eq!(history[1], "bar");
}
