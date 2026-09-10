use raylib::prelude::Vector3;

// Minecraft lesson: flat colors only, no textures, no reflection/refraction.
// Copy (no heap) so returning it per-intersection is cheap — no clone() alloc.
#[derive(Clone, Copy)]
pub struct Material {
    pub diffuse: Vector3,
    pub specular: f32,
}

impl Material {
    pub fn new(diffuse: Vector3, specular: f32) -> Self {
        Material { diffuse, specular }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Vector3::zero(),
            specular: 0.0,
        }
    }
}
