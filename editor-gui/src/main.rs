mod input;

use editor_core::editor::EditorState;
use eframe::egui;
use input::{InputAction, InputHandler};

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
    should_quit: bool,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            editor_state: EditorState::new(),
            input_handler: InputHandler::new(),
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

        ctx.input(|i| {
            for event in &i.events {
                match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        modifiers,
                        ..
                    } => {
                        if let Some(action) = self.input_handler.handle_key_event(*key, modifiers) {
                            self.handle_action(action);
                        }
                    }
                    egui::Event::Text(text) => {
                        if let Some(action) = self.input_handler.handle_text_input(text) {
                            self.handle_action(action);
                        }
                    }
                    _ => {}
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Editor-rs GUI");

            ui.separator();

            let buffer = self.editor_state.current_buffer();
            ui.label(format!("Current buffer: {} lines", buffer.line_count()));
            let cursor = self.editor_state.cursor();
            ui.label(format!(
                "Line: {}, Col: {}",
                cursor.line + 1,
                cursor.column + 1
            ));
            ui.separator();

            let buffer_content = self.editor_state.current_buffer().content();
            ui.label(&buffer_content);
        });
    }
}

impl EditorApp {
    fn handle_action(&mut self, action: InputAction) {
        match action {
            InputAction::Quit => {
                self.should_quit = true;
            }
            InputAction::Command(cmd) => {
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
        }
    }
}
