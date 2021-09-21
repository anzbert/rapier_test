// WINDOW:
pub const ASPECT_RATIO: f32 = 16.0 / 9.0;
pub const PIXEL_W: i32 = 1280;
pub const PIXEL_H: i32 = (PIXEL_W as f32 / ASPECT_RATIO) as i32;

// BALL:
pub const BALL_RESTITUTION: f32 = 0.9;
pub const BALL_GRAV_SCALE: f32 = 0.1;
pub const BALL_DENSITY: f32 = 0.1;

// PLAYER:
pub const PLAYER_RESTITUTION: f32 = 0.7;
