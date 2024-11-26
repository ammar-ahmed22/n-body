use bevy::prelude::*;


#[derive(Resource, Default, Debug)]
pub struct MouseState {
  pub click: Option<Vec2>,
  pub release: Option<Vec2>,
  pub dragging: Option<Vec2>,
  pub is_held: bool,
  pub hold_dur: f32,
}

