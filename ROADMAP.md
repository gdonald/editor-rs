# Editor-rs Development Roadmap

## Technology Stack

### Core Libraries
- **Text Buffer**: `ropey` - Fast rope data structure for large text buffers
- **Syntax Highlighting**: `tree-sitter` - Modern incremental parsing library
  - Alternative: `syntect` - Simpler but less powerful option
- **Clipboard**: `arboard` - Cross-platform clipboard access
- **Serialization**: `serde` with `toml` - For configuration files
- **File Watching**: `notify` - Watch for external file changes
- **Regex**: `regex` - For search and validation
- **Git Integration**: `git2` - libgit2 bindings for version history tracking
- **HTTP Client**: `reqwest` - For AI API calls (post v1.0)
- **Async Runtime**: `tokio` - For async AI operations (post v1.0)

### TUI Frontend Libraries
- **TUI Framework**: `ratatui` (v0.26+) - Modern, actively maintained TUI library
- **Terminal Backend**: `crossterm` - Cross-platform terminal manipulation
  - Works seamlessly with ratatui
  - Handles input, cursor positioning, colors, etc.
- **File Dialogs**: `rfd` - Native file dialogs (can work in terminal via fallback)

### GUI Frontend Libraries
- **GUI Framework**: `egui` (v0.27+) - Immediate mode GUI, easy to use
  - **Application Framework**: `eframe` - Window management for egui
  - **Rendering**: `wgpu` - Modern graphics API (via egui/eframe)
- **Font Rendering**: Built into egui
- **File Dialogs**: `rfd` - Native file dialogs
- Alternative GUI options to consider:
  - `iced` - More structured, Elm-inspired architecture
  - `slint` - Declarative UI with good performance

### Development Tools
- **Testing**: Built-in Rust `cargo test`
- **Coverage**: `cargo-tarpaulin`
- **Benchmarking**: `criterion`
- **Linting**: `clippy`
- **Formatting**: `rustfmt`

## Phase 1: Project Foundation

### 1.1 Project Structure Setup
- [x] Create library crate for core editor logic
- [x] Create binary crate for TUI frontend (`editor-tui`)
- [x] Create binary crate for GUI frontend (`editor-gui`)
- [x] Set up workspace in root `Cargo.toml`
- [x] Configure dependencies for each crate
- [x] Set up CI/CD pipeline configuration

### 1.2 Core Architecture Design
- [x] Define `EditorState` structure
- [x] Define `Command` enum for all editor operations
- [x] Define `EditorView` trait for frontend abstraction
- [x] Design buffer/document data structure
- [x] Design cursor position representation
- [x] Create error types for the editor

## Phase 2: Core Text Buffer Implementation

### 2.1 Basic Buffer Operations
- [x] Implement `Buffer` struct with rope or gap buffer
- [x] Implement insert character at position
- [x] Implement delete character at position
- [x] Implement insert string at position
- [x] Implement delete range
- [x] Implement line-based operations (get line, line count)
- [x] Write comprehensive tests for buffer operations

### 2.2 Cursor Management
- [x] Implement `Cursor` struct (line, column)
- [x] Implement cursor movement (up, down, left, right)
- [x] Implement cursor movement (start of line, end of line)
- [x] Implement cursor movement (start of file, end of file)
- [x] Implement cursor movement (word forward, word backward)
- [x] Implement cursor movement (page up, page down)
- [x] Handle cursor bounds checking
- [x] Implement virtual cursor column (maintain column when moving up/down)
- [x] Write tests for cursor movement

### 2.4 Text Editing Enhancements
- [x] Implement indentation/dedentation (Tab/Shift+Tab)
- [x] Implement auto-indentation on new line
- [x] Implement line wrapping (soft wrap and hard wrap)
- [x] Implement insert vs overwrite mode
- [x] Implement trim trailing whitespace
- [x] Write tests for text editing enhancements

### 2.3 Multi-Cursor Support (Optional for v1)
- [x] Design multi-cursor data structure
- [x] Implement adding/removing cursors
- [x] Implement cursor merging logic
- [x] Update buffer operations to work with multiple cursors
- [x] Write tests for multi-cursor operations

## Phase 3: File Operations

### 3.1 File I/O
- [x] Implement file reading into buffer
- [x] Implement buffer writing to file
- [x] Handle different line endings (LF, CRLF)
- [x] Implement encoding detection (UTF-8, etc.)
- [x] Handle large files efficiently
- [x] Add error handling for I/O operations
- [x] Write tests for file operations

### 3.2 File Metadata
- [x] Track file path
- [x] Track modified state (dirty flag)
- [x] Track file permissions
- [x] Implement unsaved changes detection
- [x] Implement read-only file handling
- [x] Implement binary file detection
- [x] Write tests for metadata tracking

### 3.3 File Safety and Recovery
- [x] Implement auto-save functionality
- [x] Implement backup file creation
- [x] Implement crash recovery (save recovery data)
- [x] Implement file watching (reload on external change)
- [x] Implement prompt to save on quit with unsaved changes
- [x] Handle corrupted file scenarios
- [x] Handle out of disk space errors
- [x] Write tests for file safety features

