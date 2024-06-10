use nalgebra_glm::Vec3;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;
mod line;
mod triangle;

use framebuffer::Framebuffer;
use triangle::Triangle;

fn main() {
  let window_width = 800;
  let window_height = 600;

  let framebuffer_width = 80;
  let framebuffer_height = 60;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Framebuffer Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  framebuffer.set_background_color(0x333355);


  while window.is_open() {
    // listen to inputs
    if window.is_key_down(Key::Escape) {
      break;
    }

    // Clear the framebuffer
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    let v1 = Vec3::new(20.0, 10.0, 0.0);
    let v2 = Vec3::new(60.0, 50.0, 0.0);
    let v3 = Vec3::new(10.0, 50.0, 0.0);
    framebuffer.triangle(v1, v2, v3);

    // Update the window with the framebuffer contents
    window
      .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
      .unwrap();

    std::thread::sleep(frame_delay);
  }
}
