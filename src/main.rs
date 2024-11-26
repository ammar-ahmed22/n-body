use std::process::exit;
use bevy::prelude::*;
use anyhow::Result;
use n_body::particle::Particle;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup, spawn_particles).chain())
        .add_systems(Update, (update_particles, update_particle_render).chain())
        .run()
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn handle_error<T>(result: Result<T>) -> T {
    match result {
        Ok(value) => value,
        Err(e) => {
            error!("{}", e);
            exit(1);
        }
    }
}

fn spawn_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut big = Particle::default();
    handle_error(big.set_radius(100.0));
    handle_error(big.set_density(0.01));

    let mut small1 = Particle::default();
    handle_error(small1.set_radius(10.0));
    handle_error(small1.set_density(0.0001));
    small1.set_pos(Vec2::new(-200.0, 0.0));
    small1.set_vel(Vec2::new(0.0, 110.0));


    let mut small2 = Particle::default();
    handle_error(small2.set_radius(10.0));
    handle_error(small2.set_density(0.0001));
    small2.set_pos(Vec2::new(250.0, 0.0));
    small2.set_vel(Vec2::new(0.0, -115.0));

    commands.spawn(small2.bundle(Color::GREEN, &mut meshes, &mut materials));
    commands.spawn(big.bundle(Color::ORANGE_RED, &mut meshes, &mut materials));
    commands.spawn(small1.bundle(Color::BLUE, &mut meshes, &mut materials));
}

fn update_particles(
    mut query: Query<(Entity, &mut Particle)>,
    time: Res<Time>
) {
    let particles: Vec<(Entity, Particle)> = query.iter().map(|(e, p)| (e, p.clone())).collect();
    for (entity_a, mut a) in query.iter_mut() {
        for (entity_b, b) in &particles {
            if entity_a == *entity_b {
                continue;
            }
            let a_pos = a.position();
            let b_pos = b.position();
            let dist = b_pos - a_pos;
            let combined_radii = b.radius() + a.radius();
            let d = dist.length();
            if d < combined_radii {
                // Position handling for collisions
                let overlap = 0.5 * (combined_radii - d);
                let n =(a_pos - b_pos) / d;
                a.set_pos(a_pos + n * overlap);

                // Velocity handling for collisions
                let rel_vel = a.velocity() - b.velocity();
                let col_normal = rel_vel.dot(n);
                if col_normal <= 0.0 {
                    let a_inv_mass = 1.0 / a.mass();
                    let b_inv_mass = 1.0 / b.mass();
                    let restitution = 0.8_f32;
                    let impulse_mag = (-(1.0 + restitution) * col_normal) / (a_inv_mass + b_inv_mass);
                    let impulse = n * impulse_mag;
                    let a_vel = a.velocity();
                    a.set_vel(a_vel + impulse * a_inv_mass);
                }
                info!("collision!!!");
            }
            const G: f32 = 66.7;
            let mag = (G * a.mass() * b.mass()) / (d.powf(2.0));
            // info!("magnitude: {}, massA: {}, massB: {}", mag, a.mass(), b.mass());
            let force = dist.normalize() * mag;
            a.add_force(force);
        }
        // a.add_force(Vec2::new(0.0, -9.8));
        a.update(time.delta_seconds());
    }
}

fn update_particle_render(
    mut query: Query<(&mut Transform, &Particle)>
) {
    for (mut transform, particle) in query.iter_mut() {
        transform.translation = particle.position().extend(0.0);
    }
}
