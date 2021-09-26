use macroquad::miniquad::date::now;

use crate::nalgebra::Vector2;
use crate::*;

// use std::time::{Duration, Instant};

fn corner_to_center(corner: Vector2<f32>, size: Vector2<f32>) -> Vector2<f32> {
    let x = corner.x + (size.x / 2.0);
    let y = corner.y + (size.y / 2.0);
    vector![x, y]
}

#[derive(Debug)]
pub struct Player {
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub rot: f32,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub jump_state: usize,
    pub jump_time: f64,
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
            .restitution(PLAYER_RESTITUTION)
            .build();

        let player_collider_handle = coll_set.insert_with_parent(collider, player_handle, body_set);

        Player {
            pos,
            rot: 0.0,
            size,
            body_handle: player_handle,
            collider_handle: player_collider_handle,
            jump_state: 0,
            jump_time: now(),
        }
    }

    pub fn set_jump_state(&mut self, state: usize, body_set: &mut RigidBodySet) {
        // if now() - self.jump_time > 0.2 {
        match state {
            0 => {
                self.jump_time = now();
                self.jump_state = 0;
            }
            1 | 2 => {
                if self.jump_state < 2 {
                    let rigid_body = body_set.get_mut(self.body_handle).unwrap();
                    rigid_body.apply_impulse(vector![0.0, -80.0], true);
                    self.jump_state += 1;
                    self.jump_time = now();
                }
            }
            _ => {}
        };
        println!("state {}", self.jump_state);
        // }
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self.body_handle].translation();
        let rotation = body_set[self.body_handle].rotation().angle();
        // let iso = body_set[self.body_handle].position();

        // println!("player pos - x: {} y: {}", translation.x, translation.y);

        utils::draw_line_center(
            pos_vec_mtr_to_pxl(vector![translation.x, translation.y]),
            rotation,
            size_mtr_to_pxl(self.size.x),
            size_mtr_to_pxl(self.size.y),
            PURPLE,
        );

        draw_circle(
            pos_x_mtr_to_pxl(translation.x),
            pos_y_mtr_to_pxl(translation.y),
            size_mtr_to_pxl(0.3),
            BLUE,
        );
    }
}

#[derive(Debug)]
pub struct FootBall {
    pub pos: Vector2<f32>,
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
            .gravity_scale(BALL_GRAV_SCALE)
            .build();
        let ball_handle = body_set.insert(body);

        let collider = ColliderBuilder::ball(radius)
            .restitution(BALL_RESTITUTION)
            .density(BALL_DENSITY)
            .build();

        let ball_collider_handle = coll_set.insert_with_parent(collider, ball_handle, body_set);

        FootBall {
            pos,
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
            pos_x_mtr_to_pxl(translation.x),
            pos_y_mtr_to_pxl(translation.y),
            16,
            size_mtr_to_pxl(self.radius),
            rotation,
            YELLOW,
        );

        draw_circle(
            pos_x_mtr_to_pxl(translation.x),
            pos_y_mtr_to_pxl(translation.y),
            size_mtr_to_pxl(0.3),
            BLUE,
        );
    }
}

pub struct Solid {
    pub name: String,
    pub pos: Vector2<f32>,
    pub size: Vector2<f32>,
    pub body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Solid {
    pub fn new(
        name: String,
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
            name,
            pos,
            size,
            body_handle: solid_handle,
            collider_handle: solid_collider_handle,
        }
    }
    pub fn new_with_contact_event(
        name: String,
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

        let collider = ColliderBuilder::cuboid(size.x / 2.0, size.y / 2.0)
            .active_events(ActiveEvents::CONTACT_EVENTS)
            .build();
        let solid_collider_handle = coll_set.insert_with_parent(collider, solid_handle, body_set);

        Solid {
            name,
            pos,
            size,
            body_handle: solid_handle,
            collider_handle: solid_collider_handle,
        }
    }
    pub fn draw(&self, body_set: &RigidBodySet, _coll_set: &ColliderSet) {
        // let cuboid = _coll_set[self.collider_handle].shape().as_cuboid(); // and then access its dimensions with
        // println!("{} : {:?}", self.name, cuboid.unwrap().half_extents);

        let translation = body_set[self.body_handle].translation();
        let corner_x = translation.x - self.size.x / 2.0;
        let corner_y = translation.y - self.size.y / 2.0;
        draw_rectangle(
            pos_x_mtr_to_pxl(corner_x),
            pos_y_mtr_to_pxl(corner_y),
            size_mtr_to_pxl(self.size.x),
            size_mtr_to_pxl(self.size.y),
            GREEN,
        );

        draw_circle(
            pos_x_mtr_to_pxl(translation.x),
            pos_y_mtr_to_pxl(translation.y),
            size_mtr_to_pxl(0.3),
            BLUE,
        );
    }
}
