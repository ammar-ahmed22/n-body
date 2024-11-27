use crate::particle::Particle;
use crate::resources;
use crate::resources::constants;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

fn stats_section(
    ui: &mut egui::Ui,
    particles_query: &Query<&Particle>,
    diagnostics: &Res<DiagnosticsStore>,
) {
    let particles = particles_query.iter().count();
    let fps = match diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        Some(fps) => fps.average().unwrap_or(0.0),
        None => 0.0,
    };
    egui::Grid::new("sim_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            let labels = vec![
                ("# of Particles:", format!("{}", particles)),
                ("FPS:", format!("{:.1}", fps)),
            ];
            for (label, value) in labels {
                ui.label(label);
                ui.label(value);
                ui.end_row();
            }
        });
}

fn params_section(ui: &mut egui::Ui, numeric_constants: &mut constants::NumericConstants) {
    egui::Grid::new("params_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            for constant in numeric_constants.to_vec_mut() {
                ui.label(constant.label.clone());
                ui.add(
                    egui::DragValue::new(&mut constant.value)
                        .speed(constant.speed)
                        .clamp_range(constant.range.clone()),
                );
                ui.end_row();
            }
        });
}

fn color_input(ui: &mut egui::Ui, color: &mut Color) {
    let [r, g, b, a] = color.as_rgba_u8();
    let mut input_color = egui::Color32::from_rgba_unmultiplied(r, g, b, a);
    ui.color_edit_button_srgba(&mut input_color);
    let [nr, ng, nb, na] = input_color.to_array();
    *color = Color::rgba_u8(nr, ng, nb, na);
}

fn controls_section(ui: &mut egui::Ui, controls: &mut resources::controls::Controls) {
    egui::Grid::new("control_grid")
        .num_columns(2)
        .spacing([40.0, 4.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Trace Path");
            ui.checkbox(&mut controls.show_path, "");
            ui.end_row();
            ui.label("Particle Color");
            color_input(ui, &mut controls.particle_color);
            ui.end_row();
            ui.label("Particle Stroke");
            color_input(ui, &mut controls.particle_stroke);
            ui.end_row();
        });
}

pub fn gui(
    mut contexts: EguiContexts,
    mut state: ResMut<resources::SimulationState>,
    particles_query: Query<&Particle>,
    diagnostics: Res<DiagnosticsStore>,
) {
    egui::Window::new("n-body")
        .resizable([true, false])
        .default_width(280.0)
        .show(contexts.ctx_mut(), |ui| {
            ui.heading("Simulation");
            stats_section(ui, &particles_query, &diagnostics);
            ui.separator();
            ui.heading("Parameters");
            params_section(ui, &mut state.numeric_constants);
            ui.separator();
            ui.heading("Controls");
            controls_section(ui, &mut state.controls);
        });
}

// pub fn absorb_gui_inputs(
//     // mut mouse: ResMut<ButtonInput<MouseButton>>,
//     mut event_reader: EventReader<MouseButtonInput>,
//     mut event_writer: EventWriter<MouseButtonInput>,
//     mut contexts: EguiContexts
// ) {
//     if !contexts.ctx_mut().is_pointer_over_area() {
//         for event in event_reader.read() {
//             event_writer.send(event.clone());
//         }
//     }
// }
