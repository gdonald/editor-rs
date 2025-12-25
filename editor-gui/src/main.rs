mod history_renderer;
mod input;
mod renderer;
mod stats_renderer;

use editor_core::editor::EditorState;
use eframe::egui;
use history_renderer::HistoryRenderer;
use input::{InputAction, InputHandler};
use renderer::Renderer;
use stats_renderer::StatsRenderer;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Editor-rs"),
        ..Default::default()
    };

    eframe::run_native(
        "Editor-rs",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::default()))),
    )
}

struct EditorApp {
    editor_state: EditorState,
    input_handler: InputHandler,
    renderer: Renderer,
    history_renderer: HistoryRenderer,
    stats_renderer: StatsRenderer,
    should_quit: bool,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            editor_state: EditorState::new(),
            input_handler: InputHandler::new(),
            renderer: Renderer::new(),
            history_renderer: HistoryRenderer::new(),
            stats_renderer: StatsRenderer::new(),
            should_quit: false,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        let is_history_browser_open = self.editor_state.is_history_browser_open();
        let is_history_stats_open = self.editor_state.is_history_stats_open();

        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        let action = if is_history_browser_open {
                            self.input_handler
                                .handle_history_browser_key_event(*key, modifiers)
                        } else if is_history_stats_open {
                            self.input_handler
                                .handle_history_stats_key_event(*key, modifiers)
                        } else {
                            self.input_handler.handle_key_event(*key, modifiers)
                        };

                        if let Some(action) = action {
                            self.handle_action(action);
                        }
                    }
                    egui::Event::Text(text) => {
                        if !is_history_browser_open && !is_history_stats_open {
                            if let Some(action) = self.input_handler.handle_text_input(text) {
                                self.handle_action(action);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });

        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            self.renderer.render_status_bar(ui, &self.editor_state);
        });

        if is_history_browser_open {
            let diff_content = self.editor_state.get_history_diff().ok().flatten();
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(30, 30, 30);
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);

                if let Some(history_browser) = self.editor_state.history_browser_mut() {
                    self.history_renderer
                        .render(ui, history_browser, diff_content);
                }
            });
        } else if is_history_stats_open {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(30, 30, 30);
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);

                if let Some(stats) = self.editor_state.history_stats() {
                    self.stats_renderer.render(ui, stats);
                }
            });
        } else {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.style_mut().visuals.extreme_bg_color = egui::Color32::from_rgb(30, 30, 30);
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);

                if let Some(scroll_delta) = self.renderer.render(ui, &self.editor_state, ctx) {
                    let lines_to_scroll = scroll_delta.abs();
                    for _ in 0..lines_to_scroll {
                        if scroll_delta > 0 {
                            let _ = self
                                .editor_state
                                .execute_command(editor_core::Command::MoveCursorUp);
                        } else {
                            let _ = self
                                .editor_state
                                .execute_command(editor_core::Command::MoveCursorDown);
                        }
                    }
                }
            });
        }
    }
}

impl EditorApp {
    fn handle_action(&mut self, action: InputAction) {
        match action {
            InputAction::Quit => {
                self.should_quit = true;
            }
            InputAction::Command(cmd) => {
                self.renderer.reset_cursor_blink();
                if let Err(e) = self.editor_state.execute_command(cmd) {
                    self.editor_state
                        .set_status_message(format!("Error: {}", e));
                }
            }
            InputAction::OpenFile => {
                self.editor_state
                    .set_status_message("Open file dialog not yet implemented".to_string());
            }
            InputAction::Search => {
                self.editor_state
                    .set_status_message("Search dialog not yet implemented".to_string());
            }
            InputAction::Replace => {
                self.editor_state
                    .set_status_message("Replace dialog not yet implemented".to_string());
            }
            InputAction::GotoLine => {
                self.editor_state
                    .set_status_message("Goto line dialog not yet implemented".to_string());
            }
            InputAction::SelectAll => {
                self.editor_state
                    .set_status_message("Select all not yet implemented".to_string());
            }
            InputAction::CloseHistoryStats => {
                self.editor_state.close_history_stats();
            }
            InputAction::SetBaseCommit => {
                if let Some(browser) = self.editor_state.history_browser() {
                    let index = browser.selected_index();
                    if let Err(e) = self
                        .editor_state
                        .execute_command(editor_core::Command::HistorySetBaseCommit(index))
                    {
                        self.editor_state
                            .set_status_message(format!("Error: {}", e));
                    }
                }
            }
        }
    }
}
