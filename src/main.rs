// main.rs

mod framebuffer;
mod triangle;
mod line;

use framebuffer::Framebuffer;
use triangle::triangle;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

fn transform(vertex: Vector3, translation: Vector3, scale: f32, rotation: f32, center: Vector3) -> Vector3 {
    let mut new_vertex = vertex;

    // Translate to origin
    new_vertex.x -= center.x;
    new_vertex.y -= center.y;

    // Rotate
    let cos_theta = (rotation * PI / 180.0).cos();
    let sin_theta = (rotation * PI / 180.0).sin();
    let rotated_x = new_vertex.x * cos_theta - new_vertex.y * sin_theta;
    let rotated_y = new_vertex.x * sin_theta + new_vertex.y * cos_theta;
    new_vertex.x = rotated_x;
    new_vertex.y = rotated_y;

    // Scale
    new_vertex.x *= scale;
    new_vertex.y *= scale;

    // Translate back
    new_vertex.x += center.x;
    new_vertex.y += center.y;

    // Translate
    new_vertex.x += translation.x;
    new_vertex.y += translation.y;

    new_vertex
}

fn render(
    framebuffer: &mut Framebuffer,
    translation: Vector3,
    scale: f32,
    rotation: f32,
) {
    framebuffer.clear();
    framebuffer.set_current_color(Color::GREEN);

    let v1 = Vector3::new(100.0, 100.0, 0.0);
    let v2 = Vector3::new(200.0, 100.0, 0.0);
    let v3 = Vector3::new(150.0, 200.0, 0.0);

    let center = Vector3::new((v1.x + v2.x + v3.x) / 3.0, (v1.y + v2.y + v3.y) / 3.0, 0.0);

    let t1 = transform(v1, translation, scale, rotation, center);
    let t2 = transform(v2, translation, scale, rotation, center);
    let t3 = transform(v3, translation, scale, rotation, center);

    triangle(framebuffer, t1, t2, t3);
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

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation = 0.0;
    let mut scale = 1.0f32;

    while !window.window_should_close() {
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            translation.x += 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            translation.x -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            translation.y -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            translation.y += 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            scale += 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            scale -= 0.1;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            rotation -= 5.0;
        }
        if window.is_key_down(KeyboardKey::KEY_R) {
            rotation += 5.0;
        }

        render(&mut framebuffer, translation, scale, rotation);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}
