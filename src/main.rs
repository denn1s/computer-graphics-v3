
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
mod framebuffer;

fn main() {
  let window_width = 800;
  let window_height = 600;

  let framebuffer_width = 80;
  let framebuffer_height = 60;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Framebuffer Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  framebuffer.set_background_color(0x333355);

  let mut x = 1 as i32;
  let mut speed = 1 as i32;

  while window.is_open() {
    // listen to inputs
    if window.is_key_down(Key::Escape) {
      break;
    }

    // prepare variables for rendering
    if x as usize == framebuffer_width {
      speed = -1;
    }
    if x == 0 {
      speed = 1;
    }
    x += speed;

    // Clear the framebuffer
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(0xFFDDDD);
    framebuffer.point(x as usize, 40);

    // Update the window with the framebuffer contents
    window
      .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
      .unwrap();

    std::thread::sleep(frame_delay);
  }
}
