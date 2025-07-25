use raylib::prelude::Color;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 4],
    pub specular: f32,
    pub refractive_index: f32,
    pub texture_id: Option<char>,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture_id: Option<char>,
    ) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            refractive_index,
            texture_id,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::BLACK,
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 0.0,
            texture_id: None,
        }
    }
}
