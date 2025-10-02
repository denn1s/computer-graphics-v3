// triangle.rs

use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::*;

pub fn triangle(
    framebuffer: &mut Framebuffer,
    v1: Vector3,
    v2: Vector3,
    v3: Vector3,
) {
    let a = Vector2::new(v1.x, v1.y);
    let b = Vector2::new(v2.x, v2.y);
    let c = Vector2::new(v3.x, v3.y);

    line(framebuffer, a, b);
    line(framebuffer, b, c);
    line(framebuffer, c, a);
}