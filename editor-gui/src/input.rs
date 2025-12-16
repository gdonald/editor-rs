use editor_core::{CaseMode, Command, CursorPosition};

#[allow(dead_code)]
pub struct KeyBindings {
    pub quit_modifier: bool,
    pub quit_key: char,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            quit_modifier: true,
            quit_key: 'q',
        }
    }
}

pub struct InputHandler {
    key_bindings: KeyBindings,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            key_bindings: KeyBindings::default(),
        }
    }

    #[allow(dead_code)]
    pub fn with_key_bindings(mut self, key_bindings: KeyBindings) -> Self {
        self.key_bindings = key_bindings;
        self
    }

    pub fn handle_key_event(
        &mut self,
        key: egui::Key,
        modifiers: &egui::Modifiers,
    ) -> Option<InputAction> {
        let ctrl = modifiers.ctrl || modifiers.command;
        let alt = modifiers.alt;
        let shift = modifiers.shift;

        match key {
            egui::Key::Q if ctrl && self.key_bindings.quit_key == 'q' => Some(InputAction::Quit),

            egui::Key::Backspace if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::Backspace))
            }
            egui::Key::Delete if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::DeleteChar))
            }
            egui::Key::Enter if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::NewLine))
            }
            egui::Key::Tab if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::Indent))
            }
            egui::Key::Tab if !ctrl && !alt && shift => Some(InputAction::Command(Command::Dedent)),

            egui::Key::ArrowUp if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorUp))
            }
            egui::Key::ArrowDown if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorDown))
            }
            egui::Key::ArrowLeft if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorLeft))
            }
            egui::Key::ArrowRight if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorRight))
            }

            egui::Key::ArrowLeft if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorWordLeft))
            }
            egui::Key::ArrowRight if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveCursorWordRight))
            }

            egui::Key::Home if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveToStartOfLine))
            }
            egui::Key::End if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveToEndOfLine))
            }

            egui::Key::Home if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveToStartOfFile))
            }
            egui::Key::End if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::MoveToEndOfFile))
            }

            egui::Key::PageUp if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::PageUp))
            }
            egui::Key::PageDown if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::PageDown))
            }

            egui::Key::S if ctrl && !alt && !shift => Some(InputAction::Command(Command::Save)),
            egui::Key::O if ctrl && !alt && !shift => Some(InputAction::OpenFile),
            egui::Key::N if ctrl && !alt && !shift => Some(InputAction::Command(Command::New)),
            egui::Key::W if ctrl && !alt && !shift => Some(InputAction::Command(Command::Close)),

            egui::Key::Z if ctrl && !alt && !shift => Some(InputAction::Command(Command::Undo)),
            egui::Key::Y if ctrl && !alt && !shift => Some(InputAction::Command(Command::Redo)),
            egui::Key::Z if ctrl && !alt && shift => Some(InputAction::Command(Command::Redo)),

            egui::Key::C if ctrl && !alt && !shift => Some(InputAction::Command(Command::Copy)),
            egui::Key::X if ctrl && !alt && !shift => Some(InputAction::Command(Command::Cut)),
            egui::Key::V if ctrl && !alt && !shift => Some(InputAction::Command(Command::Paste)),

            egui::Key::A if ctrl && !alt && !shift => Some(InputAction::SelectAll),

            egui::Key::F if ctrl && !alt && !shift => Some(InputAction::Search),
            egui::Key::H if ctrl && !alt && !shift => Some(InputAction::Replace),
            egui::Key::G if ctrl && !alt && !shift => Some(InputAction::GotoLine),

            egui::Key::F3 if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::NextMatch))
            }
            egui::Key::F3 if !ctrl && !alt && shift => {
                Some(InputAction::Command(Command::PreviousMatch))
            }

            egui::Key::D if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::DuplicateLine))
            }
            egui::Key::K if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::DeleteLine))
            }
            egui::Key::J if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::JoinLines))
            }

            egui::Key::ArrowUp if ctrl && !alt && shift => {
                Some(InputAction::Command(Command::MoveLinesUp))
            }
            egui::Key::ArrowDown if ctrl && !alt && shift => {
                Some(InputAction::Command(Command::MoveLinesDown))
            }

            egui::Key::Slash if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ToggleLineComment))
            }
            egui::Key::Slash if ctrl && !alt && shift => {
                Some(InputAction::Command(Command::ToggleBlockComment))
            }

            egui::Key::U if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ChangeCase {
                    mode: CaseMode::Upper,
                }))
            }
            egui::Key::U if ctrl && !alt && shift => {
                Some(InputAction::Command(Command::ChangeCase {
                    mode: CaseMode::Lower,
                }))
            }

            egui::Key::B if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::JumpToMatchingBracket))
            }

            egui::Key::M if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ToggleBookmark))
            }
            egui::Key::F2 if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::NextBookmark))
            }
            egui::Key::F2 if !ctrl && !alt && shift => {
                Some(InputAction::Command(Command::PreviousBookmark))
            }

            egui::Key::Insert if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ToggleOverwriteMode))
            }

            egui::Key::R if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ToggleReadOnly))
            }

            egui::Key::Escape if !ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::ClearSecondaryCursors))
            }

            egui::Key::T if ctrl && !alt && !shift => {
                Some(InputAction::Command(Command::OpenHistoryBrowser))
            }

            _ => None,
        }
    }

    pub fn handle_history_browser_key_event(
        &mut self,
        key: egui::Key,
        modifiers: &egui::Modifiers,
    ) -> Option<InputAction> {
        let ctrl = modifiers.ctrl || modifiers.command;

        match key {
            egui::Key::Q if ctrl && self.key_bindings.quit_key == 'q' => Some(InputAction::Quit),
            egui::Key::ArrowUp => Some(InputAction::Command(Command::HistoryNavigatePrevious)),
            egui::Key::ArrowDown => Some(InputAction::Command(Command::HistoryNavigateNext)),
            egui::Key::Enter => Some(InputAction::Command(Command::HistoryViewDiff)),
            egui::Key::Tab => Some(InputAction::Command(Command::HistoryToggleFileList)),
            egui::Key::Escape => Some(InputAction::Command(Command::CloseHistoryBrowser)),
            egui::Key::Q if !ctrl => Some(InputAction::Command(Command::CloseHistoryBrowser)),
            egui::Key::F => Some(InputAction::Command(Command::HistoryToggleFileList)),
            _ => None,
        }
    }

    pub fn handle_text_input(&mut self, text: &str) -> Option<InputAction> {
        if text.len() == 1 {
            let c = text.chars().next().unwrap();
            Some(InputAction::Command(Command::InsertChar(c)))
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn handle_mouse_click(&mut self, pos: egui::Pos2, line_height: f32) -> Option<InputAction> {
        let line = (pos.y / line_height).floor() as usize;
        let column = (pos.x / 8.0).floor() as usize;
        Some(InputAction::Command(Command::MouseClick(CursorPosition {
            line,
            column,
        })))
    }

    #[allow(dead_code)]
    pub fn handle_mouse_drag(&mut self, pos: egui::Pos2, line_height: f32) -> Option<InputAction> {
        let line = (pos.y / line_height).floor() as usize;
        let column = (pos.x / 8.0).floor() as usize;
        Some(InputAction::Command(Command::MouseDrag(CursorPosition {
            line,
            column,
        })))
    }

    #[allow(dead_code)]
    pub fn handle_scroll(&mut self, delta: f32) -> Option<InputAction> {
        if delta > 0.0 {
            Some(InputAction::Command(Command::MoveCursorUp))
        } else if delta < 0.0 {
            Some(InputAction::Command(Command::MoveCursorDown))
        } else {
            None
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
}
