use crate::nalgebra::*;

use crate::*;

// pub fn map_range_f32(source: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
//     to_range.0 + (source - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
// }

pub fn turn_around(input: &Unit<Complex<f32>>) -> Unit<Complex<f32>> {
    let output = input.angle() + PI;
    UnitComplex::new(output)
}

pub fn size_mtr_to_pxl(metres: f32) -> f32 {
    metres * PIXELS_PER_METRE
}
pub fn pos_x_mtr_to_pxl(pos_x: f32) -> f32 {
    pos_x * PIXELS_PER_METRE
}
pub fn pos_y_mtr_to_pxl(pos_y: f32) -> f32 {
    let ceiling = screen_height() / 2.0 - size_mtr_to_pxl(ARENA_HEIGHT) / 2.0;
    ceiling + (pos_y * PIXELS_PER_METRE)
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
    let rot_radians = rotation; // rotate to neutral

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
