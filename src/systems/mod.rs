pub mod gui;
pub mod input;
pub mod particles;
pub mod path;

use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
