use bevy::prelude::*;
pub mod constants;
pub mod input;
pub mod controls;


#[derive(Resource)]
pub struct SimulationState {
  pub numeric_constants: constants::NumericConstants,
  pub controls: controls::Controls,
}

impl SimulationState {
  pub fn new() -> Self {
    Self {
      numeric_constants: constants::NumericConstants::new(),
      controls: Default::default()
    }
  }
}