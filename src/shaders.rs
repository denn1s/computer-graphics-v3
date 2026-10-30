use crate::shader_examples::{console_styles, planets, stars};
use crate::{Uniforms, fragment::Fragment};
use raylib::prelude::*;

pub const NAMES: &[&str] = &[
    "Live shader",
    "Flat",
    "Gouraud",
    "Per-fragment lighting",
    "N64 combiner",
    "Toon",
    "Bands",
    "Rocky planet",
    "Gas giant",
    "Ice world",
    "Animated sun",
    "Noise star",
];

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    match uniforms.shader_index {
        0 => live_shader(fragment, uniforms),
        1 => console_styles::flat(fragment, uniforms),
        2 => console_styles::gouraud(fragment, uniforms),
        3 => console_styles::smooth(fragment, uniforms),
        4 => console_styles::n64(fragment, uniforms),
        5 => planets::toon(fragment, uniforms),
        6 => planets::bands(fragment, uniforms),
        7 => planets::rocky(fragment, uniforms),
        8 => planets::gas_giant(fragment, uniforms),
        9 => planets::ice(fragment, uniforms),
        10 => stars::sun(fragment, uniforms),
        _ => stars::noise_star(fragment, uniforms),
    }
}

pub fn live_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(1.0, 0.4, 0.1)
}
