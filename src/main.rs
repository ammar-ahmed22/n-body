use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_egui::EguiPlugin;
use n_body::resources::input;
use n_body::resources;
use n_body::systems;

fn main() {
    App::new()
        .insert_resource(resources::SimulationState::new())
        .insert_resource(input::MouseState::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(ShapePlugin)
        .add_systems(
            Startup,
            (systems::setup, systems::particles::spawn_initial).chain(),
        )
        .add_systems(Update, systems::gui::gui)
        .add_systems(Update, (systems::input::mouse_hold, systems::particles::spawn_input))
        .add_systems(Update, (systems::path::update, systems::path::render).chain())
        .add_systems(
            Update,
            (systems::particles::update, systems::particles::render).chain(),
        )
        .run()
}
