use std::f32::consts::SQRT_2;

use crate::nalgebra::Vector2;
use crate::*;

#[derive(Debug)]
pub struct Player {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub size: Vector2<f32>,
    pub rot: f32,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Player {
    pub fn new(
        pos: Vector2<f32>,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
    ) -> Player {
        let size = vector![50.0, 50.0];

        let body = RigidBodyBuilder::new_dynamic()
            .translation(pos)
            .rotation(0.0)
            .build();
        let player_handle = body_set.insert(body);

        let half_size = size / 2.0;
        let collider = ColliderBuilder::cuboid(half_size.x, half_size.y)
            .restitution(0.7)
            .build();

        let player_collider_handle = coll_set.insert_with_parent(collider, player_handle, body_set);

        Player {
            pos,
            vel: vector![0.0, 0.0],
            rot: 0.0,
            size,
            body_handle: player_handle,
            collider_handle: player_collider_handle,
        }
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self.body_handle].translation();
        let rotation = body_set[self.body_handle].rotation().angle().to_degrees();
        let center_x = translation.x + self.size.x;
        let center_y = translation.y + self.size.y;
        // draw_rectangle(corner_x, corner_y, self.size.x, self.size.y, RED);
        draw_poly_lines(
            center_x,
            center_y,
            4,
            self.size.x / 2.0 * SQRT_2,
            rotation + 45.0,
            2.0,
            RED,
        );
    }

    // pub fn apply_vel (&self)
}

pub struct Solid {
    pub pos: Vector2<f32>,
    // pub vel: Vector2<f32>,
    pub size: Vector2<f32>,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Solid {
    pub fn new(
        pos: Vector2<f32>,
        size: Vector2<f32>,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
    ) -> Solid {
        let body = RigidBodyBuilder::new_static()
            .translation(pos)
            .rotation(0.0)
            .lock_translations()
            .build();
        let solid_handle = body_set.insert(body);

        let collider = ColliderBuilder::cuboid(size.x, size.y).build();
        let solid_collider_handle = coll_set.insert_with_parent(collider, solid_handle, body_set);

        Solid {
            pos,
            size,
            body_handle: solid_handle,
            collider_handle: solid_collider_handle,
        }
    }
    pub fn draw(&self, body_set: &RigidBodySet) {
        let handle_position = body_set[self.body_handle].translation();
        let corner_x = handle_position.x;
        let corner_y = handle_position.y;
        draw_rectangle(corner_x, corner_y, self.size.x, self.size.y, GREEN);
    }
}
