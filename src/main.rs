use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod framebuffer;
use framebuffer::Framebuffer;

fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

fn draw_cell(framebuffer: &mut Framebuffer, x: usize, y: usize, block_size: usize, cell: char) {
  if cell == ' ' {
    return;
  }

  for x in x..x + block_size {
    for y in y..y + block_size {
      match cell {
        '+' | '-' | '|' => {
          framebuffer.set_current_color(0xFFDDDD);
          framebuffer.point(x, y);
        },
        _ => (),
      }
    }
  } 
}

fn render(framebuffer: &mut Framebuffer) {
  let maze = load_maze("./maze.txt");
  let block_size = 100;  // 10 pixels each block

  for row in 0..maze.len() {
    for col in 0..maze[row].len() {
      let x = col * block_size;
      let y = row * block_size;
      draw_cell(framebuffer, x, y, block_size, maze[row][col]);
    }
  } 
}

fn main() {
  let window_width = 1300;
  let window_height = 900;

  let framebuffer_width = 1300;
  let framebuffer_height = 900;

  let frame_delay = Duration::from_millis(16);

  let mut framebuffer = framebuffer::Framebuffer::new(framebuffer_width, framebuffer_height);

  let mut window = Window::new(
    "Rust Graphics - Maze Example",
    window_width,
    window_height,
    WindowOptions::default(),
  ).unwrap();

  // move the window around
  window.set_position(100, 100);
  window.update();

  framebuffer.set_background_color(0x333355);

  while window.is_open() {
    // listen to inputs
    if window.is_key_down(Key::Escape) {
      break;
    }

    // Clear the framebuffer
    framebuffer.clear();

    // Draw some stuff
    render(&mut framebuffer);

    // Update the window with the framebuffer contents
    window
      .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
      .unwrap();

    std::thread::sleep(frame_delay);
  }
}
