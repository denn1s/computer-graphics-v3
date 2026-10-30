pub mod console_styles;
pub mod noise_helpers;
pub mod passes;
pub mod planets;
pub mod stars;

use raylib::prelude::*;

pub fn mix(a: Vector3, b: Vector3, amount: f32) -> Vector3 {
    a * (1.0 - amount) + b * amount
}

pub fn smoothstep(start: f32, end: f32, value: f32) -> f32 {
    let t = ((value - start) / (end - start)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}
