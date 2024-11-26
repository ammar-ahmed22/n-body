use bevy::prelude::*;
use std::ops::RangeInclusive;

pub struct NumericConstant {
    pub value: f32,
    pub range: RangeInclusive<f32>,
    pub speed: f32,
    pub label: String,
}

impl NumericConstant {
    pub fn new(value: f32, range: RangeInclusive<f32>, speed: f32, label: &str) -> Self {
        return Self {
            value,
            range,
            speed,
            label: label.to_string(),
        };
    }
}

#[derive(Resource)]
pub struct NumericConstants {
    pub g: NumericConstant,
    pub restitution: NumericConstant,
}

impl NumericConstants {
    pub fn new() -> Self {
        return Self {
            g: NumericConstant::new(66.7, 0.0..=100.0, 0.1, "Gravitational Force Constant"),
            restitution: NumericConstant::new(0.8, 0.0..=1.0, 0.01, "Elastic Restitution"),
        };
    }

    pub fn to_vec_mut(&mut self) -> Vec<&mut NumericConstant> {
        let mut vec = vec![];
        vec.push(&mut self.g);
        vec.push(&mut self.restitution);
        return vec;
    }
}
