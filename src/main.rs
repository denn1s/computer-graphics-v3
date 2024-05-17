mod framebuffer;

// use nalgebra_glm as glm;
use minifb::{Key, Scale, Window, WindowOptions};
use framebuffer::{Framebuffer, WIDTH, HEIGHT};

fn main() {
  let mut window = Window::new(
    "Framebuffer example",
    WIDTH,
    HEIGHT,
    WindowOptions {
      scale: Scale::X16,
      // borderless: true,
      ..WindowOptions::default()
    },
  ).unwrap();
  window.set_target_fps(15);

  let mut framebuffer = Framebuffer::new();

  let mut x: i32 = 40;
  let mut y: i32 = 30;
  let mut speed_x: i32 = 1;
  let mut speed_y: i32 = 1;


  while window.is_open() && !window.is_key_down(Key::Escape) {
    framebuffer.clear();

    if x == WIDTH as i32 || x == 0 {
      speed_x = speed_x * -1;
    }

    if y == HEIGHT as i32 || y == 0 {
      speed_y = speed_y * -1;
    }

    x += speed_x;
    y += speed_y;

    framebuffer.point(x as usize, y as usize, 0xFFFFFF);

    window.update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT).unwrap();
  }
}
