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

fn transform(vertex: Vector3, translation: Vector3, scale: f32, rotation: Vector3) -> Vector3 {
    let (sin_x, cos_x) = (rotation.x * PI / 180.0).sin_cos();
    let (sin_y, cos_y) = (rotation.y * PI / 180.0).sin_cos();
    let (sin_z, cos_z) = (rotation.z * PI / 180.0).sin_cos();

    let mut new_vertex = vertex;

    // Rotate X
    let rotated_y = new_vertex.y * cos_x - new_vertex.z * sin_x;
    let rotated_z = new_vertex.y * sin_x + new_vertex.z * cos_x;
    new_vertex.y = rotated_y;
    new_vertex.z = rotated_z;

    // Rotate Y
    let rotated_x = new_vertex.x * cos_y + new_vertex.z * sin_y;
    let rotated_z = -new_vertex.x * sin_y + new_vertex.z * cos_y;
    new_vertex.x = rotated_x;
    new_vertex.z = rotated_z;

    // Rotate Z
    let rotated_x = new_vertex.x * cos_z - new_vertex.y * sin_z;
    let rotated_y = new_vertex.x * sin_z + new_vertex.y * cos_z;
    new_vertex.x = rotated_x;
    new_vertex.y = rotated_y;

    // Scale
    new_vertex.x *= scale;
    new_vertex.y *= scale;
    new_vertex.z *= scale;

    // Translate
    new_vertex.x += translation.x;
    new_vertex.y += translation.y;
    new_vertex.z += translation.z;

    new_vertex
}

fn render_cube(
    framebuffer: &mut Framebuffer,
    center: Vector3,
    translation: Vector3,
    scale: f32,
    rotation: Vector3,
) {
    let v1 = Vector3::new(center.x - 0.5, center.y - 0.5, center.z - 0.5); 
    let v2 = Vector3::new(center.x + 0.5, center.y - 0.5, center.z - 0.5);
    let v3 = Vector3::new(center.x + 0.5, center.y + 0.5, center.z - 0.5);
    let v4 = Vector3::new(center.x - 0.5, center.y + 0.5, center.z - 0.5);
    let v5 = Vector3::new(center.x - 0.5, center.y - 0.5, center.z + 0.5);
    let v6 = Vector3::new(center.x + 0.5, center.y - 0.5, center.z + 0.5);
    let v7 = Vector3::new(center.x + 0.5, center.y + 0.5, center.z + 0.5);
    let v8 = Vector3::new(center.x - 0.5, center.y + 0.5, center.z + 0.5);

    let t1 = transform(v1, translation, scale, rotation);
    let t2 = transform(v2, translation, scale, rotation);
    let t3 = transform(v3, translation, scale, rotation);
    let t4 = transform(v4, translation, scale, rotation);
    let t5 = transform(v5, translation, scale, rotation);
    let t6 = transform(v6, translation, scale, rotation);
    let t7 = transform(v7, translation, scale, rotation);
    let t8 = transform(v8, translation, scale, rotation);

    // Front face
    triangle(framebuffer, t1, t2, t4);
    triangle(framebuffer, t2, t3, t4);

    // Back face
    triangle(framebuffer, t5, t6, t8);
    triangle(framebuffer, t6, t7, t8);

    // Right face
    triangle(framebuffer, t2, t6, t3);
    triangle(framebuffer, t6, t7, t3);

    // Left face
    triangle(framebuffer, t1, t5, t4);
    triangle(framebuffer, t5, t8, t4);

    // Top face
    triangle(framebuffer, t3, t7, t4);
    triangle(framebuffer, t7, t8, t4);

    // Bottom face
    triangle(framebuffer, t1, t2, t5);
    triangle(framebuffer, t2, t6, t5);
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

    let mut translation = Vector3::new(400.0, 300.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 100.0f32;

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
            scale += 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            scale -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_Q) {
            rotation.x -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            rotation.x += 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_E) {
            rotation.y -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_R) {
            rotation.y += 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_T) {
            rotation.z -= 1.0;
        }
        if window.is_key_down(KeyboardKey::KEY_Y) {
            rotation.z += 1.0;
        }

        framebuffer.clear();
        framebuffer.set_current_color(Color::GREEN);
        let vertex = Vector3::new(0.0, 0.0, 0.0);
        render_cube(&mut framebuffer, vertex, translation, scale, rotation);

        framebuffer.swap_buffers(&mut window, &raylib_thread);

        thread::sleep(Duration::from_millis(16));
    }
}