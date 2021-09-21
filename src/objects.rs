use std::f32::consts::{PI, SQRT_2};

use crate::nalgebra::Vector2;
use crate::*;

fn corner_to_center(corner: Vector2<f32>, size: Vector2<f32>) -> Vector2<f32> {
    let x = corner.x + (size.x / 2.0);
    let y = corner.y + (size.y / 2.0);
    vector![x, y]
}

fn draw_line_center(
    center: Vector2<f32>,
    rot_degrees: f32,
    thickness: f32,
    length: f32,
    color: Color,
) {
    let half_length = length / 2.0;
    let rot_radians = (rot_degrees + 90.0).to_radians();

    let x_origin_1 = -half_length;
    let x_origin_2 = half_length;
    let y_origin_1 = 0.0;
    let y_origin_2 = 0.0;

    let x_rot_1 = x_origin_1 * rot_radians.cos() - y_origin_1 * rot_radians.sin();
    let y_rot_1 = x_origin_1 * rot_radians.sin() + y_origin_1 * rot_radians.cos();
    let x_rot_2 = x_origin_2 * rot_radians.cos() - y_origin_2 * rot_radians.sin();
    let y_rot_2 = x_origin_2 * rot_radians.sin() + y_origin_2 * rot_radians.cos();

    let x1 = x_rot_1 + center.x;
    let y1 = y_rot_1 + center.y;
    let x2 = x_rot_2 + center.x;
    let y2 = y_rot_2 + center.y;

    draw_line(x1, y1, x2, y2, thickness, color);
}

#[derive(Debug)]
pub struct Player {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub rot: f32,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Player {
    pub fn new(
        pos: Vector2<f32>,
        size: Vector2<f32>,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
    ) -> Player {
        let size = size;

        let body = RigidBodyBuilder::new_dynamic()
            .translation(corner_to_center(pos, size))
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
            rot: 0.0,
            size,
            body_handle: player_handle,
            collider_handle: player_collider_handle,
        }
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self.body_handle].translation();

        let rotation = body_set[self.body_handle].rotation().angle().to_degrees();

        draw_line_center(
            vector![translation.x, translation.y],
            rotation,
            self.size.x,
            self.size.y,
            PURPLE,
        );

        draw_circle(translation.x, translation.y, 5.0, BLUE);
    }
}

#[derive(Debug)]
pub struct FootBall {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub radius: f32,
    pub rot: f32,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl FootBall {
    pub fn new(
        pos: Vector2<f32>,
        radius: f32,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
    ) -> FootBall {
        let body = RigidBodyBuilder::new_dynamic()
            .translation(pos)
            .rotation(0.0)
            .gravity_scale(0.1)
            .build();
        let ball_handle = body_set.insert(body);

        let collider = ColliderBuilder::ball(radius)
            .restitution(0.9)
            .density(0.1)
            .build();

        let ball_collider_handle = coll_set.insert_with_parent(collider, ball_handle, body_set);

        FootBall {
            pos,
            vel: vector![0.0, 0.0],
            rot: 0.0,
            radius,
            body_handle: ball_handle,
            collider_handle: ball_collider_handle,
        }
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self.body_handle].translation();
        let rotation = body_set[self.body_handle].rotation().angle().to_degrees();
        draw_poly(
            translation.x,
            translation.y,
            16,
            self.radius,
            rotation,
            YELLOW,
        );

        draw_circle(translation.x, translation.y, 5.0, BLUE);
    }
}

pub struct Solid {
    pub pos: Vector2<f32>,
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
            .translation(corner_to_center(pos, size))
            .rotation(0.0)
            .build();
        let solid_handle = body_set.insert(body);

        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0).build();
        let solid_collider_handle = coll_set.insert_with_parent(collider, solid_handle, body_set);

        Solid {
            pos,
            size,
            body_handle: solid_handle,
            collider_handle: solid_collider_handle,
        }
    }
    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self.body_handle].translation();
        let corner_x = translation.x - self.size.x / 2.0;
        let corner_y = translation.y - self.size.y / 2.0;
        draw_rectangle(corner_x, corner_y, self.size.x, self.size.y, GREEN);

        draw_circle(translation.x, translation.y, 5.0, BLUE);
    }
}
