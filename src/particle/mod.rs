mod bundle;

use bevy::prelude::*;
use anyhow::Result;
use crate::particle::bundle::ParticleBundle;

#[derive(Component, Clone)]
pub struct Particle {
    pos: Vec2,
    vel: Vec2,
    acc: Vec2,
    radius: f32,
    density: f32,
}

impl Default for Particle {
    fn default() -> Self {
        Self {
            pos: Vec2::default(),
            vel: Vec2::default(),
            acc: Vec2::default(),
            density: 1.0,
            radius: 0.0
        }
    }
}

impl Particle {
    /// Set the position
    /// 
    /// ### Arguments
    /// - pos `Vec2`: position to set to
    pub fn set_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    /// Set the velocity
    /// 
    /// ### Arguments
    /// - vel `Vec2`: velocity to set to
    pub fn set_vel(&mut self, vel: Vec2) {
        self.vel = vel;
    }

    /// Set the acceleration
    /// 
    /// ### Arguments
    /// - acc `Vec2`: acceleration to set to
    pub fn set_acc(&mut self, acc: Vec2) {
        self.acc = acc;
    }

    /// Set the radius
    /// 
    /// ### Arguments
    /// - rad `f32`: radius to set to
    /// ### Returns
    /// - `Result<()>`: `Ok` if radius is in the allowed range, `Err` if not
    pub fn set_radius(&mut self, rad: f32) -> Result<()> {
        if rad < 0.0 {
            return Err(anyhow::anyhow!("Radius must be greater than 0!"));
        }
        self.radius = rad;
        return Ok(());
    }

    /// Set the density
    /// 
    /// ### Arguments
    /// - den `f32`: density to set to
    /// ### Returns
    /// - `Result<()>`: `Ok` if radius is in the allowed range, `Err` if not
    pub fn set_density(&mut self, den: f32) -> Result<()> {
        if den < 0.0 {
            return Err(anyhow::anyhow!("Density must be greater than 0!"));
        }
        self.density = den;
        return Ok(())
    }

    /// Gets the mass
    pub fn mass(&self) -> f32 {
        return self.density * 4.0 * std::f32::consts::FRAC_PI_3 * self.radius.powf(3.0)
    }

    /// Sets the mass by altering the density
    /// 
    /// ### Arguments
    /// - mass `f32`: mass to set to
    pub fn set_mass_with_density(&mut self, mass: f32) {
        self.density = mass / (4.0 * std::f32::consts::FRAC_PI_3 * self.radius.powf(3.0))
    }

    /// Sets the mass by altering the radius
    /// 
    /// ### Arguments
    /// - mass `f32`: mass to set to
    pub fn set_mass_with_radius(&mut self, mass: f32) {
        self.radius = (mass / (self.density * 4.0 * std::f32::consts::FRAC_PI_3)).powf(1.0/3.0);
    }

    /// Creates a ParticleBundle
    /// 
    /// ### Arguments
    /// - color `Color`: The color to render the particle as
    /// - meshes `&mut ResMut<Assets<Mesh>>`: Mesh resource
    /// - materials: `&mut ResMut<Assets<ColorMaterial>>: Materials resource
    pub fn bundle(&self, color: Color, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) -> ParticleBundle {
        return ParticleBundle::new(self.clone(), color, meshes, materials)
    }

    /// Updates the particles position using Semi-implicit Euler integration
    pub fn update(&mut self, dt: f32) {
        self.vel += self.acc * dt;
        self.pos += self.vel * dt;
        self.acc = Vec2::ZERO;
    }

    /// Gets the position
    pub fn position(&self) -> Vec2 {
        return self.pos;
    }

    /// Gets the velocity
    pub fn velocity(&self) -> Vec2 {
      return self.vel;
    }

    /// Gets the acceleration
    pub fn acceleration(&self) -> Vec2 {
      return self.acc;
    }

    /// Gets the radius
    pub fn radius(&self) -> f32 {
      return self.radius;
    }

    /// Adds a force through Newton's Second Law (F = m a)
    /// 
    /// i.e. If F = m a, therefore, a = F / m
    pub fn add_force(&mut self, f: Vec2) {
      self.acc += f / self.mass();
    }
}