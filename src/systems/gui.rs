use crate::particle::Particle;
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
                ("Particles:", format!("{}", particles)),
                ("FPS:", format!("{:.1}", fps)),
            ];
            for (label, value) in labels {
                ui.label(label);
                ui.label(value);
                ui.end_row();
            }
        });
}

fn params_section(ui: &mut egui::Ui, numeric_constants: &mut ResMut<constants::NumericConstants>) {
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

pub fn gui(
    mut contexts: EguiContexts,
    mut numeric_constants: ResMut<constants::NumericConstants>,
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
            params_section(ui, &mut numeric_constants);
        });
}
