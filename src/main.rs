pub use macroquad::prelude::*;
pub use rapier2d::prelude::*;

mod objects;
use objects::*;

// Macroquad window config:
fn window_conf() -> Conf {
    Conf {
        window_title: "carzz".to_owned(),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        // sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let sw = screen_width();
    let sh = screen_height();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    let player = Player::new(vector![300.0, 300.0]);

    let player_handle = rigid_body_set.insert(player.body());
    let player_coll =
        collider_set.insert_with_parent(player.collider(), player_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity = vector![0.0, 9.81];
    let integration_parameters = IntegrationParameters {
        dt: 1.0 / 60.0, // in 1 / fps?
        ..Default::default()
    };
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut joint_set = JointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    /* Run the game loop, stepping the simulation once per frame. */
    loop {
        clear_background(GRAY);

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

        // player.draw(&rigid_body_set[player_handle].translation());

        let player_body = &rigid_body_set[player_handle];
        // player_body.draw();
        println!("Ball altitude: {:?}", player_coll);

        // draw_circle(
        //     ball_body.translation().x * 50.0,
        //     ball_body.translation().y * 50.0,
        //     10.0,
        //     RED,
        // );

        next_frame().await
    }
}
