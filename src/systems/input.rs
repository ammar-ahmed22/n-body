use crate::resources::input;
use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::EguiContexts;

fn window2world(window: &Window, camera: &Transform, pos: &Vec2) -> Vec2 {
    let norm = Vec3::new(
        pos.x - window.width() / 2.,
        (pos.y - window.height() / 2.) * -1.,
        0.,
    );
    (*camera * norm).truncate()
}

pub fn mouse_hold(
    mut mouse_state: ResMut<input::MouseState>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_button_events: EventReader<MouseButtonInput>,
    mut contexts: EguiContexts,
    camera_transform: Query<&Transform, With<Camera2d>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let camera_transform = camera_transform.single();

    if !contexts.ctx_mut().is_pointer_over_area() {
        for event in mouse_button_events.read() {
            match event.button {
                MouseButton::Left => {
                    if event.state == ButtonState::Pressed {
                        if let Some(cursor_pos) = window.cursor_position() {
                            mouse_state.click =
                                Some(window2world(window, camera_transform, &cursor_pos));
                        }
                        mouse_state.is_held = true;
                        mouse_state.dragging = None;
                        mouse_state.release = None;
                    } else if event.state == ButtonState::Released {
                        if let Some(cursor_pos) = window.cursor_position() {
                            mouse_state.release =
                                Some(window2world(window, camera_transform, &cursor_pos));
                        }
                        mouse_state.is_held = false;
                    }
                }
                _ => {}
            };
        }
    }

    if mouse_state.is_held {
        for event in cursor_moved_events.read() {
            mouse_state.dragging = Some(window2world(window, camera_transform, &event.position));
        }
    }
}
