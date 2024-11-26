use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use n_body::resources::constants;
use n_body::systems;

fn main() {
    App::new()
        .insert_resource(constants::NumericConstants::new())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(
            Startup,
            (systems::setup, systems::particles::spawn_initial).chain(),
        )
        .add_systems(Update, systems::gui::gui)
        .add_systems(
            Update,
            (systems::particles::update, systems::particles::render).chain(),
        )
        .run()
}
