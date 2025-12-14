use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use editor_core::{CaseMode, Command, CursorPosition};

pub struct KeyBindings {
    pub quit_key: KeyCode,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit_key: KeyCode::Char('q'),
        }
    }
}

pub struct InputHandler {
    key_bindings: KeyBindings,
    mouse_enabled: bool,
    drag_start: Option<CursorPosition>,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_bindings: KeyBindings::default(),
            mouse_enabled: true,
            drag_start: None,
        }
    }

    pub fn with_key_bindings(mut self, key_bindings: KeyBindings) -> Self {
        self.key_bindings = key_bindings;
        self
    }

    pub fn with_mouse_enabled(mut self, enabled: bool) -> Self {
        self.mouse_enabled = enabled;
        self
    }

    pub fn handle_event(&mut self, event: Event) -> Option<InputAction> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) if self.mouse_enabled => self.handle_mouse_event(mouse_event),
            Event::Resize(_, _) => Some(InputAction::Resize),
            _ => None,
        }
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Option<InputAction> {
        let ctrl = key_event.modifiers.contains(KeyModifiers::CONTROL);
        let alt = key_event.modifiers.contains(KeyModifiers::ALT);
        let shift = key_event.modifiers.contains(KeyModifiers::SHIFT);

        match (key_event.code, ctrl, alt, shift) {
            (code, _, _, _) if code == self.key_bindings.quit_key && ctrl => {
                Some(InputAction::Quit)
            }

            (KeyCode::Char(c), false, false, false) => {
                Some(InputAction::Command(Command::InsertChar(c)))
            }
            (KeyCode::Char(c), false, false, true) => Some(InputAction::Command(
                Command::InsertChar(c.to_ascii_uppercase()),
            )),

            (KeyCode::Backspace, false, false, false) => {
                Some(InputAction::Command(Command::Backspace))
            }
            (KeyCode::Delete, false, false, false) => {
                Some(InputAction::Command(Command::DeleteChar))
            }
            (KeyCode::Enter, false, false, false) => Some(InputAction::Command(Command::NewLine)),
            (KeyCode::Tab, false, false, false) => Some(InputAction::Command(Command::Indent)),
            (KeyCode::Tab, false, false, true) | (KeyCode::BackTab, false, false, false) => {
                Some(InputAction::Command(Command::Dedent))
            }

            (KeyCode::Up, false, false, false) => Some(InputAction::Command(Command::MoveCursorUp)),
            (KeyCode::Down, false, false, false) => {
                Some(InputAction::Command(Command::MoveCursorDown))
            }
            (KeyCode::Left, false, false, false) => {
                Some(InputAction::Command(Command::MoveCursorLeft))
            }
            (KeyCode::Right, false, false, false) => {
                Some(InputAction::Command(Command::MoveCursorRight))
            }

            (KeyCode::Left, true, false, false) => {
                Some(InputAction::Command(Command::MoveCursorWordLeft))
            }
            (KeyCode::Right, true, false, false) => {
                Some(InputAction::Command(Command::MoveCursorWordRight))
            }

            (KeyCode::Home, false, false, false) => {
                Some(InputAction::Command(Command::MoveToStartOfLine))
            }
            (KeyCode::End, false, false, false) => {
                Some(InputAction::Command(Command::MoveToEndOfLine))
            }

            (KeyCode::Home, true, false, false) => {
                Some(InputAction::Command(Command::MoveToStartOfFile))
            }
            (KeyCode::End, true, false, false) => {
                Some(InputAction::Command(Command::MoveToEndOfFile))
            }

            (KeyCode::PageUp, false, false, false) => Some(InputAction::Command(Command::PageUp)),
            (KeyCode::PageDown, false, false, false) => {
                Some(InputAction::Command(Command::PageDown))
            }

            (KeyCode::Char('s'), true, false, false) => Some(InputAction::Command(Command::Save)),
            (KeyCode::Char('o'), true, false, false) => Some(InputAction::OpenFile),
            (KeyCode::Char('n'), true, false, false) => Some(InputAction::Command(Command::New)),
            (KeyCode::Char('w'), true, false, false) => Some(InputAction::Command(Command::Close)),

            (KeyCode::Char('z'), true, false, false) => Some(InputAction::Command(Command::Undo)),
            (KeyCode::Char('y'), true, false, false) | (KeyCode::Char('z'), true, false, true) => {
                Some(InputAction::Command(Command::Redo))
            }

            (KeyCode::Char('c'), true, false, false) => Some(InputAction::Command(Command::Copy)),
            (KeyCode::Char('x'), true, false, false) => Some(InputAction::Command(Command::Cut)),
            (KeyCode::Char('v'), true, false, false) => Some(InputAction::Command(Command::Paste)),

            (KeyCode::Char('a'), true, false, false) => Some(InputAction::SelectAll),

            (KeyCode::Char('f'), true, false, false) => Some(InputAction::Search),
            (KeyCode::Char('h'), true, false, false) => Some(InputAction::Replace),
            (KeyCode::Char('g'), true, false, false) => Some(InputAction::GotoLine),

            (KeyCode::F(3), false, false, false) => Some(InputAction::Command(Command::NextMatch)),
            (KeyCode::F(3), false, false, true) => {
                Some(InputAction::Command(Command::PreviousMatch))
            }

            (KeyCode::Char('d'), true, false, false) => {
                Some(InputAction::Command(Command::DuplicateLine))
            }
            (KeyCode::Char('k'), true, false, false) => {
                Some(InputAction::Command(Command::DeleteLine))
            }
            (KeyCode::Char('j'), true, false, false) => {
                Some(InputAction::Command(Command::JoinLines))
            }

            (KeyCode::Up, true, false, true) => Some(InputAction::Command(Command::MoveLinesUp)),
            (KeyCode::Down, true, false, true) => {
                Some(InputAction::Command(Command::MoveLinesDown))
            }

            (KeyCode::Char('/'), true, false, false) => {
                Some(InputAction::Command(Command::ToggleLineComment))
            }
            (KeyCode::Char('/'), true, false, true) => {
                Some(InputAction::Command(Command::ToggleBlockComment))
            }

            (KeyCode::Char('u'), true, false, false) => {
                Some(InputAction::Command(Command::ChangeCase {
                    mode: CaseMode::Upper,
                }))
            }
            (KeyCode::Char('u'), true, false, true) => {
                Some(InputAction::Command(Command::ChangeCase {
                    mode: CaseMode::Lower,
                }))
            }

            (KeyCode::Char('b'), true, false, false) => {
                Some(InputAction::Command(Command::JumpToMatchingBracket))
            }

            (KeyCode::Char('m'), true, false, false) => {
                Some(InputAction::Command(Command::ToggleBookmark))
            }
            (KeyCode::F(2), false, false, false) => {
                Some(InputAction::Command(Command::NextBookmark))
            }
            (KeyCode::F(2), false, false, true) => {
                Some(InputAction::Command(Command::PreviousBookmark))
            }

            (KeyCode::Insert, false, false, false) => {
                Some(InputAction::Command(Command::ToggleOverwriteMode))
            }

            (KeyCode::Char('r'), true, false, false) => {
                Some(InputAction::Command(Command::ToggleReadOnly))
            }

            (KeyCode::Esc, false, false, false) => {
                Some(InputAction::Command(Command::ClearSecondaryCursors))
            }

            _ => None,
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) -> Option<InputAction> {
        let position = CursorPosition {
            line: mouse_event.row as usize,
            column: mouse_event.column as usize,
        };

        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.drag_start = Some(position);
                Some(InputAction::Command(Command::MouseClick(position)))
            }
            MouseEventKind::Drag(MouseButton::Left) => {
                if self.drag_start.is_some() {
                    Some(InputAction::Command(Command::MouseDrag(position)))
                } else {
                    None
                }
            }
            MouseEventKind::Up(MouseButton::Left) => {
                if self.drag_start.is_some() {
                    self.drag_start = None;
                    Some(InputAction::Command(Command::MouseDragEnd(position)))
                } else {
                    None
                }
            }
            MouseEventKind::ScrollDown => Some(InputAction::Command(Command::MoveCursorDown)),
            MouseEventKind::ScrollUp => Some(InputAction::Command(Command::MoveCursorUp)),
            _ => None,
        }
    }
}

impl Default for InputHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub enum InputAction {
    Command(Command),
    Quit,
    OpenFile,
    Search,
    Replace,
    GotoLine,
    SelectAll,
    Resize,
}
