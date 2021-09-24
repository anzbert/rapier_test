use crate::nalgebra::Vector2;
use std::f32::consts::PI;

use crate::*;

pub fn size_mtr_to_pxl(metres: f32) -> f32 {
    metres * PIXELS_PER_METRE
}
pub fn pos_x_mtr_to_pxl(pos_x: f32) -> f32 {
    pos_x * PIXELS_PER_METRE
}
pub fn pos_y_mtr_to_pxl(pos_y: f32) -> f32 {
    let floor = screen_height() / 2.0 - size_mtr_to_pxl(ARENA_HEIGHT) / 2.0;
    floor + (pos_y * PIXELS_PER_METRE)
}
pub fn pos_vec_mtr_to_pxl(position: Vector2<f32>) -> Vector2<f32> {
    let x = pos_x_mtr_to_pxl(position.x);
    let y = pos_y_mtr_to_pxl(position.y);
    vector![x, y]
}

pub fn draw_line_center(
    center: Vector2<f32>,
    rotation: f32,
    thickness: f32,
    length: f32,
    color: Color,
) {
    let half_length = length / 2.0;
    let rot_radians = rotation + PI / 2.0; // rotate to neutral

    // create at origin:
    let x_origin_1 = -half_length;
    let x_origin_2 = half_length;
    let y_origin_1 = 0.0;
    let y_origin_2 = 0.0;

    // apply rotation:
    let x_rot_1 = x_origin_1 * rot_radians.cos() - y_origin_1 * rot_radians.sin();
    let y_rot_1 = x_origin_1 * rot_radians.sin() + y_origin_1 * rot_radians.cos();
    let x_rot_2 = x_origin_2 * rot_radians.cos() - y_origin_2 * rot_radians.sin();
    let y_rot_2 = x_origin_2 * rot_radians.sin() + y_origin_2 * rot_radians.cos();

    // translate to center:
    let x1 = x_rot_1 + center.x;
    let y1 = y_rot_1 + center.y;
    let x2 = x_rot_2 + center.x;
    let y2 = y_rot_2 + center.y;

    draw_line(x1, y1, x2, y2, thickness, color);
}
