use raylib::prelude::Color;
use crate::texture::Texture;
use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub diffuse: Color,
    pub albedo: [f32; 4],
    pub specular: f32,
    pub refractive_index: f32,
    pub texture: Option<Arc<Texture>>,
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
    ) -> Self {
        Material {
            diffuse,
            albedo,
            specular,
            refractive_index,
            texture: None,
        }
    }

    pub fn new_with_texture(
        texture: Arc<Texture>,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
    ) -> Self {
        Material {
            diffuse: Color::BLACK,
            albedo,
            specular,
            refractive_index,
            texture: Some(texture),
        }
    }

    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            texture.get_color(u, v)
        } else {
            self.diffuse
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::BLACK,
            albedo: [0.0, 0.0, 0.0, 0.0],
            specular: 0.0,
            refractive_index: 0.0,
            texture: None,
        }
    }
}