use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use editor_core::Command;
use editor_tui::input::{InputAction, InputHandler};

#[test]
fn test_basic_character_input() {
    let mut handler = InputHandler::new();
    let event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let action = handler.handle_event(event, false, false);

    assert!(action.is_some());
    if let Some(InputAction::Command(_)) = action {
    } else {
        panic!("Expected Command action");
    }
}

#[test]
fn test_uppercase_character_input() {
    let mut handler = InputHandler::new();
    let event = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::SHIFT));
    let action = handler.handle_event(event, false, false);

    assert!(action.is_some());
    if let Some(InputAction::Command(_)) = action {
    } else {
        panic!("Expected Command action");
    }
}

#[test]
fn test_arrow_key_navigation() {
    let mut handler = InputHandler::new();

    let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(up, false, false),
        Some(InputAction::Command(_))
    ));

    let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(down, false, false),
        Some(InputAction::Command(_))
    ));

    let left = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(left, false, false),
        Some(InputAction::Command(_))
    ));

    let right = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(right, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_ctrl_arrow_word_navigation() {
    let mut handler = InputHandler::new();

    let ctrl_left = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_left, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_right = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_right, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_home_end_keys() {
    let mut handler = InputHandler::new();

    let home = Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(home, false, false),
        Some(InputAction::Command(_))
    ));

    let end = Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(end, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_home = Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_home, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_end = Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_end, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_page_up_down() {
    let mut handler = InputHandler::new();

    let page_up = Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(page_up, false, false),
        Some(InputAction::Command(_))
    ));

    let page_down = Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(page_down, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_editing_keys() {
    let mut handler = InputHandler::new();

    let backspace = Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(backspace, false, false),
        Some(InputAction::Command(_))
    ));

    let delete = Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(delete, false, false),
        Some(InputAction::Command(_))
    ));

    let enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(enter, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_tab_indent() {
    let mut handler = InputHandler::new();

    let tab = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(tab, false, false),
        Some(InputAction::Command(_))
    ));

    let shift_tab = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::SHIFT));
    assert!(matches!(
        handler.handle_event(shift_tab, false, false),
        Some(InputAction::Command(_))
    ));

    let backtab = Event::Key(KeyEvent::new(KeyCode::BackTab, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(backtab, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_file_operations() {
    let mut handler = InputHandler::new();

    let ctrl_s = Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_s, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_o = Event::Key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_o, false, false),
        Some(InputAction::OpenFile)
    ));

    let ctrl_n = Event::Key(KeyEvent::new(KeyCode::Char('n'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_n, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_w = Event::Key(KeyEvent::new(KeyCode::Char('w'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_w, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_undo_redo() {
    let mut handler = InputHandler::new();

    let ctrl_z = Event::Key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_z, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_y = Event::Key(KeyEvent::new(KeyCode::Char('y'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_y, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_shift_z = Event::Key(KeyEvent::new(
        KeyCode::Char('z'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(ctrl_shift_z, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_clipboard_operations() {
    let mut handler = InputHandler::new();

    let ctrl_c = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_c, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_x = Event::Key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_x, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_v = Event::Key(KeyEvent::new(KeyCode::Char('v'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_v, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_search_and_replace() {
    let mut handler = InputHandler::new();

    let ctrl_f = Event::Key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_f, false, false),
        Some(InputAction::Search)
    ));

    let ctrl_h = Event::Key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_h, false, false),
        Some(InputAction::Replace)
    ));

    let f3 = Event::Key(KeyEvent::new(KeyCode::F(3), KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(f3, false, false),
        Some(InputAction::Command(_))
    ));

    let shift_f3 = Event::Key(KeyEvent::new(KeyCode::F(3), KeyModifiers::SHIFT));
    assert!(matches!(
        handler.handle_event(shift_f3, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_line_operations() {
    let mut handler = InputHandler::new();

    let ctrl_d = Event::Key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_d, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_k = Event::Key(KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_k, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_j = Event::Key(KeyEvent::new(KeyCode::Char('j'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_j, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_shift_up = Event::Key(KeyEvent::new(
        KeyCode::Up,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(ctrl_shift_up, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_shift_down = Event::Key(KeyEvent::new(
        KeyCode::Down,
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(ctrl_shift_down, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_comment_toggle() {
    let mut handler = InputHandler::new();

    let ctrl_slash = Event::Key(KeyEvent::new(KeyCode::Char('/'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_slash, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_shift_slash = Event::Key(KeyEvent::new(
        KeyCode::Char('/'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(ctrl_shift_slash, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_case_change() {
    let mut handler = InputHandler::new();

    let ctrl_u = Event::Key(KeyEvent::new(KeyCode::Char('u'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_u, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_shift_u = Event::Key(KeyEvent::new(
        KeyCode::Char('u'),
        KeyModifiers::CONTROL | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(ctrl_shift_u, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_bookmark_operations() {
    let mut handler = InputHandler::new();

    let ctrl_m = Event::Key(KeyEvent::new(KeyCode::Char('m'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_m, false, false),
        Some(InputAction::Command(_))
    ));

    let f2 = Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(f2, false, false),
        Some(InputAction::Command(_))
    ));

    let shift_f2 = Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::SHIFT));
    assert!(matches!(
        handler.handle_event(shift_f2, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_special_commands() {
    let mut handler = InputHandler::new();

    let ctrl_b = Event::Key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_b, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_g = Event::Key(KeyEvent::new(KeyCode::Char('g'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_g, false, false),
        Some(InputAction::GotoLine)
    ));

    let ctrl_a = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_a, false, false),
        Some(InputAction::SelectAll)
    ));

    let insert = Event::Key(KeyEvent::new(KeyCode::Insert, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(insert, false, false),
        Some(InputAction::Command(_))
    ));

    let ctrl_r = Event::Key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_r, false, false),
        Some(InputAction::Command(_))
    ));

    let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    assert!(matches!(
        handler.handle_event(esc, false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_quit_action() {
    let mut handler = InputHandler::new();

    let ctrl_q = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_q, false, false),
        Some(InputAction::Quit)
    ));
}

#[test]
fn test_mouse_click() {
    let mut handler = InputHandler::new();

    let mouse_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };

    let action = handler.handle_event(Event::Mouse(mouse_event), false, false);
    assert!(matches!(action, Some(InputAction::Command(_))));
}

#[test]
fn test_mouse_drag() {
    let mut handler = InputHandler::new();

    let down_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };
    handler.handle_event(Event::Mouse(down_event), false, false);

    let drag_event = MouseEvent {
        kind: MouseEventKind::Drag(MouseButton::Left),
        column: 15,
        row: 7,
        modifiers: KeyModifiers::NONE,
    };

    let action = handler.handle_event(Event::Mouse(drag_event), false, false);
    assert!(matches!(action, Some(InputAction::Command(_))));
}

#[test]
fn test_mouse_drag_end() {
    let mut handler = InputHandler::new();

    let down_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };
    handler.handle_event(Event::Mouse(down_event), false, false);

    let up_event = MouseEvent {
        kind: MouseEventKind::Up(MouseButton::Left),
        column: 15,
        row: 7,
        modifiers: KeyModifiers::NONE,
    };

    let action = handler.handle_event(Event::Mouse(up_event), false, false);
    assert!(matches!(action, Some(InputAction::Command(_))));
}

#[test]
fn test_mouse_scroll() {
    let mut handler = InputHandler::new();

    let scroll_down = MouseEvent {
        kind: MouseEventKind::ScrollDown,
        column: 0,
        row: 0,
        modifiers: KeyModifiers::NONE,
    };
    assert!(matches!(
        handler.handle_event(Event::Mouse(scroll_down), false, false),
        Some(InputAction::Command(_))
    ));

    let scroll_up = MouseEvent {
        kind: MouseEventKind::ScrollUp,
        column: 0,
        row: 0,
        modifiers: KeyModifiers::NONE,
    };
    assert!(matches!(
        handler.handle_event(Event::Mouse(scroll_up), false, false),
        Some(InputAction::Command(_))
    ));
}

#[test]
fn test_mouse_disabled() {
    let mut handler = InputHandler::new().with_mouse_enabled(false);

    let mouse_event = MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 10,
        row: 5,
        modifiers: KeyModifiers::NONE,
    };

    let action = handler.handle_event(Event::Mouse(mouse_event), false, false);
    assert!(action.is_none());
}

#[test]
fn test_resize_event() {
    let mut handler = InputHandler::new();

    let resize_event = Event::Resize(80, 24);
    let action = handler.handle_event(resize_event, false, false);
    assert!(matches!(action, Some(InputAction::Resize)));
}

#[test]
fn test_unhandled_key() {
    let mut handler = InputHandler::new();

    let unhandled = Event::Key(KeyEvent::new(
        KeyCode::Char('z'),
        KeyModifiers::ALT | KeyModifiers::SHIFT,
    ));
    assert!(handler.handle_event(unhandled, false, false).is_none());
}

#[test]
fn test_history_browser_up_down_navigation() {
    let mut handler = InputHandler::new();

    let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let action = handler.handle_event(up, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::HistoryNavigatePrevious))
    ));

    let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let action = handler.handle_event(down, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::HistoryNavigateNext))
    ));
}

#[test]
fn test_history_browser_enter_key() {
    let mut handler = InputHandler::new();

    let enter = Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let action = handler.handle_event(enter, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::HistoryViewDiff))
    ));
}

#[test]
fn test_history_browser_tab_key() {
    let mut handler = InputHandler::new();

    let tab = Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
    let action = handler.handle_event(tab, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::HistoryToggleFileList))
    ));
}

#[test]
fn test_history_browser_escape_key() {
    let mut handler = InputHandler::new();

    let esc = Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    let action = handler.handle_event(esc, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::CloseHistoryBrowser))
    ));
}

#[test]
fn test_history_browser_q_key() {
    let mut handler = InputHandler::new();

    let q = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    let action = handler.handle_event(q, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::CloseHistoryBrowser))
    ));
}

#[test]
fn test_history_browser_f_key() {
    let mut handler = InputHandler::new();

    let f = Event::Key(KeyEvent::new(KeyCode::Char('f'), KeyModifiers::NONE));
    let action = handler.handle_event(f, true, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::HistoryToggleFileList))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_history_browser_ctrl_q_still_quits() {
    let mut handler = InputHandler::new();

    let ctrl_q = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::CONTROL));
    let action = handler.handle_event(ctrl_q, true, false);
    assert!(matches!(action, Some(InputAction::Quit)));
}

#[test]
fn test_history_browser_ignores_normal_keys() {
    let mut handler = InputHandler::new();

    let a = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
    let action = handler.handle_event(a, true, false);
    assert!(action.is_none());
}

#[test]
fn test_normal_mode_arrow_keys_not_history_navigation() {
    let mut handler = InputHandler::new();

    let up = Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    let action = handler.handle_event(up, false, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorUp))
    ));

    let down = Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let action = handler.handle_event(down, false, false);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorDown))
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_key_for_save() {
    let mut handler = InputHandler::new();

    let cmd_s = Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_s, false, false),
        Some(InputAction::Command(Command::Save))
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_key_for_copy() {
    let mut handler = InputHandler::new();

    let cmd_c = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_c, false, false),
        Some(InputAction::Command(Command::Copy))
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_key_for_undo() {
    let mut handler = InputHandler::new();

    let cmd_z = Event::Key(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_z, false, false),
        Some(InputAction::Command(Command::Undo))
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_shift_z_for_redo() {
    let mut handler = InputHandler::new();

    let cmd_shift_z = Event::Key(KeyEvent::new(
        KeyCode::Char('z'),
        KeyModifiers::SUPER | KeyModifiers::SHIFT,
    ));
    assert!(matches!(
        handler.handle_event(cmd_shift_z, false, false),
        Some(InputAction::Command(Command::Redo))
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_ctrl_key_does_not_trigger_save() {
    let mut handler = InputHandler::new();

    let ctrl_s = Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    let action = handler.handle_event(ctrl_s, false, false);
    assert!(action.is_none());
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_q_quits() {
    let mut handler = InputHandler::new();

    let cmd_q = Event::Key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_q, false, false),
        Some(InputAction::Quit)
    ));
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_cmd_arrow_for_word_movement() {
    let mut handler = InputHandler::new();

    let cmd_left = Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_left, false, false),
        Some(InputAction::Command(Command::MoveCursorWordLeft))
    ));

    let cmd_right = Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::SUPER));
    assert!(matches!(
        handler.handle_event(cmd_right, false, false),
        Some(InputAction::Command(Command::MoveCursorWordRight))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_non_macos_ctrl_key_for_save() {
    let mut handler = InputHandler::new();

    let ctrl_s = Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_s, false, false),
        Some(InputAction::Command(Command::Save))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_non_macos_ctrl_key_for_copy() {
    let mut handler = InputHandler::new();

    let ctrl_c = Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
    assert!(matches!(
        handler.handle_event(ctrl_c, false, false),
        Some(InputAction::Command(Command::Copy))
    ));
}

#[test]
#[cfg(not(target_os = "macos"))]
fn test_non_macos_super_key_does_not_trigger_save() {
    let mut handler = InputHandler::new();

    let super_s = Event::Key(KeyEvent::new(KeyCode::Char('s'), KeyModifiers::SUPER));
    let action = handler.handle_event(super_s, false, false);
    assert!(action.is_none());
}
