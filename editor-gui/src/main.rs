use editor_core::command::Command;
use editor_core::editor::EditorState;
use eframe::egui;

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
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            editor_state: EditorState::new(),
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Editor-rs GUI");

            if ui.button("New Buffer").clicked() {
                let _ = self.editor_state.execute_command(Command::New);
            }

            ui.separator();

            let buffer = self.editor_state.current_buffer();
            let mut content = buffer.content();
            ui.label(format!("Current buffer: {} lines", buffer.line_count()));
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut content)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY),
                );
            });
        });
    }
}
