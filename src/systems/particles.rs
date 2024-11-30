use crate::error::handle_error;
use crate::particle::Particle;
use crate::physics;
use crate::resources;
use crate::resources::input;
use crate::utils;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// Sizes
const RAD: f32 = 5.0;
const BIG_DENSITY: f32 = 1e4; // extremely dense (simulating a sun, large mass)
const SMALL_DENSITY: f32 = 0.1; // smaller (simulating a planet, smaller mass (relatively))
const MED_DENSITY: f32 = 1.0; // a larger planet

const ORBIT_SPACING: f32 = 50.0;
const ORBIT_START: f32 = 150.0;
const NUM_ORBITS: usize = 2;

pub fn spawn_initial(mut commands: Commands, state: Res<resources::SimulationState>) {
    let mut big = Particle::default();
    handle_error(big.set_radius(RAD));
    handle_error(big.set_density(BIG_DENSITY));

    let mut side: f32 = 1.0;
    for i in 1..NUM_ORBITS + 1 {
        let mut p = Particle::default();
        p.set_radius(RAD).unwrap();
        p.set_density(MED_DENSITY).unwrap();
        let pos = Vec2::new(ORBIT_START * side + ORBIT_SPACING * (i as f32) * side, 0.0);
        let vel = physics::orbital_velocity(
            pos,
            big.position(),
            big.mass(),
            state.numeric_constants.g.value,
        );
        p.set_pos(pos);
        p.set_vel(vel);
        if side == 1.0 {
            side = -1.0;
        } else {
            side = 1.0;
        }
        commands.spawn(p.bundle(Color::WHITE, None));
    }

    commands.spawn(big.bundle(Color::RED, None));
}

#[derive(Component)]
pub struct SpawnIndicator;

pub fn spawn_input(
    mut commands: Commands,
    mut mouse_state: ResMut<input::MouseState>,
    spawn_indicators: Query<Entity, With<SpawnIndicator>>,
) {
    if let Some(released) = mouse_state.release {
        if let Some(clicked) = mouse_state.click {
            let mut p = Particle::default();
            handle_error(p.set_radius(RAD));
            handle_error(p.set_density(SMALL_DENSITY));
            p.set_pos(clicked);
            let vel = clicked - released;
            p.set_vel(vel);
            commands.spawn(p.bundle(Color::WHITE, None));
            *mouse_state = input::MouseState::default();
        }
    }
    // Despawn any old lines (should only be 1)
    for entity in spawn_indicators.iter() {
        commands.entity(entity).despawn();
    }

    if let Some(drag) = mouse_state.dragging {
        if let None = mouse_state.release {
            if let Some(clicked) = mouse_state.click {
                commands.spawn((
                    utils::LineBundle::new(clicked, drag, Color::WHITE, 1.0),
                    SpawnIndicator,
                ));
            }
        }
    }
}
const SUBSTEPS: usize = 1;
pub fn update(
    mut query: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
    state: Res<resources::SimulationState>,
) {
    let particles: Vec<(Entity, Particle)> = query.iter().map(|(e, p)| (e, p.clone())).collect();
    let dt = time.delta_seconds();
    let sub_dt = dt / (SUBSTEPS as f32);
    for _ in 0..SUBSTEPS {
        for (entity_a, mut a) in query.iter_mut() {
            for (entity_b, b) in &particles {
                if entity_a == *entity_b {
                    continue;
                }
                if physics::is_intersecting(&a, b) {
                    physics::resolve_intersection(&mut a, b);
                    physics::resolve_collision(
                        &mut a,
                        b,
                        state.numeric_constants.restitution.value,
                    );
                }
                physics::attract(&mut a, b, state.numeric_constants.g.value);
            }
            physics::update_particle(&mut a, sub_dt);
        }
    }
}

pub fn render(mut query: Query<(&mut Transform, &Particle, &mut Fill)>) {
    let mut min_mass = f32::MAX;
    let mut max_mass = f32::MIN;
    for (_, particle, _) in query.iter() {
        min_mass = min_mass.min(particle.mass());
        max_mass = max_mass.max(particle.mass());
    }
    for (mut transform, particle, mut fill) in query.iter_mut() {
        let pos = particle.position();
        transform.translation = pos.extend(0.0);
        
        // Set the fill color according to mass (most massive is black, least massive is white)
        let normalized_mass = utils::math::normalize(particle.mass(), min_mass, max_mass, 0.0, 1.0);
        *fill = Fill::color(Color::rgb(
                1.0 - normalized_mass,
                1.0 - normalized_mass,
                1.0 - normalized_mass,
        ));
    }
}
