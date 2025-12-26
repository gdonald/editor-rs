use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DialogType {
    Search,
    Replace,
    GotoLine,
    Help,
}

#[derive(Debug, Clone)]
pub struct Dialog {
    pub dialog_type: DialogType,
    pub input: String,
    pub replace_input: Option<String>,
    pub cursor_position: usize,
    pub editing_replace: bool,
}

impl Dialog {
    pub fn new(dialog_type: DialogType) -> Self {
        Self {
            dialog_type,
            input: String::new(),
            replace_input: if matches!(dialog_type, DialogType::Replace) {
                Some(String::new())
            } else {
                None
            },
            cursor_position: 0,
            editing_replace: false,
        }
    }

    pub fn insert_char(&mut self, c: char) {
        if self.editing_replace {
            if let Some(ref mut replace) = self.replace_input {
                replace.insert(self.cursor_position, c);
                self.cursor_position += 1;
            }
        } else {
            self.input.insert(self.cursor_position, c);
            self.cursor_position += 1;
        }
    }

    pub fn delete_char(&mut self) {
        if self.editing_replace {
            if let Some(ref mut replace) = self.replace_input {
                if self.cursor_position < replace.len() {
                    replace.remove(self.cursor_position);
                }
            }
        } else if self.cursor_position < self.input.len() {
            self.input.remove(self.cursor_position);
        }
    }

    pub fn backspace(&mut self) {
        if self.editing_replace {
            if let Some(ref mut replace) = self.replace_input {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                    replace.remove(self.cursor_position);
                }
            }
        } else if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.input.remove(self.cursor_position);
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        let text = if self.editing_replace {
            self.replace_input.as_deref().unwrap_or("")
        } else {
            &self.input
        };

