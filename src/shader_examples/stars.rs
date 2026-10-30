use super::{mix, noise_helpers::fbm};
use crate::{Uniforms, fragment::Fragment};
use raylib::prelude::*;

pub fn sun(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let p = fragment.object_position;
    let time = uniforms.time;
    let warp = (p.x * 11.0 + time).sin() * (p.z * 13.0 - time * 0.7).sin();
    let wave = (p.y * 24.0 + warp * 3.0 + time * 1.5).sin() * 0.5 + 0.5;
    let cells = (p.x * 31.0 + p.z * 17.0 + time).sin() * 0.5 + 0.5;
    let heat = (wave * 0.75 + cells * 0.25).powf(1.5);
    mix(
        Vector3::new(0.45, 0.015, 0.002),
        Vector3::new(1.0, 0.9, 0.25),
        heat,
    )
}

pub fn noise_star(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let drift = Vector3::new(0.0, uniforms.time * 0.25, uniforms.time * 0.1);
    let heat = fbm(&uniforms.noise, fragment.object_position * 7.0 + drift);
    let heat = ((heat - 0.25) * 2.0).clamp(0.0, 1.0);
    mix(
        Vector3::new(0.25, 0.005, 0.04),
        Vector3::new(1.0, 0.85, 0.3),
        heat,
    )
}
