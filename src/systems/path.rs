use bevy::prelude::*;
use crate::particle;
use crate::utils;

pub fn update(
  mut query: Query<(&mut particle::path::Path, &particle::Particle)>,
) {
  for (mut trail, particle) in query.iter_mut() {
    trail.add_point(particle.position());
  }
}

#[derive(Component)]
pub struct ParticlePath;

pub fn render(
  mut commands: Commands,
  query: Query<&particle::path::Path>,
  particle_paths: Query<Entity, With<ParticlePath>>
) {

  for entity in particle_paths.iter() {
    commands.entity(entity).despawn();
  }

  for path in query.iter() {
    if path.size() < 2 {
      continue;
    }

    commands.spawn((
      utils::LineBundle::path(&path.points, Color::WHITE, 1.0),
      ParticlePath
    ));
  }
}