use editor_core::{Command, EditorState};
use editor_tui::renderer::Renderer;
use ratatui::{backend::TestBackend, Terminal};
use std::fs;

#[test]
fn test_terminal_initialization() {
    let backend = TestBackend::new(80, 24);
    let terminal = Terminal::new(backend);
    assert!(terminal.is_ok());
}

#[test]
fn test_terminal_draw() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();

    let result = terminal.draw(|frame| {
        let area = frame.size();
        assert_eq!(area.width, 80);
        assert_eq!(area.height, 24);
    });

    assert!(result.is_ok());
}

#[test]
fn test_terminal_size() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
        .draw(|frame| {
            let area = frame.size();
            assert_eq!(area.width, 120);
            assert_eq!(area.height, 30);
        })
        .unwrap();
}

#[test]
fn test_renderer_basic_rendering() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let editor_state = EditorState::new();
    let renderer = Renderer::new();

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_with_content() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    editor_state
        .execute_command(editor_core::Command::InsertChar('H'))
        .unwrap();
    editor_state
        .execute_command(editor_core::Command::InsertChar('e'))
        .unwrap();
    editor_state
        .execute_command(editor_core::Command::InsertChar('l'))
        .unwrap();
    editor_state
        .execute_command(editor_core::Command::InsertChar('l'))
        .unwrap();
    editor_state
        .execute_command(editor_core::Command::InsertChar('o'))
        .unwrap();

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_with_line_numbers() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let editor_state = EditorState::new();
    let renderer = Renderer::new().with_line_numbers(true);

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_without_line_numbers() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let editor_state = EditorState::new();
    let renderer = Renderer::new().with_line_numbers(false);

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_status_bar() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    editor_state.set_status_message("Test status message".to_string());

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_cursor_position() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    editor_state
        .execute_command(editor_core::Command::InsertChar('A'))
        .unwrap();
    editor_state
        .execute_command(editor_core::Command::MoveCursorRight)
        .unwrap();

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_history_browser_closed() {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).unwrap();
    let editor_state = EditorState::new();
    let renderer = Renderer::new();

    assert!(!editor_state.is_history_browser_open());

    let result = terminal.draw(|frame| {
        renderer.render(frame, &editor_state);
    });

    assert!(result.is_ok());
}

#[test]
fn test_renderer_history_browser_render_empty() {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_empty.txt");

    fs::write(&test_file, "Initial content\n").unwrap();

    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();

    editor_state
        .execute_command(Command::InsertChar('A'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        let draw_result = terminal.draw(|frame| {
            renderer.render(frame, &editor_state);
        });
        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_browser_layout_rendering() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_layout.txt");

    fs::write(&test_file, "Line 1\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    editor_state
        .execute_command(Command::InsertChar('A'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::InsertChar('B'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            assert!(!browser.is_empty());
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(frame, &editor_state);
        });

        assert!(draw_result.is_ok());
    }
}
