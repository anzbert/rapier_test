use rapier2d::na::Vector2;

use crate::*;

pub struct Player {
    pub pos: Vector2<f32>,
    pub vel: Vector2<f32>,
    pub size: Vector2<f32>,
    pub coll_handle: Option<ColliderHandle>,
}

impl Player {
    pub fn new(pos: Vector2<f32>) -> Player {
        let size = vector![100.0, 50.0];

        let half_size = size / 2.0;
        // let collider = ColliderBuilder::cuboid(half_size[0], half_size[1])
        //     .restitution(0.7)
        //     .build();

        Player {
            pos,
            vel: vector![0.0, 0.0],
            size,
            coll_handle: None,
        }
    }

    pub fn draw(&self, handle_position: &Vector<Real>) {
        let corner_x = &handle_position[0] - self.size[0] / 2.0;
        let corner_y = &handle_position[1] - self.size[1] / 2.0;
        draw_rectangle(corner_x, corner_y, self.size[0], self.size[1], RED);
    }
    pub fn collider(&self) -> Collider {
        let half_size = self.size / 2.0;
        let collider = ColliderBuilder::cuboid(half_size[0], half_size[1])
            .restitution(0.7)
            .build();
        collider
    }
    pub fn body(&self) -> RigidBody {
        RigidBodyBuilder::new_dynamic()
            .translation(self.pos)
            .rotation(0.0)
            .build()
    }
}

pub trait Draw {
    fn draw(&self);
}
impl Draw for RigidBody {
    fn draw(&self) {
        let pos = self.translation();
        // println!("{:#?}", self.);
        // self.colliders();

        // let corner_x = pos[0] - self.size[0] / 2.0;
        // let corner_y = pos[1] - self.size[1] / 2.0;
        // draw_rectangle(corner_x, corner_y, self.size[0], self.size[1], RED);
    }
}
