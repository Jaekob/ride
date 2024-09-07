// src/core/ui/code_editor.rs
use bevy_egui::{egui, EguiContexts};

pub fn draw_code_editor(egui_context: &mut EguiContexts) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.heading("Code Editor");
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut String::from("// Write your Rust code here...\n"))
                    .desired_rows(20),
            );
        });
    });
}
