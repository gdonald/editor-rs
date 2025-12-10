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
