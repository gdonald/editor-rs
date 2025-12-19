# editor-rs

Rust-based code editor.

###

![https://gdonald.github.io/88x31/i/vibe_coded.gif](https://gdonald.github.io/88x31/i/vibe_coded.gif)

## Core features
- Insert and overwrite editing modes
- Indent and dedent lines with auto-indentation on new lines
- Soft and hard line wrapping helpers
- Trim trailing whitespace across the buffer
- Multi-cursor editing with add/remove commands and merge handling
- Line ending detection and preservation (LF, CRLF)
- UTF-8 encoding detection and validation
- Optimized handling of large files (>10MB) with buffered I/O

## File Safety and Recovery
- Auto-save functionality with configurable enable/disable
- Automatic backup file creation (.backup extension)
- Crash recovery with recovery data persistence
- File watching and external modification detection
- Reload from disk capability
- Unsaved changes tracking
- Corrupted file detection and error handling
- Disk full error handling with graceful failures

## Keyboard Commands

**Note:** On macOS, use `Cmd` instead of `Ctrl` for all shortcuts below. Other operating systems use `Ctrl`.

### Basic Text Editing
- `Char` - Insert character
- `Shift+Char` - Insert uppercase character
- `Backspace` - Delete character before cursor
- `Delete` - Delete character at cursor
- `Enter` - Insert new line
- `Tab` - Indent line
- `Shift+Tab` or `BackTab` - Dedent line

### Cursor Movement
- `Arrow Keys` - Move cursor up/down/left/right
- `Ctrl+Left/Right Arrow` - Move cursor word left/right
- `Home` - Move to start of line
- `End` - Move to end of line
- `Ctrl+Home` - Move to start of file
- `Ctrl+End` - Move to end of file
- `PageUp/PageDown` - Page up/down

### File Operations
- `Ctrl+S` - Save
- `Ctrl+O` - Open file
- `Ctrl+N` - New file
- `Ctrl+W` - Close file
- `Ctrl+Q` - Quit editor

### Undo/Redo
- `Ctrl+Z` - Undo
- `Ctrl+Y` or `Ctrl+Shift+Z` - Redo

### Clipboard
- `Ctrl+C` - Copy
- `Ctrl+X` - Cut
- `Ctrl+V` - Paste
- `Ctrl+A` - Select all

### Search & Replace
- `Ctrl+F` - Open search dialog
- `Ctrl+H` - Open replace dialog
- `Ctrl+G` - Go to line
- `F3` - Next match
- `Shift+F3` - Previous match

### Line Operations
- `Ctrl+D` - Duplicate line
- `Ctrl+K` - Delete line
- `Ctrl+J` - Join lines
- `Ctrl+Shift+Up/Down Arrow` - Move lines up/down

### Code Formatting
- `Ctrl+/` - Toggle line comment
- `Ctrl+Shift+/` - Toggle block comment
- `Ctrl+U` - Change to uppercase
- `Ctrl+Shift+U` - Change to lowercase

### Navigation & Bookmarks
- `Ctrl+B` - Jump to matching bracket
- `Ctrl+M` - Toggle bookmark
- `F2` - Next bookmark
- `Shift+F2` - Previous bookmark

### Modes & Settings
- `Insert` - Toggle overwrite mode
- `Ctrl+R` - Toggle read-only mode
- `Escape` - Clear secondary cursors (multi-cursor mode)

### History Browser (GUI only)
- `Ctrl+T` - Open history browser

#### When History Browser is Open:
- `Up/Down Arrow` - Navigate commits
- `Enter` - View diff for current commit
- `Tab` or `F` - Toggle file list
- `Escape` or `Q` - Close history browser

### Mouse Support
- Left-click to position cursor
- Left-click drag for selection
- Mouse scroll wheel for navigation
- (GUI only) Double-click for word selection
- (GUI only) Triple-click for line selection
