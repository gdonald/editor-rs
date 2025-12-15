use editor_core::EditorState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct Renderer {
    show_line_numbers: bool,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
        }
    }

    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn render(&self, frame: &mut Frame, editor_state: &EditorState) {
        let area = frame.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(area);

        let editor_area = chunks[0];
        let status_area = chunks[1];

        if editor_state.is_history_browser_open() {
            self.render_history_browser(frame, editor_state, editor_area);
        } else {
            self.render_editor_area(frame, editor_state, editor_area);
        }
        self.render_status_bar(frame, editor_state, status_area);
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
        self.render_file_list(frame, browser, files_area);
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
            lines.push(Line::from(Span::styled(
                "No commits found",
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
            lines.push(Line::from(Span::styled(
                "No commit selected",
                Style::default().fg(Color::DarkGray),
            )));
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, inner_area);
    }

    fn render_file_list(
        &self,
        frame: &mut Frame,
        browser: &editor_core::HistoryBrowser,
        area: Rect,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Changed Files ")
            .border_style(Style::default().fg(Color::Cyan));

        let inner_area = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = Vec::new();

        if let Some(_commit) = browser.selected_commit() {
            lines.push(Line::from(Span::styled(
                "File diff view coming in 6.5.4.7",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            lines.push(Line::from(Span::styled(
                "No commit selected",
                Style::default().fg(Color::DarkGray),
            )));
        }

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
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
