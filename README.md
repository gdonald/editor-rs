# editor-rs

Rust-based code editor.

## Core features
- Insert and overwrite editing modes
- Indent and dedent lines with auto-indentation on new lines
- Soft and hard line wrapping helpers
- Trim trailing whitespace across the buffer
- Multi-cursor editing with add/remove commands and merge handling
- Line ending detection and preservation (LF, CRLF)
- UTF-8 encoding detection and validation
- Optimized handling of large files (>10MB) with buffered I/O
