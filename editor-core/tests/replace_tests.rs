use editor_core::{Command, EditorState};

#[test]
fn test_replace_next_single_occurrence() {
    let mut editor = EditorState::new();
    let content = "hello world";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceNext {
            find: "world".to_string(),
            replace: "universe".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "hello universe");
}

#[test]
fn test_replace_next_with_selection() {
    let mut editor = EditorState::new();
    let content = "foo bar foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();

    editor
        .execute_command(Command::ReplaceNext {
            find: "foo".to_string(),
            replace: "baz".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "baz bar foo");

    let cursor = editor.cursor();
    assert_eq!(cursor.line, 0);
    assert_eq!(cursor.column, 11);
}

#[test]
fn test_replace_next_moves_to_next_match() {
    let mut editor = EditorState::new();
    let content = "foo bar foo baz foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();

    editor
        .execute_command(Command::ReplaceNext {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "xxx bar foo baz foo");

    let sel = editor.selection().unwrap();
    assert_eq!(sel.start().column, 8);
    assert_eq!(sel.end().column, 11);
}

#[test]
fn test_replace_all_multiple_occurrences() {
    let mut editor = EditorState::new();
    let content = "foo bar foo baz foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "xxx bar xxx baz xxx");
}

#[test]
fn test_replace_all_no_matches() {
    let mut editor = EditorState::new();
    let content = "hello world";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "notfound".to_string(),
            replace: "replacement".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "hello world");
}

#[test]
fn test_replace_all_multiline() {
    let mut editor = EditorState::new();
    let content = "foo\nbar\nfoo\nbaz\nfoo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "xxx\nbar\nxxx\nbaz\nxxx");
}

#[test]
fn test_replace_in_selection() {
    let mut editor = EditorState::new();
    let content = "foo bar foo baz foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::Search("foo".to_string()))
        .unwrap();
    editor.execute_command(Command::NextMatch).unwrap();

    editor
        .execute_command(Command::ReplaceInSelection {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "foo bar xxx baz foo");
}

#[test]
fn test_replace_in_selection_no_selection() {
    let mut editor = EditorState::new();
    let content = "foo bar foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceInSelection {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "foo bar foo");
}

#[test]
fn test_replace_history() {
    let mut editor = EditorState::new();
    let content = "foo bar";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "bar".to_string(),
            replace: "yyy".to_string(),
        })
        .unwrap();

    let history = editor.replace_history();
    assert_eq!(history.len(), 2);
    assert_eq!(history[0], ("foo".to_string(), "xxx".to_string()));
    assert_eq!(history[1], ("bar".to_string(), "yyy".to_string()));
}

#[test]
fn test_replace_history_no_duplicates() {
    let mut editor = EditorState::new();
    let content = "foo bar foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    let history = editor.replace_history();
    assert_eq!(history.len(), 1);
}

#[test]
fn test_replace_with_empty_string() {
    let mut editor = EditorState::new();
    let content = "foo bar foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), " bar ");
}

#[test]
fn test_replace_empty_find_string() {
    let mut editor = EditorState::new();
    let content = "hello world";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "hello world");
}

#[test]
fn test_replace_case_sensitive() {
    let mut editor = EditorState::new();
    let content = "Foo foo FOO";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    let mut opts = editor.search_options();
    opts.case_sensitive = true;
    editor.set_search_options(opts);

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "bar".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "Foo bar FOO");
}

#[test]
fn test_replace_case_insensitive() {
    let mut editor = EditorState::new();
    let content = "Foo foo FOO";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    let mut opts = editor.search_options();
    opts.case_sensitive = false;
    editor.set_search_options(opts);

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "bar".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "bar bar bar");
}

#[test]
fn test_replace_regex() {
    let mut editor = EditorState::new();
    let content = "test123 test456";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    let mut opts = editor.search_options();
    opts.use_regex = true;
    editor.set_search_options(opts);

    editor
        .execute_command(Command::ReplaceAll {
            find: r"test\d+".to_string(),
            replace: "number".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "number number");
}

#[test]
fn test_replace_whole_word() {
    let mut editor = EditorState::new();
    let content = "foo foobar barfoo foo";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    let mut opts = editor.search_options();
    opts.whole_word = true;
    editor.set_search_options(opts);

    editor
        .execute_command(Command::ReplaceAll {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "xxx foobar barfoo xxx");
}

#[test]
fn test_replace_next_wrap_around() {
    let mut editor = EditorState::new();
    let content = "foo bar baz";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    use editor_core::CursorPosition;
    editor
        .execute_command(Command::MouseClick(CursorPosition::new(0, 8)))
        .unwrap();

    editor
        .execute_command(Command::ReplaceNext {
            find: "foo".to_string(),
            replace: "xxx".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "xxx bar baz");
}

#[test]
fn test_replace_longer_replacement() {
    let mut editor = EditorState::new();
    let content = "a b a";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "a".to_string(),
            replace: "longer".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "longer b longer");
}

#[test]
fn test_replace_shorter_replacement() {
    let mut editor = EditorState::new();
    let content = "longer word longer";
    editor
        .buffer_mut()
        .set_content(content.to_string())
        .unwrap();

    editor
        .execute_command(Command::ReplaceAll {
            find: "longer".to_string(),
            replace: "a".to_string(),
        })
        .unwrap();

    assert_eq!(editor.buffer().content(), "a word a");
}
