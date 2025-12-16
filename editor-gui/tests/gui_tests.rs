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
