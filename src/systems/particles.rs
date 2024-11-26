use crate::error::handle_error;
use crate::particle::Particle;
use crate::resources::constants;
use bevy::prelude::*;

// Constants
// const G: f32 = 66.7;
// const RESTITUTION: f32 = 0.8;

// Sizes
const BIG_DENSITY: f32 = 0.01;
const BIG_RAD: f32 = 100.0;

const SMALL_DENSITY: f32 = 0.0001;
const SMALL_RAD: f32 = 10.0;

pub fn spawn_initial(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut big = Particle::default();
    handle_error(big.set_radius(BIG_RAD));
    handle_error(big.set_density(BIG_DENSITY));

    let mut small1 = Particle::default();
    handle_error(small1.set_radius(SMALL_RAD));
    handle_error(small1.set_density(SMALL_DENSITY));
    small1.set_pos(Vec2::new(-200.0, 0.0));
    small1.set_vel(Vec2::new(0.0, 110.0));

    let mut small2 = Particle::default();
    handle_error(small2.set_radius(SMALL_RAD));
    handle_error(small2.set_density(SMALL_DENSITY));
    small2.set_pos(Vec2::new(250.0, 0.0));
    small2.set_vel(Vec2::new(0.0, -115.0));

    commands.spawn(small2.bundle(Color::WHITE, &mut meshes, &mut materials));
    commands.spawn(big.bundle(Color::WHITE, &mut meshes, &mut materials));
    commands.spawn(small1.bundle(Color::WHITE, &mut meshes, &mut materials));
}

pub fn update(
    mut query: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
    constants: Res<constants::NumericConstants>,
) {
    let particles: Vec<(Entity, Particle)> = query.iter().map(|(e, p)| (e, p.clone())).collect();
    for (entity_a, mut a) in query.iter_mut() {
        for (entity_b, b) in &particles {
            if entity_a == *entity_b {
                continue;
            }
            a.handle_collision(b, constants.restitution.value);
            a.handle_attraction(b, constants.g.value);
        }
        a.update(time.delta_seconds());
    }
}

pub fn render(mut query: Query<(&mut Transform, &Particle)>) {
    for (mut transform, particle) in query.iter_mut() {
        transform.translation = particle.position().extend(0.0);
    }
}
