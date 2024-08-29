extern crate image;
use image::{ImageReader, Pixel, DynamicImage, GenericImageView};
use std::fmt;
use crate::color::Color;

#[derive(Clone)]
pub struct Texture {
  image: DynamicImage,
  pub width: usize,
  pub height: usize,
  color_array: Vec<Vec<u32>>,
}

impl Texture {
  pub fn new(file_path: &str) -> Texture {
    let img = ImageReader::open(file_path).unwrap().decode().unwrap();
    let width = img.width() as usize;
    let height = img.height() as usize;
    let mut texture = Texture {
      image: img,
      width,
      height,
      color_array: vec![vec![0; height as usize]; width as usize],
    };
    texture.load_color_array();
    texture
  }

  fn load_color_array(&mut self) {
    for x in 0..self.width {
      for y in 0..self.height {
        let pixel = self.image.get_pixel(x as u32, y as u32).to_rgb();
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];
        let color = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
        self.color_array[x as usize][y as usize] = color;
      }
    }
  }

  pub fn get_color_as_hex(&self, x: usize, y: usize) -> u32 {
    if x >= self.width || y >= self.height {
      0xFF00FF
    } else {
      self.color_array[x as usize][y as usize]
    }
  }

  pub fn get_color(&self, x: usize, y: usize) -> Color {
    if x >= self.width || y >= self.height {
      Color::from_hex(0xFF00FF)
    } else {
      Color::from_hex(self.color_array[x][y])
    }
  }
}

impl fmt::Debug for Texture {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Texture")
      .field("width", &self.width)
      .field("height", &self.height)
      .finish()
  }
}
