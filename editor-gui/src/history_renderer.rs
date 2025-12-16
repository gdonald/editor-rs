use editor_core::{CommitInfo, DiffViewMode, HistoryBrowser};
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

    pub fn render(&mut self, ui: &mut egui::Ui, history_browser: &mut HistoryBrowser) {
        egui::SidePanel::left("commit_list_panel")
            .resizable(true)
            .default_width(self.commit_list_width)
            .width_range(200.0..=600.0)
            .show_inside(ui, |ui| {
                self.commit_list_width = ui.available_width();
                self.render_commit_list(ui, history_browser);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            self.render_details_area(ui, history_browser);
        });
    }

    fn render_commit_list(&self, ui: &mut egui::Ui, history_browser: &mut HistoryBrowser) {
        ui.heading("Commits");
        ui.separator();

        let selected_index = history_browser.selected_index();
        let commits_len = history_browser.commits().len();
        let mut new_selection = None;
        let mut view_diff = false;

        egui::ScrollArea::vertical()
            .id_salt("commit_list_scroll")
            .show(ui, |ui| {
                for index in 0..commits_len {
                    if let Some(commit) = history_browser.commits().get(index) {
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

                if commits_len == 0 {
                    ui.label("No commits found");
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

    fn render_details_area(&mut self, ui: &mut egui::Ui, history_browser: &HistoryBrowser) {
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
                self.render_diff_view(ui, history_browser);
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

    fn render_diff_view(&self, ui: &mut egui::Ui, _history_browser: &HistoryBrowser) {
        ui.heading("Diff");
        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("diff_view_scroll")
            .show(ui, |ui| {
                ui.label("Diff view will be implemented in future task");
            });
    }

    pub fn commit_list_width(&self) -> f32 {
        self.commit_list_width
    }

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