### 3.4 File History and Sessions
- [x] Implement recent files list (simple: last N opened files, configurable cap)
- [x] Implement session persistence (remember open files with cursor positions)
- [x] Implement session restoration on startup (auto-restore)
- [x] Support multiple concurrent editor instances (unique session files)
- [x] Write tests for file history and sessions
- [ ] Future enhancement: Git-based recent files (opt-in, use git history to show recently modified files from Phase 6.5)

## Phase 4: Editor Commands

### 4.1 Basic Editing Commands
- [x] Implement `InsertChar` command
- [x] Implement `DeleteChar` command
- [x] Implement `Backspace` command
- [x] Implement `NewLine` command
- [x] Implement `DeleteLine` command
- [x] Implement `DuplicateLine` command
- [x] Implement `MoveLinesUp/Down` command
- [x] Implement `JoinLines` command
- [x] Implement `SortLines` command (alphabetical, numerical)
- [x] Implement `ChangeCase` commands (upper, lower, title)
- [x] Implement `TransposeCharacters` command
- [x] Write tests for editing commands

### 4.1b Mouse Interaction Commands
- [x] Implement mouse click to position cursor
- [x] Implement mouse drag to select text
- [x] Implement double-click to select word
- [x] Implement triple-click to select line
- [x] Implement rectangular/block selection
- [x] Write tests for mouse interactions

### 4.2 Navigation Commands
- [x] Implement cursor movement commands
- [x] Implement page up/down commands
- [x] Implement goto line command
- [x] Implement jump to matching bracket/paren
- [ ] Implement jump to definition (basic, before LSP)
- [x] Implement scroll with offset (keep N lines visible)
- [x] Write tests for navigation commands

### 4.2b Bookmarks and Markers
- [x] Implement bookmark creation/deletion
- [x] Implement jump to bookmark
- [x] Implement bookmark persistence
- [x] Write tests for bookmarks

### 4.3 File Commands
- [x] Implement `Open` command
- [x] Implement `Save` command
- [x] Implement `SaveAs` command
- [x] Implement `New` command
- [x] Implement `Close` command
- [x] Write tests for file commands

### 4.4 Selection and Clipboard
- [x] Implement selection start/end
- [x] Implement visual selection mode
- [x] Implement copy command
- [x] Implement cut command
- [x] Implement paste command
- [x] Implement clipboard integration
- [ ] Implement clipboard history
- [x] Write tests for selection and clipboard

### 4.5 Code Intelligence Commands
- [x] Implement bracket/paren matching
- [x] Implement auto-closing brackets/quotes
- [x] Implement comment toggling (line comments)
- [x] Implement comment toggling (block comments)
- [x] Implement code folding (collapse/expand)
- [x] Write tests for code intelligence

### 4.6 Editor Modes
- [x] Implement read-only mode support
- [x] Design plugin/mode infrastructure (for Vim/Emacs modes)
- [ ] (Optional) Implement basic Vim mode switching
- [x] Write tests for editor modes

## Phase 5: Search and Replace

### 5.1 Basic Search
- [x] Implement forward search
- [x] Implement backward search
- [x] Implement case-sensitive search
- [x] Implement case-insensitive search
- [x] Implement search result highlighting
- [x] Implement jump to next/previous match
- [x] Implement incremental search (search as you type)
- [x] Implement search history
- [x] Write tests for search functionality

### 5.2 Advanced Search
- [x] Implement regex search
- [x] Implement whole word search
- [x] Implement search in selection
- [x] Write tests for advanced search

### 5.3 Replace
- [x] Implement single replace
- [x] Implement replace all
- [x] Implement replace in selection
- [ ] Add confirmation for replace operations
- [x] Implement replace history
- [x] Write tests for replace functionality

### 5.4 Performance and Memory
- [x] Implement lazy loading for very large files
- [x] Implement virtual scrolling (render only visible lines)
- [x] Implement memory limits and warnings
- [x] Handle out of memory scenarios gracefully
- [x] Write performance tests for large files

## Phase 6: Undo/Redo System

### 6.1 History Management
- [x] Design undo/redo stack structure
- [x] Implement command wrapping for undo
- [x] Implement undo operation
- [x] Implement redo operation
- [x] Implement history limits (memory management)
- [x] Group related commands for single undo
- [x] Write tests for undo/redo

## Phase 6.5: Git-Based Time Machine (KILLER FEATURE)

### 6.5.1 Core Git History Infrastructure
- [x] Design hidden git repository structure (`~/.editor-rs/history/{project-hash}/`)
- [x] Implement project identification (directory-based or single file)
- [x] Implement git repository initialization for new projects/files
- [x] Implement git repository opening for existing projects/files
- [x] Generate unique project hashes for storage paths
- [x] Write tests for git repository management

### 6.5.2 Auto-Commit on Save

#### 6.5.2.1 Single File Auto-Commit (Minimal)
- [x] Add `auto_commit_on_save()` method to GitHistoryManager
- [x] Implement git add for single file in hidden repo
- [x] Implement git commit with timestamp in commit message
- [x] Hook auto-commit into EditorState::save() after successful buffer save
- [x] Write tests for single file auto-commit
- [x] Test that auto-commit doesn't affect user's actual git repos

