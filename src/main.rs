// main.rs

mod framebuffer;
mod line;
mod triangle;

use framebuffer::Framebuffer;
use triangle::triangle;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;

fn render(framebuffer: &mut Framebuffer) {
    // Clear the framebuffer
    framebuffer.clear();

    // Draw some points
    framebuffer.set_current_color(Color::GREEN);
    let v1 = Vector2::new(100.0, 100.0);
    let v2 = Vector2::new(200.0, 100.0);
    let v3 = Vector2::new(150.0, 200.0);
    triangle(framebuffer, v1, v2, v3);
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Window Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    framebuffer.set_background_color(Color::new(50, 50, 100, 255));

    while !window.window_should_close() {
        // 1. clear framebuffer
        framebuffer.clear();

        // 2. draw in the screen
        render(&mut framebuffer);

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}