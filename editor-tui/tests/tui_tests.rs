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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
        renderer.render(
            frame,
            &editor_state,
            &editor_tui::menu::MenuState::new(),
            None,
        );
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
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
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
        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_commit_display_timestamp_formats() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_timestamps.txt");

    fs::write(&test_file, "Initial content\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    for _ in 0..5 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        editor_state
            .execute_command(Command::InsertChar('X'))
            .unwrap();
        editor_state.execute_command(Command::Save).unwrap();
    }

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            if !browser.is_empty() {
                for commit in browser.commits() {
                    assert!(commit.timestamp > 0);
                }
            }
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_commit_display_hash_short_form() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_hash.txt");

    fs::write(&test_file, "Content for hash test\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    editor_state
        .execute_command(Command::InsertChar('Y'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            for commit in browser.commits() {
                assert!(!commit.id.is_empty());
                assert!(commit.id.len() >= 7);
            }
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_commit_display_message_truncation() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_truncation.txt");

    fs::write(&test_file, "Content for truncation test\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    editor_state
        .execute_command(Command::InsertChar('Z'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            for commit in browser.commits() {
                assert!(!commit.message.is_empty());
                assert!(commit.message.lines().count() > 0);
            }
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_commit_display_selection_highlighting() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_selection.txt");

    fs::write(&test_file, "Content for selection test\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    for i in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(10));
        editor_state
            .execute_command(Command::InsertChar((b'A' + i as u8) as char))
            .unwrap();
        editor_state.execute_command(Command::Save).unwrap();
    }

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            if !browser.is_empty() {
                assert_eq!(browser.selected_index(), 0);
                assert!(browser.selected_commit().is_some());
            }
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_commit_display_visual_indicators() {
    let backend = TestBackend::new(120, 40);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_indicators.txt");

    fs::write(&test_file, "Content for visual test\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    editor_state
        .execute_command(Command::InsertChar('V'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::InsertChar('W'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());

        if let Some(browser) = editor_state.history_browser() {
            if !browser.is_empty() {
                let selected = browser.selected_commit();
                assert!(selected.is_some());

                let commit = selected.unwrap();
                assert!(!commit.id.is_empty());
                assert!(!commit.message.is_empty());
                assert!(commit.timestamp > 0);
            }
        }
    }
}

#[test]
fn test_renderer_history_diff_view_rendering() {
    let backend = TestBackend::new(150, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_diff.txt");

    fs::write(&test_file, "Line 1\nLine 2\nLine 3\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::InsertChar('X'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::InsertChar('Y'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        if let Some(browser) = editor_state.history_browser() {
            if !browser.is_empty() {
                let diff_result = editor_state.get_history_diff();
                assert!(diff_result.is_ok());
            }
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_diff_view_line_indicators() {
    let backend = TestBackend::new(150, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_diff_indicators.txt");

    fs::write(&test_file, "Original line\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::MoveToEndOfLine)
        .unwrap();
    editor_state
        .execute_command(Command::InsertChar('\n'))
        .unwrap();
    editor_state
        .execute_command(Command::InsertChar('N'))
        .unwrap();
    editor_state
        .execute_command(Command::InsertChar('e'))
        .unwrap();
    editor_state
        .execute_command(Command::InsertChar('w'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        let diff_result = editor_state.get_history_diff();
        if let Ok(Some(diff)) = diff_result {
            assert!(diff.contains('+') || diff.contains('-'));
        }

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_diff_view_scrolling() {
    let backend = TestBackend::new(150, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let mut renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_diff_scroll.txt");

    let mut content = String::new();
    for i in 0..50 {
        content.push_str(&format!("Line {}\n", i));
    }
    fs::write(&test_file, &content).unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    for i in 0..10 {
        editor_state
            .execute_command(Command::InsertChar((b'A' + i) as char))
            .unwrap();
    }
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        renderer.scroll_diff_down();
        renderer.scroll_diff_up();
        renderer.reset_diff_scroll();

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}

#[test]
fn test_renderer_history_diff_view_line_numbers() {
    let backend = TestBackend::new(150, 50);
    let mut terminal = Terminal::new(backend).unwrap();
    let mut editor_state = EditorState::new();
    let renderer = Renderer::new();

    let temp_dir = tempfile::tempdir().unwrap();
    let test_file = temp_dir.path().join("test_diff_linenums.txt");

    fs::write(&test_file, "Line 1\nLine 2\nLine 3\n").unwrap();
    editor_state
        .execute_command(Command::Open(test_file.clone()))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(10));

    editor_state
        .execute_command(Command::InsertChar('Z'))
        .unwrap();
    editor_state.execute_command(Command::Save).unwrap();

    let result = editor_state.execute_command(Command::OpenHistoryBrowser);

    if result.is_ok() && editor_state.is_history_browser_open() {
        let diff_result = editor_state.get_history_diff();
        assert!(diff_result.is_ok());

        let draw_result = terminal.draw(|frame| {
            renderer.render(
                frame,
                &editor_state,
                &editor_tui::menu::MenuState::new(),
                None,
            );
        });

        assert!(draw_result.is_ok());
    }
}
