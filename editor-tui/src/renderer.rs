use crate::menu::{MenuState, MenuType};
use editor_core::EditorState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub struct Renderer {
    pub show_line_numbers: bool,
    pub show_status_bar: bool,
    diff_scroll_offset: usize,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            show_status_bar: true,
            diff_scroll_offset: 0,
        }
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn toggle_line_numbers(&mut self) {
        self.show_line_numbers = !self.show_line_numbers;
    }

    pub fn toggle_status_bar(&mut self) {
        self.show_status_bar = !self.show_status_bar;
    }

    pub fn scroll_diff_up(&mut self) {
        self.diff_scroll_offset = self.diff_scroll_offset.saturating_sub(1);
    }

    pub fn scroll_diff_down(&mut self) {
        self.diff_scroll_offset = self.diff_scroll_offset.saturating_add(1);
    }

    pub fn reset_diff_scroll(&mut self) {
        self.diff_scroll_offset = 0;
    }

    pub fn render(&self, frame: &mut Frame, editor_state: &EditorState, menu_state: &MenuState) {
        let area = frame.size();

        let status_bar_height = if self.show_status_bar { 1 } else { 0 };
        let menu_bar_height = 1;

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(menu_bar_height),
                Constraint::Min(1),
                Constraint::Length(status_bar_height),
            ])
            .split(area);

        let menu_area = chunks[0];
        let editor_area = chunks[1];
        let status_area = if self.show_status_bar {
            Some(chunks[2])
        } else {
            None
        };

        self.render_menu_bar(frame, menu_state, menu_area);

        if editor_state.is_history_browser_open() {
            self.render_history_browser(frame, editor_state, editor_area);
        } else if editor_state.is_history_stats_open() {
            self.render_history_stats(frame, editor_state, editor_area);
        } else {
            self.render_editor_area(frame, editor_state, editor_area);
        }

        if let Some(status_area) = status_area {
            self.render_status_bar(frame, editor_state, status_area);
        }

        if menu_state.is_menu_open() {
            self.render_open_menu(frame, menu_state, menu_area);
        }
    }

    fn render_editor_area(&self, frame: &mut Frame, editor_state: &EditorState, area: Rect) {
        let (text_area, line_number_width) = if self.show_line_numbers {
            let buffer = editor_state.current_buffer();
            let line_count = buffer.line_count();
            let num_width = line_count.to_string().len().max(3) + 1;

            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(num_width as u16), Constraint::Min(1)])
                .split(area);

            self.render_line_numbers(frame, editor_state, chunks[0]);
            (chunks[1], num_width)
        } else {
            (area, 0)
        };

        self.render_text_buffer(frame, editor_state, text_area, line_number_width);
    }

    fn render_line_numbers(&self, frame: &mut Frame, editor_state: &EditorState, area: Rect) {
        let buffer = editor_state.current_buffer();
        let viewport_top = editor_state.viewport_top();
        let viewport_height = area.height as usize;

        let mut lines = Vec::new();
        for i in 0..viewport_height {
            let line_num = viewport_top + i;
            if line_num < buffer.line_count() {
                let line_text =
                    format!("{:>width$} ", line_num + 1, width = area.width as usize - 1);
                lines.push(Line::from(Span::styled(
                    line_text,
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                lines.push(Line::from(Span::styled(
                    " ".repeat(area.width as usize),
                    Style::default(),
                )));
            }
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }

    fn render_text_buffer(
        &self,
        frame: &mut Frame,
        editor_state: &EditorState,
        area: Rect,
        _line_number_width: usize,
    ) {
        let buffer = editor_state.current_buffer();
        let viewport_top = editor_state.viewport_top();
        let viewport_height = area.height as usize;
        let cursor = editor_state.cursor();

        let mut lines = Vec::new();
        for i in 0..viewport_height {
            let line_num = viewport_top + i;
            if line_num < buffer.line_count() {
                if let Ok(line_text) = buffer.line(line_num) {
                    let is_cursor_line = line_num == cursor.line;
                    let style = if is_cursor_line {
                        Style::default().bg(Color::Rgb(40, 40, 40))
                    } else {
                        Style::default()
                    };

                    lines.push(Line::from(Span::styled(line_text, style)));
                } else {
                    lines.push(Line::from(""));
                }
            } else {
                lines.push(Line::from("~"));
            }
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);

        let cursor_screen_line = cursor.line.saturating_sub(viewport_top);
        let cursor_x = area.x + cursor.column as u16;
        let cursor_y = area.y + cursor_screen_line as u16;

        if cursor_screen_line < viewport_height && cursor_x < area.x + area.width {
            frame.set_cursor(cursor_x, cursor_y);
        }
    }

    fn render_status_bar(&self, frame: &mut Frame, editor_state: &EditorState, area: Rect) {
        let buffer = editor_state.current_buffer();
        let cursor = editor_state.cursor();

        let file_name = buffer
            .file_path()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .unwrap_or("[No Name]");

        let modified = if buffer.is_modified() { " [+]" } else { "" };

        let read_only = if buffer.is_read_only() { " [RO]" } else { "" };

        let overwrite = if editor_state.overwrite_mode() {
            " [OVR]"
        } else {
            ""
        };

        let cursor_info = format!(" {}:{} ", cursor.line + 1, cursor.column + 1);

        let status_message = editor_state.status_message();
        let left_text = if status_message.is_empty() {
            format!(" {}{}{}{}", file_name, modified, read_only, overwrite)
        } else {
            format!(" {} ", status_message)
        };

        let left_width = left_text.len();
        let right_width = cursor_info.len();
        let padding_width = area.width.saturating_sub((left_width + right_width) as u16) as usize;
        let padding = " ".repeat(padding_width);

        let status_text = format!("{}{}{}", left_text, padding, cursor_info);

        let status_line = Paragraph::new(status_text).style(
            Style::default()
                .bg(Color::DarkGray)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        frame.render_widget(status_line, area);
    }

    fn render_history_browser(&self, frame: &mut Frame, editor_state: &EditorState, area: Rect) {
        let browser = match editor_state.history_browser() {
            Some(b) => b,
            None => return,
        };

        let main_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(area);

        let left_area = main_chunks[0];
        let right_area = main_chunks[1];

        let right_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
            .split(right_area);

        let details_area = right_chunks[0];
        let files_area = right_chunks[1];

        self.render_commit_list(frame, browser, left_area);
        self.render_commit_details(frame, browser, details_area);
        self.render_file_list(frame, browser, editor_state, files_area);
    }

    fn render_commit_list(
        &self,
        frame: &mut Frame,
        browser: &editor_core::HistoryBrowser,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Commit History ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let commits = browser.commits();
        let selected_idx = browser.selected_index();

        let visible_height = inner_area.height as usize;
        let scroll_offset = if selected_idx >= visible_height {
            selected_idx - visible_height + 1
        } else {
            0
        };

        let mut lines = Vec::new();
        for (idx, commit) in commits
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(visible_height)
        {
            let is_selected = idx == selected_idx;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Rgb(40, 40, 60))
            } else {
                Style::default().fg(Color::White)
            };

            let short_hash = &commit.id[..7.min(commit.id.len())];
            let timestamp = self.format_timestamp(commit.timestamp);
            let message_preview = commit.message.lines().next().unwrap_or("");
            let truncated_message = if message_preview.len() > 40 {
                format!("{}...", &message_preview[..37])
            } else {
                message_preview.to_string()
            };

            let line_text = format!("{} {} {}", short_hash, timestamp, truncated_message);

            lines.push(Line::from(Span::styled(line_text, style)));
        }

        if lines.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "No commit history yet",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Save the file to create your first auto-commit",
                Style::default().fg(Color::DarkGray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }

    fn render_commit_details(
        &self,
        frame: &mut Frame,
        browser: &editor_core::HistoryBrowser,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Commit Details ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        if let Some(commit) = browser.selected_commit() {
            lines.push(Line::from(Span::styled(
                format!("Commit: {}", commit.id),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));

            lines.push(Line::from(Span::styled(
                format!("Author: {} <{}>", commit.author_name, commit.author_email),
                Style::default().fg(Color::Green),
            )));

            lines.push(Line::from(Span::styled(
                format!("Date: {}", self.format_full_timestamp(commit.timestamp)),
                Style::default().fg(Color::Blue),
            )));

            lines.push(Line::from(""));

            for message_line in commit.message.lines() {
                lines.push(Line::from(Span::styled(
                    format!("  {}", message_line),
                    Style::default().fg(Color::White),
                )));
            }
        } else {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "No commit selected",
                Style::default().fg(Color::Gray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }

    fn render_file_list(
        &self,
        frame: &mut Frame,
        browser: &editor_core::HistoryBrowser,
        editor_state: &EditorState,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Diff View ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        if browser.selected_commit().is_some() {
            if let Some((from_commit, to_commit)) = browser.get_diff_commits() {
                lines.push(Line::from(vec![
                    Span::styled("Comparing: ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &from_commit.id[..7.min(from_commit.id.len())],
                        Style::default().fg(Color::Red),
                    ),
                    Span::styled(" → ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        &to_commit.id[..7.min(to_commit.id.len())],
                        Style::default().fg(Color::Green),
                    ),
                ]));
                lines.push(Line::from(""));

                match editor_state.get_history_diff() {
                    Ok(Some(diff)) => {
                        let diff_lines: Vec<&str> = diff.lines().collect();
                        let visible_height = inner_area.height.saturating_sub(2) as usize;
                        let max_scroll = diff_lines.len().saturating_sub(visible_height);
                        let scroll_offset = self.diff_scroll_offset.min(max_scroll);

                        for (idx, diff_line) in diff_lines
                            .iter()
                            .enumerate()
                            .skip(scroll_offset)
                            .take(visible_height)
                        {
                            let (style, prefix) = if diff_line.starts_with('+') {
                                (
                                    Style::default().fg(Color::Green).bg(Color::Rgb(0, 40, 0)),
                                    "+",
                                )
                            } else if diff_line.starts_with('-') {
                                (
                                    Style::default().fg(Color::Red).bg(Color::Rgb(40, 0, 0)),
                                    "-",
                                )
                            } else if diff_line.starts_with("@@") {
                                (
                                    Style::default()
                                        .fg(Color::Cyan)
                                        .add_modifier(Modifier::BOLD),
                                    "@",
                                )
                            } else if diff_line.starts_with("diff")
                                || diff_line.starts_with("index")
                            {
                                (Style::default().fg(Color::Yellow), " ")
                            } else {
                                (Style::default().fg(Color::White), " ")
                            };

                            let line_number = format!("{:4} ", scroll_offset + idx + 1);
                            let line_number_span =
                                Span::styled(line_number, Style::default().fg(Color::DarkGray));

                            let content = if diff_line.starts_with('+')
                                || diff_line.starts_with('-')
                                || diff_line.starts_with(' ')
                            {
                                &diff_line[1..]
                            } else {
                                diff_line
                            };

                            let content_span =
                                Span::styled(format!("{}{}", prefix, content), style);

                            lines.push(Line::from(vec![line_number_span, content_span]));
                        }

                        if diff_lines.len() > visible_height {
                            let scroll_info =
                                format!(" [Scroll: {}/{}] ", scroll_offset + 1, max_scroll + 1);
                            lines.push(Line::from(Span::styled(
                                scroll_info,
                                Style::default().fg(Color::DarkGray),
                            )));
                        }
                    }
                    Ok(None) => {
                        lines.push(Line::from(Span::styled(
                            "No parent commit to compare",
                            Style::default().fg(Color::DarkGray),
                        )));
                    }
                    Err(e) => {
                        lines.push(Line::from(Span::styled(
                            format!("Error getting diff: {}", e),
                            Style::default().fg(Color::Red),
                        )));
                    }
                }
            } else {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "No parent commit to compare",
                    Style::default().fg(Color::Gray),
                )));
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    "This is the first commit",
                    Style::default().fg(Color::DarkGray),
                )));
            }
        } else {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "No commit selected",
                Style::default().fg(Color::Gray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }

    fn render_history_stats(&self, frame: &mut Frame, editor_state: &EditorState, area: Rect) {
        let stats = match editor_state.history_stats() {
            Some(s) => s,
            None => return,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" History Statistics ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        lines.push(Line::from(Span::styled(
            "Repository Statistics",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        lines.push(Line::from(vec![
            Span::styled("Total Commits: ", Style::default().fg(Color::Green)),
            Span::styled(
                stats.total_commits.to_string(),
                Style::default().fg(Color::White),
            ),
        ]));

        let size_mb = stats.repository_size as f64 / (1024.0 * 1024.0);
        lines.push(Line::from(vec![
            Span::styled("Repository Size: ", Style::default().fg(Color::Green)),
            Span::styled(
                format!("{:.2} MB", size_mb),
                Style::default().fg(Color::White),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::styled("Large Files: ", Style::default().fg(Color::Green)),
            Span::styled(
                stats.large_file_count.to_string(),
                Style::default().fg(Color::White),
            ),
        ]));

        if stats.large_file_count > 0 {
            let large_size_mb = stats.total_large_file_size as f64 / (1024.0 * 1024.0);
            lines.push(Line::from(vec![
                Span::styled("Total Large File Size: ", Style::default().fg(Color::Green)),
                Span::styled(
                    format!("{:.2} MB", large_size_mb),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        if let Some((oldest, newest)) = stats.date_range {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Date Range",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            lines.push(Line::from(vec![
                Span::styled("Oldest Commit: ", Style::default().fg(Color::Green)),
                Span::styled(
                    self.format_full_timestamp(oldest),
                    Style::default().fg(Color::White),
                ),
            ]));

            lines.push(Line::from(vec![
                Span::styled("Newest Commit: ", Style::default().fg(Color::Green)),
                Span::styled(
                    self.format_full_timestamp(newest),
                    Style::default().fg(Color::White),
                ),
            ]));
        }

        if !stats.file_stats.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(Span::styled(
                "Top Files by Commit Count",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));

            for (idx, file_stat) in stats.file_stats.iter().take(10).enumerate() {
                let size_kb = file_stat.total_size as f64 / 1024.0;
                let size_color = if file_stat.is_large {
                    Color::Red
                } else {
                    Color::Blue
                };

                let mut spans = vec![
                    Span::styled(
                        format!("{}. ", idx + 1),
                        Style::default().fg(Color::DarkGray),
                    ),
                    Span::styled(
                        format!("{:4} commits ", file_stat.commit_count),
                        Style::default().fg(Color::Cyan),
                    ),
                    Span::styled(
                        format!("{:8.2} KB ", size_kb),
                        Style::default().fg(size_color),
                    ),
                ];

                if file_stat.is_large {
                    spans.push(Span::styled(
                        format!("{} [LARGE]", file_stat.path),
                        Style::default().fg(Color::White),
                    ));
                } else {
                    spans.push(Span::styled(
                        file_stat.path.clone(),
                        Style::default().fg(Color::White),
                    ));
                }

                lines.push(Line::from(spans));
            }

            if stats.file_stats.len() > 10 {
                lines.push(Line::from(""));
                lines.push(Line::from(Span::styled(
                    format!("... and {} more files", stats.file_stats.len() - 10),
                    Style::default().fg(Color::DarkGray),
                )));
            }
        }

        lines.push(Line::from(""));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press Esc or q to close",
            Style::default().fg(Color::DarkGray),
        )));

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }

    fn format_timestamp(&self, timestamp: i64) -> String {
        use chrono::{Local, TimeZone};

        let dt = Local.timestamp_opt(timestamp, 0).unwrap();
        let now = Local::now();

        let duration = now.signed_duration_since(dt);

        if duration.num_seconds() < 60 {
            "just now".to_string()
        } else if duration.num_minutes() < 60 {
            format!("{}m ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{}h ago", duration.num_hours())
        } else if duration.num_days() < 7 {
            format!("{}d ago", duration.num_days())
        } else if duration.num_days() < 30 {
            format!("{}w ago", duration.num_days() / 7)
        } else if duration.num_days() < 365 {
            format!("{}mo ago", duration.num_days() / 30)
        } else {
            format!("{}y ago", duration.num_days() / 365)
        }
    }

    fn format_full_timestamp(&self, timestamp: i64) -> String {
        use chrono::{Local, TimeZone};

        let dt = Local.timestamp_opt(timestamp, 0).unwrap();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }

    fn render_menu_bar(&self, frame: &mut Frame, menu_state: &MenuState, area: Rect) {
        let menu_types = MenuType::all();
        let mut spans = Vec::new();

        for (idx, menu_type) in menu_types.iter().enumerate() {
            let is_selected = menu_state.active && idx == menu_state.selected_menu;
            let style = if is_selected {
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::White)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            spans.push(Span::styled(" ", Style::default()));
            spans.push(Span::styled(menu_type.title(), style));
            spans.push(Span::styled(" ", Style::default()));
        }

        let padding_width = area
            .width
            .saturating_sub(spans.iter().map(|s| s.content.len() as u16).sum());
        if padding_width > 0 {
            spans.push(Span::styled(
                " ".repeat(padding_width as usize),
                Style::default(),
            ));
        }

        let menu_bar = Paragraph::new(Line::from(spans))
            .style(Style::default().bg(Color::DarkGray).fg(Color::White));

        frame.render_widget(menu_bar, area);
    }

    fn render_open_menu(&self, frame: &mut Frame, menu_state: &MenuState, menu_bar_area: Rect) {
        let menu = match &menu_state.open_menu {
            Some(m) => m,
            None => return,
        };

        let menu_types = MenuType::all();
        let mut x_offset = 1u16;
        for (idx, menu_type) in menu_types.iter().enumerate() {
            if idx == menu_state.selected_menu {
                break;
            }
            x_offset += menu_type.title().len() as u16 + 2;
        }

        let max_label_width = menu
            .items
            .iter()
            .map(|item| item.label.len())
            .max()
            .unwrap_or(10);
        let max_shortcut_width = menu
            .items
            .iter()
            .filter_map(|item| item.shortcut.as_ref().map(|s| s.len()))
            .max()
            .unwrap_or(0);

        let menu_width = (max_label_width + max_shortcut_width + 4).min(40) as u16;
        let menu_height = (menu.items.len() + 2).min(20) as u16;

        let menu_area = Rect {
            x: x_offset.min(menu_bar_area.width.saturating_sub(menu_width)),
            y: menu_bar_area.y + 1,
            width: menu_width,
            height: menu_height,
        };

        frame.render_widget(Clear, menu_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .style(Style::default().bg(Color::Rgb(30, 30, 30)));

        let inner_area = block.inner(menu_area);
        frame.render_widget(block, menu_area);

        let mut lines = Vec::new();
        for (idx, item) in menu.items.iter().enumerate() {
            if item.is_separator() {
                let separator = "─".repeat(inner_area.width as usize);
                lines.push(Line::from(Span::styled(
                    separator,
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                let is_selected = idx == menu.selected_index;
                let bg_color = if is_selected {
                    Color::Rgb(50, 50, 70)
                } else {
                    Color::Rgb(30, 30, 30)
                };

                let mut spans = Vec::new();

                spans.push(Span::styled(" ", Style::default().bg(bg_color)));

                let label_style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .bg(bg_color)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::White).bg(bg_color)
                };
                spans.push(Span::styled(item.label.clone(), label_style));

                let padding_len = max_label_width.saturating_sub(item.label.len()) + 2;
                spans.push(Span::styled(
                    " ".repeat(padding_len),
                    Style::default().bg(bg_color),
                ));

                if let Some(shortcut) = &item.shortcut {
                    let shortcut_style = Style::default().fg(Color::DarkGray).bg(bg_color);
                    spans.push(Span::styled(shortcut.clone(), shortcut_style));
                }

                let total_len: usize = spans.iter().map(|s| s.content.len()).sum();
                let remaining = inner_area.width as usize - total_len;
                if remaining > 0 {
                    spans.push(Span::styled(
                        " ".repeat(remaining),
                        Style::default().bg(bg_color),
                    ));
                }

                lines.push(Line::from(spans));
            }
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
