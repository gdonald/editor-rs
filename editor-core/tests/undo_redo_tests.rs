use editor_core::{Command, CursorPosition, Edit, EditorState, History, HistoryEntry};

#[test]
fn test_history_new() {
    let history = History::new();
    assert!(!history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_stack_len(), 0);
    assert_eq!(history.redo_stack_len(), 0);
}

#[test]
fn test_history_push_single_entry() {
    let mut history = History::new();
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 5)],
        None,
        None,
    );

    history.push(entry);
    assert!(history.can_undo());
    assert!(!history.can_redo());
    assert_eq!(history.undo_stack_len(), 1);
}

#[test]
fn test_history_undo() {
    let mut history = History::new();
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 5)],
        None,
        None,
    );

    history.push(entry.clone());
    let result = history.undo();
    assert!(result.is_ok());
    assert!(!history.can_undo());
    assert!(history.can_redo());
}

#[test]
fn test_history_redo() {
    let mut history = History::new();
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 5)],
        None,
        None,
    );

    history.push(entry.clone());
    history.undo().ok();
    let result = history.redo();
    assert!(result.is_ok());
    assert!(history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_history_undo_empty() {
    let mut history = History::new();
    let result = history.undo();
    assert!(result.is_err());
}

#[test]
fn test_history_redo_empty() {
    let mut history = History::new();
    let result = history.redo();
    assert!(result.is_err());
}

#[test]
fn test_history_clear() {
    let mut history = History::new();
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 5)],
        None,
        None,
    );

    history.push(entry.clone());
    history.clear();
    assert!(!history.can_undo());
    assert!(!history.can_redo());
}

#[test]
fn test_history_max_entries() {
    let mut history = History::new().with_max_entries(3);

    for i in 0..5 {
        let edit = Edit::Insert {
            position: CursorPosition::new(0, 0),
            text: format!("text{}", i),
        };
        let entry = HistoryEntry::new(
            vec![edit],
            vec![CursorPosition::new(0, 0)],
            vec![CursorPosition::new(0, i)],
            None,
            None,
        );
        history.push(entry);
    }

    assert_eq!(history.undo_stack_len(), 3);
}

#[test]
fn test_history_push_clears_redo_stack() {
    let mut history = History::new();
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 5)],
        None,
        None,
    );

    history.push(entry.clone());
    history.undo().ok();
    assert_eq!(history.redo_stack_len(), 1);

    history.push(entry);
    assert_eq!(history.redo_stack_len(), 0);
}

#[test]
fn test_edit_invert_insert() {
    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let inverted = edit.invert();
    match inverted {
        Edit::Delete { position, text } => {
            assert_eq!(position, CursorPosition::new(0, 0));
            assert_eq!(text, "hello");
        }
        _ => panic!("Expected Delete edit"),
    }
}

#[test]
fn test_edit_invert_delete() {
    let edit = Edit::Delete {
        position: CursorPosition::new(0, 0),
        text: "hello".to_string(),
    };
    let inverted = edit.invert();
    match inverted {
        Edit::Insert { position, text } => {
            assert_eq!(position, CursorPosition::new(0, 0));
            assert_eq!(text, "hello");
        }
        _ => panic!("Expected Insert edit"),
    }
}

#[test]
fn test_edit_invert_replace() {
    let edit = Edit::Replace {
        position: CursorPosition::new(0, 0),
        old_text: "hello".to_string(),
        new_text: "world".to_string(),
    };
    let inverted = edit.invert();
    match inverted {
        Edit::Replace {
            position,
            old_text,
            new_text,
        } => {
            assert_eq!(position, CursorPosition::new(0, 0));
            assert_eq!(old_text, "world");
            assert_eq!(new_text, "hello");
        }
        _ => panic!("Expected Replace edit"),
    }
}

#[test]
fn test_editor_state_undo_redo_insert() {
    let mut state = EditorState::new();
    state.execute_command(Command::InsertChar('h')).unwrap();
    assert!(state.current_buffer().content().starts_with("h"));

    state.execute_command(Command::Undo).unwrap();
    assert!(!state.current_buffer().content().contains('h'));

    state.execute_command(Command::Redo).unwrap();
    assert!(state.current_buffer().content().starts_with("h"));
}

