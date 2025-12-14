use editor_core::EditorState;
use eframe::egui;
use std::time::{Duration, Instant};

pub struct Renderer {
    show_line_numbers: bool,
    font_size: f32,
    line_height: f32,
    char_width: f32,
    cursor_blink_state: bool,
    last_blink_time: Instant,
    blink_interval: Duration,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            show_line_numbers: true,
            font_size: 14.0,
            line_height: 18.0,
            char_width: 8.4,
            cursor_blink_state: true,
            last_blink_time: Instant::now(),
            blink_interval: Duration::from_millis(530),
        }
    }

    #[allow(dead_code)]
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        editor_state: &EditorState,
        ctx: &egui::Context,
    ) -> Option<i32> {
        self.update_cursor_blink();
        ctx.request_repaint_after(self.blink_interval / 2);
        let available_size = ui.available_size();
        let mut scroll_delta = None;

        let (line_numbers_width, text_start_x) = if self.show_line_numbers {
            let buffer = editor_state.current_buffer();
            let line_count = buffer.line_count();
            let num_digits = line_count.to_string().len().max(3);
            let width = (num_digits as f32 + 1.0) * self.char_width + 10.0;
            (width, width)
        } else {
            (0.0, 0.0)
        };

        let viewport_top = editor_state.viewport_top();
        let viewport_height = (available_size.y / self.line_height).ceil() as usize;

        let (response, painter) = ui.allocate_painter(
            available_size,
            egui::Sense::click().union(egui::Sense::drag()),
        );

        if self.show_line_numbers {
            self.render_line_numbers(
                &painter,
                editor_state,
                line_numbers_width,
                viewport_top,
                viewport_height,
            );
        }

        self.render_text_buffer(
            &painter,
            editor_state,
            text_start_x,
            viewport_top,
            viewport_height,
        );

        self.render_selection(
            &painter,
            editor_state,
            text_start_x,
            viewport_top,
            viewport_height,
        );

        self.render_cursor(&painter, editor_state, text_start_x, viewport_top);

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let relative_pos = pos - response.rect.min;
                let _line = ((relative_pos.y / self.line_height).floor() as usize) + viewport_top;
                let _column = ((relative_pos.x - text_start_x) / self.char_width).floor() as usize;
            }
        }

        ui.input(|i| {
            let scroll = i.smooth_scroll_delta.y;
            if scroll != 0.0 {
                scroll_delta = Some((scroll / self.line_height) as i32);
            }
        });

        scroll_delta
    }

    fn render_line_numbers(
        &self,
        painter: &egui::Painter,
        editor_state: &EditorState,
        width: f32,
        viewport_top: usize,
        viewport_height: usize,
    ) {
        let buffer = editor_state.current_buffer();
        let font_id = egui::FontId::monospace(self.font_size);
        let color = egui::Color32::DARK_GRAY;

        for i in 0..viewport_height {
            let line_num = viewport_top + i;
            if line_num < buffer.line_count() {
                let line_text = format!(
                    "{:>width$} ",
                    line_num + 1,
                    width = width as usize / self.char_width as usize - 2
                );
                let y = i as f32 * self.line_height;
                painter.text(
                    egui::pos2(5.0, y),
                    egui::Align2::LEFT_TOP,
                    line_text,
                    font_id.clone(),
                    color,
                );
            }
        }
    }

    fn render_text_buffer(
        &self,
        painter: &egui::Painter,
        editor_state: &EditorState,
        x_offset: f32,
        viewport_top: usize,
        viewport_height: usize,
    ) {
        let buffer = editor_state.current_buffer();
        let cursor = editor_state.cursor();
        let font_id = egui::FontId::monospace(self.font_size);
        let text_color = egui::Color32::WHITE;
        let current_line_bg = egui::Color32::from_rgb(40, 40, 40);

        for i in 0..viewport_height {
            let line_num = viewport_top + i;
            if line_num < buffer.line_count() {
                let y = i as f32 * self.line_height;

                if line_num == cursor.line {
                    let rect = egui::Rect::from_min_size(
                        egui::pos2(x_offset, y),
                        egui::vec2(painter.clip_rect().width() - x_offset, self.line_height),
                    );
                    painter.rect_filled(rect, 0.0, current_line_bg);
                }

                if let Ok(line_text) = buffer.line(line_num) {
                    painter.text(
                        egui::pos2(x_offset, y),
                        egui::Align2::LEFT_TOP,
                        line_text,
                        font_id.clone(),
                        text_color,
                    );
                }
            } else {
                let y = i as f32 * self.line_height;
                painter.text(
                    egui::pos2(x_offset, y),
                    egui::Align2::LEFT_TOP,
                    "~",
                    font_id.clone(),
                    egui::Color32::DARK_GRAY,
                );
            }
        }
    }

    fn render_selection(
        &self,
        painter: &egui::Painter,
        editor_state: &EditorState,
        x_offset: f32,
        viewport_top: usize,
        viewport_height: usize,
    ) {
        if let Some(selection) = editor_state.selection() {
            let start = selection.start();
            let end = selection.end();
            let selection_bg = egui::Color32::from_rgb(60, 90, 140);

            for i in 0..viewport_height {
                let line_num = viewport_top + i;
                if line_num < start.line || line_num > end.line {
                    continue;
                }

                let buffer = editor_state.current_buffer();
                if line_num >= buffer.line_count() {
                    continue;
                }

                let line_text = buffer.line(line_num).unwrap_or_default();
                let y = i as f32 * self.line_height;

                let (start_col, end_col) = if line_num == start.line && line_num == end.line {
                    (start.column, end.column)
                } else if line_num == start.line {
                    (start.column, line_text.len())
                } else if line_num == end.line {
                    (0, end.column)
                } else {
                    (0, line_text.len())
                };

                let x1 = x_offset + (start_col as f32 * self.char_width);
                let x2 = x_offset + (end_col as f32 * self.char_width);

                let rect = egui::Rect::from_min_size(
                    egui::pos2(x1, y),
                    egui::vec2(x2 - x1, self.line_height),
                );
                painter.rect_filled(rect, 0.0, selection_bg);
            }
        }
    }

    fn render_cursor(
        &self,
        painter: &egui::Painter,
        editor_state: &EditorState,
        x_offset: f32,
        viewport_top: usize,
    ) {
        if !self.cursor_blink_state {
            return;
        }

        let cursor = editor_state.cursor();

        if cursor.line >= viewport_top {
            let screen_line = cursor.line - viewport_top;
            let x = x_offset + (cursor.column as f32 * self.char_width);
            let y = screen_line as f32 * self.line_height;

            let cursor_rect =
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(2.0, self.line_height));
            painter.rect_filled(cursor_rect, 0.0, egui::Color32::WHITE);
        }
    }

    fn update_cursor_blink(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_blink_time) >= self.blink_interval {
            self.cursor_blink_state = !self.cursor_blink_state;
            self.last_blink_time = now;
        }
    }

    pub fn reset_cursor_blink(&mut self) {
        self.cursor_blink_state = true;
        self.last_blink_time = Instant::now();
    }

    pub fn render_status_bar(&self, ui: &mut egui::Ui, editor_state: &EditorState) {
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

        let status_message = editor_state.status_message();
        let left_text = if status_message.is_empty() {
            format!("{}{}{}{}", file_name, modified, read_only, overwrite)
        } else {
            status_message.to_string()
        };

        let right_text = format!("{}:{}", cursor.line + 1, cursor.column + 1);

        ui.horizontal(|ui| {
            ui.label(&left_text);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(&right_text);
            });
        });
    }
}

impl Default for Renderer {
    fn default() -> Self {
        Self::new()
    }
}
