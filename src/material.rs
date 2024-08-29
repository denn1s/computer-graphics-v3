use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
  pub diffuse: Color,
  pub specular: f32,
  pub albedo: [f32; 4],
  pub refractive_index: f32,
  pub texture: Option<Texture>,
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
      specular,
      albedo,
      refractive_index,
      texture: None,
    }
  }

  pub fn new_with_texture(
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
    texture: Texture,
  ) -> Self {
    Material {
      diffuse: Color::new(0, 0, 0), // Default color, will be overridden by texture
      specular,
      albedo,
      refractive_index,
      texture: Some(texture),
    }
  }

  pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
    match &self.texture {
      Some(texture) => {
        let x = (u * (texture.width - 1) as f32).round() as usize;
        let y = ((1.0 - v) * (texture.height - 1) as f32).round() as usize;
        texture.get_color(x, y)
      },
      None => self.diffuse,
    }
  }

  pub fn black() -> Self {
    Material {
      diffuse: Color::new(0, 0, 0),
      specular: 0.0,
      albedo: [0.0, 0.0, 0.0, 0.0],
      refractive_index: 0.0,
      texture: None,
    }
  }
}
