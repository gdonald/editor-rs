use editor_core::git_history::HistoryStats;
use eframe::egui;

pub struct StatsRenderer {}

impl StatsRenderer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&self, ui: &mut egui::Ui, stats: &HistoryStats) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add_space(20.0);

                ui.heading("History Statistics");
                ui.add_space(20.0);

                ui.separator();
                ui.add_space(10.0);

                ui.heading("Repository Statistics");
                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Total Commits:").color(egui::Color32::GREEN));
                    ui.label(
                        egui::RichText::new(stats.total_commits.to_string())
                            .color(egui::Color32::WHITE),
                    );
                });

                let size_mb = stats.repository_size as f64 / (1024.0 * 1024.0);
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Repository Size:").color(egui::Color32::GREEN));
                    ui.label(
                        egui::RichText::new(format!("{:.2} MB", size_mb))
                            .color(egui::Color32::WHITE),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Large Files:").color(egui::Color32::GREEN));
                    ui.label(
                        egui::RichText::new(stats.large_file_count.to_string())
                            .color(egui::Color32::WHITE),
                    );
                });

                if stats.large_file_count > 0 {
                    let large_size_mb = stats.total_large_file_size as f64 / (1024.0 * 1024.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Total Large File Size:")
                                .color(egui::Color32::GREEN),
                        );
                        ui.label(
                            egui::RichText::new(format!("{:.2} MB", large_size_mb))
                                .color(egui::Color32::WHITE),
                        );
                    });
                }

                if let Some((oldest, newest)) = stats.date_range {
                    ui.add_space(20.0);
                    ui.heading("Date Range");
                    ui.add_space(10.0);

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Oldest Commit:").color(egui::Color32::GREEN));
                        ui.label(
                            egui::RichText::new(Self::format_timestamp(oldest))
                                .color(egui::Color32::WHITE),
                        );
                    });

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Newest Commit:").color(egui::Color32::GREEN));
                        ui.label(
                            egui::RichText::new(Self::format_timestamp(newest))
                                .color(egui::Color32::WHITE),
                        );
                    });
                }

                if !stats.file_stats.is_empty() {
                    ui.add_space(20.0);
                    ui.heading("Top Files by Commit Count");
                    ui.add_space(10.0);

                    egui::Grid::new("file_stats_grid")
                        .striped(true)
                        .spacing([10.0, 5.0])
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("#").color(egui::Color32::DARK_GRAY));
                            ui.label(
                                egui::RichText::new("Commits").color(egui::Color32::DARK_GRAY),
                            );
                            ui.label(egui::RichText::new("Size").color(egui::Color32::DARK_GRAY));
                            ui.label(egui::RichText::new("Path").color(egui::Color32::DARK_GRAY));
                            ui.end_row();

                            for (idx, file_stat) in stats.file_stats.iter().take(10).enumerate() {
                                let size_kb = file_stat.total_size as f64 / 1024.0;
                                let size_color = if file_stat.is_large {
                                    egui::Color32::from_rgb(255, 100, 100)
                                } else {
                                    egui::Color32::from_rgb(100, 150, 255)
                                };

                                ui.label(
                                    egui::RichText::new(format!("{}.", idx + 1))
                                        .color(egui::Color32::DARK_GRAY),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{}", file_stat.commit_count))
                                        .color(egui::Color32::from_rgb(0, 200, 200)),
                                );
                                ui.label(
                                    egui::RichText::new(format!("{:.2} KB", size_kb))
                                        .color(size_color),
                                );
                                let path_text = if file_stat.is_large {
                                    format!("{} [LARGE]", file_stat.path)
                                } else {
                                    file_stat.path.clone()
                                };
                                ui.label(
                                    egui::RichText::new(&path_text).color(egui::Color32::WHITE),
                                );
                                ui.end_row();
                            }
                        });

                    if stats.file_stats.len() > 10 {
                        ui.add_space(10.0);
                        ui.label(
                            egui::RichText::new(format!(
                                "... and {} more files",
                                stats.file_stats.len() - 10
                            ))
                            .color(egui::Color32::DARK_GRAY),
                        );
                    }
                }

                ui.add_space(30.0);
                ui.label(
                    egui::RichText::new("Press Esc or q to close").color(egui::Color32::DARK_GRAY),
                );
            });
        });
    }

    fn format_timestamp(timestamp: i64) -> String {
        use chrono::{Local, TimeZone};

        let dt = Local.timestamp_opt(timestamp, 0).unwrap();
        dt.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}

impl Default for StatsRenderer {
    fn default() -> Self {
        Self::new()
    }
}
