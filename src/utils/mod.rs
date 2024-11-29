pub mod math;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Bundle)]
pub struct LineBundle {
    shape_bundle: ShapeBundle,
    stroke: Stroke,
}

impl LineBundle {
    pub fn new(start: Vec2, end: Vec2, color: Color, stroke: f32) -> Self {
        Self {
            shape_bundle: ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Line(start, end)),
                ..default()
            },
            stroke: Stroke::new(color, stroke),
        }
    }

    pub fn path(points: &Vec<Vec2>, color: Color, stroke: f32) -> Self {
        let mut builder = PathBuilder::new();
        builder.move_to(points[0]);
        for window in points.windows(3) {
            // let p0 = window[0];
            let p1 = window[1];
            let p2 = window[2];
            builder.quadratic_bezier_to(p1, p2);
        }

        let path = builder.build();
        return Self {
            shape_bundle: ShapeBundle { path, ..default() },
            stroke: Stroke::new(color, stroke),
        };
    }
}
