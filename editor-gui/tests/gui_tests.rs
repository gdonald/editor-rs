use editor_core::command::Command;
use editor_core::editor::EditorState;

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
