pub use macroquad::prelude::*;
pub use rapier2d::prelude::*;
use std::collections::HashMap;
use std::f32::consts::PI;

mod constants;
use constants::*;

mod objects;
use objects::*;

mod car;
use car::CarStates;

mod utils;
use crate::utils::*;

// Macroquad WINDOW CONFIG:
fn window_conf() -> Conf {
    Conf {
        window_title: "boink".to_owned(),
        window_width: PIXEL_W,
        window_height: PIXEL_H,
        high_dpi: false,
        fullscreen: false,
        // sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

// MAIN:
#[macroquad::main(window_conf)]
async fn main() {
    //////////////////////////////////////////////////////////
    /* Create Rapier elements necessary for the simulation. */
    let gravity = vector![0.0, 29.81];
    // let integration_parameters = IntegrationParameters {
    //     dt: get_frame_time(), // maybe needs to be in the game loop ?
    //     ..Default::default()
    // };
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();
    let mut joint_set = JointSet::new();

    // ADD OBJECTS:
    let mut balls: Vec<&FootBall> = Vec::new();
    let mut solids: Vec<&Solid> = Vec::new();

    let ball = FootBall::new(
        vector![ARENA_WIDTH / 2.0, ARENA_HEIGHT - 10.0],
        BALL_RADIUS,
        &mut rigid_body_set,
        &mut collider_set,
    );
    balls.push(&ball);

    // ARENA:
    let wall_thickness = 2.0;

    let floor = Solid::new_with_contact_event(
        "floor".to_string(),
        vector![0.0, ARENA_HEIGHT - wall_thickness],
        vector![ARENA_WIDTH, wall_thickness],
        &mut rigid_body_set,
        &mut collider_set,
    );
    solids.push(&floor);

    let ceiling = Solid::new(
        "ceiling".to_string(),
        vector![0.0, 0.0],
        vector![ARENA_WIDTH, wall_thickness],
        &mut rigid_body_set,
        &mut collider_set,
    );
    solids.push(&ceiling);

    let wall_left = Solid::new(
        "wall_left".to_string(),
        vector![0.0, 0.0],
        vector![wall_thickness, ARENA_HEIGHT],
        &mut rigid_body_set,
        &mut collider_set,
    );
    solids.push(&wall_left);

    let wall_right = Solid::new(
        "wall_right".to_string(),
        vector![ARENA_WIDTH - wall_thickness, 0.0],
        vector![wall_thickness, ARENA_HEIGHT],
        &mut rigid_body_set,
        &mut collider_set,
    );
    solids.push(&wall_right);

    // key variable:
    // let mut jump_pressed = false;

    let mut carzz = car::Car::new(
        vector![ARENA_WIDTH / 4.0, ARENA_HEIGHT - 4.0],
        &mut rigid_body_set,
        &mut collider_set,
        &mut joint_set,
    );

    // GAME LOOP:
    /* Run the game loop, stepping the simulation once per frame. */
    loop {
        clear_background(GRAY);

        // enable quitting with CMD+Q on macos:
        if let "macos" = std::env::consts::OS {
            if is_key_down(KeyCode::LeftSuper) && is_key_down(KeyCode::Q) {
                return; // return from main -> quit
            }
        }

        // if is_key_down(KeyCode::Q) {
        //     carzz.spin(-100.0, &mut rigid_body_set);
        // }
        // if is_key_down(KeyCode::E) {
        //     carzz.spin(100.0, &mut rigid_body_set);
        // }

        if is_key_down(KeyCode::Right) && carzz.get_car_state() == CarStates::Ground {
            carzz.drive(5000.0, &mut rigid_body_set)
        }
        if is_key_down(KeyCode::Left) && carzz.get_car_state() == CarStates::Ground {
            carzz.drive(-5000.0, &mut rigid_body_set)
        }

        if is_key_down(KeyCode::Right) && carzz.get_car_state() == CarStates::Air {
            carzz.spin(100.0, &mut rigid_body_set);
        }
        if is_key_down(KeyCode::Left) && carzz.get_car_state() == CarStates::Air {
            carzz.spin(-100.0, &mut rigid_body_set);
        }

        if is_key_down(KeyCode::Up) {
            carzz.jump(&mut rigid_body_set);
        }

        if is_key_down(KeyCode::Space) {
            carzz.boost(-300.0, &mut rigid_body_set);
        }
        // if is_key_down(KeyCode::Space) && is_key_down(KeyCode::Right) {
        //     carzz.boost(300.0, Side::Right, &mut rigid_body_set);
        // }

        // UPDATE PHYSICS:
        let integration_parameters = IntegrationParameters {
            dt: get_frame_time(), // maybe needs to be in the game loop ?
            prediction_distance: 0.008,
            ..Default::default()
        };

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut joint_set,
            &mut ccd_solver,
            &physics_hooks,
            &event_handler,
        );

        // UPDATE GRAPHIC ELEMENTS:
        for b in balls.iter() {
            b.draw(&rigid_body_set);
        }
        for s in solids.iter() {
            s.draw(&rigid_body_set, &collider_set);
        }

        // carzzz
        if carzz.query_wheels_collision(floor.collider_handle, &narrow_phase) {
            carzz.set_car_state(CarStates::Ground);
        } else {
            carzz.set_car_state(CarStates::Air);
        }

        carzz.draw(&rigid_body_set);

        next_frame().await
    }
}
