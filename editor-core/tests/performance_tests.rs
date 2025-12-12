use editor_core::{Buffer, EditorError, EditorState};
use std::fs;
use std::io::Write;
use tempfile::TempDir;

#[test]
fn test_memory_limit_default() {
    let buffer = Buffer::new();
    assert_eq!(buffer.memory_limit(), Some(500_000_000));
}

#[test]
fn test_set_memory_limit() {
    let mut buffer = Buffer::new();
    buffer.set_memory_limit(Some(1_000_000));
    assert_eq!(buffer.memory_limit(), Some(1_000_000));
}

#[test]
fn test_disable_memory_limit() {
    let mut buffer = Buffer::new();
    buffer.set_memory_limit(None);
    assert_eq!(buffer.memory_limit(), None);
}

#[test]
fn test_estimated_memory_usage() {
    let content = "Hello, World!\n".repeat(100);
    let buffer = Buffer::from_string(&content);
    let usage = buffer.estimated_memory_usage();
    assert!(usage > 0);
    assert!(usage >= content.len());
}

#[test]
fn test_check_memory_usage_under_limit() {
    let content = "Hello, World!\n";
    let buffer = Buffer::from_string(content);
    assert!(buffer.check_memory_usage().is_ok());
}

#[test]
fn test_check_memory_usage_exceeds_limit() {
    let content = "x".repeat(1_000_000);
    let mut buffer = Buffer::from_string(&content);
    buffer.set_memory_limit(Some(100));

    let result = buffer.check_memory_usage();
    assert!(result.is_err());
    match result {
        Err(EditorError::OutOfMemory(_)) => {}
        _ => panic!("Expected OutOfMemory error"),
    }
}

#[test]
fn test_file_size_tracking() {
    let content = "Hello, World!\n".repeat(100);
    let buffer = Buffer::from_string(&content);
    assert_eq!(buffer.file_size(), Some(content.len() as u64));
}

#[test]
fn test_file_too_large_error() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("large_file.txt");

    let mut file = fs::File::create(&file_path).unwrap();
    let large_content = "x".repeat(10_000_000);
    for _ in 0..51 {
        file.write_all(large_content.as_bytes()).unwrap();
    }
    file.flush().unwrap();
    drop(file);

    let result = Buffer::from_file(file_path.clone());
    assert!(result.is_err());
    match result {
        Err(EditorError::FileTooLarge { path, size, limit }) => {
            assert!(path.contains("large_file.txt"));
            assert!(size > limit);
            assert_eq!(limit, 500_000_000);
        }
        _ => panic!("Expected FileTooLarge error"),
    }
}

#[test]
fn test_large_file_lazy_loading() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("medium_file.txt");

    let mut file = fs::File::create(&file_path).unwrap();
    let line = "This is a test line with some content.\n";
    for _ in 0..300_000 {
        file.write_all(line.as_bytes()).unwrap();
    }
    file.flush().unwrap();
    drop(file);

    let buffer = Buffer::from_file(file_path).unwrap();
    assert!(buffer.line_count() > 0);
    assert!(buffer.content().len() > 10_000_000);
}

#[test]
fn test_virtual_viewport_basic() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    let viewport = state.get_virtual_viewport(20);
    assert_eq!(viewport.start_line, 0);
    assert_eq!(viewport.end_line, 20);
    assert_eq!(viewport.visible_lines.len(), 20);
    assert!(viewport.visible_lines[0].starts_with("Line 0"));
    assert!(viewport.visible_lines[19].starts_with("Line 19"));
}

#[test]
fn test_virtual_viewport_scrolled() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    state.adjust_viewport_to_cursor(20);

    for _ in 0..50 {
        state
            .execute_command(editor_core::Command::MoveCursorDown)
            .ok();
    }
    state.adjust_viewport_to_cursor(20);

    let viewport = state.get_virtual_viewport(20);
    assert!(viewport.start_line <= 50);
    assert!(viewport.end_line > 50);
}

