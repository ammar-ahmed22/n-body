use crate::particle::path::Path;
use crate::particle::Particle;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Bundle)]
pub struct ParticleBundle {
    shape_bundle: ShapeBundle,
    fill: Fill,
    stroke: Stroke,
    particle: Particle,
    path: Path,
}

impl ParticleBundle {
    pub fn new(particle: Particle, color: Color, stroke: Option<Stroke>) -> Self {
        ParticleBundle {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: particle.radius(),
                    ..default()
                }),
                ..default()
            },
            particle,
            path: Path::new(200),
            fill: Fill::color(color),
            stroke: if let Some(stroke) = stroke {
                stroke
            } else {
                Stroke::new(Color::WHITE, 1.0)
            },
        }
    }
}

impl Default for ParticleBundle {
    fn default() -> Self {
        Self {
            fill: Fill::color(Color::WHITE),
            stroke: Stroke::new(Color::WHITE, 1.0),
            ..default()
        }
    }
}
