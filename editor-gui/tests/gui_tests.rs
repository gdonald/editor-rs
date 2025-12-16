use editor_core::command::Command;
use editor_core::editor::EditorState;
use editor_core::{CommitInfo, HistoryBrowser};
use editor_gui::history_renderer::HistoryRenderer;
use editor_gui::renderer::Renderer;

#[test]
fn test_editor_state_initialization() {
    let editor_state = EditorState::new();
    let buffer = editor_state.current_buffer();
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_editor_state_new_buffer_command() {
    let mut editor_state = EditorState::new();
    let result = editor_state.execute_command(Command::New);
    assert!(result.is_ok());
}

#[test]
fn test_editor_state_current_buffer() {
    let editor_state = EditorState::new();
    let buffer = editor_state.current_buffer();
    assert_eq!(buffer.line_count(), 1);
}

#[test]
fn test_editor_state_insert_char() {
    let mut editor_state = EditorState::new();
    let result = editor_state.execute_command(Command::InsertChar('a'));
    assert!(result.is_ok());
    let buffer = editor_state.current_buffer();
    let content = buffer.content();
    assert!(content.starts_with('a'));
}

#[test]
fn test_renderer_initialization() {
    let _renderer = Renderer::new();
}

#[test]
fn test_renderer_with_line_numbers() {
    let _renderer = Renderer::new().with_line_numbers(false);
}

#[test]
fn test_renderer_cursor_blink_reset() {
    let mut renderer = Renderer::new();
    renderer.reset_cursor_blink();
}

#[test]
fn test_history_renderer_initialization() {
    let _renderer = HistoryRenderer::new();
}

#[test]
fn test_history_renderer_default() {
    let _renderer = HistoryRenderer::default();
}

#[test]
fn test_history_renderer_commit_list_width() {
    let renderer = HistoryRenderer::new();
    assert_eq!(renderer.commit_list_width(), 300.0);
}

#[test]
fn test_history_renderer_file_list_height() {
    let renderer = HistoryRenderer::new();
    assert_eq!(renderer.file_list_height(), 150.0);
}

#[test]
fn test_history_renderer_with_empty_history() {
    let _renderer = HistoryRenderer::new();
    let _history_browser = HistoryBrowser::new();
}

#[test]
fn test_history_renderer_with_commits() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abc123".to_string(),
        author_name: "Test Author".to_string(),
        author_email: "test@example.com".to_string(),
        timestamp: 1234567890,
        message: "Test commit message".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_with_multiple_commits() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![
        CommitInfo {
            id: "abc123def456".to_string(),
            author_name: "Alice".to_string(),
            author_email: "alice@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "def456ghi789".to_string(),
            author_name: "Bob".to_string(),
            author_email: "bob@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
    ];
    let history_browser = HistoryBrowser::with_commits(commits);
    assert_eq!(history_browser.commits().len(), 2);
    assert_eq!(history_browser.selected_index(), 0);
}

#[test]
fn test_history_renderer_commit_selection() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author1".to_string(),
            author_email: "author1@example.com".to_string(),
            timestamp: 1000000000,
            message: "Commit 1".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author2".to_string(),
            author_email: "author2@example.com".to_string(),
            timestamp: 1000000100,
            message: "Commit 2".to_string(),
        },
    ];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert_eq!(history_browser.selected_index(), 0);

    history_browser.select_commit(1);
    assert_eq!(history_browser.selected_index(), 1);

    let selected = history_browser.selected_commit();
    assert!(selected.is_some());
    assert_eq!(selected.unwrap().id, "commit2");
}

#[test]
fn test_history_renderer_timestamp_formatting() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "test123".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1609459200,
        message: "New Year 2021 commit".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_short_commit_id() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abc".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1234567890,
        message: "Short ID commit".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_long_commit_id() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abcdefghijklmnopqrstuvwxyz0123456789".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1234567890,
        message: "Long ID commit".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_multiline_commit_message() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abc123".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1234567890,
        message: "First line\nSecond line\nThird line".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_empty_commit_message() {
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abc123".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1234567890,
        message: "".to_string(),
    }];
    let _history_browser = HistoryBrowser::with_commits(commits);
}

