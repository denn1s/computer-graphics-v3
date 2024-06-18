use minifb::{Key, Window};
use std::f32::consts::PI;

pub struct Player {
  pub x: usize,
  pub y: usize,
  pub a: f32, // angle of view 
}

pub fn process_events(window: &Window, player: &mut Player) {
  const MOVE_SPEED: f32 = 10.0;
  const ROTATION_SPEED: f32 = PI / 10.0;

  if window.is_key_down(Key::Left) {
    player.a += ROTATION_SPEED;
  }
  if window.is_key_down(Key::Right) {
    player.a -= ROTATION_SPEED;
  }
  if window.is_key_down(Key::Up) {
    player.x = (player.x as f32 + MOVE_SPEED * player.a.cos()) as usize;
    player.y = (player.y as f32 + MOVE_SPEED * player.a.sin()) as usize;
  }
}
