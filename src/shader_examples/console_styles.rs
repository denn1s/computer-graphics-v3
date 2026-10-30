use super::mix;
use crate::{Uniforms, fragment::Fragment};
use raylib::prelude::*;

pub fn flat(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(0.7, 0.8, 1.0) * (0.12 + fragment.flat_intensity * 0.88)
}

pub fn gouraud(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(0.7, 0.8, 1.0) * (0.12 + fragment.gouraud_intensity * 0.88)
}

pub fn smooth(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(0.7, 0.8, 1.0) * (0.12 + fragment.intensity * 0.88)
}

pub fn combine(a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Vector3 {
    Vector3::new(
        (a.x - b.x) * c.x + d.x,
        (a.y - b.y) * c.y + d.y,
        (a.z - b.z) * c.z + d.z,
    )
}

pub fn n64(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let texel = uniforms.mask.sample(fragment.uv);
    let primitive = mix(
        Vector3::new(1.0, 0.3, 0.02),
        Vector3::new(0.3, 1.0, 0.8),
        uniforms.time.sin() * 0.5 + 0.5,
    );
    let environment = Vector3::new(0.03, 0.04, 0.22);
    let recolored = combine(primitive, environment, texel, environment);
    let shade = Vector3::new(1.0, 1.0, 1.0) * (0.15 + fragment.gouraud_intensity * 0.85);
    combine(recolored, Vector3::zero(), shade, Vector3::zero())
}