        if self.cursor_position < text.len() {
            self.cursor_position += 1;
        }
    }

    pub fn move_to_start(&mut self) {
        self.cursor_position = 0;
    }

    pub fn move_to_end(&mut self) {
        let text = if self.editing_replace {
            self.replace_input.as_deref().unwrap_or("")
        } else {
            &self.input
        };

        self.cursor_position = text.len();
    }

    pub fn switch_field(&mut self) {
        if self.replace_input.is_some() {
            self.editing_replace = !self.editing_replace;
            self.cursor_position = if self.editing_replace {
                self.replace_input.as_ref().unwrap().len()
            } else {
                self.input.len()
            };
        }
    }

    pub fn title(&self) -> &str {
        match self.dialog_type {
            DialogType::Search => "Search",
            DialogType::Replace => "Replace",
            DialogType::GotoLine => "Go to Line",
            DialogType::Help => "Keyboard Shortcuts",
        }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let dialog_width = match self.dialog_type {
            DialogType::Help => 80,
            _ => 60,
        };
        let dialog_height = match self.dialog_type {
            DialogType::Help => 30,
            DialogType::Replace => 10,
            _ => 7,
        };

        let dialog_area = centered_rect(dialog_width, dialog_height, area);

        frame.render_widget(Clear, dialog_area);

        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" {} ", self.title()))
            .border_style(Style::default().fg(Color::Cyan))
            .style(Style::default().bg(Color::Rgb(30, 30, 30)));

        let inner_area = block.inner(dialog_area);
        frame.render_widget(block, dialog_area);

        match self.dialog_type {
            DialogType::Search | DialogType::GotoLine => {
                self.render_single_input(frame, inner_area);
            }
            DialogType::Replace => {
                self.render_replace_dialog(frame, inner_area);
            }
            DialogType::Help => {
                self.render_help(frame, inner_area);
            }
        }
    }

    fn render_single_input(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(area);

        let label = match self.dialog_type {
            DialogType::Search => "Search for:",
            DialogType::GotoLine => "Line number:",
            _ => "",
        };

        let label_paragraph = Paragraph::new(label).style(Style::default().fg(Color::White));
        frame.render_widget(label_paragraph, chunks[0]);

        let input_paragraph = Paragraph::new(self.input.as_str()).style(
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Rgb(40, 40, 40)),
        );
        frame.render_widget(input_paragraph, chunks[1]);

        if chunks[1].x + (self.cursor_position as u16) < chunks[1].x + chunks[1].width {
            frame.set_cursor(chunks[1].x + (self.cursor_position as u16), chunks[1].y);
        }

        let help_text = match self.dialog_type {
            DialogType::Search => "Enter: Search  |  Esc: Cancel",
            DialogType::GotoLine => "Enter: Go  |  Esc: Cancel",
            _ => "",
        };

        let help_paragraph = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
        frame.render_widget(help_paragraph, chunks[2]);
    }

    fn render_replace_dialog(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Min(1),
            ])
            .split(area);

        let search_label = Paragraph::new("Search for:").style(Style::default().fg(Color::White));
        frame.render_widget(search_label, chunks[0]);

        let search_style = if !self.editing_replace {
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Rgb(40, 40, 40))
        } else {
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30))
        };
        let search_input = Paragraph::new(self.input.as_str()).style(search_style);
        frame.render_widget(search_input, chunks[1]);

        let replace_label =
            Paragraph::new("Replace with:").style(Style::default().fg(Color::White));
        frame.render_widget(replace_label, chunks[2]);

        let replace_style = if self.editing_replace {
            Style::default()
                .fg(Color::Yellow)
                .bg(Color::Rgb(40, 40, 40))
        } else {
            Style::default().fg(Color::White).bg(Color::Rgb(30, 30, 30))
        };
        let replace_text = self.replace_input.as_deref().unwrap_or("");
        let replace_input = Paragraph::new(replace_text).style(replace_style);
        frame.render_widget(replace_input, chunks[3]);

        let active_chunk = if self.editing_replace {
            chunks[3]
        } else {
            chunks[1]
        };
        if active_chunk.x + (self.cursor_position as u16) < active_chunk.x + active_chunk.width {
            frame.set_cursor(
                active_chunk.x + (self.cursor_position as u16),
                active_chunk.y,
            );
        }

        let help_text = "Tab: Switch fields  |  Enter: Replace All  |  Esc: Cancel";
        let help_paragraph = Paragraph::new(help_text).style(Style::default().fg(Color::DarkGray));
        frame.render_widget(help_paragraph, chunks[4]);
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let shortcuts = vec![
            ("", "File Operations", true),
            ("Ctrl+N", "New file", false),
            ("Ctrl+O", "Open file", false),
            ("Ctrl+S", "Save file", false),
            ("Ctrl+W", "Close file", false),
            ("Ctrl+Q", "Quit", false),
            ("", "", false),
            ("", "Editing", true),
            ("Ctrl+Z", "Undo", false),
            ("Ctrl+Y / Ctrl+Shift+Z", "Redo", false),
            ("Ctrl+C", "Copy", false),
            ("Ctrl+X", "Cut", false),
            ("Ctrl+V", "Paste", false),
            ("Ctrl+D", "Duplicate line", false),
            ("Ctrl+K", "Delete line", false),
            ("Ctrl+J", "Join lines", false),
            ("", "", false),
            ("", "Navigation", true),
            ("Ctrl+F", "Search", false),
            ("Ctrl+H", "Replace", false),
            ("Ctrl+G", "Go to line", false),
            ("F3 / Shift+F3", "Next/Previous match", false),
            ("Ctrl+B", "Jump to matching bracket", false),
            ("", "", false),
            ("", "Menu", true),
            (
                "Alt+F/E/V/S/T/H",
                "Open File/Edit/View/Search/Tools/Help menu",
                false,
            ),
            ("", "", false),
            ("Press Esc to close", "", false),
        ];

        let mut lines = Vec::new();
        for (key, desc, is_header) in shortcuts {
            if is_header {
                lines.push(Line::from(Span::styled(
                    desc,
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )));
            } else if key.is_empty() && desc.is_empty() {
                lines.push(Line::from(""));
            } else if key.is_empty() {
                lines.push(Line::from(Span::styled(
                    desc,
                    Style::default().fg(Color::DarkGray),
                )));
            } else {
                let key_width = 30;
                let padded_key = format!("{:width$}", key, width = key_width);
                lines.push(Line::from(vec![
                    Span::styled(padded_key, Style::default().fg(Color::Cyan)),
                    Span::styled(desc, Style::default().fg(Color::White)),
                ]));
            }
        }

        let paragraph = Paragraph::new(lines);
        frame.render_widget(paragraph, area);
    }
}

fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((r.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((r.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}
