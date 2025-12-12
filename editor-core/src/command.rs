use crate::cursor::CursorPosition;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseMode {
    Upper,
    Lower,
    Title,
}

#[derive(Debug, Clone)]
pub enum Command {
    InsertChar(char),
    DeleteChar,
    Backspace,
    NewLine,
    DeleteLine,
    DuplicateLine,
    MoveLinesUp,
    MoveLinesDown,
    JoinLines,
    SortLines { numerical: bool },
    ChangeCase { mode: CaseMode },
    TransposeCharacters,
    Indent,
    Dedent,

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
    AddCursor(CursorPosition),
    RemoveCursor(usize),
    ClearSecondaryCursors,

    MouseClick(CursorPosition),
    MouseDragStart(CursorPosition),
    MouseDrag(CursorPosition),
    MouseDragEnd(CursorPosition),
    MouseDoubleClick(CursorPosition),
    MouseTripleClick(CursorPosition),
    ToggleBlockSelection,

    ToggleOverwriteMode,
    HardWrap(usize),
    SetSoftWrap(usize),
    TrimTrailingWhitespace,

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
    ReplaceNext { find: String, replace: String },
    ReplaceAll { find: String, replace: String },
    ReplaceInSelection { find: String, replace: String },

    GotoLine(usize),
    JumpToMatchingBracket,
    InsertCharWithAutoClose(char),

    ToggleLineComment,
    ToggleBlockComment,
    FoldCode,
    UnfoldCode,

    ToggleReadOnly,

    ToggleBookmark,
    AddNamedBookmark(String),
    RemoveBookmark(usize),
    JumpToBookmark(usize),
    JumpToNamedBookmark(String),
    NextBookmark,
    PreviousBookmark,
    ClearAllBookmarks,

    NextMatch,
    PreviousMatch,

    Quit,
}

impl Command {
    pub fn is_editing_command(&self) -> bool {
        matches!(
            self,
            Command::InsertChar(_)
                | Command::InsertCharWithAutoClose(_)
                | Command::DeleteChar
                | Command::Backspace
                | Command::NewLine
                | Command::DeleteLine
                | Command::DuplicateLine
                | Command::MoveLinesUp
                | Command::MoveLinesDown
                | Command::JoinLines
                | Command::SortLines { .. }
                | Command::ChangeCase { .. }
                | Command::TransposeCharacters
                | Command::Indent
                | Command::Dedent
                | Command::ToggleOverwriteMode
                | Command::HardWrap(_)
                | Command::SetSoftWrap(_)
                | Command::TrimTrailingWhitespace
                | Command::Paste
                | Command::Cut
                | Command::ReplaceNext { .. }
                | Command::ReplaceAll { .. }
                | Command::ReplaceInSelection { .. }
                | Command::Undo
                | Command::Redo
                | Command::ToggleLineComment
                | Command::ToggleBlockComment
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
                | Command::AddCursor(_)
                | Command::RemoveCursor(_)
                | Command::ClearSecondaryCursors
                | Command::GotoLine(_)
                | Command::MouseClick(_)
                | Command::MouseDragStart(_)
                | Command::MouseDrag(_)
                | Command::MouseDragEnd(_)
                | Command::MouseDoubleClick(_)
                | Command::MouseTripleClick(_)
                | Command::ToggleBlockSelection
                | Command::JumpToMatchingBracket
                | Command::ToggleBookmark
                | Command::AddNamedBookmark(_)
                | Command::RemoveBookmark(_)
                | Command::JumpToBookmark(_)
                | Command::JumpToNamedBookmark(_)
                | Command::NextBookmark
                | Command::PreviousBookmark
                | Command::ClearAllBookmarks
                | Command::NextMatch
                | Command::PreviousMatch
        )
    }

    pub fn is_file_command(&self) -> bool {
        matches!(
            self,
            Command::Open(_) | Command::Save | Command::SaveAs(_) | Command::Close | Command::New
        )
    }
}
