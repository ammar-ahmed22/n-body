use bevy::prelude::*;


#[derive(Component, Default)]
pub struct Path {
  pub points: Vec<Vec2>,
  max_size: usize
}

impl Path {
  pub fn new(max_size: usize) -> Self {
    Self {
      points: Vec::new(),
      max_size
    }
  }

  pub fn add_point(&mut self, p: Vec2) {
    self.points.push(p);
    if self.points.len() > self.max_size {
      self.points.remove(0);
    }
  }

  pub fn size(&self) -> usize {
    return self.points.len();
  }

  pub fn capacity(&self) -> usize {
    return self.max_size;
  }

  pub fn to_vertices(&self) -> Vec<[f32; 3]> {
    self.points
      .iter()
      .map(|&pos| [pos.x, pos.y, 0.0])
      .collect()
  }

  pub fn to_indices(&self) -> Vec<u32> {
    let mut indices = Vec::new();
    for (i, _) in self.points.iter().enumerate() {
      if i > 0 {
        let prev = i - 1;
        indices.push(prev as u32);
        indices.push(i as u32);
      }
    }
    return indices;
  }
}