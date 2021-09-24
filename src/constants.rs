// WINDOW:
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const PIXEL_W: i32 = 1280;
pub const PIXEL_H: i32 = (PIXEL_W as f32 / ASPECT_RATIO) as i32;

pub const ARENA_WIDTH: f32 = 105.0; // in m
pub const ARENA_HEIGHT: f32 = 40.0; // in m

pub const PIXELS_PER_METRE: f32 = PIXEL_W as f32 / ARENA_WIDTH;

pub const CAR_LENGTH: f32 = 4.5; // in m
pub const CAR_HEIGHT: f32 = 1.65; // in m

pub const BALL_RADIUS: f32 = 2.5; // in m

// BALL:
pub const BALL_RESTITUTION: f32 = 0.9;
pub const BALL_GRAV_SCALE: f32 = 0.1;
pub const BALL_DENSITY: f32 = 0.5;

// PLAYER:
pub const PLAYER_RESTITUTION: f32 = 0.3;
