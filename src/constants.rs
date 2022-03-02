use std::f64::consts::PI;

pub const MAP_WIDTH: usize = 64;
pub const MAP_HEIGHT: usize = 64;
pub const WIDTH_2D: u32 = 1024;
pub const HEIGHT_2D: u32 = 1024;
pub const MAP_SCALE_H: u32 = HEIGHT_2D / MAP_HEIGHT as u32;
pub const MAP_SCALE_W: u32 = WIDTH_2D / MAP_WIDTH as u32;
pub const ANGLE_DOWN: f64 = 0.0;
pub const ANGLE_UP: f64 = PI;
pub const ANGLE_LEFT: f64 = 3.0 * PI / 2.0;
pub const ANGLE_RIGHT: f64 = PI / 2.0;
pub const STATUS_LINES: u32 = 40;
pub const BASE_WIDTH: u32 = 320;
pub const BASE_HEIGHT: u32 = 200;
pub const WALLPIC_WIDTH: usize = 64;
// where does this value come from?
pub const TILE_SIZE: f64 = 4.8;

// ok this is not a constant, we may move it to an util module later, or rename this
pub fn norm_angle(a: f64) -> f64 {
    let nrots = (a / (2.0 * PI)).trunc() - if a < 0.0 { 1.0 } else { 0.0 };
    a - nrots * 2.0 * PI
}
