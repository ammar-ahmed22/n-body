use bevy::prelude::*;
use bevy::sprite::{ MaterialMesh2dBundle, Mesh2dHandle };
use crate::particle::Particle;


#[derive(Bundle)]
pub struct ParticleBundle {
    mesh_bundle: MaterialMesh2dBundle<ColorMaterial>,
    particle: Particle,
}

impl ParticleBundle {
    pub fn new(
        particle: Particle,
        color: Color,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        ParticleBundle {
            mesh_bundle: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Mesh::from(Circle::new(particle.radius)))),
                material: materials.add(ColorMaterial::from(color)),
                transform: Transform::from_translation(particle.position().extend(0.0)),
                ..default()
            },
            particle
        }
    }
}