#[test]
fn test_virtual_viewport_at_end() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    for _ in 0..100 {
        state
            .execute_command(editor_core::Command::MoveCursorDown)
            .ok();
    }
    state.adjust_viewport_to_cursor(20);

    let viewport = state.get_virtual_viewport(20);
    let total_lines = state.buffer().line_count();
    assert!(viewport.start_line <= total_lines.saturating_sub(1));
    assert_eq!(viewport.end_line, total_lines);
}

#[test]
fn test_virtual_viewport_small_file() {
    let mut state = EditorState::new();
    let content = "Line 1\nLine 2\nLine 3\n";
    state.buffer_mut().set_content(content.to_string()).unwrap();

    let viewport = state.get_virtual_viewport(20);
    let total_lines = state.buffer().line_count();
    assert_eq!(viewport.start_line, 0);
    assert_eq!(viewport.end_line, total_lines);
    assert_eq!(viewport.visible_lines.len(), total_lines);
}

#[test]
fn test_adjust_viewport_scrolls_down() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    for _ in 0..30 {
        state
            .execute_command(editor_core::Command::MoveCursorDown)
            .ok();
    }

    let initial_viewport_top = state.viewport_top();
    state.adjust_viewport_to_cursor(20);
    let new_viewport_top = state.viewport_top();

    assert!(new_viewport_top >= initial_viewport_top);
}

#[test]
fn test_adjust_viewport_scrolls_up() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    for _ in 0..50 {
        state
            .execute_command(editor_core::Command::MoveCursorDown)
            .ok();
    }
    state.adjust_viewport_to_cursor(20);

    let initial_viewport_top = state.viewport_top();

    for _ in 0..30 {
        state
            .execute_command(editor_core::Command::MoveCursorUp)
            .ok();
    }
    state.adjust_viewport_to_cursor(20);

    let new_viewport_top = state.viewport_top();
    assert!(new_viewport_top < initial_viewport_top);
}

#[test]
fn test_viewport_respects_scroll_offset() {
    let mut state = EditorState::new();
    let content = (0..100)
        .map(|i| format!("Line {}\n", i))
        .collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    state.set_scroll_offset(10);

    for _ in 0..20 {
        state
            .execute_command(editor_core::Command::MoveCursorDown)
            .ok();
    }
    state.adjust_viewport_to_cursor(30);

    let cursor_line = state.cursor().line;
    let viewport_top = state.viewport_top();

    assert!(cursor_line >= viewport_top + state.scroll_offset());
}

#[test]
fn test_memory_check_with_no_limit() {
    let content = "x".repeat(1_000_000);
    let mut buffer = Buffer::from_string(&content);
    buffer.set_memory_limit(None);

    assert!(buffer.check_memory_usage().is_ok());
}

#[test]
fn test_large_file_preserves_content() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.txt");

    let mut file = fs::File::create(&file_path).unwrap();
    let expected_content = "Test line\n".repeat(1_500_000);
    file.write_all(expected_content.as_bytes()).unwrap();
    file.flush().unwrap();
    drop(file);

    let buffer = Buffer::from_file(file_path).unwrap();
    let actual_content = buffer.content();
    let actual_line_count = buffer.line_count();

    assert_eq!(actual_content.len(), expected_content.len());
    assert!(
        actual_line_count >= 1_500_000,
        "Expected at least 1,500,000 lines, got {}",
        actual_line_count
    );
}

#[test]
fn test_virtual_viewport_boundary_conditions() {
    let mut state = EditorState::new();
    let content = (0..5).map(|i| format!("Line {}\n", i)).collect::<String>();
    state.buffer_mut().set_content(content).unwrap();

    let total_lines = state.buffer().line_count();
    let viewport = state.get_virtual_viewport(100);
    assert_eq!(viewport.start_line, 0);
    assert_eq!(viewport.end_line, total_lines);
    assert_eq!(viewport.visible_lines.len(), total_lines);
}
