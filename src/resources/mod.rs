use bevy::prelude::*;
pub mod constants;
pub mod controls;
pub mod input;

#[derive(Resource)]
pub struct SimulationState {
    pub numeric_constants: constants::NumericConstants,
    pub controls: controls::Controls,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self::new()
    }
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            numeric_constants: constants::NumericConstants::new(),
            controls: Default::default(),
        }
    }
}