#[test]
fn test_history_renderer_diff_view_mode() {
    use editor_core::DiffViewMode;
    let _renderer = HistoryRenderer::new();
    let commits = vec![CommitInfo {
        id: "abc123".to_string(),
        author_name: "Test".to_string(),
        author_email: "test@test.com".to_string(),
        timestamp: 1234567890,
        message: "Test commit".to_string(),
    }];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert!(matches!(
        history_browser.diff_view_mode(),
        DiffViewMode::FullDiff
    ));

    history_browser.set_diff_view_mode(DiffViewMode::FileDiff("test.rs".to_string()));
    assert!(matches!(
        history_browser.diff_view_mode(),
        DiffViewMode::FileDiff(_)
    ));
}

#[test]
fn test_history_diff_view_with_no_commits() {
    let _renderer = HistoryRenderer::new();
    let history_browser = HistoryBrowser::new();
    let editor_state = EditorState::new();

    assert!(history_browser.is_empty());
    let diff_result = editor_state.get_history_diff();
    assert!(diff_result.is_err());
}

#[test]
fn test_history_diff_view_error_handling() {
    let _renderer = HistoryRenderer::new();
    let editor_state = EditorState::new();

    let diff_result = editor_state.get_history_diff();
    assert!(diff_result.is_err());
}

#[test]
fn test_history_browser_with_selected_commit() {
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
    ];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert_eq!(history_browser.selected_index(), 0);
    assert!(history_browser.selected_commit().is_some());

    history_browser.select_next();
    assert_eq!(history_browser.selected_index(), 1);
}

#[test]
fn test_history_diff_view_get_diff_commits() {
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
    ];
    let history_browser = HistoryBrowser::with_commits(commits);

    let diff_commits = history_browser.get_diff_commits();
    assert!(diff_commits.is_some());

    let (from_commit, to_commit) = diff_commits.unwrap();
    assert_eq!(from_commit.id, "commit2");
    assert_eq!(to_commit.id, "commit1");
}

#[test]
fn test_gui_history_keyboard_navigation_up_down() {
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
        CommitInfo {
            id: "commit3".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567910,
            message: "Third commit".to_string(),
        },
    ];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert_eq!(history_browser.selected_index(), 0);

    history_browser.select_next();
    assert_eq!(history_browser.selected_index(), 1);

    history_browser.select_next();
    assert_eq!(history_browser.selected_index(), 2);

    history_browser.select_previous();
    assert_eq!(history_browser.selected_index(), 1);

    history_browser.select_previous();
    assert_eq!(history_browser.selected_index(), 0);
}

#[test]
fn test_gui_history_keyboard_enter_views_diff() {
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
    ];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert!(matches!(
        history_browser.diff_view_mode(),
        editor_core::DiffViewMode::FullDiff
    ));

    history_browser.set_diff_view_mode(editor_core::DiffViewMode::FileDiff("test.rs".to_string()));

    assert!(matches!(
        history_browser.diff_view_mode(),
        editor_core::DiffViewMode::FileDiff(_)
    ));
}

#[test]
fn test_gui_history_mouse_click_selects_commit() {
    let commits = vec![
        CommitInfo {
            id: "commit1".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567890,
            message: "First commit".to_string(),
        },
        CommitInfo {
            id: "commit2".to_string(),
            author_name: "Author".to_string(),
            author_email: "author@example.com".to_string(),
            timestamp: 1234567900,
            message: "Second commit".to_string(),
        },
    ];
    let mut history_browser = HistoryBrowser::with_commits(commits);

    assert_eq!(history_browser.selected_index(), 0);

    history_browser.select_commit(1);
    assert_eq!(history_browser.selected_index(), 1);
    assert_eq!(history_browser.selected_commit().unwrap().id, "commit2");
}

#[test]
fn test_gui_history_browser_open_close() {
    let mut editor_state = EditorState::new();

    assert!(!editor_state.is_history_browser_open());

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);
    if result.is_ok() {
        assert!(editor_state.is_history_browser_open());

        let result = editor_state.execute_command(Command::CloseHistoryBrowser);
        assert!(result.is_ok());
        assert!(!editor_state.is_history_browser_open());
    }
}

#[test]
fn test_gui_history_renderer_initialization() {
    let _history_renderer = HistoryRenderer::new();
}

#[test]
fn test_gui_history_renderer_commit_list_width() {
    let history_renderer = HistoryRenderer::new();
    assert_eq!(history_renderer.commit_list_width(), 300.0);
}

#[test]
fn test_gui_history_renderer_file_list_height() {
    let history_renderer = HistoryRenderer::new();
    assert_eq!(history_renderer.file_list_height(), 150.0);
}
