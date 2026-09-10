// Fast framebuffer: single RGBA byte buffer rendered on CPU threads,
// uploaded to ONE reused GPU texture per frame.
//
// The old version called Image::draw_pixel per pixel and
// load_texture_from_image every frame (a full GPU realloc + upload).
// This version keeps `pixels` around and calls update_texture().
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pixels: Vec<u8>,
    texture: Texture2D,
}

impl Framebuffer {
    pub fn new(
        window: &mut RaylibHandle,
        thread: &RaylibThread,
        width: u32,
        height: u32,
    ) -> Self {
        let img = Image::gen_image_color(width as i32, height as i32, Color::BLACK);
        let texture = window
            .load_texture_from_image(thread, &img)
            .expect("failed to create texture");
        // Opaque black to start
        let mut pixels = vec![0u8; (width * height * 4) as usize];
        for px in pixels.chunks_exact_mut(4) {
            px[3] = 255;
        }
        Framebuffer {
            width,
            height,
            pixels,
            texture,
        }
    }

    pub fn pixels_mut(&mut self) -> &mut [u8] {
        &mut self.pixels
    }

    pub fn present(
        &mut self,
        window: &mut RaylibHandle,
        thread: &RaylibThread,
        hud: &str,
        preview: bool,
    ) {
        let _ = self.texture.update_texture(&self.pixels);
        let mut d = window.begin_drawing(thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&self.texture, 0, 0, Color::WHITE);
        d.draw_fps(10, 10);
        d.draw_text(hud, 10, 30, 20, Color::WHITE);
        if preview {
            d.draw_text("PREVIEW 50% - release to refine", 10, 52, 20, Color::YELLOW);
        }
    }
}
