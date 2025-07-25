use raylib::prelude::*;

pub struct Texture {
    pub image: Image,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let image = Image::load_image(path).expect("Failed to load texture");
        Texture { image }
    }

    pub fn get_color(&self, u: f32, v: f32) -> Color {
        let x = (u * (self.image.width() - 1) as f32) as i32;
        let y = ((1.0 - v) * (self.image.height() - 1) as f32) as i32;
        // This is a workaround for the raylib-rs issue where get_color requires a mutable reference
        let mut image = self.image.clone();
        image.get_color(x, y)
    }
}