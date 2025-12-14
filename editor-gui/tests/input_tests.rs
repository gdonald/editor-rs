use editor_core::{CaseMode, Command};
use editor_gui::input::{InputAction, InputHandler, KeyBindings};
use eframe::egui;

#[test]
fn test_input_handler_initialization() {
    let _handler = InputHandler::new();
}

#[test]
fn test_input_handler_with_custom_key_bindings() {
    let bindings = KeyBindings {
        quit_modifier: true,
        quit_key: 'x',
    };
    let _handler = InputHandler::new().with_key_bindings(bindings);
}

#[test]
fn test_handle_text_input_single_char() {
    let mut handler = InputHandler::new();
    let action = handler.handle_text_input("a");
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::InsertChar('a')))
    ));
}

#[test]
fn test_handle_text_input_multiple_chars() {
    let mut handler = InputHandler::new();
    let action = handler.handle_text_input("abc");
    assert!(action.is_none());
}

#[test]
fn test_handle_key_event_backspace() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Backspace, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::Backspace))
    ));
}

#[test]
fn test_handle_key_event_delete() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Delete, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::DeleteChar))
    ));
}

#[test]
fn test_handle_key_event_enter() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Enter, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::NewLine))
    ));
}

#[test]
fn test_handle_key_event_tab() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Tab, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::Indent))
    ));
}

#[test]
fn test_handle_key_event_shift_tab() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Tab, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::Dedent))
    ));
}

#[test]
fn test_handle_key_event_arrow_up() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::ArrowUp, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorUp))
    ));
}

#[test]
fn test_handle_key_event_arrow_down() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::ArrowDown, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorDown))
    ));
}

#[test]
fn test_handle_key_event_arrow_left() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::ArrowLeft, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorLeft))
    ));
}

#[test]
fn test_handle_key_event_arrow_right() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::ArrowRight, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorRight))
    ));
}

#[test]
fn test_handle_key_event_ctrl_arrow_left() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::ArrowLeft, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorWordLeft))
    ));
}

#[test]
fn test_handle_key_event_ctrl_arrow_right() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::ArrowRight, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorWordRight))
    ));
}

#[test]
fn test_handle_key_event_home() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Home, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveToStartOfLine))
    ));
}

#[test]
fn test_handle_key_event_end() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::End, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveToEndOfLine))
    ));
}

#[test]
fn test_handle_key_event_ctrl_home() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Home, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveToStartOfFile))
    ));
}

#[test]
fn test_handle_key_event_ctrl_end() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::End, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveToEndOfFile))
    ));
}

#[test]
fn test_handle_key_event_page_up() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::PageUp, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::PageUp))
    ));
}

#[test]
fn test_handle_key_event_page_down() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::PageDown, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::PageDown))
    ));
}

#[test]
fn test_handle_key_event_ctrl_s() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::S, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Save))));
}

#[test]
fn test_handle_key_event_ctrl_o() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::O, &modifiers);
    assert!(matches!(action, Some(InputAction::OpenFile)));
}

#[test]
fn test_handle_key_event_ctrl_n() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::N, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::New))));
}

#[test]
fn test_handle_key_event_ctrl_w() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::W, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Close))));
}

#[test]
fn test_handle_key_event_ctrl_z() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Z, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Undo))));
}

#[test]
fn test_handle_key_event_ctrl_y() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Y, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Redo))));
}

#[test]
fn test_handle_key_event_ctrl_shift_z() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Z, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Redo))));
}

#[test]
fn test_handle_key_event_ctrl_c() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::C, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Copy))));
}

#[test]
fn test_handle_key_event_ctrl_x() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::X, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Cut))));
}

#[test]
fn test_handle_key_event_ctrl_v() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::V, &modifiers);
    assert!(matches!(action, Some(InputAction::Command(Command::Paste))));
}

#[test]
fn test_handle_key_event_ctrl_a() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::A, &modifiers);
    assert!(matches!(action, Some(InputAction::SelectAll)));
}

#[test]
fn test_handle_key_event_ctrl_f() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::F, &modifiers);
    assert!(matches!(action, Some(InputAction::Search)));
}

