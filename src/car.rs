// use macroquad::miniquad::date::now;

use crate::nalgebra::Vector2;
use crate::*;

const HEIGHT: f32 = 1.5;
const LENGTH: f32 = 6.0;
const WHEEL_RADIUS: f32 = 0.5;
const WHEEL_FRONT_X_OFFSET: f32 = 1.5;
const WHEEL_FRONT_Y_OFFSET: f32 = 0.2;
const WHEEL_BACK_X_OFFSET: f32 = -1.5;
const WHEEL_BACK_Y_OFFSET: f32 = 0.2;

#[derive(Debug)]
enum CarPart {
    Body {
        body_handle: RigidBodyHandle,
        coll_handle: ColliderHandle,
        joint_handle: Option<JointHandle>,
    },
    Wheel {
        body_handle: RigidBodyHandle,
        coll_handle: ColliderHandle,
        joint_handle: Option<JointHandle>,
    },
}

enum SelectPart {
    Body,
    Wheel,
}

// struct Part {
//     joint_handle: JointHandle,
//     body_handle: RigidBodyHandle,
//     coll_handle: ColliderHandle,
// }

impl CarPart {
    fn new(
        shape: SelectPart,
        position: Vector2<f32>,
        half_extents: Vector2<f32>,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
    ) -> CarPart {
        let body = RigidBodyBuilder::new_dynamic()
            .translation(position)
            .build();
        let body_handle = body_set.insert(body);

        let collider = match shape {
            SelectPart::Body => ColliderBuilder::cuboid(half_extents.x, half_extents.y).build(),
            SelectPart::Wheel => ColliderBuilder::ball(half_extents.x).build(),
        };
        let coll_handle = coll_set.insert_with_parent(collider, body_handle, body_set);

        match shape {
            SelectPart::Body => CarPart::Body {
                body_handle,
                coll_handle,
                joint_handle: None,
            },
            SelectPart::Wheel => CarPart::Wheel {
                body_handle,
                coll_handle,
                joint_handle: None,
            },
        }
    }

    fn get_body_handle(&self) -> RigidBodyHandle {
        match self {
            CarPart::Body { body_handle, .. } => *body_handle,
            CarPart::Wheel { body_handle, .. } => *body_handle,
        }
    }
}

pub struct Car {
    velocity: Vector2<f32>,
    position: Vector2<f32>,
    parts: Vec<CarPart>,
}

impl Car {
    pub fn new(
        position: Vector2<f32>,
        body_set: &mut RigidBodySet,
        coll_set: &mut ColliderSet,
        joint_set: &mut JointSet,
    ) -> Car {
        let car_body = CarPart::new(
            SelectPart::Body,
            position,
            vector![CAR_LENGTH / 2.0, CAR_HEIGHT / 2.0],
            body_set,
            coll_set,
        );

        let wheel_front_position = position + vector![WHEEL_FRONT_X_OFFSET, WHEEL_FRONT_Y_OFFSET];
        let wheel_front = CarPart::new(
            SelectPart::Wheel,
            wheel_front_position,
            vector![WHEEL_RADIUS, 0.],
            body_set,
            coll_set,
        );

        let wheel_back_position = position + vector![WHEEL_BACK_X_OFFSET, WHEEL_BACK_Y_OFFSET];
        let wheel_back = CarPart::new(
            SelectPart::Wheel,
            wheel_back_position,
            vector![WHEEL_RADIUS, 0.],
            body_set,
            coll_set,
        );

        // ASSEMBLE CAR:
        let _wheel_front_joint = BallJoint::new(
            point![0.0, 0.0],
            point![WHEEL_FRONT_X_OFFSET, WHEEL_FRONT_Y_OFFSET],
        );
        joint_set.insert(
            wheel_front.get_body_handle(),
            car_body.get_body_handle(),
            _wheel_front_joint,
        );

        let _wheel_back_joint = BallJoint::new(
            point![0.0, 0.0],
            point![WHEEL_BACK_X_OFFSET, WHEEL_BACK_Y_OFFSET],
        );
        joint_set.insert(
            wheel_back.get_body_handle(),
            car_body.get_body_handle(),
            _wheel_back_joint,
        );

        Car {
            position,
            velocity: vector![0.0, 0.0],
            parts: vec![wheel_front, wheel_back, car_body],
        }
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        for part in self.parts.iter() {
            let translation = body_set[part.get_body_handle()].translation();
            let rotation = body_set[part.get_body_handle()].rotation().angle();
            // let iso = body_set[self.body_handle].position();
            // println!("player pos - x: {} y: {}", translation.x, translation.y);

            match part {
                CarPart::Body { .. } => {
                    utils::draw_line_center(
                        pos_vec_mtr_to_pxl(vector![translation.x, translation.y]),
                        rotation,
                        size_mtr_to_pxl(CAR_HEIGHT),
                        size_mtr_to_pxl(CAR_LENGTH),
                        RED,
                    );
                }
                CarPart::Wheel { .. } => {
                    draw_poly(
                        pos_x_mtr_to_pxl(translation.x),
                        pos_y_mtr_to_pxl(translation.y),
                        8,
                        size_mtr_to_pxl(WHEEL_RADIUS),
                        rotation,
                        ORANGE,
                    );
                }
            }
            draw_circle(
                pos_x_mtr_to_pxl(translation.x),
                pos_y_mtr_to_pxl(translation.y),
                size_mtr_to_pxl(0.3),
                BLUE,
            );
        }
    }
}
