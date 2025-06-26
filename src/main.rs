// main.rs

mod line;
mod framebuffer;

use raylib::prelude::*;
use framebuffer::Framebuffer;
use line::line;
use std::thread;
use std::time::Duration;

fn render(
    framebuffer: &mut Framebuffer,
    translate_x: f32,
    translate_y: f32,
) {
    framebuffer.set_current_color(Color::GREEN);
    line(
        framebuffer,
        Vector2::new(50.0 + translate_x, 50.0 + translate_y),
        Vector2::new(350.0 + translate_x, 350.0 + translate_y),
    );

    framebuffer.set_current_color(Color::RED);
    line(
        framebuffer,
        Vector2::new(350.0 + translate_x, 50.0 + translate_y),
        Vector2::new(50.0 + translate_x, 350.0 + translate_y),
    );
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

    let mut translate_x = 0.0;
    let mut translate_y = 0.0;

    while !window.window_should_close() {
        translate_x += 1.0;
        translate_y += 1.0;

        // 1. clear framebuffer
        framebuffer.clear();

        // 2. draw in the screen
        render(&mut framebuffer, translate_x, translate_y);

        // 3. swap buffers
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}



