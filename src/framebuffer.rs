use raylib::prelude::*;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub colors: Vec<Vector3>,
    pub depths: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            colors: vec![Vector3::zero(); width * height],
            depths: vec![f32::INFINITY; width * height],
        }
    }

    pub fn clear(&mut self) {
        self.colors.fill(Vector3::new(0.04, 0.08, 0.18));
        self.depths.fill(f32::INFINITY);
    }

    pub fn bytes(&self) -> Vec<u8> {
        self.colors
            .iter()
            .flat_map(|color| {
                [
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                ]
            })
            .collect()
    }

    pub fn save(&self, path: &str) {
        let bytes = self.bytes();
        let mut image = Image::gen_image_color(self.width as i32, self.height as i32, Color::BLACK);
        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) * 4;
                image.draw_pixel(
                    x as i32,
                    y as i32,
                    Color::new(bytes[i], bytes[i + 1], bytes[i + 2], 255),
                );
            }
        }
        image.export_image(path);
    }
}
