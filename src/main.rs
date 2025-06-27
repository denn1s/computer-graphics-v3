// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;
mod textures;

use line::line;
use maze::{Maze, load_maze};
use caster::{cast_ray, Intersect};
use framebuffer::Framebuffer;
use player::{Player, process_events};
use textures::TextureManager;

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;
mod enemy;
use enemy::{Enemy};

const TRANSPARENT_COLOR: Color = Color::new(152, 0, 136, 255);

fn draw_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    enemy: &Enemy,
    texture_manager: &TextureManager
) {
    // Calculate angle from player to enemy
    let sprite_a = (enemy.pos.y - player.pos.y).atan2(enemy.pos.x - player.pos.x);

    // Normalize angle difference to [-PI, PI]
    let mut angle_diff = sprite_a - player.a;
    while angle_diff > std::f32::consts::PI {
        angle_diff -= 2.0 * std::f32::consts::PI;
    }
    while angle_diff < -std::f32::consts::PI {
        angle_diff += 2.0 * std::f32::consts::PI;
    }

    // If enemy is outside player's FOV, skip drawing
    if angle_diff.abs() > player.fov / 2.0 {
        return;
    }

    // Distance from player to enemy
    let sprite_d = ((player.pos.x - enemy.pos.x).powi(2) + (player.pos.y - enemy.pos.y).powi(2)).sqrt();

    if sprite_d < 50.0 || sprite_d > 1000.0 {
        return;
    }

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    // Calculate sprite size on screen (scale inversely proportional to distance)
    let sprite_size = (screen_height / sprite_d) * 70.0;

    // Calculate horizontal screen position (centered)
    let screen_x = ((angle_diff / player.fov) + 0.5) * screen_width;

    // Calculate top-left corner of sprite on screen
    let start_x = (screen_x - sprite_size / 2.0).max(0.0) as usize;
    let start_y = (screen_height / 2.0 - sprite_size / 2.0).max(0.0) as usize;

    let sprite_size_usize = sprite_size as usize;

    let end_x = (start_x + sprite_size_usize).min(framebuffer.width as usize);
    let end_y = (start_y + sprite_size_usize).min(framebuffer.height as usize);

    for x in start_x..end_x {
        for y in start_y..end_y {
            // Map screen pixel to texture coordinates (assuming 128x128 texture)
            let tx = ((x - start_x) * 128 / sprite_size_usize) as u32;
            let ty = ((y - start_y) * 128 / sprite_size_usize) as u32;

            let color = texture_manager.get_pixel_color('e', tx, ty);

            // Skip transparent pixels
            if color != TRANSPARENT_COLOR {
                framebuffer.set_current_color(color);
                framebuffer.set_pixel(x as u32, y as u32);
            }
        }
    }
}


fn draw_cell(
  framebuffer: &mut Framebuffer,
  xo: usize,
  yo: usize,
  block_size: usize,
  cell: char,
) {
  if cell == ' ' {
    return;
  }
  framebuffer.set_current_color(Color::WHITE);

  for x in xo..xo + block_size {
    for y in yo..yo + block_size {
      framebuffer.set_pixel(x as u32, y as u32);
    }
  }
}

pub fn render_maze(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
) {
  for (row_index, row) in maze.iter().enumerate() {
    for (col_index, &cell) in row.iter().enumerate() {
      let xo = col_index * block_size;
      let yo = row_index * block_size;
      draw_cell(framebuffer, xo, yo, block_size, cell);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  let num_rays = 5;
  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    cast_ray(framebuffer, &maze, &player, a, block_size, true);
  }
}

fn render_world(
  framebuffer: &mut Framebuffer,
  maze: &Maze,
  block_size: usize,
  player: &Player,
  texture_cache: &TextureManager,
) {
  let num_rays = framebuffer.width;
  let hh = framebuffer.height as f32 / 2.0;

  // Draw sky and floor
  for i in 0..framebuffer.width {
    framebuffer.set_current_color(Color::SKYBLUE);
    for j in 0..(framebuffer.height / 2) {
      framebuffer.set_pixel(i, j);
    }
    framebuffer.set_current_color(Color::GAINSBORO);
    for j in (framebuffer.height / 2)..framebuffer.height {
      framebuffer.set_pixel(i, j);
    }
  }

  framebuffer.set_current_color(Color::WHITESMOKE);

  for i in 0..num_rays {
    let current_ray = i as f32 / num_rays as f32;
    let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
    let intersect = cast_ray(framebuffer, &maze, &player, a, block_size, false);

    let distance_to_wall = intersect.distance;
    let distance_to_projection_plane = 70.0;
    let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

    let stake_top = (hh - (stake_height / 2.0)) as usize;
    let stake_bottom = (hh + (stake_height / 2.0)) as usize;

    for y in stake_top..stake_bottom {
      let ty = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32) * 128.0;

      let color = texture_cache.get_pixel_color(intersect.impact, intersect.tx as u32, ty as u32);
      framebuffer.set_current_color(color);
      framebuffer.set_pixel(i, y as u32);
    }
  }
}

fn render_enemies(framebuffer: &mut Framebuffer, player: &Player, texture_cache: &TextureManager) {
  let enemies = vec![
    Enemy::new(250.0, 250.0, 'e'),
    // Enemy::new(450.0, 450.0, 'e'),
    // Enemy::new(650.0, 650.0, 'e'),
  ];

  for enemy in &enemies {
    draw_sprite(framebuffer, &player, enemy, texture_cache);
  }
}

fn main() {
  let window_width = 1300;
  let window_height = 900;
  let block_size = 100;

  let (mut window, raylib_thread) = raylib::init()
    .size(window_width, window_height)
    .title("Raycaster Example")
    .log_level(TraceLogLevel::LOG_WARNING)
    .build();

  let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
  framebuffer.set_background_color(Color::new(50, 50, 100, 255));

  let maze = load_maze("maze.txt");
  let mut player = Player {
    pos: Vector2::new(150.0, 150.0),
    a: PI / 3.0,
    fov: PI / 3.0,
  };

  // Initialize texture cache once
  let texture_cache = TextureManager::new(&mut window, &raylib_thread);

  while !window.window_should_close() {
    framebuffer.clear();

    process_events(&mut player, &window);

    let mut mode = "3D";

    if window.is_key_down(KeyboardKey::KEY_M) {
      mode = if mode == "2D" { "3D" } else { "2D" };
    }

    if mode == "2D" {
      render_maze(&mut framebuffer, &maze, block_size, &player);
    } else {
      render_world(&mut framebuffer, &maze, block_size, &player, &texture_cache);
      render_enemies(&mut framebuffer, &player, &texture_cache);
    }

    framebuffer.swap_buffers(&mut window, &raylib_thread);

    thread::sleep(Duration::from_millis(16));
  }
}
