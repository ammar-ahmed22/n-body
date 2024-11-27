use bevy::prelude::*;
use crate::particle;
use crate::utils;
use crate::resources;

pub fn update(
  mut query: Query<(&mut particle::path::Path, &particle::Particle)>,
  state: Res<resources::SimulationState>
) {
  if state.controls.show_path {
    for (mut path, particle) in query.iter_mut() {
      path.add_point(particle.position());
    }
  } else {
    for (mut path, _) in query.iter_mut() {
      path.reset();
    }
  }
}

#[derive(Component)]
pub struct ParticlePath;

pub fn render(
  mut commands: Commands,
  query: Query<&particle::path::Path>,
  particle_paths: Query<Entity, With<ParticlePath>>,
  state: Res<resources::SimulationState>
) {

  for entity in particle_paths.iter() {
    commands.entity(entity).despawn();
  }

  if state.controls.show_path {
    for path in query.iter() {
      if path.size() < 2 {
        continue;
      }
  
      commands.spawn((
        utils::LineBundle::path(&path.points, Color::rgba(1.0, 1.0, 1.0, 0.5), 1.0),
        ParticlePath
      ));
    }
  }
}