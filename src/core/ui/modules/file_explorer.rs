use bevy::prelude::*;  // Ensure you're using the necessary Bevy prelude
use bevy_egui::{egui, EguiContexts};
use crate::core::ui::initiate_window::CurrentFileState;

pub fn draw_file_explorer(
    egui_context: &mut EguiContexts,
    mut file_state: ResMut<CurrentFileState>,  // Use ResMut correctly
) {
    egui::SidePanel::left("file_explorer")
        .resizable(false)
        .default_width(200.0)
        .show(egui_context.ctx_mut(), |ui| {
            ui.heading("Explorer");
            ui.separator();
            ui.label("Project");
            ui.collapsing("src", |ui| {
                if ui.button("main.rs").clicked() {
                    file_state.current_file = "main.rs".to_string();
                    file_state.explorer_folder = "src".to_string();
                }
                if ui.button("lib.rs").clicked() {
                    file_state.current_file = "lib.rs".to_string();
                    file_state.explorer_folder = "src".to_string();
                }
            });
        });
}
