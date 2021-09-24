pub use macroquad::prelude::*;
pub use rapier2d::prelude::*;

mod constants;
use constants::*;

mod objects;
use objects::*;

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
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // ADD OBJECTS:
    let mut players: Vec<&Player> = Vec::new();
    let mut balls: Vec<&FootBall> = Vec::new();
    let mut solids: Vec<&Solid> = Vec::new();

    let player1 = Player::new(
        vector![10.0, ARENA_HEIGHT - 10.0],
        vector![CAR_LENGTH, CAR_HEIGHT],
        &mut rigid_body_set,
        &mut collider_set,
    );
    players.push(&player1);

    let ball = FootBall::new(
        vector![ARENA_WIDTH / 2.0, ARENA_HEIGHT - 10.0],
        BALL_RADIUS,
        &mut rigid_body_set,
        &mut collider_set,
    );
    balls.push(&ball);

    // ARENA:
    let wall_thickness = 2.0;

    let floor = Solid::new(
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

    //////////////////////////////////////////////////////////
    /* Create Rapier elements necessary for the simulation. */
    let gravity = vector![0.0, 9.81];
    // let integration_parameters = IntegrationParameters {
    //     dt: get_frame_time() * 4.0, // maybe needs to be in the game loop ?
    //     ..Default::default()
    // };
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

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

        // UPDATE CONTROLS:
        if is_key_down(KeyCode::Right) {
            let rigid_body = rigid_body_set.get_mut(player1.body_handle).unwrap();
            rigid_body.apply_impulse(vector![10.0, 0.0], true);
        }
        if is_key_down(KeyCode::Left) {
            let rigid_body = rigid_body_set.get_mut(player1.body_handle).unwrap();
            rigid_body.apply_impulse(vector![-10.0, 0.0], true);
        }
        if is_key_down(KeyCode::Up) {
            let rigid_body = rigid_body_set.get_mut(player1.body_handle).unwrap();
            rigid_body.apply_impulse(vector![0.0, -10.0], true);
        }
        if is_key_down(KeyCode::Q) {
            let rigid_body = rigid_body_set.get_mut(player1.body_handle).unwrap();
            rigid_body.apply_torque_impulse(-1.0, true);
        }
        if is_key_down(KeyCode::E) {
            let rigid_body = rigid_body_set.get_mut(player1.body_handle).unwrap();
            rigid_body.apply_torque_impulse(1.0, true);
        }

        // UPDATE PHYSICS:
        let frame_time = get_frame_time();
        // let fps = 1.0 / get_fps() as f32;
        // println!("frm time: {}", frame_time);

        let integration_parameters = IntegrationParameters {
            dt: frame_time, // maybe needs to be in the game loop ?
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
        for p in players.iter() {
            p.draw(&rigid_body_set);
        }
        for b in balls.iter() {
            b.draw(&rigid_body_set);
        }
        for s in solids.iter() {
            s.draw(&rigid_body_set, &collider_set);
        }

        next_frame().await
    }
}
