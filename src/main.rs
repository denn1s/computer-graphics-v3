use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;
mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;
mod player;
use player::{Player, process_events};
mod caster;
use caster::{cast_ray};

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
  if cell == ' ' {
    return;
  }

  framebuffer.set_current_color(0xFFDDDD);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.point(x, y);
    }
  } 
}

fn render(framebuffer: &mut Framebuffer, player: &Player) {
  let maze = load_maze("./maze.txt");
  let block_size = 100;  // 100 pixels each block

  // draw the minimap
  for row in 0..maze.len() {
    for col in 0..maze[row].len() {
      draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
    }
  }

  // draw the player
  framebuffer.set_current_color(0xFFDDDD);
  framebuffer.point(player.x, player.y);

  // draw what the player sees
  cast_ray(framebuffer, &maze, player.x, player.y, player.a, block_size);
}

fn main() {
  let window_width = 1300;
  let window_height = 900;

  let framebuffer_width = 1300;
  let framebuffer_height = 900;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Maze Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  // move the window around
  window.set_position(100, 100);
  window.update();

  // initialize values
  framebuffer.set_background_color(0x333355);
  let mut player = Player {
    x: 150,
    y: 150,
    a: PI / 3.0,
  };

  while window.is_open() {
    // listen to inputs
    if window.is_key_down(Key::Escape) {
      break;
    }
    process_events(&window, &mut player);

    // Clear the framebuffer
    framebuffer.clear();

    // Draw some stuff
    render(&mut framebuffer, &player);

    // Update the window with the framebuffer contents
    window
      .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
      .unwrap();

    std::thread::sleep(frame_delay);
  }
}
