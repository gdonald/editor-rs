use editor_core::EditorState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
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

        self.render_editor_area(frame, editor_state, editor_area);
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
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
