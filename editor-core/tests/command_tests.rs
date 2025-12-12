use editor_core::{Command, CursorPosition};
use std::path::PathBuf;

#[test]
fn test_command_is_editing_command() {
    assert!(Command::InsertChar('a').is_editing_command());
    assert!(Command::DeleteChar.is_editing_command());
    assert!(Command::Backspace.is_editing_command());
    assert!(Command::NewLine.is_editing_command());
    assert!(Command::DeleteLine.is_editing_command());
    assert!(Command::Indent.is_editing_command());
    assert!(Command::Dedent.is_editing_command());
    assert!(Command::ToggleOverwriteMode.is_editing_command());
    assert!(Command::HardWrap(5).is_editing_command());
    assert!(Command::SetSoftWrap(5).is_editing_command());
    assert!(Command::TrimTrailingWhitespace.is_editing_command());
    assert!(Command::Paste.is_editing_command());

    assert!(!Command::MoveCursorUp.is_editing_command());
    assert!(!Command::Save.is_editing_command());
}

#[test]
fn test_command_is_navigation_command() {
    assert!(Command::MoveCursorUp.is_navigation_command());
    assert!(Command::MoveCursorDown.is_navigation_command());
    assert!(Command::MoveCursorLeft.is_navigation_command());
    assert!(Command::MoveCursorRight.is_navigation_command());
    assert!(Command::MoveToStartOfLine.is_navigation_command());
    assert!(Command::MoveToEndOfLine.is_navigation_command());
    assert!(Command::MoveToStartOfFile.is_navigation_command());
    assert!(Command::MoveToEndOfFile.is_navigation_command());
    assert!(Command::MoveCursorWordLeft.is_navigation_command());
    assert!(Command::MoveCursorWordRight.is_navigation_command());
    assert!(Command::PageUp.is_navigation_command());
    assert!(Command::PageDown.is_navigation_command());
    assert!(Command::GotoLine(5).is_navigation_command());
    assert!(Command::AddCursor(CursorPosition::zero()).is_navigation_command());
    assert!(Command::RemoveCursor(0).is_navigation_command());
    assert!(Command::ClearSecondaryCursors.is_navigation_command());

    assert!(!Command::InsertChar('a').is_navigation_command());
    assert!(!Command::Save.is_navigation_command());
}

#[test]
fn test_command_is_file_command() {
    assert!(Command::Open(PathBuf::from("test.txt")).is_file_command());
    assert!(Command::Save.is_file_command());
    assert!(Command::SaveAs(PathBuf::from("test.txt")).is_file_command());
    assert!(Command::Close.is_file_command());
    assert!(Command::New.is_file_command());

    assert!(!Command::InsertChar('a').is_file_command());
    assert!(!Command::MoveCursorUp.is_file_command());
}

#[test]
fn test_command_clone() {
    let cmd1 = Command::InsertChar('a');
    let cmd2 = cmd1.clone();

    assert!(cmd1.is_editing_command());
    assert!(cmd2.is_editing_command());
}

#[test]
fn test_command_search() {
    let cmd = Command::Search("test".to_string());
    assert!(!cmd.is_editing_command());
    assert!(!cmd.is_navigation_command());
    assert!(!cmd.is_file_command());
}

#[test]
fn test_command_replace() {
    let cmd = Command::ReplaceNext {
        find: "old".to_string(),
        replace: "new".to_string(),
    };
    assert!(cmd.is_editing_command());
    assert!(!cmd.is_navigation_command());
    assert!(!cmd.is_file_command());

    let cmd = Command::ReplaceAll {
        find: "old".to_string(),
        replace: "new".to_string(),
    };
    assert!(cmd.is_editing_command());
    assert!(!cmd.is_navigation_command());
    assert!(!cmd.is_file_command());

    let cmd = Command::ReplaceInSelection {
        find: "old".to_string(),
        replace: "new".to_string(),
    };
    assert!(cmd.is_editing_command());
    assert!(!cmd.is_navigation_command());
    assert!(!cmd.is_file_command());
}

#[test]
fn test_command_undo_redo() {
    assert!(Command::Undo.is_editing_command());
    assert!(Command::Redo.is_editing_command());
    assert!(!Command::Undo.is_navigation_command());
    assert!(!Command::Redo.is_navigation_command());
    assert!(!Command::Undo.is_file_command());
    assert!(!Command::Redo.is_file_command());
}

#[test]
fn test_command_clipboard() {
    assert!(!Command::Copy.is_editing_command());
    assert!(Command::Cut.is_editing_command());
    assert!(Command::Paste.is_editing_command());
}
