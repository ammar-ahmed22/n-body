use bevy::prelude::*;


#[derive(Component)]
pub struct Line;

pub fn line(commands: &mut Commands, start: Vec2, end: Vec2, stroke: f32, color: Color) {
  let dir = end - start;
  let len = dir.length();
  let angle = dir.y.atan2(dir.x);

  let translation = ((start + end) / 2.).extend(0.0);
  let rotation = Quat::from_rotation_z(angle);
  let scale = Vec2::new(len, stroke).extend(1.0);

  commands.spawn((
    SpriteBundle {
      transform: Transform {
        translation,
        rotation,
        scale,
        ..default()
      },
      sprite: Sprite {
        color,
        ..default()
      },
      ..default()
    },
    Line
  ));

}
