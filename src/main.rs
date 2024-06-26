use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f64::consts::PI;
mod framebuffer;
use framebuffer::Framebuffer;
mod maze;
use maze::load_maze;
mod player;
use player::{Player, process_events};
mod caster;
use caster::{cast_ray};

fn cell_to_color(cell: char) -> u32 {
  match cell {
    '+' => {
      return 0xDDFFFF;
    },
    '-' => {
      return 0xFFDDFF;
    },
    '|' => {
      return 0xFFFFDD;
    },
    'g' => {
      return 0xFF0000;
    },
    _ => {
      return 0xFFFFFF;
    },
  }
}

fn draw_stake(framebuffer: &mut Framebuffer, x: usize, h: f64, cell: char) {
  let color = cell_to_color(cell);
  framebuffer.set_current_color(color);

  let start = (450.0 / 2.0 - h / 2.0).floor() as usize;
  let finish = (450.0 / 2.0 + h / 2.0).ceil() as usize;
  
  for y in start..finish {
    framebuffer.point(x, y);
  }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
  if cell == ' ' {
    return;
  }

  let color = cell_to_color(cell);
  framebuffer.set_current_color(color);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.point(x, y);
    }
  } 
}

fn render(framebuffer: &mut Framebuffer, player: &Player) {
  let maze = load_maze("./maze.txt");
  let block_size = 50; 

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
  for i in 0..650 {
    let a = player.a - (player.fov / 2.0) + (player.fov * (i as f64 / 650.0));
    let intersect = cast_ray(framebuffer, &maze, player.x, player.y, a, block_size);
    let x = 650 + i; 
    // let cosine = 1.0 as f64;
    let cosine = (player.a - a).cos() as f64;
    let d = intersect.distance * (framebuffer.width as f64 / framebuffer.height as f64) * cosine;
    let h = (450.0 / d) * 30.0;

    draw_stake(framebuffer, x, h, intersect.impact);
  }
}

fn main() {
  let window_width = 2600;
  let window_height = 900;

  let framebuffer_width = 1300;
  let framebuffer_height = 450;

  let frame_delay = Duration::from_millis(0);

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
    x: 115,
    y: 129,
    a: 0.0,
    fov: PI / 3.0,
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
