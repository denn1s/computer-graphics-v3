use raylib::prelude::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 2],
    pub specular: f32,
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2]) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::BLACK,
            albedo: [0.0, 0.0],
            specular: 0.0,
        }
    }
}