use crate::nalgebra::Vector2;
use crate::*;

const HEIGHT: f32 = 1.6;
const LENGTH: f32 = 10.0;
const WHEEL_RADIUS: f32 = 1.8;
const WHEEL_FRONT_X_OFFSET: f32 = 3.4;
const WHEEL_FRONT_Y_OFFSET: f32 = 0.0;
const WHEEL_BACK_X_OFFSET: f32 = -3.4;
const WHEEL_BACK_Y_OFFSET: f32 = 0.0;

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
            .additional_mass(50.0)
            .linear_damping(0.5)
            .angular_damping(5.0)
            .build();
        let body_handle = body_set.insert(body);

        let collider = match shape {
            SelectPart::Body => ColliderBuilder::cuboid(half_extents.x, half_extents.y)
                .collision_groups(InteractionGroups::new(0b0100, 0b1101))
                .build(),
            SelectPart::Wheel => ColliderBuilder::ball(half_extents.x)
                .friction(8.0)
                // .density(1.5)
                .collision_groups(InteractionGroups::new(0b0010, 0b0011))
                .build(),
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
    fn get_coll_handle(&self) -> ColliderHandle {
        match self {
            CarPart::Body { coll_handle, .. } => *coll_handle,
            CarPart::Wheel { coll_handle, .. } => *coll_handle,
        }
    }
}
#[derive(PartialEq, Eq, Hash)]
enum CarComponents {
    WheelFront,
    WheelBack,
    CarBody,
}
#[derive(PartialEq)]
pub enum CarStates {
    Air,
    Ground,
}
pub struct Car {
    _velocity: Vector2<f32>,
    position: Vector2<f32>,
    parts: HashMap<CarComponents, CarPart>,
    _joint_handles: Vec<JointHandle>,
    state: CarStates,
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
            vector![LENGTH / 2.0, HEIGHT / 2.0],
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
        let wheel_front_joint = BallJoint::new(
            point![0.0, 0.0],
            point![WHEEL_FRONT_X_OFFSET, WHEEL_FRONT_Y_OFFSET],
        );

        let wheel_front_joint_handle = joint_set.insert(
            wheel_front.get_body_handle(),
            car_body.get_body_handle(),
            wheel_front_joint,
        );

        let wheel_back_joint = BallJoint::new(
            point![0.0, 0.0],
            point![WHEEL_BACK_X_OFFSET, WHEEL_BACK_Y_OFFSET],
        );

        let wheel_back_joint_handle = joint_set.insert(
            wheel_back.get_body_handle(),
            car_body.get_body_handle(),
            wheel_back_joint,
        );

        let mut component_map = HashMap::with_capacity(3);

        component_map.insert(CarComponents::CarBody, car_body);
        component_map.insert(CarComponents::WheelFront, wheel_front);
        component_map.insert(CarComponents::WheelBack, wheel_back);

        Car {
            position,
            _velocity: vector![0.0, 0.0],
            parts: component_map,
            _joint_handles: vec![wheel_front_joint_handle, wheel_back_joint_handle],
            state: CarStates::Ground,
        }
    }

    pub fn get_car_state(&self) -> CarStates {
        match self.state {
            CarStates::Air => CarStates::Air,
            CarStates::Ground => CarStates::Ground,
        }
    }

    pub fn query_wheels_collision(
        &self,
        other_coll_handle: ColliderHandle,
        narrow_phase: &NarrowPhase,
    ) -> bool {
        let wheel_front_coll = self
            .parts
            .get(&CarComponents::WheelFront)
            .unwrap()
            .get_coll_handle();
        let wheel_back_coll = self
            .parts
            .get(&CarComponents::WheelBack)
            .unwrap()
            .get_coll_handle();

        match narrow_phase.contact_pair(wheel_front_coll, other_coll_handle) {
            Some(pair) => {
                if !pair.has_any_active_contact {
                    return false;
                }
            }
            None => return false,
        };
        match narrow_phase.contact_pair(wheel_back_coll, other_coll_handle) {
            Some(pair) => {
                if !pair.has_any_active_contact {
                    return false;
                }
            }
            None => return false,
        };

        true
    }

    pub fn set_car_state(&mut self, new_state: CarStates) {
        self.state = new_state;
    }

    pub fn drive(&self, torque: f32, body_set: &mut RigidBodySet) {
        let wheel_front_body = body_set
            .get_mut(
                self.parts
                    .get(&CarComponents::WheelFront)
                    .unwrap()
                    .get_body_handle(),
            )
            .unwrap();
        wheel_front_body.apply_torque(torque, true);

        let wheel_back_body = body_set
            .get_mut(
                self.parts
                    .get(&CarComponents::WheelBack)
                    .unwrap()
                    .get_body_handle(),
            )
            .unwrap();
        wheel_back_body.apply_torque(torque, true);
    }

    pub fn jump(&self, body_set: &mut RigidBodySet) {
        match self.state {
            CarStates::Ground => {
                let rigid_body = body_set
                    .get_mut(
                        self.parts
                            .get(&CarComponents::CarBody)
                            .unwrap()
                            .get_body_handle(),
                    )
                    .unwrap();
                rigid_body.apply_impulse(vector![0.0, -6000.0], true);
            }
            CarStates::Air => {}
        }
    }

    pub fn boost(&self, force: f32, body_set: &mut RigidBodySet) {
        let rigid_body = body_set
            .get_mut(
                self.parts
                    .get(&CarComponents::CarBody)
                    .unwrap()
                    .get_body_handle(),
            )
            .unwrap();

        let rotation = rigid_body.rotation();

        let final_rot = if rotation.angle() < -0.5 * PI || rotation.angle() > 0.5 * PI {
            utils::turn_around(rotation)
        } else {
            *rotation
        };

        let boost_vector = final_rot.transform_vector(&vector!(0.0, 1.0));

        rigid_body.apply_impulse(boost_vector * force, true);
    }

    pub fn spin(&self, torque: f32, body_set: &mut RigidBodySet) {
        let rigid_body = body_set
            .get_mut(
                self.parts
                    .get(&CarComponents::CarBody)
                    .unwrap()
                    .get_body_handle(),
            )
            .unwrap();
        rigid_body.apply_torque_impulse(torque, true);
    }

    pub fn draw(&self, body_set: &RigidBodySet) {
        let translation = body_set[self
            .parts
            .get(&CarComponents::CarBody)
            .unwrap()
            .get_body_handle()]
        .translation();
        let rotation = body_set[self
            .parts
            .get(&CarComponents::CarBody)
            .unwrap()
            .get_body_handle()]
        .rotation()
        .angle();
        utils::draw_line_center(
            pos_vec_mtr_to_pxl(vector![translation.x, translation.y]),
            rotation,
            size_mtr_to_pxl(HEIGHT),
            size_mtr_to_pxl(LENGTH),
            RED,
        );

        for (component, part) in self.parts.iter() {
            match component {
                &CarComponents::WheelBack | &CarComponents::WheelFront => {
                    let translation = body_set[part.get_body_handle()].translation();
                    let rotation = body_set[part.get_body_handle()].rotation().angle();

                    draw_poly(
                        pos_x_mtr_to_pxl(translation.x),
                        pos_y_mtr_to_pxl(translation.y),
                        8,
                        size_mtr_to_pxl(WHEEL_RADIUS),
                        rotation.to_degrees(),
                        ORANGE,
                    );
                    draw_circle(
                        pos_x_mtr_to_pxl(translation.x),
                        pos_y_mtr_to_pxl(translation.y),
                        size_mtr_to_pxl(0.2),
                        BLUE,
                    );
                }
                _ => {}
            }
        }
    }
}
