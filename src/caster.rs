// caster.rs

use raylib::color::Color;

use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub fn cast_ray(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player, a: f32, block_size: usize) {
  let mut d = 0.0;

  framebuffer.set_current_color(Color::WHITESMOKE);

  loop {
    let cos = d * a.cos();
    let sin = d * a.sin();
    let x = (player.pos.x + cos) as usize;
    let y = (player.pos.y + sin) as usize;

    let i = x / block_size;
    let j = y / block_size;

    if maze[j][i] != ' ' {
      return;
    }

    framebuffer.set_pixel(x as u32, y as u32);

    d += 10.0;
  }
}


