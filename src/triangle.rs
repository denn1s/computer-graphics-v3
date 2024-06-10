// triangle.rs
use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm::Vec3;

pub trait Triangle {
    fn triangle(&mut self, v1: Vec3, v2: Vec3, v3: Vec3);
}

impl Triangle for Framebuffer {
    fn triangle(&mut self, v1: Vec3, v2: Vec3, v3: Vec3) {
        self.line(v1, v2);
        self.line(v2, v3);
        self.line(v3, v1);
    }
}
