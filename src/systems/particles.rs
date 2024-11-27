use crate::error::handle_error;
use crate::particle::Particle;
use crate::physics;
use crate::resources;
use crate::resources::input;
use crate::utils;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

// Constants
// const G: f32 = 66.7;
// const RESTITUTION: f32 = 0.8;

// Sizes
const BIG_DENSITY: f32 = 0.01;
const BIG_RAD: f32 = 100.0;

const SMALL_DENSITY: f32 = 0.0001;
const SMALL_RAD: f32 = 10.0;

pub fn spawn_initial(mut commands: Commands) {
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

    commands.spawn(small2.bundle(Color::WHITE, None));
    commands.spawn(big.bundle(Color::WHITE, None));
    commands.spawn(small1.bundle(Color::WHITE, None));
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
            handle_error(p.set_radius(SMALL_RAD));
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

pub fn update(
    mut query: Query<(Entity, &mut Particle)>,
    time: Res<Time>,
    state: Res<resources::SimulationState>,
) {
    let particles: Vec<(Entity, Particle)> = query.iter().map(|(e, p)| (e, p.clone())).collect();
    for (entity_a, mut a) in query.iter_mut() {
        for (entity_b, b) in &particles {
            if entity_a == *entity_b {
                continue;
            }
            if physics::is_intersecting(&a, b) {
                physics::resolve_intersection(&mut a, b);
                physics::resolve_collision(&mut a, b, state.numeric_constants.restitution.value);
            }
            physics::attract(&mut a, b, state.numeric_constants.g.value);
        }
        physics::update_particle(&mut a, time.delta_seconds());
    }
}

pub fn render(
    mut query: Query<(&mut Transform, &Particle, &mut Fill, &mut Stroke)>,
    state: Res<resources::SimulationState>,
) {
    for (mut transform, particle, mut fill, mut stroke) in query.iter_mut() {
        let pos = particle.position();
        transform.translation = pos.extend(0.0);
        *fill = Fill::color(state.controls.particle_color);
        *stroke = Stroke::new(state.controls.particle_stroke, 1.0);
    }
}
