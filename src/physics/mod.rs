use crate::particle::Particle;
use bevy::prelude::*;

/// Integrates a vector with respect to time given it's first derivative
/// Uses semi-implicit Euler integration
///
/// ### Arguments
/// - `v` The vector to integrate
/// - `dv` The first derivative of the vector
/// - `dt` The timestep
///
/// ### Returns
/// `Vec2` The integrated vector
pub fn integrate(v: Vec2, dv: Vec2, dt: f32) -> Vec2 {
    v + dv * dt
}

/// Updates a particle's physics with semi-implicit Euler integration
///
/// ### Arguments
/// - `p` The particle to update
/// - `dt` The timestep
pub fn update_particle(p: &mut Particle, dt: f32) {
    p.set_vel(integrate(p.velocity(), p.acceleration(), dt));
    p.set_pos(integrate(p.position(), p.velocity(), dt));
    p.set_acc(Vec2::ZERO);
}

/// Checks if two particles are intersection
///
/// ### Arguments
/// - `a` Particle
/// - `b` Particle
///
/// ### Returns
/// `bool` Are the particles intersecting
pub fn is_intersecting(a: &Particle, b: &Particle) -> bool {
    let dist_sq = (a.position() - b.position()).length_squared();
    let combined_radii = a.radius() + b.radius();
    if dist_sq < combined_radii * combined_radii {
        return true;
    }
    return false;
}

/// Resolves the position of a particle to remove intersections  
///   
/// ***NOTE***: does not check for intersection
///
/// ### Arguments
/// - `a` The particle that will be moved
/// - `b` The particle that is intersecting
pub fn resolve_intersection(a: &mut Particle, b: &Particle) {
    let d = b.position() - a.position();
    let dist = d.length();
    let combined_radii = a.radius() + b.radius();
    let overlap = 0.5 * (combined_radii - dist);
    let normal = d / dist;

    let prev = a.position();
    a.set_pos(prev + normal * overlap);
}

/// Resolves the velocity of a particle after an elastic collision
///
/// ***NOTE***: does not check for intersection
///
/// ### Arguments
/// - `a` The particle that will be moved
/// - `b` The particle that is colliding
/// - `r` The co-efficient of restitution [0.0 - 1.0] (values outside of the range may produce unexpected results)
pub fn resolve_collision(a: &mut Particle, b: &Particle, r: f32) {
    if r < 0.0 || r > 1.0 {
        warn!("Co-efficient of restitution, 'r' = {:.1}, is outside of the range, [0.0 - 1.0]. May produce unexpected results!", r);
    }
    let normal = (a.position() - b.position()).normalize();
    let rel_vel = a.velocity() - b.velocity();
    let col_normal = rel_vel.dot(normal);
    if col_normal < 0.0 {
        let a_inv_mass = 1.0 / a.mass();
        let b_inv_mass = 1.0 / b.mass();

        let impulse_mag = (-(1.0 + r) * col_normal) / (a_inv_mass + b_inv_mass);
        let impulse = normal + impulse_mag;

        let prev = a.velocity();
        a.set_vel(prev + impulse * a_inv_mass);
    }
}

/// Attracts a particle to another due to the gravitational force
///
/// ### Arguments
/// - `a` The particle that will be attracted (force added)
/// - `b` The particle that is atracting
pub fn attract(a: &mut Particle, b: &Particle, g: f32) {
    let d = b.position() - a.position();
    let dist = d.length();
    let mag = (g * a.mass() * b.mass()) / (dist.powf(2.0));
    let f = d.normalize() * mag;
    a.add_force(f);
}

pub fn generate_particle_grid(center: Vec2, n: usize, radius: f32, density: f32) -> Vec<Particle> {
    let mut particles: Vec<Particle> = Vec::new();
    let n_f = n as f32;
    let total_size = (n_f * 2.0 * radius) + 2.0 * radius;
    let start = Vec2::new(
        center.x - (total_size / 2.0) + radius,
        center.x - (total_size / 2.0) + radius,
    );
    for i in 0..n {
        for j in 0..n {
            let x = start.x + 2.0 * radius * (j as f32);
            let y = start.y + 2.0 * radius * (i as f32);
            let mut p = Particle::default();
            p.set_radius(radius).unwrap();
            p.set_density(density).unwrap();
            p.set_pos(Vec2::new(x, y));
            particles.push(p);
        }
    }

    return particles;
}

pub fn orbital_velocity(satellite: Vec2, planet: Vec2, planet_mass: f32, g: f32) -> Vec2 {
    let r = satellite - planet;
    let mag = ((g * planet_mass) / r.length()).sqrt();
    let t = Vec2::new(-r.y, r.x).normalize();
    return t * mag;
}