#[test]
fn test_editor_state_can_undo_redo() {
    let mut state = EditorState::new();
    assert!(!state.can_undo());
    assert!(!state.can_redo());

    state.execute_command(Command::InsertChar('h')).unwrap();
    assert!(state.can_undo());
    assert!(!state.can_redo());

    state.execute_command(Command::Undo).unwrap();
    assert!(!state.can_undo());
    assert!(state.can_redo());
}

#[test]
fn test_editor_state_clear_history() {
    let mut state = EditorState::new();
    state.execute_command(Command::InsertChar('h')).unwrap();
    assert!(state.can_undo());

    state.clear_history();
    assert!(!state.can_undo());
    assert!(!state.can_redo());
}

#[test]
fn test_editor_state_history_memory_usage() {
    let mut state = EditorState::new();
    let initial_usage = state.history_memory_usage();

    state.execute_command(Command::InsertChar('h')).unwrap();
    let usage_after = state.history_memory_usage();

    assert!(usage_after > initial_usage);
}

#[test]
fn test_editor_state_undo_stack_len() {
    let mut state = EditorState::new();
    assert_eq!(state.undo_stack_len(), 0);

    state.execute_command(Command::InsertChar('h')).unwrap();
    let len_after_first = state.undo_stack_len();
    assert!(len_after_first >= 1);

    state.execute_command(Command::InsertChar('i')).unwrap();
    let len_after_second = state.undo_stack_len();
    assert!(len_after_second >= len_after_first);
}

#[test]
fn test_editor_state_redo_stack_len() {
    let mut state = EditorState::new();
    state.execute_command(Command::InsertChar('h')).unwrap();
    assert_eq!(state.redo_stack_len(), 0);

    state.execute_command(Command::Undo).unwrap();
    assert_eq!(state.redo_stack_len(), 1);
}

#[test]
fn test_editor_state_undo_redo_multiple_commands() {
    let mut state = EditorState::new();
    state.execute_command(Command::InsertChar('h')).unwrap();
    state.execute_command(Command::InsertChar('e')).unwrap();
    state.execute_command(Command::InsertChar('l')).unwrap();
    state.execute_command(Command::InsertChar('l')).unwrap();
    state.execute_command(Command::InsertChar('o')).unwrap();

    assert!(state.current_buffer().content().starts_with("hello"));

    state.execute_command(Command::Undo).unwrap();
    let content_after_one_undo = state.current_buffer().content();
    assert!(content_after_one_undo.len() < "hello\n".len());

    state.execute_command(Command::Redo).unwrap();
    assert!(state.current_buffer().content().starts_with("hello"));
}

#[test]
fn test_history_memory_usage() {
    let mut history = History::new();
    assert_eq!(history.memory_usage(), 0);

    let edit = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "hello world".to_string(),
    };
    let entry = HistoryEntry::new(
        vec![edit],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 11)],
        None,
        None,
    );

    history.push(entry);
    assert!(history.memory_usage() >= "hello world".len());
}

#[test]
fn test_history_grouping_timeout() {
    let mut history = History::new().with_group_timeout(1000);

    let edit1 = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "a".to_string(),
    };
    let entry1 = HistoryEntry::new(
        vec![edit1],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 1)],
        None,
        None,
    )
    .with_grouped(true);

    history.push(entry1);

    let edit2 = Edit::Insert {
        position: CursorPosition::new(0, 1),
        text: "b".to_string(),
    };
    let entry2 = HistoryEntry::new(
        vec![edit2],
        vec![CursorPosition::new(0, 1)],
        vec![CursorPosition::new(0, 2)],
        None,
        None,
    )
    .with_grouped(true);

    history.push(entry2);

    assert_eq!(history.undo_stack_len(), 1);
}

#[test]
fn test_history_no_grouping_for_non_grouped() {
    let mut history = History::new();

    let edit1 = Edit::Insert {
        position: CursorPosition::new(0, 0),
        text: "a".to_string(),
    };
    let entry1 = HistoryEntry::new(
        vec![edit1],
        vec![CursorPosition::new(0, 0)],
        vec![CursorPosition::new(0, 1)],
        None,
        None,
    )
    .with_grouped(false);

    history.push(entry1);

    let edit2 = Edit::Insert {
        position: CursorPosition::new(0, 1),
        text: "b".to_string(),
    };
    let entry2 = HistoryEntry::new(
        vec![edit2],
        vec![CursorPosition::new(0, 1)],
        vec![CursorPosition::new(0, 2)],
        None,
        None,
    )
    .with_grouped(false);

    history.push(entry2);

    assert_eq!(history.undo_stack_len(), 2);
}
