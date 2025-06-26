// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

mod line;
mod framebuffer;
mod maze;
mod caster;
mod player;

use line::line;
use maze::{Maze,load_maze};
use caster::{cast_ray};
use framebuffer::Framebuffer;
use player::{Player, process_events};

use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;


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

    framebuffer.set_current_color(Color::new(0xFF, 0xDD, 0xDD, 0xFF));

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

    let px = player.pos.x as u32;
    let py = player.pos.y as u32;

    framebuffer.set_pixel(px, py);

    cast_ray(framebuffer, &maze, &player, block_size);
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
    };

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();

        // 2. move the player on user input
        process_events(&mut player, &window);

        // 3 draw the maze, top down view
        render_maze(&mut framebuffer, &maze, block_size, &player);

        // 4. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
