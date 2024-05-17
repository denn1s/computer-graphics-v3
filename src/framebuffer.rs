pub const WIDTH: usize = 80;
pub const HEIGHT: usize = 60;

pub struct Framebuffer {
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new() -> Self {
        Framebuffer {
            buffer: vec![0; WIDTH * HEIGHT],
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.buffer.iter_mut() {
            *pixel = 0; 
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < WIDTH && y < HEIGHT {
            self.buffer[y * WIDTH + x] = color;
        }
    }
}