#[test]
fn test_handle_key_event_ctrl_h() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::H, &modifiers);
    assert!(matches!(action, Some(InputAction::Replace)));
}

#[test]
fn test_handle_key_event_ctrl_g() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::G, &modifiers);
    assert!(matches!(action, Some(InputAction::GotoLine)));
}

#[test]
fn test_handle_key_event_f3() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::F3, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::NextMatch))
    ));
}

#[test]
fn test_handle_key_event_shift_f3() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::F3, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::PreviousMatch))
    ));
}

#[test]
fn test_handle_key_event_ctrl_d() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::D, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::DuplicateLine))
    ));
}

#[test]
fn test_handle_key_event_ctrl_k() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::K, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::DeleteLine))
    ));
}

#[test]
fn test_handle_key_event_ctrl_j() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::J, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::JoinLines))
    ));
}

#[test]
fn test_handle_key_event_ctrl_shift_arrow_up() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::ArrowUp, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveLinesUp))
    ));
}

#[test]
fn test_handle_key_event_ctrl_shift_arrow_down() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::ArrowDown, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveLinesDown))
    ));
}

#[test]
fn test_handle_key_event_ctrl_slash() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Slash, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ToggleLineComment))
    ));
}

#[test]
fn test_handle_key_event_ctrl_shift_slash() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Slash, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ToggleBlockComment))
    ));
}

#[test]
fn test_handle_key_event_ctrl_u() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::U, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ChangeCase {
            mode: CaseMode::Upper
        }))
    ));
}

#[test]
fn test_handle_key_event_ctrl_shift_u() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::U, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ChangeCase {
            mode: CaseMode::Lower
        }))
    ));
}

#[test]
fn test_handle_key_event_ctrl_b() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::B, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::JumpToMatchingBracket))
    ));
}

#[test]
fn test_handle_key_event_ctrl_m() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::M, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ToggleBookmark))
    ));
}

#[test]
fn test_handle_key_event_f2() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::F2, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::NextBookmark))
    ));
}

#[test]
fn test_handle_key_event_shift_f2() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        shift: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::F2, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::PreviousBookmark))
    ));
}

#[test]
fn test_handle_key_event_insert() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Insert, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ToggleOverwriteMode))
    ));
}

#[test]
fn test_handle_key_event_ctrl_r() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::R, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ToggleReadOnly))
    ));
}

#[test]
fn test_handle_key_event_escape() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers::default();
    let action = handler.handle_key_event(egui::Key::Escape, &modifiers);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::ClearSecondaryCursors))
    ));
}

#[test]
fn test_handle_key_event_ctrl_q() {
    let mut handler = InputHandler::new();
    let modifiers = egui::Modifiers {
        ctrl: true,
        ..Default::default()
    };
    let action = handler.handle_key_event(egui::Key::Q, &modifiers);
    assert!(matches!(action, Some(InputAction::Quit)));
}

#[test]
fn test_handle_mouse_click() {
    let mut handler = InputHandler::new();
    let pos = egui::Pos2::new(80.0, 32.0);
    let action = handler.handle_mouse_click(pos, 16.0);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MouseClick(_)))
    ));
}

#[test]
fn test_handle_mouse_drag() {
    let mut handler = InputHandler::new();
    let pos = egui::Pos2::new(80.0, 32.0);
    let action = handler.handle_mouse_drag(pos, 16.0);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MouseDrag(_)))
    ));
}

#[test]
fn test_handle_scroll_down() {
    let mut handler = InputHandler::new();
    let action = handler.handle_scroll(-10.0);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorDown))
    ));
}

#[test]
fn test_handle_scroll_up() {
    let mut handler = InputHandler::new();
    let action = handler.handle_scroll(10.0);
    assert!(matches!(
        action,
        Some(InputAction::Command(Command::MoveCursorUp))
    ));
}

#[test]
fn test_handle_scroll_zero() {
    let mut handler = InputHandler::new();
    let action = handler.handle_scroll(0.0);
    assert!(action.is_none());
}
