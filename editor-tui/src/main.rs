use crossterm::{
    event::{self},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use editor_core::EditorState;
use editor_tui::dialog::Dialog;
use editor_tui::input::InputHandler;
use editor_tui::menu::MenuState;
use editor_tui::renderer::Renderer;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = setup_terminal()?;
    let mut editor_state = EditorState::new();
    let mut input_handler = InputHandler::new();
    let mut renderer = Renderer::new();
    let mut menu_state = MenuState::new();
    let mut dialog: Option<Dialog> = None;

    let result = run_event_loop(
        &mut terminal,
        &mut editor_state,
        &mut input_handler,
        &mut renderer,
        &mut menu_state,
        &mut dialog,
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
    renderer: &mut Renderer,
    menu_state: &mut MenuState,
    dialog: &mut Option<Dialog>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|frame| {
            renderer.render(frame, editor_state, menu_state, dialog.as_ref());
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            let event = event::read()?;
            let is_history_browser_open = editor_state.is_history_browser_open();
            let is_history_stats_open = editor_state.is_history_stats_open();
            let is_menu_active = menu_state.active;
            let is_dialog_open = dialog.is_some();
            if let Some(action) = input_handler.handle_event(
                event,
                is_history_browser_open,
                is_history_stats_open,
                is_menu_active,
                is_dialog_open,
            ) {
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
                        use editor_tui::dialog::{Dialog, DialogType};
                        *dialog = Some(Dialog::new(DialogType::Search));
                    }
                    editor_tui::input::InputAction::Replace => {
                        use editor_tui::dialog::{Dialog, DialogType};
                        *dialog = Some(Dialog::new(DialogType::Replace));
                    }
                    editor_tui::input::InputAction::GotoLine => {
                        use editor_tui::dialog::{Dialog, DialogType};
                        *dialog = Some(Dialog::new(DialogType::GotoLine));
                    }
                    editor_tui::input::InputAction::SelectAll => {
                        editor_state
                            .set_status_message("Select all not yet implemented".to_string());
                    }
                    editor_tui::input::InputAction::Resize => {}
                    editor_tui::input::InputAction::CloseHistoryStats => {
                        editor_state.close_history_stats();
                    }
                    editor_tui::input::InputAction::SetBaseCommit => {
                        if let Some(browser) = editor_state.history_browser() {
                            let index = browser.selected_index();
                            if let Err(e) = editor_state
                                .execute_command(editor_core::Command::HistorySetBaseCommit(index))
                            {
                                editor_state.set_status_message(format!("Error: {}", e));
                            }
                        }
                    }
                    editor_tui::input::InputAction::ActivateMenuBar => {
                        menu_state.activate();
                    }
                    editor_tui::input::InputAction::DeactivateMenuBar => {
                        menu_state.deactivate();
                    }
                    editor_tui::input::InputAction::MenuUp => {
                        if let Some(menu) = &mut menu_state.open_menu {
                            menu.move_selection_up();
                        }
                    }
                    editor_tui::input::InputAction::MenuDown => {
                        if let Some(menu) = &mut menu_state.open_menu {
                            menu.move_selection_down();
                        } else {
                            menu_state.open_current_menu();
                        }
                    }
                    editor_tui::input::InputAction::MenuLeft => {
                        menu_state.move_menu_left();
                    }
                    editor_tui::input::InputAction::MenuRight => {
                        menu_state.move_menu_right();
                    }
                    editor_tui::input::InputAction::MenuSelect => {
                        if let Some(menu_action) = menu_state.get_selected_action() {
                            menu_state.deactivate();
                            handle_menu_action(menu_action, editor_state, renderer, dialog);
                        }
                    }
                    editor_tui::input::InputAction::MenuAction(menu_action) => {
                        menu_state.deactivate();
                        handle_menu_action(menu_action, editor_state, renderer, dialog);
                    }
                    editor_tui::input::InputAction::DialogInsertChar(c) => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.insert_char(c);
                        }
                    }
                    editor_tui::input::InputAction::DialogBackspace => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.backspace();
                        }
                    }
                    editor_tui::input::InputAction::DialogDelete => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.delete_char();
                        }
                    }
                    editor_tui::input::InputAction::DialogMoveCursorLeft => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.move_cursor_left();
                        }
                    }
                    editor_tui::input::InputAction::DialogMoveCursorRight => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.move_cursor_right();
                        }
                    }
                    editor_tui::input::InputAction::DialogMoveToStart => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.move_to_start();
                        }
                    }
                    editor_tui::input::InputAction::DialogMoveToEnd => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.move_to_end();
                        }
                    }
                    editor_tui::input::InputAction::DialogConfirm => {
                        handle_dialog_confirm(dialog, editor_state);
                    }
                    editor_tui::input::InputAction::DialogCancel => {
                        *dialog = None;
                    }
                    editor_tui::input::InputAction::DialogSwitchField => {
                        if let Some(ref mut dlg) = dialog {
                            dlg.switch_field();
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

fn handle_dialog_confirm(dialog: &mut Option<Dialog>, editor_state: &mut EditorState) {
    use editor_core::Command;
    use editor_tui::dialog::DialogType;

    if let Some(dlg) = dialog.take() {
        match dlg.dialog_type {
            DialogType::Search => {
                if !dlg.input.is_empty() {
                    if let Err(e) = editor_state.execute_command(Command::Search(dlg.input.clone()))
                    {
                        editor_state.set_status_message(format!("Error: {}", e));
                    }
                }
            }
            DialogType::Replace => {
                if !dlg.input.is_empty() {
                    let replace_text = dlg.replace_input.unwrap_or_default();
                    if let Err(e) = editor_state.execute_command(Command::ReplaceAll {
                        find: dlg.input,
                        replace: replace_text,
                    }) {
                        editor_state.set_status_message(format!("Error: {}", e));
                    }
                }
            }
            DialogType::GotoLine => {
                if let Ok(line_number) = dlg.input.parse::<usize>() {
                    if let Err(e) = editor_state.execute_command(Command::GotoLine(line_number)) {
                        editor_state.set_status_message(format!("Error: {}", e));
                    }
                } else {
                    editor_state.set_status_message("Invalid line number".to_string());
                }
            }
            DialogType::Help => {}
        }
    }
}

fn handle_menu_action(
    action: editor_tui::menu::MenuAction,
    editor_state: &mut EditorState,
    renderer: &mut Renderer,
    dialog: &mut Option<Dialog>,
) {
    use editor_tui::dialog::DialogType;
    use editor_tui::menu::MenuAction;

    match action {
        MenuAction::ExecuteCommand(cmd) => {
            if let Err(e) = editor_state.execute_command(cmd) {
                editor_state.set_status_message(format!("Error: {}", e));
            }
        }
        MenuAction::OpenFile => {
            editor_state.set_status_message("Open file dialog not yet implemented".to_string());
        }
        MenuAction::SaveAs => {
            editor_state.set_status_message("Save As dialog not yet implemented".to_string());
        }
        MenuAction::Quit => {
            std::process::exit(0);
        }
        MenuAction::Search => {
            *dialog = Some(Dialog::new(DialogType::Search));
        }
        MenuAction::Replace => {
            *dialog = Some(Dialog::new(DialogType::Replace));
        }
        MenuAction::GotoLine => {
            *dialog = Some(Dialog::new(DialogType::GotoLine));
        }
        MenuAction::SelectAll => {
            editor_state.set_status_message("Select all not yet implemented".to_string());
        }
        MenuAction::ShowHelp => {
            *dialog = Some(Dialog::new(DialogType::Help));
        }
        MenuAction::ShowAbout => {
            editor_state.set_status_message("About: Editor-rs v0.1.0".to_string());
        }
        MenuAction::ToggleLineNumbers => {
            renderer.toggle_line_numbers();
            editor_state.set_status_message(format!(
                "Line numbers {}",
                if renderer.show_line_numbers {
                    "enabled"
                } else {
                    "disabled"
                }
            ));
        }
        MenuAction::ToggleStatusBar => {
            renderer.toggle_status_bar();
            editor_state.set_status_message(format!(
                "Status bar {}",
                if renderer.show_status_bar {
                    "enabled"
                } else {
                    "disabled"
                }
            ));
        }
    }
}
