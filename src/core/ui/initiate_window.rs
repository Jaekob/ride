use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_winit::WinitWindows;
use crate::core::ui::modules::code_editor::draw_code_editor;
use crate::core::ui::modules::file_explorer::draw_file_explorer;
use crate::core::ui::modules::terminal::draw_terminal;

#[derive(Resource, Default)]
pub struct WindowDragState {
    pub is_dragging: bool,
    pub initial_cursor_position: Option<Vec2>,
}

#[derive(Default, Resource)]  // Mark CurrentFileState as a Bevy resource
pub struct CurrentFileState {
    pub current_file: String,
    pub explorer_folder: String,
}

// Function to handle window dragging
pub fn handle_window_drag(
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut window_drag_state: ResMut<WindowDragState>,
    winit_windows: NonSend<WinitWindows>,
) {
    if window_drag_state.is_dragging {
        if let Ok((window_entity, _)) = windows.get_single() {
            if let Some(winit_window) = winit_windows.get_window(window_entity) {
                for event in cursor_moved_events.read() {
                    if let Some(last_position) = window_drag_state.initial_cursor_position {
                        let delta = event.position - last_position;

                        if let Ok(current_position) = winit_window.outer_position() {
                            let new_position = winit::dpi::LogicalPosition::new(
                                current_position.x + delta.x as i32,
                                current_position.y + delta.y as i32,
                            );
                            winit_window.set_outer_position(new_position);
                        }
                    }
                    window_drag_state.initial_cursor_position = Some(event.position);
                }
            }
        }
    }
}

// Function to draw the title bar
pub fn draw_title_bar(
    egui_context: &mut EguiContexts,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    window_drag_state: &mut ResMut<WindowDragState>,
    winit_windows: NonSend<WinitWindows>,
    file_state: &mut ResMut<CurrentFileState>,
) {
    egui::TopBottomPanel::top("title_bar").show(egui_context.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            ui.label("*");

            egui::menu::bar(ui, |ui| {
                // File Menu
                ui.menu_button("File", |ui| {
                    if ui.button("New File").clicked() {
                        file_state.current_file = "new_file.rs".to_string();
                    }
                    if ui.button("Open File").clicked() {
                        file_state.current_file = "open_file.rs".to_string();
                    }
                    if ui.button("Save").clicked() {}
                });

                // Additional menus (Edit, Selection, etc.)
                ui.menu_button("Edit", |ui| { /* Edit options here */ });
                ui.menu_button("Selection", |ui| { /* Selection options here */ });
                ui.menu_button("View", |ui| { /* View options here */ });
                ui.menu_button("Go", |ui| { /* Go options here */ });
                ui.menu_button("Run", |ui| { /* Run options here */ });
                ui.menu_button("Terminal", |ui| { /* Terminal options here */ });
                ui.menu_button("Help", |ui| { /* Help options here */ });

                // Window Control Buttons (Close, Maximize, Minimize)
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                    if ui.button("❌").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("🗖").clicked() {
                        if let Ok((window_entity, window)) = windows.get_single() {
                            if window.mode == WindowMode::Windowed {
                                if let Some(winit_window) = winit_windows.get_window(window_entity) {
                                    winit_window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(None)));
                                }
                            }
                        }
                    }
                    if ui.button("-").clicked() {
                        if let Ok((window_entity, _)) = windows.get_single() {
                            if let Some(winit_window) = winit_windows.get_window(window_entity) {
                                winit_window.set_minimized(true);
                            }
                        }
                    }
                });
            });

            // Handle window dragging
            if ui.input(|i| i.pointer.any_pressed() && i.pointer.interact_pos().is_some()) {
                window_drag_state.is_dragging = true;
                window_drag_state.initial_cursor_position = ui.input(|i| i.pointer.interact_pos())
                    .map(|p| Vec2::new(p.x, p.y));
            } else if ui.input(|i| i.pointer.any_released()) {
                window_drag_state.is_dragging = false;
            }
        });
    });
}

// Function to update the window title
pub fn update_window_title(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    file_state: Res<CurrentFileState>,
) {
    if let Ok(mut window) = windows.get_single_mut() {
        let mut title_parts = vec![];

        if !file_state.current_file.is_empty() {
            title_parts.push(file_state.current_file.clone());
        }

        if !file_state.explorer_folder.is_empty() {
            title_parts.push(file_state.explorer_folder.clone());
        }

        title_parts.push("RIDE".to_string());
        window.title = title_parts.join(" - ");
    }
}

// Function to draw the entire UI (file explorer, code editor, terminal)
pub fn draw_ui(
    mut egui_context: EguiContexts,
    windows: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut window_drag_state: ResMut<WindowDragState>,
    mut file_state: ResMut<CurrentFileState>,
    winit_windows: NonSend<WinitWindows>,
) {
    draw_title_bar(&mut egui_context, windows, &mut window_drag_state, winit_windows, &mut file_state);
    draw_file_explorer(&mut egui_context, file_state);
    draw_code_editor(&mut egui_context);
    draw_terminal(&mut egui_context);
}
