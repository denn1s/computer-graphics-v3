// triangle.rs

use crate::framebuffer::Framebuffer;
use crate::line::line;
use raylib::prelude::*;

pub fn triangle(
    framebuffer: &mut Framebuffer,
    v1: Vector2,
    v2: Vector2,
    v3: Vector2,
) {
    line(framebuffer, v1, v2);
    line(framebuffer, v2, v3);
    line(framebuffer, v3, v1);
}