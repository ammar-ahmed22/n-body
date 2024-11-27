use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Path {
    pub points: Vec<Vec2>,
    max_size: usize,
}

impl Path {
    pub fn new(max_size: usize) -> Self {
        Self {
            points: Vec::new(),
            max_size,
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

    pub fn reset(&mut self) {
        if self.points.len() == 0 {
            return;
        }
        self.points = Vec::new();
    }
}
