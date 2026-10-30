use super::{
    mix,
    noise_helpers::{fbm, sample},
    smoothstep,
};
use crate::{Uniforms, fragment::Fragment};
use raylib::prelude::*;

pub fn toon(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let light = if fragment.intensity > 0.7 {
        1.0
    } else if fragment.intensity > 0.3 {
        0.6
    } else {
        0.2
    };
    Vector3::new(0.3, 0.8, 1.0) * light
}

pub fn bands(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let wave = (fragment.object_position.y * 22.0 + uniforms.time).sin();
    let color = mix(
        Vector3::new(0.2, 0.04, 0.4),
        Vector3::new(0.9, 0.4, 0.7),
        wave * 0.5 + 0.5,
    );
    color * (0.12 + fragment.intensity * 0.88)
}

pub fn rocky(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let height = fbm(&uniforms.noise, fragment.object_position * 3.0);
    let ocean = Vector3::new(0.02, 0.1, 0.35);
    let land = mix(
        Vector3::new(0.12, 0.3, 0.1),
        Vector3::new(0.65, 0.52, 0.3),
        height,
    );
    let color = mix(ocean, land, smoothstep(0.48, 0.52, height));
    let snow = smoothstep(0.65, 0.78, height);
    mix(color, Vector3::new(0.9, 0.95, 1.0), snow) * (0.1 + fragment.intensity * 0.9)
}

pub fn gas_giant(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let position = fragment.object_position;
    let drift = Vector3::new(uniforms.time * 0.12, 0.0, 0.0);
    let turbulence = sample(&uniforms.noise, position * 5.0 + drift);
    let bands = (position.y * 28.0 + turbulence * 4.0).sin() * 0.5 + 0.5;
    let color = mix(
        Vector3::new(0.3, 0.08, 0.025),
        Vector3::new(1.0, 0.83, 0.55),
        bands,
    );
    color * (0.12 + fragment.intensity * 0.88)
}

pub fn ice(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let veins = sample(&uniforms.noise, fragment.object_position * 7.0).abs();
    let color = mix(
        Vector3::new(0.04, 0.2, 0.45),
        Vector3::new(0.65, 0.9, 1.0),
        smoothstep(0.02, 0.16, veins),
    );
    let view = (uniforms.camera_position - fragment.world_position).normalized();
    let rim = (1.0 - fragment.normal.dot(view).max(0.0)).powf(3.0);
    color * (0.15 + fragment.intensity * 0.85) + Vector3::new(0.1, 0.4, 0.6) * rim
}
