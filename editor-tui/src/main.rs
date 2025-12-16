use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor_core::EditorState;
use editor_tui::input::InputHandler;
use editor_tui::renderer::Renderer;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let mut editor_state = EditorState::new();
    let mut input_handler = InputHandler::new();
    let renderer = Renderer::new();

    let result = run_event_loop(
        &mut terminal,
        &mut editor_state,
        &mut input_handler,
        &renderer,
    );

    cleanup_terminal(terminal)?;

    result
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn cleanup_terminal(
    mut terminal: Terminal<CrosstermBackend<io::Stdout>>,
) -> Result<(), Box<dyn std::error::Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

fn run_event_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    editor_state: &mut EditorState,
    input_handler: &mut InputHandler,
    renderer: &Renderer,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            renderer.render(frame, editor_state);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            let is_history_browser_open = editor_state.is_history_browser_open();
            if let Some(action) = input_handler.handle_event(event, is_history_browser_open) {
                match action {
                    editor_tui::input::InputAction::Quit => break,
                    editor_tui::input::InputAction::Command(cmd) => {
                        if let Err(e) = editor_state.execute_command(cmd) {
                            editor_state.set_status_message(format!("Error: {}", e));
                        }
                    }
                    editor_tui::input::InputAction::OpenFile => {
                        editor_state
                            .set_status_message("Open file dialog not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::Search => {
                        editor_state
                            .set_status_message("Search dialog not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::Replace => {
                        editor_state
                            .set_status_message("Replace dialog not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::GotoLine => {
                        editor_state
                            .set_status_message("Goto line dialog not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::SelectAll => {
                        editor_state
                            .set_status_message("Select all not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::Resize => {}
                }
            }
        }
    }

    Ok(())
}
