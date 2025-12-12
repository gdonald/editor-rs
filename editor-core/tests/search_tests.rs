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
        use_regex: false,
        whole_word: false,
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

#[test]
fn test_regex_search_basic() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo123 bar456 baz789".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.use_regex = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search(r"\d+".to_string()))
        .unwrap();

    let cursor = editor.cursor();
    assert_eq!(cursor.column, 6);

    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 3);
    assert_eq!(sel.end().column, 6);
}

#[test]
fn test_regex_search_next() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo123 bar456 baz789".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.use_regex = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search(r"\d+".to_string()))
        .unwrap();
    assert_eq!(editor.cursor().column, 6);

    editor.execute_command(Command::NextMatch).unwrap();
    assert_eq!(editor.cursor().column, 13);

    editor.execute_command(Command::NextMatch).unwrap();
    assert_eq!(editor.cursor().column, 20);
}

#[test]
fn test_regex_search_word_boundary() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("word words wordy".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.use_regex = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search(r"\bword\b".to_string()))
        .unwrap();

    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 0);
    assert_eq!(sel.end().column, 4);

    editor.execute_command(Command::NextMatch).unwrap();
    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 0);
    assert_eq!(sel.end().column, 4);
}

#[test]
fn test_whole_word_search() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("word words wordy sword".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.whole_word = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search("word".to_string()))
        .unwrap();

    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 0);
    assert_eq!(sel.end().column, 4);

    editor.execute_command(Command::NextMatch).unwrap();
    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 0);
    assert_eq!(sel.end().column, 4);
}

#[test]
fn test_whole_word_no_match() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("words wordy sword".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.whole_word = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search("word".to_string()))
        .unwrap();

    assert_eq!(editor.cursor().line, 0);
    assert_eq!(editor.cursor().column, 0);
    assert!(editor.selection().is_none());
}

#[test]
fn test_whole_word_multiline() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo bar\nbar baz\nfoobar".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.whole_word = true;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search("bar".to_string()))
        .unwrap();

    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().line, 0);
    assert_eq!(sel.start().column, 4);

    editor.execute_command(Command::NextMatch).unwrap();
    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().line, 1);
    assert_eq!(sel.start().column, 0);

    editor.execute_command(Command::NextMatch).unwrap();
    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().line, 0);
    assert_eq!(sel.start().column, 4);
}

#[test]
fn test_search_in_range() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo bar foo baz foo".to_string())
        .unwrap();

    let start_idx = 4;
    let end_idx = 15;

    let matches = editor
        .buffer()
        .find_in_range("foo", start_idx, end_idx, true, false, false);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0], 8);
}

#[test]
fn test_search_in_range_regex() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("foo123 bar456 baz789".to_string())
        .unwrap();

    let start_idx = 7;
    let end_idx = 20;

    let matches = editor
        .buffer()
        .find_in_range(r"\d+", start_idx, end_idx, true, true, false);

    assert_eq!(matches.len(), 2);
    assert_eq!(matches[0], 10);
    assert_eq!(matches[1], 17);
}

#[test]
fn test_search_in_range_whole_word() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("word words wordy sword word".to_string())
        .unwrap();

    let start_idx = 5;
    let end_idx = 27;

    let matches = editor
        .buffer()
        .find_in_range("word", start_idx, end_idx, true, false, true);

    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0], 23);
}

#[test]
fn test_regex_case_insensitive() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("Hello WORLD hello".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.use_regex = true;
    options.case_sensitive = false;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search(r"hello".to_string()))
        .unwrap();

    assert_eq!(editor.cursor().column, 5);

    editor.execute_command(Command::NextMatch).unwrap();
    assert_eq!(editor.cursor().column, 17);
}

#[test]
fn test_whole_word_case_insensitive() {
    let mut editor = EditorState::new();
    editor
        .buffer_mut()
        .set_content("Word WORD words".to_string())
        .unwrap();

    let mut options = editor.search_options();
    options.whole_word = true;
    options.case_sensitive = false;
    editor.set_search_options(options);

    editor
        .execute_command(Command::Search("word".to_string()))
        .unwrap();

    assert_eq!(editor.cursor().column, 4);

    editor.execute_command(Command::NextMatch).unwrap();
    assert_eq!(editor.cursor().column, 9);

    editor.execute_command(Command::NextMatch).unwrap();
    assert_eq!(editor.cursor().column, 4);
}
