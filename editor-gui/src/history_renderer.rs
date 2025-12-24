use editor_core::{CommitInfo, DiffLineType, DiffViewMode, HistoryBrowser, SideBySideDiff};
use eframe::egui;

pub struct HistoryRenderer {
    commit_list_width: f32,
    file_list_height: f32,
    #[allow(dead_code)]
    font_size: f32,
    #[allow(dead_code)]
    line_height: f32,
}

impl HistoryRenderer {
    pub fn new() -> Self {
        Self {
            commit_list_width: 300.0,
            file_list_height: 150.0,
            font_size: 14.0,
            line_height: 18.0,
        }
    }

    pub fn render(
        &mut self,
        ui: &mut egui::Ui,
        history_browser: &mut HistoryBrowser,
        diff_content: Option<String>,
    ) {
        egui::SidePanel::left("commit_list_panel")
            .resizable(true)
            .default_width(self.commit_list_width)
            .width_range(200.0..=600.0)
            .show_inside(ui, |ui| {
                self.commit_list_width = ui.available_width();
                self.render_commit_list(ui, history_browser);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.render_details_area(ui, history_browser, diff_content);
        });
    }

    fn render_commit_list(&self, ui: &mut egui::Ui, history_browser: &mut HistoryBrowser) {
        ui.heading("Commits");
        ui.separator();

        let mut search_text = history_browser.search_query().unwrap_or("").to_string();
        let text_edit = egui::TextEdit::singleline(&mut search_text)
            .hint_text("Search commits... (Ctrl+F)")
            .desired_width(ui.available_width());

        let response = ui.add(text_edit);

        if response.changed() {
            if search_text.is_empty() {
                history_browser.clear_search();
            } else {
                history_browser.set_search_query(Some(search_text));
            }
        }

        let mut file_filter_text = history_browser.file_filter().unwrap_or("").to_string();
        let file_filter_edit = egui::TextEdit::singleline(&mut file_filter_text)
            .hint_text("Filter by file...")
            .desired_width(ui.available_width());

        let file_filter_response = ui.add(file_filter_edit);

        if file_filter_response.changed() {
            if file_filter_text.is_empty() {
                history_browser.clear_file_filter();
            } else {
                history_browser.set_file_filter(Some(file_filter_text));
            }
        }

        if history_browser.is_searching() || history_browser.is_file_filtering() {
            ui.horizontal(|ui| {
                ui.label(format!("{} matches", history_browser.match_count()));
                if ui.button("Clear All Filters").clicked() {
                    history_browser.clear_search();
                    history_browser.clear_file_filter();
                }
            });
        }

        ui.separator();

        let selected_index = history_browser.selected_index();
        let commits_len = history_browser.commits().len();
        let mut new_selection = None;
        let mut view_diff = false;

        egui::ScrollArea::vertical()
            .id_salt("commit_list_scroll")
            .show(ui, |ui| {
                let mut visible_count = 0;
                for index in 0..commits_len {
                    if !history_browser.is_commit_visible(index) {
                        continue;
                    }

                    if let Some(commit) = history_browser.commits().get(index) {
                        visible_count += 1;
                        let is_selected = index == selected_index;
                        let response = self.render_commit_item(ui, commit, is_selected);

                        if response.clicked() {
                            new_selection = Some(index);
                        }

                        if response.double_clicked() {
                            new_selection = Some(index);
                            view_diff = true;
                        }
                    }
                }

                if visible_count == 0 {
                    ui.vertical_centered(|ui| {
                        ui.add_space(20.0);
                        if history_browser.is_searching() {
                            ui.label(
                                egui::RichText::new("No matching commits found")
                                    .color(egui::Color32::GRAY)
                                    .size(16.0),
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new("Try a different search query")
                                    .color(egui::Color32::DARK_GRAY)
                                    .size(12.0),
                            );
                        } else {
                            ui.label(
                                egui::RichText::new("No commit history yet")
                                    .color(egui::Color32::GRAY)
                                    .size(16.0),
                            );
                            ui.add_space(10.0);
                            ui.label(
                                egui::RichText::new(
                                    "Save the file to create your first auto-commit",
                                )
                                .color(egui::Color32::DARK_GRAY)
                                .size(12.0),
                            );
                        }
                    });
                }
            });

        if let Some(index) = new_selection {
            history_browser.select_commit(index);
            if view_diff {
                history_browser.set_diff_view_mode(DiffViewMode::FullDiff);
            }
        }
    }

