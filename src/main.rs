use nalgebra_glm::{Vec3, Mat3};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;

use framebuffer::Framebuffer;
use triangle::Triangle;

fn render_square(
    framebuffer: &mut Framebuffer,
    start: Vec3,
    size: f32,
    translation: Vec3,
    scale: f32,
    rotation: f32,
) {
    let v1 = start;
    let v2 = Vec3::new(start.x + size, start.y, start.z);
    let v3 = Vec3::new(start.x + size, start.y + size, start.z);
    let v4 = Vec3::new(start.x, start.y + size, start.z);
    let center = Vec3::new(start.x + size / 2.0, start.y + size / 2.0, start.z);

    let t1 = transform_using_matrix2(v1, translation, scale, rotation, center);
    let t2 = transform_using_matrix2(v2, translation, scale, rotation, center);
    let t3 = transform_using_matrix2(v3, translation, scale, rotation, center);
    let t4 = transform_using_matrix2(v4, translation, scale, rotation, center);

    framebuffer.triangle(t1, t2, t4);
    framebuffer.triangle(t2, t3, t4);
}

fn transform(vertex: Vec3, translation: Vec3, scale: f32) -> Vec3 {
    Vec3::new(
        (vertex.x + translation.x) * scale,
        (vertex.y + translation.y) * scale,
        (vertex.z + translation.z) * scale,
    )
}

fn transform_using_matrix(vertex: Vec3, translation: Vec3, scale: f32) -> Vec3 {
    let transform_matrix = Mat3::new(
        scale, 0.0,   translation.x,
        0.0,   scale, translation.y,
        0.0,   0.0,   1.0,
    );

    transform_matrix * vertex
}

fn transform_using_matrix2(vertex: Vec3, translation: Vec3, scale: f32, rotation: f32, center: Vec3) -> Vec3 {
    let move_to_center = Mat3::new(
        1.0,  0.0, -center.x,
        0.0,  1.0, -center.y,
        0.0,  0.0, 1.0
    );
    let transform_matrix = Mat3::new(
        scale, 0.0,   translation.x,
        0.0,   scale, translation.y,
        0.0,   0.0,   1.0,
    );
    let cos_theta = (rotation * PI / 180.0).cos();
    let sin_theta = (rotation * PI / 180.0).sin();
    let rotation_matrix = Mat3::new(
        cos_theta,  -sin_theta, 0.0,
        sin_theta,  cos_theta,  0.0,
        0.0,        0.0,        1.0,
    );
    let move_to_back = Mat3::new(
        1.0,  0.0, center.x,
        0.0,  1.0, center.y,
        0.0,  0.0, 1.0
    );

    // transform_matrix * vertex

    let homogeneous_vertex = Vec3::new(vertex.x, vertex.y, 1.0);
    let homogeneous_vertex_transformed = move_to_back * transform_matrix * rotation_matrix * move_to_center * homogeneous_vertex;
    
    Vec3::new(
        homogeneous_vertex_transformed.x / homogeneous_vertex_transformed.z,
        homogeneous_vertex_transformed.y / homogeneous_vertex_transformed.z,
        vertex.z,
    )
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust Graphics - Framebuffer Example",
        window_width,
        window_height,
        WindowOptions::default(),
    )
        .unwrap();

    // move the window around
    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(0.0, 0.0, 0.0);
    let mut rotation = 0.0;
    let mut scale = 1.0f32;

    while window.is_open() {
        // listen to inputs
        if window.is_key_down(Key::Escape) {
            break;
        }
        if window.is_key_down(Key::Right) {
            translation.x += 1.0;
        }
        if window.is_key_down(Key::Left) {
            translation.x -= 1.0;
        }
        if window.is_key_down(Key::Up) {
            translation.y -= 1.0;
        }
        if window.is_key_down(Key::Down) {
            translation.y += 1.0;
        }
        if window.is_key_down(Key::S) {
            scale += 0.1;
        }
        if window.is_key_down(Key::A) {
            scale -= 0.1;
        }
        if window.is_key_down(Key::E) {
            rotation -= 5.0;
        }
        if window.is_key_down(Key::R) {
            rotation += 5.0;
        }

        // Clear the framebuffer
        framebuffer.clear();

        // Draw some points
        framebuffer.set_current_color(0xFFDDDD);
        let vertex = Vec3::new(30.0, 20.0, 1.0);
        render_square(&mut framebuffer, vertex, 10.0, translation, scale, rotation);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
