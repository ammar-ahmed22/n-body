use crate::particle::Particle;
use crate::resources::constants;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn parameters(
    mut contexts: EguiContexts,
    mut numeric_constants: ResMut<constants::NumericConstants>,
) {
    egui::Window::new("🔧 Parameters")
        .resizable([true, false])
        .show(contexts.ctx_mut(), |ui| {
            ui.scope(|ui| {
                egui::Grid::new("params_grid")
                    .num_columns(2)
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
                    })
            })
        });
}

pub fn stats(
    mut contexts: EguiContexts,
    particles_query: Query<&Particle>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let mut num_particles = 0;
    for _ in particles_query.iter() {
        num_particles += 1;
    }

    let fps = match diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps_diagnostic| fps_diagnostic.average())
    {
        Some(fps) => fps,
        None => 0.0,
    };

    egui::Window::new("📊 Simulation Stats")
        .resizable([true, false])
        .show(contexts.ctx_mut(), |ui| {
            ui.scope(|ui| {
                egui::Grid::new("stats_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        let labels = vec![
                            ("Particles:", format!("{}", num_particles)),
                            ("FPS:", format!("{:.1}", fps)),
                        ];
                        for (label, value) in labels {
                            ui.label(label);
                            ui.label(value);
                            ui.end_row();
                        }
                    })
            })
        });
}