    fn render_commit_item(
        &self,
        ui: &mut egui::Ui,
        commit: &CommitInfo,
        is_selected: bool,
    ) -> egui::Response {
        let (rect, mut response) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 80.0), egui::Sense::click());

        let is_hovered = response.hovered();

        let fill_color = if is_selected {
            egui::Color32::from_rgb(60, 90, 140)
        } else if is_hovered {
            egui::Color32::from_rgb(45, 45, 45)
        } else {
            egui::Color32::from_rgb(30, 30, 30)
        };

        ui.painter().rect_filled(rect, 0.0, fill_color);

        let content_rect = rect.shrink(8.0);
        let mut ui = ui.new_child(egui::UiBuilder::new().max_rect(content_rect));

        let short_id = if commit.id.len() >= 7 {
            &commit.id[0..7]
        } else {
            &commit.id
        };
        ui.label(
            egui::RichText::new(short_id)
                .color(egui::Color32::YELLOW)
                .monospace(),
        );

        let first_line = commit.message.lines().next().unwrap_or("");
        ui.label(egui::RichText::new(first_line).color(egui::Color32::WHITE));

        ui.label(
            egui::RichText::new(format!("{} <{}>", commit.author_name, commit.author_email))
                .color(egui::Color32::GRAY)
                .small(),
        );

        let timestamp_str = format_timestamp(commit.timestamp);
        ui.label(
            egui::RichText::new(timestamp_str)
                .color(egui::Color32::GRAY)
                .small(),
        );

        if is_hovered {
            response = response.on_hover_cursor(egui::CursorIcon::PointingHand);
        }

        response
    }

    fn render_details_area(
        &mut self,
        ui: &mut egui::Ui,
        history_browser: &HistoryBrowser,
        diff_content: Option<String>,
    ) {
        if let Some(commit) = history_browser.selected_commit() {
            egui::TopBottomPanel::top("commit_details_panel")
                .resizable(true)
                .default_height(200.0)
                .height_range(100.0..=400.0)
                .show_inside(ui, |ui| {
                    self.render_commit_details(ui, commit);
                });

            let diff_mode = history_browser.diff_view_mode();
            if matches!(diff_mode, DiffViewMode::FileDiff(_)) {
                egui::TopBottomPanel::bottom("file_list_panel")
                    .resizable(true)
                    .default_height(self.file_list_height)
                    .height_range(100.0..=300.0)
                    .show_inside(ui, |ui| {
                        self.file_list_height = ui.available_height();
                        self.render_file_list(ui, history_browser);
                    });
            }

            egui::CentralPanel::default().show_inside(ui, |ui| {
                self.render_diff_view(ui, diff_content, history_browser.is_side_by_side());
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No commit selected");
            });
        }
    }

    fn render_commit_details(&self, ui: &mut egui::Ui, commit: &CommitInfo) {
        ui.heading("Commit Details");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("commit_details_scroll")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Commit:").strong());
                    ui.label(egui::RichText::new(&commit.id).monospace());
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Author:").strong());
                    ui.label(format!("{} <{}>", commit.author_name, commit.author_email));
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Date:").strong());
                    ui.label(format_timestamp(commit.timestamp));
                });

                ui.add_space(10.0);
                ui.label(egui::RichText::new("Message:").strong());
                ui.separator();
                ui.label(&commit.message);
            });
    }

    fn render_file_list(&self, ui: &mut egui::Ui, _history_browser: &HistoryBrowser) {
        ui.heading("Files Changed");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("file_list_scroll")
            .show(ui, |ui| {
                ui.label("File list will be implemented in future task");
            });
    }

    fn render_diff_view(
        &self,
        ui: &mut egui::Ui,
        diff_content: Option<String>,
        side_by_side: bool,
    ) {
        ui.heading(if side_by_side {
            "Diff (Side-by-Side)"
        } else {
            "Diff"
        });
        ui.separator();

        match diff_content {
            Some(diff) => {
                if side_by_side {
                    self.render_side_by_side_diff(ui, &diff);
                } else {
                    egui::ScrollArea::both()
                        .id_salt("diff_view_scroll")
                        .show(ui, |ui| {
                            self.render_diff_content(ui, &diff);
                        });
                }
            }
            None => {
                ui.centered_and_justified(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("No diff available")
                                .color(egui::Color32::GRAY)
                                .size(14.0),
                        );
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(
                                "This is the first commit or no parent commit to compare with",
                            )
                            .color(egui::Color32::DARK_GRAY)
                            .size(12.0),
                        );
                    });
                });
            }
        }
    }

    fn render_diff_content(&self, ui: &mut egui::Ui, diff: &str) {
        let font_id = egui::FontId::monospace(self.font_size);
        let line_height = self.line_height;

        let lines: Vec<&str> = diff.lines().collect();
        let num_lines = lines.len();
        let line_num_width = num_lines.to_string().len().max(3);

        for (line_num, line) in lines.iter().enumerate() {
            ui.horizontal(|ui| {
                let line_num_text = format!("{:>width$}", line_num + 1, width = line_num_width);
                ui.label(
                    egui::RichText::new(line_num_text)
                        .font(font_id.clone())
                        .color(egui::Color32::DARK_GRAY),
                );

                ui.separator();

                let (text_color, bg_color) = if line.starts_with('+') {
                    (
                        egui::Color32::from_rgb(100, 255, 100),
                        Some(egui::Color32::from_rgb(30, 60, 30)),
                    )
                } else if line.starts_with('-') {
                    (
                        egui::Color32::from_rgb(255, 100, 100),
                        Some(egui::Color32::from_rgb(60, 30, 30)),
                    )
                } else if line.starts_with("@@") {
                    (
                        egui::Color32::from_rgb(100, 200, 255),
                        Some(egui::Color32::from_rgb(30, 40, 50)),
                    )
                } else if line.starts_with("diff --git") || line.starts_with("index ") {
                    (egui::Color32::GRAY, None)
                } else {
                    (egui::Color32::WHITE, None)
                };

                let response = ui.allocate_response(
                    egui::vec2(ui.available_width(), line_height),
                    egui::Sense::hover(),
                );

                if let Some(bg) = bg_color {
                    ui.painter().rect_filled(response.rect, 0.0, bg);
                }

                ui.painter().text(
                    response.rect.left_top() + egui::vec2(4.0, 0.0),
                    egui::Align2::LEFT_TOP,
                    line,
                    font_id.clone(),
                    text_color,
                );
            });
        }
    }

    fn render_side_by_side_diff(&self, ui: &mut egui::Ui, diff: &str) {
        let font_id = egui::FontId::monospace(self.font_size);
        let line_height = self.line_height;

        let side_by_side_diff = SideBySideDiff::from_unified_diff(diff);

        egui::ScrollArea::both()
            .id_salt("side_by_side_diff_scroll")
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("Before").strong());
                        ui.separator();

                        for line in &side_by_side_diff.left_lines {
                            self.render_side_by_side_line(ui, line, &font_id, line_height, true);
                        }
                    });

                    ui.separator();

                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("After").strong());
                        ui.separator();

                        for line in &side_by_side_diff.right_lines {
                            self.render_side_by_side_line(ui, line, &font_id, line_height, false);
                        }
                    });
                });
            });
    }

    fn render_side_by_side_line(
        &self,
        ui: &mut egui::Ui,
        line: &editor_core::DiffLine,
        font_id: &egui::FontId,
        line_height: f32,
        is_left: bool,
    ) {
        ui.horizontal(|ui| {
            let line_num_text = if let Some(old_num) = line.old_line_num {
                if is_left {
                    format!("{:>4}", old_num + 1)
                } else {
                    "    ".to_string()
                }
            } else if let Some(new_num) = line.new_line_num {
                if !is_left {
                    format!("{:>4}", new_num + 1)
                } else {
                    "    ".to_string()
                }
            } else {
                "    ".to_string()
            };

            ui.label(
                egui::RichText::new(line_num_text)
                    .font(font_id.clone())
                    .color(egui::Color32::DARK_GRAY),
            );

            ui.separator();

            let (text_color, bg_color) = match line.line_type {
                DiffLineType::Addition => (
                    egui::Color32::from_rgb(100, 255, 100),
                    Some(egui::Color32::from_rgb(30, 60, 30)),
                ),
                DiffLineType::Deletion => (
                    egui::Color32::from_rgb(255, 100, 100),
                    Some(egui::Color32::from_rgb(60, 30, 30)),
                ),
                DiffLineType::Context => (egui::Color32::WHITE, None),
                DiffLineType::FileHeader => (egui::Color32::LIGHT_BLUE, None),
                DiffLineType::Hunk => (
                    egui::Color32::from_rgb(100, 200, 255),
                    Some(egui::Color32::from_rgb(30, 40, 50)),
                ),
                DiffLineType::Header => (egui::Color32::GRAY, None),
            };

            let response = ui.allocate_response(
                egui::vec2(ui.available_width().max(400.0), line_height),
                egui::Sense::hover(),
            );

            if let Some(bg) = bg_color {
                ui.painter().rect_filled(response.rect, 0.0, bg);
            }

            ui.painter().text(
                response.rect.left_top() + egui::vec2(4.0, 0.0),
                egui::Align2::LEFT_TOP,
                &line.content,
                font_id.clone(),
                text_color,
            );
        });
    }

    #[allow(dead_code)]
    pub fn commit_list_width(&self) -> f32 {
        self.commit_list_width
    }

    #[allow(dead_code)]
    pub fn file_list_height(&self) -> f32 {
        self.file_list_height
    }
}

impl Default for HistoryRenderer {
    fn default() -> Self {
        Self::new()
    }
}

fn format_timestamp(timestamp: i64) -> String {
    use chrono::{Local, TimeZone};

    let datetime = Local.timestamp_opt(timestamp, 0);
    match datetime {
        chrono::LocalResult::Single(dt) => dt.format("%Y-%m-%d %H:%M:%S").to_string(),
        _ => "Invalid timestamp".to_string(),
    }
}