#### 6.5.2.2 Multi-File Auto-Commit Support
- [x] Implement "save all unsaved files" functionality in EditorState
- [x] Modify auto-commit to handle multiple files in single commit
- [x] Update commit message to list all files changed
- [x] Write tests for multi-file auto-commit

#### 6.5.2.3 Error Handling and Robustness
- [x] Handle git commit failures gracefully (log but don't block save)
- [x] Handle case where file is outside project directory
- [x] Handle case where hidden repo is corrupted
- [x] Add configuration option to enable/disable auto-commit (default: enabled)
- [x] Write tests for error scenarios

### 6.5.3 Auto-Commit Interval (Optional)
- [ ] Implement configurable auto-commit interval
- [ ] Default auto-commit interval to "off"
- [ ] Implement timer-based auto-commit when enabled
- [ ] Add configuration option for auto-commit interval
- [ ] Write tests for interval-based auto-commit

### 6.5.4 History Browsing UI

#### 6.5.4.1 Core History Data API
- [x] Add method to GitHistoryManager to list all commits
- [x] Add method to get commit details (author, timestamp, message)
- [x] Add method to get list of files changed in a commit
- [x] Add method to get diff between two commits
- [x] Add method to get diff for a specific file between commits
- [x] Write tests for history data API

#### 6.5.4.2 History Browser State
- [x] Design HistoryBrowser struct to hold browsing state
- [x] Implement commit list storage and pagination
- [x] Implement selected commit tracking
- [x] Implement selected file tracking (for file-level diffs)
- [x] Implement diff view state (current vs selected commit)
- [x] Write tests for history browser state

#### 6.5.4.3 History Browser Commands
- [x] Add OpenHistoryBrowser command to Command enum
- [x] Add CloseHistoryBrowser command
- [x] Add HistoryNavigateNext command (next commit in list)
- [x] Add HistoryNavigatePrevious command (previous commit)
- [x] Add HistorySelectCommit command
- [x] Add HistoryToggleFileList command
- [x] Add HistoryViewDiff command
- [x] Write tests for history browser commands

#### 6.5.4.4 EditorState History Integration
- [x] Add history_browser field to EditorState
- [x] Implement open_history_browser() method
- [x] Implement close_history_browser() method
- [x] Implement is_history_browser_open() method
- [x] Hook up history browser commands to EditorState
- [x] Write tests for EditorState history integration

#### 6.5.4.5 TUI History Rendering - Basic Layout
- [x] Design TUI layout for history browser (split view)
- [x] Implement commit list panel rendering
- [x] Implement commit details panel rendering
- [x] Implement file list panel rendering
- [x] Add scrolling support for commit list
- [x] Write tests for TUI history layout

#### 6.5.4.6 TUI History Rendering - Commit Display
- [x] Format commit timestamps in human-readable form
- [x] Render commit hash (short form)
- [x] Render commit message (truncated if needed)
- [x] Highlight selected commit
- [x] Add visual indicators for current position
- [x] Write tests for commit display formatting

#### 6.5.4.7 TUI History Rendering - Diff View
- [x] Implement diff panel rendering
- [x] Render line-by-line diff with +/- indicators
- [ ] Add syntax highlighting to diff view
- [x] Implement scrolling in diff view
- [x] Add line numbers to diff view
- [x] Write tests for diff view rendering

#### 6.5.4.8 TUI History Input Handling
- [x] Map up/down arrows to navigate commit list
- [x] Map Enter to view commit details/diff
- [x] Map Tab to switch between panels
- [x] Map Esc to close history browser
- [x] Map 'q' to close history browser
- [x] Map 'f' to toggle file list view
- [x] Write tests for history input handling

#### 6.5.4.9 GUI History Rendering - Basic Layout
- [x] Design GUI layout for history browser (split view with egui)
- [x] Implement commit list panel (scrollable list)
- [x] Implement commit details panel
- [x] Implement file list panel
- [x] Add resizable splitters between panels
- [x] Write tests for GUI history layout

#### 6.5.4.10 GUI History Rendering - Commit Display
- [x] Format commit timestamps for GUI display
- [x] Render commit list with proper fonts and colors
- [x] Implement commit selection highlighting
- [x] Add hover effects for commits
- [x] Implement double-click to view diff
- [x] Write tests for GUI commit display

#### 6.5.4.11 GUI History Rendering - Diff View
- [x] Implement diff view widget in egui
- [x] Render diff with proper formatting and colors
- [x] Add line numbers to diff view
- [x] Implement scrolling in diff view
- [x] Add syntax highlighting to diff
- [x] Write tests for GUI diff view

#### 6.5.4.12 GUI History Input Handling
- [x] Handle mouse clicks to select commits
- [x] Handle keyboard navigation (up/down arrows)
- [x] Handle Enter key to view diff
- [x] Handle Esc key to close history browser
- [x] Handle mouse scroll in commit list
- [x] Write tests for GUI history input

#### 6.5.4.13 History Browser Polish and Edge Cases
- [x] Handle empty history (no commits yet)
- [x] Handle single commit edge case
- [ ] Add loading indicators for slow operations
- [ ] Optimize performance for large commit histories
- [x] Add error handling for git operations
- [ ] Add keyboard shortcuts help overlay
- [x] Write tests for edge cases

### 6.5.5 Time Travel Operations
- [x] Implement restore from historical commit
- [x] Implement restore specific file from history
- [x] Implement preview before restore
- [x] Handle conflicts with current unsaved changes
- [ ] Implement cherry-pick from history (restore specific changes)
- [x] Write tests for time travel operations

### 6.5.6 History Management

#### 6.5.6.1 Git Repository Optimization
- [x] Add method to GitHistoryManager to run git gc (garbage collection)
- [x] Implement automatic gc trigger (e.g., after N commits)
- [x] Implement configurable gc thresholds
- [x] Add error handling for gc failures
- [x] Write tests for git gc functionality

#### 6.5.6.2 History Statistics API
- [x] Add method to get total commit count
- [x] Add method to get repository size on disk
- [x] Add method to get date range of history (oldest to newest)
- [x] Add method to get per-file statistics (commits, size)
- [x] Add method to get commits per day/week/month
- [x] Write tests for statistics API

#### 6.5.6.3 History Statistics Display
- [x] Design HistoryStats struct to hold statistics data
- [x] Add ShowHistoryStats command to Command enum
- [x] Implement statistics gathering in EditorState
- [x] Implement TUI rendering of statistics (commit count, size, date range)
- [x] Implement GUI rendering of statistics
- [x] Write tests for statistics display

#### 6.5.6.4 Retention Policy Configuration
- [x] Design RetentionPolicy enum (Forever, Days(u32), Commits(usize), Size(u64))
- [x] Add retention_policy field to configuration
- [x] Default retention policy: Forever
- [x] Add method to check if commit should be retained
- [x] Add configuration loading for retention policy
- [x] Write tests for retention policy configuration

#### 6.5.6.5 Manual History Cleanup
- [x] Add method to GitHistoryManager to delete old commits
- [x] Implement cleanup based on retention policy
- [x] Add CleanupHistory command to Command enum
- [ ] Implement cleanup confirmation dialog (TUI)
- [ ] Implement cleanup confirmation dialog (GUI)
- [ ] Show statistics before/after cleanup
- [x] Write tests for manual cleanup

#### 6.5.6.6 Automatic History Cleanup
- [x] Implement automatic cleanup trigger (on save, on startup, etc.)
- [x] Add configurable automatic cleanup option (default: off)
- [ ] Implement background cleanup to avoid blocking
- [ ] Add cleanup progress indicator
- [x] Write tests for automatic cleanup

#### 6.5.6.7 Large File Handling

##### 6.5.6.7.1 Configuration Infrastructure
- [x] Design LargeFileConfig struct with threshold, strategy, and options
- [x] Add large_file_threshold_mb field (default: 50 MB)
- [x] Add LargeFileStrategy enum (Warn, Skip, Error, Lfs)
- [x] Add exclude_large_files_from_history boolean option (default: false)
- [x] Add with_large_file_config() method to GitHistoryManager builder
- [x] Write tests for LargeFileConfig creation and defaults

##### 6.5.6.7.2 File Size Detection
- [x] Add get_file_size() helper method in commit.rs
- [x] Add check_file_size() method that returns size and whether it exceeds threshold
- [x] Add is_large_file() predicate method
- [x] Modify auto_commit_on_save_multiple() to check file sizes before copying
- [x] Write tests for file size detection with various file sizes

##### 6.5.6.7.3 Warning System
- [x] Add LargeFileWarning error variant to EditorError enum
- [x] Add log_large_file_warning() helper function
- [x] Modify commit process to emit warnings for large files (when strategy is Warn)
- [x] Include file path and size in warning messages
- [x] Write tests for warning generation

##### 6.5.6.7.4 Exclusion Logic (Skip Strategy)
- [x] Implement file filtering in auto_commit_on_save_multiple() to skip large files
- [x] Add commit message notation when files are skipped (e.g., "2 large files excluded")
- [x] Track excluded files in commit metadata or message
- [x] Return list of skipped files to caller
- [x] Write tests for file exclusion behavior

##### 6.5.6.7.5 Error Strategy
- [x] Implement Error strategy that blocks commit if large file detected
- [x] Return descriptive error with file path and size
- [ ] Allow users to override with explicit flag if needed
- [x] Write tests for error blocking behavior

##### 6.5.6.7.6 Git LFS Integration (Optional)
- [x] Research git2-rs LFS support or external git-lfs integration
- [ ] DEFERRED: No mature Rust LFS solution exists (git2-rs Issue #956, gix-lfs v0.0.0)
- [ ] DEFERRED: Current strategies (Warn/Skip/Error) cover use cases adequately
- [ ] DEFERRED: LFS designed for public repos, not ideal for hidden history storage
- [ ] DEFERRED: Revisit when gix-lfs matures or clear user need emerges
- [ ] Future Option 1: Shell out to git-lfs CLI (if git-lfs binary available)
- [ ] Future Option 2: Wait for gitoxide/gix-lfs to become stable

##### 6.5.6.7.7 Compression for Large Files
- [x] Research git compression settings and optimization
- [ ] NOT NEEDED: Git's default compression is excellent (83-99%+ compression ratios)
- [ ] NOT NEEDED: git2-rs does not expose compression level API
- [ ] NOT NEEDED: Custom compression would add complexity for marginal gains
- [ ] NOT NEEDED: Git automatically applies zlib compression to all objects
- [ ] Future Option: Could set core.compression via Config API if needed

##### 6.5.6.7.8 Statistics and Reporting
- [x] Add large_file_count to HistoryStats
- [x] Add total_large_file_size to HistoryStats
- [x] Add list_large_files() method to return all large files in history
- [x] Update get_per_file_stats() to flag large files
- [x] Write tests for large file statistics

##### 6.5.6.7.9 Integration with EditorState
- [x] Add large_file_config field to EditorState
- [x] Pass large file config to GitHistoryManager during initialization
- [x] Handle warnings/errors from git history in file_ops.rs save methods
- [x] Display warnings to user (via status message or dialog)
- [x] Write tests for EditorState integration

##### 6.5.6.7.10 Edge Cases and Error Handling
- [x] Handle case where file size changes between check and commit
- [x] Handle case where file is deleted before commit
- [x] Handle case where threshold is set to 0 (all files are large)
- [x] Handle case where threshold is set to u64::MAX (no files are large)
- [x] Handle mixed commits with both normal and large files
- [x] Write tests for all edge cases

##### 6.5.6.7.11 Integration Tests for LargeFileConfig
- [x] Write integration test for LargeFileConfig demonstrating full workflow

#### 6.5.6.8 History Export and Import
- [x] Add method to export history as regular git repository
- [x] Add method to import history from regular git repository
- [x] Implement export command (ExportHistory)
- [x] Implement import command (ImportHistory)
- [ ] Add file dialogs for export/import paths
- [x] Write tests for export/import functionality

#### 6.5.6.9 History Integrity and Recovery
- [x] Add method to verify repository integrity (git fsck)
- [ ] Implement automatic integrity checks (on startup, periodic)
- [x] Add method to repair corrupted repository
- [x] Implement backup before cleanup/repair operations
- [x] Add error recovery for common git issues
- [x] Write tests for integrity and recovery

### 6.5.7 Project vs File Tracking
- [x] Implement project-level tracking (directory opened as project)
- [x] Implement single-file tracking (file opened outside project)
- [x] Implement project detection logic
- [x] Handle moving files between projects
- [x] Handle renaming projects/files
- [x] Write tests for project vs file tracking

### 6.5.8 History Panel/Tab Integration
- [x] Create history panel/tab for TUI
- [x] Create history panel/tab for GUI
- [x] Implement side-by-side diff view
- [x] Implement timeline scrubbing/navigation
- [x] Implement search within history
- [x] Implement filter history by file
- [x] Write tests for history panel integration

### 6.5.9 Advanced History Features
- [x] Implement "compare any two points in time"
- [x] Implement history annotations (user can add notes to commits)
- [x] Implement history export (export history as regular git repo)
- [x] Implement history statistics/insights
- [x] Implement visual commit graph
- [x] Write tests for advanced history features

## Phase 7: Syntax Highlighting (Optional for v1)

### 7.1 Basic Highlighting
- [ ] Integrate `syntect` or `tree-sitter` library
- [ ] Load syntax definitions
- [ ] Implement line-based highlighting
- [ ] Cache highlighting results
- [ ] Support multiple languages
- [ ] Write tests for syntax highlighting

### 7.2 Theme Support
- [ ] Load color themes
- [ ] Implement theme switching
- [ ] Support dark/light themes
- [ ] Write tests for theme loading

## Phase 8: TUI Frontend Implementation

### 8.1 TUI Setup
- [x] Set up `ratatui` and `crossterm`
- [x] Initialize terminal
- [x] Implement main event loop
- [x] Handle terminal cleanup on exit
- [x] Write basic rendering test

### 8.2 TUI Input Handling
- [x] Capture keyboard input
- [x] Map keys to editor commands
- [x] Handle special keys (arrows, function keys)
- [x] Implement key bindings configuration
- [x] Handle mouse input (optional)
- [x] Write tests for input handling

### 8.3 TUI Rendering
- [x] Render text buffer to terminal
- [x] Implement cursor rendering
- [ ] Implement cursor style (block, line, underline)
- [x] Implement scrolling (viewport management)
- [x] Render line numbers
- [x] Render status bar (file name, line/col, mode)
- [ ] Render command/message bar
- [ ] Apply syntax highlighting colors
- [x] Handle terminal resize
- [ ] Implement column guide/ruler display
- [ ] Implement whitespace visualization
- [x] Implement current line highlighting
- [ ] Implement word count/character count display
- [x] Write tests for rendering logic

### 8.4 TUI Menu System
- [ ] Implement menu bar (File, Edit, View, Search, Tools, Help)
- [ ] Implement menu keyboard navigation (Alt+F for File, etc.)
- [ ] Implement menu mouse support (click to open/select)
- [ ] Implement File menu (New, Open, Save, Save As, Close, Quit)
- [ ] Implement Edit menu (Undo, Redo, Cut, Copy, Paste, Select All)
- [ ] Implement View menu (Toggle Line Numbers, Toggle Status Bar, Zoom In/Out)
- [ ] Implement Search menu (Find, Replace, Go to Line, Next/Previous Match)
- [ ] Implement Tools menu (Toggle Comments, Change Case, Sort Lines)
- [ ] Implement Help menu (Keyboard Shortcuts, About)
- [ ] Write tests for TUI menu system

### 8.5 TUI Features
- [ ] Implement command palette/prompt
- [ ] Implement file browser/picker
- [ ] Implement quick open (fuzzy file finder)
- [ ] Implement search UI
- [ ] Implement replace UI
- [ ] Implement help/shortcuts display
- [ ] Implement symbol outline/navigation
- [ ] Write tests for TUI features

## Phase 9: GUI Frontend Implementation

### 9.1 GUI Setup
- [x] Choose GUI library (egui/iced/other)
- [x] Set up GUI framework
- [x] Create main window
- [x] Implement event loop
- [x] Write basic GUI test

### 9.2 GUI Input Handling
- [x] Capture keyboard input
- [x] Map keys to editor commands
- [x] Handle text input events
- [x] Handle mouse clicks
- [x] Handle scroll events
- [x] Write tests for input handling

### 9.3 GUI Rendering
- [x] Render text buffer to window
- [x] Implement cursor rendering with blinking
- [ ] Implement cursor style configuration (block, line, underline)
- [ ] Implement cursor blink rate configuration
- [x] Implement text selection rendering
- [x] Implement scrolling (viewport + scrollbars)
- [x] Render line numbers gutter
- [ ] Apply syntax highlighting
- [x] Handle window resize
- [ ] Implement column guide/ruler display
- [ ] Implement whitespace visualization
- [x] Implement current line highlighting
- [ ] Implement minimap (code overview)
- [ ] Implement breadcrumbs (current function/scope)
- [ ] Implement word count/character count display
- [x] Write tests for rendering logic

### 9.4 GUI Menu System
- [ ] Implement menu bar (File, Edit, View, Search, Tools, Help)
- [ ] Implement menu keyboard navigation (Alt+F for File, etc.)
- [ ] Implement menu mouse support (click to open/select)
- [ ] Implement File menu (New, Open, Save, Save As, Close, Quit)
- [ ] Implement Edit menu (Undo, Redo, Cut, Copy, Paste, Select All)
- [ ] Implement View menu (Toggle Line Numbers, Toggle Status Bar, Zoom In/Out)
- [ ] Implement Search menu (Find, Replace, Go to Line, Next/Previous Match)
- [ ] Implement Tools menu (Toggle Comments, Change Case, Sort Lines)
- [ ] Implement Help menu (Keyboard Shortcuts, About)
- [ ] Write tests for GUI menu system

### 9.5 GUI Dialogs
- [ ] Implement file open dialog
- [ ] Implement save as dialog
- [ ] Implement search dialog
- [ ] Implement replace dialog
- [ ] Implement settings/preferences dialog
- [ ] Implement about dialog
- [ ] Write tests for dialogs

### 9.6 GUI Polish
- [ ] Implement font selection
- [ ] Implement font size adjustment (keyboard zoom in/out)
- [ ] Implement line spacing adjustment
- [ ] Add icons to menus
- [ ] Implement drag and drop file opening
- [ ] Implement macOS touch bar support (optional)
- [ ] Implement macOS native menu bar integration
- [ ] Write tests for GUI polish features

### 9.7 GUI Advanced Features
- [ ] Implement quick open dialog (fuzzy file finder)
- [ ] Implement command palette
- [ ] Implement symbol outline panel
- [ ] Implement recent files panel
- [ ] Write tests for GUI advanced features

## Phase 10: Configuration System

### 10.1 Configuration File
- [ ] Design configuration file format (TOML/JSON)
- [ ] Implement configuration loading
- [ ] Implement configuration saving
- [ ] Define default configuration
- [ ] Write tests for configuration loading

### 10.2 Configurable Settings
- [ ] Tab size configuration
- [ ] Spaces vs tabs configuration
- [ ] Line ending configuration
- [ ] Theme configuration
- [ ] Font configuration (GUI)
- [ ] Key bindings configuration
- [ ] Cursor style configuration
- [ ] Cursor blink rate configuration
- [ ] Scroll offset configuration
- [ ] Show/hide line numbers
- [ ] Show/hide status bar
- [ ] Auto-save interval configuration
- [ ] Max file size limits
- [ ] Default file encoding
- [ ] Line wrapping preferences
- [ ] Whitespace visualization preferences
- [ ] Column guide position
- [ ] Git time machine auto-commit interval (default: off)
- [ ] Git time machine retention policy (default: forever)
- [ ] Git time machine storage location
- [ ] Write tests for settings application

## Phase 11: Advanced Features (Post v1.0)

### 11.1 Multiple Buffers/Tabs
- [ ] Implement buffer list management
- [ ] Implement switching between buffers
- [ ] Implement tab bar rendering (TUI)
- [ ] Implement tab bar rendering (GUI)
- [ ] Write tests for buffer management

### 11.2 Split Views
- [ ] Implement horizontal split
- [ ] Implement vertical split
- [ ] Implement focus switching between splits
- [ ] Implement split resizing
- [ ] Write tests for split views

### 11.3 Project/Directory Support
- [ ] Implement directory tree view
- [ ] Implement fuzzy file finder
- [ ] Implement project-wide search
- [ ] Implement multi-file search results buffer
- [ ] Write tests for project features

### 11.4 LSP Integration
- [ ] Integrate Language Server Protocol client
- [ ] Implement auto-completion
- [ ] Implement go-to-definition
- [ ] Implement hover information
- [ ] Implement diagnostics display
- [ ] Write tests for LSP features

### 11.5 Git Integration
- [ ] Display git status in gutter
- [ ] Implement git blame
- [ ] Implement git diff view
- [ ] Write tests for git features

### 11.6 AI-Assisted Code Editing (Post v1.0)

#### 11.6.1 AI Infrastructure
- [ ] Design AI integration architecture (plugin/module based)
- [ ] Implement async AI request handling
- [ ] Implement API key management and secure storage
- [ ] Support multiple AI providers (Claude, OpenAI, local models)
- [ ] Implement API client for Claude API
- [ ] Implement API client for OpenAI API
- [ ] Implement local model support (Ollama integration)
- [ ] Write tests for AI infrastructure

#### 11.6.2 AI Configuration
- [ ] AI disabled by default (opt-in)
- [ ] Configure AI provider selection
- [ ] Configure API key via environment variable
- [ ] Configure model selection
- [ ] Configure auto-trigger vs manual trigger
- [ ] Configure privacy settings (local-only mode)
- [ ] Write tests for AI configuration

#### 11.6.3 Inline AI Completions
- [ ] Implement ghost text rendering for suggestions (TUI)
- [ ] Implement ghost text rendering for suggestions (GUI)
- [ ] Implement AI completion trigger (manual hotkey)
- [ ] Implement AI completion trigger (automatic)
- [ ] Implement accept completion (Tab)
- [ ] Implement dismiss completion (Esc)
- [ ] Implement partial acceptance (word-by-word)
- [ ] Cache completions for performance
- [ ] Write tests for inline completions

#### 11.6.4 AI Code Actions
- [ ] Implement "Explain Code" command
- [ ] Implement "Generate from Comment" command
- [ ] Implement "Refactor Code" command
- [ ] Implement "Fix Code" command (quick fix suggestions)
- [ ] Implement "Generate Tests" command
- [ ] Implement "Generate Documentation" command
- [ ] Display AI results in panel/dialog
- [ ] Write tests for code actions

#### 11.6.5 AI Chat Interface
- [ ] Design AI chat panel UI
- [ ] Implement chat panel (TUI)
- [ ] Implement chat panel (GUI)
- [ ] Implement multi-turn conversations
- [ ] Include code context in chat
- [ ] Apply AI-suggested edits from chat
- [ ] Show code diffs in chat
- [ ] Save chat history
- [ ] Write tests for chat interface

#### 11.6.6 Advanced AI Features
- [ ] Multi-file edits from AI
- [ ] Code review by AI (analyze changes)
- [ ] Integrate AI with git history (explain commits)
- [ ] AI-powered search (semantic search)
- [ ] Custom AI prompts/templates
- [ ] AI learning from user corrections
- [ ] Write tests for advanced AI features

#### 11.6.7 Privacy and Safety
- [ ] Implement telemetry opt-out (default: no telemetry)
- [ ] Never send code without explicit permission
- [ ] Local-only mode (no network calls)
- [ ] Implement code anonymization options
- [ ] Display what will be sent to AI before sending
- [ ] Implement AI usage statistics (local only)
- [ ] Write tests for privacy features

## Phase 12: Polish and Release

### 12.1 Testing
- [ ] Achieve 100% test coverage for core
- [ ] Write integration tests for TUI
- [ ] Write integration tests for GUI
- [ ] Implement fuzz testing for buffer operations
- [ ] Implement property-based testing for undo/redo
- [ ] Implement load testing with very large files
- [ ] Implement concurrent editing tests
- [ ] Perform manual testing on all platforms (macOS, Linux)
- [ ] Fix all identified bugs

### 12.2 Documentation
- [ ] Write user documentation
- [ ] Write developer documentation
- [ ] Create architecture diagrams
- [ ] Write contribution guidelines
- [ ] Create tutorial/getting started guide
- [ ] Write keyboard shortcuts reference
- [ ] Create config file examples
- [ ] Write troubleshooting guide
- [ ] Create migration guide (if applicable)

### 12.3 Performance Optimization
- [ ] Profile core operations
- [ ] Optimize hot paths
- [ ] Benchmark against large files
- [ ] Reduce memory usage
- [ ] Optimize rendering performance

### 12.4 Release Preparation
- [ ] Create installation instructions
- [ ] Set up release binaries for macOS
- [ ] Set up release binaries for Linux distributions
- [ ] Write changelog
- [ ] Create release notes
- [ ] Tag version 1.0.0

### 12.5 Platform-Specific Enhancements
- [ ] Test on macOS (Intel and Apple Silicon)
- [ ] Test on Linux (various distributions)
- [ ] Verify Wayland support on Linux
- [ ] Verify X11 support on Linux
- [ ] Fix any platform-specific issues

## Phase 13: Maintenance and Future

### 13.1 Community
- [ ] Set up issue tracker
- [ ] Respond to bug reports
- [ ] Review pull requests
- [ ] Build community guidelines

### 13.2 Future Enhancements
- [ ] Plugin system
- [ ] Macro recording/playback
- [ ] Terminal emulator integration
- [ ] Remote editing support
- [ ] Collaborative editing
- [ ] Advanced AI features (v2.0+)
  - [ ] Context-aware completions using project-wide knowledge
  - [ ] AI pair programming mode
  - [ ] Natural language editing commands
  - [ ] Intelligent code generation from requirements

---

## Fast Track to Self-Hosting

This section outlines a minimal viable path to achieve self-hosting as quickly as possible. The goal is to make the editor usable for editing its own Rust source code, while continuing to use VSCode/Claude Code for heavy development work.

### Self-Hosting Milestone (v0.2.5)

**Target**: Basic editing of Rust files in the editor itself

**Required Features** (subset of phases):
1. **From Phase 1**: Project structure and workspace setup
2. **From Phase 2**: Basic buffer and cursor operations
3. **From Phase 3**: File I/O (open, save, basic metadata)
4. **From Phase 4**: Basic editing commands (insert, delete, backspace, newline)
5. **From Phase 4**: Basic navigation (arrow keys, home/end)
6. **From Phase 4**: Save command
7. **From Phase 7**: Rust syntax highlighting (basic)
8. **From Phase 8**: TUI rendering (text, cursor, status bar, line numbers)
9. **From Phase 11.1**: Multiple buffers/tabs (moved up from post-v1.0)
10. **From Phase 5**: Basic search (find text)

**What You Can Do**:
- ✅ Open multiple .rs files from your project
- ✅ Navigate through code
- ✅ Make simple edits
- ✅ Save files
- ✅ See syntax-highlighted Rust code
- ✅ Switch between files
- ✅ Search for text

**What You'll Still Use VSCode/Claude Code For**:
- ❌ Complex refactoring
- ❌ AI-assisted development
- ❌ LSP features (autocomplete, go-to-definition)
- ❌ Git operations
- ❌ Advanced editing features

### Fast Track Implementation Order

#### Sprint 1: Minimal Viable Editor (1-2 weeks)
- [x] Phase 1.1: Project structure setup
- [x] Phase 1.2: Core architecture design
- [ ] Phase 2.1: Basic buffer operations
- [ ] Phase 2.2: Basic cursor management (no page up/down yet)
- [ ] Phase 3.1: Basic file I/O (read/write only)
- [ ] Phase 4.1: Basic editing commands (insert, delete, backspace, newline only)
- [ ] Phase 4.2: Basic navigation (arrows, home, end only)
- [ ] Phase 4.3: Save command only

#### Sprint 2: TUI Rendering (1 week)
- [ ] Phase 8.1: TUI setup
- [ ] Phase 8.2: Basic TUI input handling
- [ ] Phase 8.3: Basic TUI rendering (text, cursor, status bar, line numbers)
- [ ] Phase 8.3: Handle terminal resize

#### Sprint 3: Multi-File Support (1 week)
- [ ] Phase 11.1: Multiple buffers/tabs (MOVED UP)
  - [ ] Buffer list management
  - [ ] Switching between buffers
  - [ ] Tab bar rendering (TUI)
- [ ] Phase 4.3: Open and Close commands

#### Sprint 4: Polish for Self-Hosting (1 week)
- [ ] Phase 7.1: Rust syntax highlighting only
- [ ] Phase 5.1: Basic search (forward search, case-sensitive)
- [ ] Phase 3.2: Track modified state (dirty flag)
- [ ] Phase 3.3: Prompt to save on quit

**Total Time Estimate**: 4-6 weeks to self-hosting milestone

### Post Self-Hosting Development Strategy

After achieving self-hosting at v0.2.5:

1. **Continue using VSCode/Claude Code** as your primary development environment
2. **Use your editor** for:
   - Quick edits to your own codebase
   - Testing new features as you build them
   - Reading your own code
3. **Gradually implement** remaining features from the roadmap
4. **Transition more work** to your editor as features mature
5. **Full transition** when Git Time Machine (v0.5.0) and other killer features make it compelling

### Benefits of This Approach

- **Early feedback**: Start using your editor ASAP
- **Motivation**: Seeing it work keeps you energized
- **Practical testing**: Real-world usage reveals bugs and UX issues
- **Flexibility**: No pressure to abandon your current tools
- **Incremental**: Build what you need, when you need it

---

## Version Milestones

### v0.1.0 - Core Foundation
- Basic buffer operations
- Basic cursor movement
- File I/O
- Simple commands

### v0.2.0 - TUI Frontend
- Working TUI with basic editing
- Syntax highlighting
- Search functionality

### v0.3.0 - GUI Frontend
- Working GUI with basic editing
- All features from TUI

### v0.4.0 - Advanced Editing
- Undo/redo
- Selection and clipboard
- Replace functionality

### v0.5.0 - Git Time Machine (KILLER FEATURE)
- Auto-commit on save to hidden git repo
- History browsing UI (timeline view)
- Time travel operations (restore from history)
- Project and file-level tracking
- Infinite undo via git history

### v0.6.0 - Configuration
- Config file support
- Customizable key bindings
- Theme support
- Time machine settings

### v1.0.0 - Production Ready
- All tests passing with 100% coverage
- Complete documentation
- Stable API
- Multiple platform binaries
- Polished Git Time Machine feature
