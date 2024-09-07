use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use core::ui::initiate_window::{CurrentFileState, draw_ui, update_window_title, handle_window_drag};

mod core;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rust IDE".to_string(),
                resolution: (1280.0, 720.0).into(),
                decorations: false,
                ..Default::default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .insert_resource(CurrentFileState::default())
        .insert_resource(core::ui::initiate_window::WindowDragState::default())
        .add_systems(Update, (draw_ui, handle_window_drag, update_window_title))
        .run();
}
