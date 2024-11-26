use bevy::prelude::*;
use n_body::systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (systems::setup, systems::particles::spawn).chain())
        .add_systems(
            Update,
            (systems::particles::update, systems::particles::render).chain(),
        )
        .run()
}
