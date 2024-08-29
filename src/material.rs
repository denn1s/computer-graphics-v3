use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
  pub diffuse: Color,
  pub specular: f32,
  pub albedo: [f32; 4],
  pub refractive_index: f32,
  pub texture: Option<Texture>,
  texture_cache: Vec<Color>,
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
      texture_cache: vec![Color::black(); 0],
    }
  }

  pub fn new_with_texture(
    specular: f32,
    albedo: [f32; 4],
    refractive_index: f32,
    texture: Texture,
  ) -> Self {
    let texture_cache = vec![Color::black(); texture.width * texture.height];
    Material {
      diffuse: Color::new(0, 0, 0), // Default color, will be overridden by texture
      specular,
      albedo,
      refractive_index,
      texture: Some(texture),
      texture_cache,
    }
  }

  pub fn get_diffuse_color(&mut self, u: f32, v: f32) -> Color {
    match &self.texture {
      Some(texture) => {
        let x = ((u * (texture.width as f32 - 1.0)) as usize).min(texture.width - 1);
        let y = (((1.0 - v) * (texture.height as f32 - 1.0)) as usize).min(texture.height - 1);
        let index = y * texture.width + x;

        if self.texture_cache[index].is_black() {
          self.texture_cache[index] = texture.get_color(x, y);
        }
        self.texture_cache[index]
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
      texture_cache: vec![Color::black(); 0],
    }
  }
}
