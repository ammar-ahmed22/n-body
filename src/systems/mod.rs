pub mod gui;
pub mod particles;
pub mod input;

use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
