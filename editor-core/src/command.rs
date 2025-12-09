use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Command {
    InsertChar(char),
    DeleteChar,
    Backspace,
    NewLine,
    DeleteLine,

    MoveCursorUp,
    MoveCursorDown,
    MoveCursorLeft,
    MoveCursorRight,
    MoveToStartOfLine,
    MoveToEndOfLine,
    MoveToStartOfFile,
    MoveToEndOfFile,
    MoveCursorWordLeft,
    MoveCursorWordRight,
    PageUp,
    PageDown,

    Open(PathBuf),
    Save,
    SaveAs(PathBuf),
    Close,
    New,

    Undo,
    Redo,

    Copy,
    Cut,
    Paste,

    SelectionStart,
    SelectionEnd,

    Search(String),
    Replace { find: String, replace: String },

    GotoLine(usize),

    Quit,
}

impl Command {
    pub fn is_editing_command(&self) -> bool {
        matches!(
            self,
            Command::InsertChar(_)
                | Command::DeleteChar
                | Command::Backspace
                | Command::NewLine
                | Command::DeleteLine
                | Command::Paste
        )
    }

    pub fn is_navigation_command(&self) -> bool {
        matches!(
            self,
            Command::MoveCursorUp
                | Command::MoveCursorDown
                | Command::MoveCursorLeft
                | Command::MoveCursorRight
                | Command::MoveToStartOfLine
                | Command::MoveToEndOfLine
                | Command::MoveToStartOfFile
                | Command::MoveToEndOfFile
                | Command::MoveCursorWordLeft
                | Command::MoveCursorWordRight
                | Command::PageUp
                | Command::PageDown
                | Command::GotoLine(_)
        )
    }

    pub fn is_file_command(&self) -> bool {
        matches!(
            self,
            Command::Open(_) | Command::Save | Command::SaveAs(_) | Command::Close | Command::New
        )
    }
}
