use raylib::prelude::*;

pub struct Mask {
    size: usize,
    pixels: Vec<f32>,
}

impl Mask {
    pub fn new() -> Self {
        let size = 64;
        let mut pixels = Vec::new();
        for y in 0..size {
            for x in 0..size {
                let pattern = (x as f32 * 0.35).sin() * (y as f32 * 0.25).cos();
                pixels.push(pattern * 0.5 + 0.5);
            }
        }
        Self { size, pixels }
    }

    pub fn sample(&self, uv: Vector2) -> Vector3 {
        let x = (uv.x.rem_euclid(1.0) * self.size as f32) as usize;
        let y = (uv.y.rem_euclid(1.0) * self.size as f32) as usize;
        Vector3::new(1.0, 1.0, 1.0) * self.pixels[y * self.size + x]
    }
}
