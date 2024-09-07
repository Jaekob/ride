// src/core/ui/terminal.rs
use bevy_egui::{egui, EguiContexts};

pub fn draw_terminal(egui_context: &mut EguiContexts) {
    egui::TopBottomPanel::bottom("terminal")
        .resizable(true)
        .default_height(200.0)
        .min_height(100.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Terminal");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.label("cargo run\nCompiling...");
            });
        });
}
