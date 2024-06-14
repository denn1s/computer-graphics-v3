use nalgebra_glm::{Vec3, Vec4, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;

use framebuffer::Framebuffer;
use triangle::Triangle;

fn transform(vertex: Vec3, translation: Vec3, scale: f32, rotation: Vec3) -> Vec3 {
    let (sin_x, cos_x) = rotation.x.to_radians().sin_cos();
    let (sin_y, cos_y) = rotation.y.to_radians().sin_cos();
    let (sin_z, cos_z) = rotation.z.to_radians().sin_cos();

    // Rotation matrix around the X axis
    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    // Rotation matrix around the Y axis
    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    // Rotation matrix around the Z axis
    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    // Combine the rotation matrices
    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    // Scale and translation matrix
    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    // Apply the transformation
    let augmented_vertex = Vec4::new(vertex.x, vertex.y, vertex.z, 1.0);
    let transformed_vertex = transform_matrix * rotation_matrix * augmented_vertex;

    Vec3::new(
        transformed_vertex.x / transformed_vertex.w,
        transformed_vertex.y / transformed_vertex.w,
        transformed_vertex.z / transformed_vertex.w,
    )
}

fn render_cube(
    framebuffer: &mut Framebuffer,
    center: Vec3,
    translation: Vec3,
    scale: f32,
    rotation: Vec3,
) {
    let v1 = Vec3::new(center.x - 0.5, center.y - 0.5, center.z - 0.5); 
    let v2 = Vec3::new(center.x + 0.5, center.y - 0.5, center.z - 0.5);
    let v3 = Vec3::new(center.x + 0.5, center.y + 0.5, center.z - 0.5);
    let v4 = Vec3::new(center.x - 0.5, center.y + 0.5, center.z - 0.5);
    let v5 = Vec3::new(center.x - 0.5, center.y - 0.5, center.z + 0.5);
    let v6 = Vec3::new(center.x + 0.5, center.y - 0.5, center.z + 0.5);
    let v7 = Vec3::new(center.x + 0.5, center.y + 0.5, center.z + 0.5);
    let v8 = Vec3::new(center.x - 0.5, center.y + 0.5, center.z + 0.5);

    let t1 = transform(v1, translation, scale, rotation);
    let t2 = transform(v2, translation, scale, rotation);
    let t3 = transform(v3, translation, scale, rotation);
    let t4 = transform(v4, translation, scale, rotation);
    let t5 = transform(v5, translation, scale, rotation);
    let t6 = transform(v6, translation, scale, rotation);
    let t7 = transform(v7, translation, scale, rotation);
    let t8 = transform(v8, translation, scale, rotation);

    // Front face
    framebuffer.triangle(t1, t2, t4);
    framebuffer.triangle(t2, t3, t4);

    // Back face
    framebuffer.triangle(t5, t6, t8);
    framebuffer.triangle(t6, t7, t8);

    // Right face
    framebuffer.triangle(t2, t6, t3);
    framebuffer.triangle(t6, t7, t3);

    // Left face
    framebuffer.triangle(t1, t5, t4);
    framebuffer.triangle(t5, t8, t4);

    // Top face
    framebuffer.triangle(t3, t7, t4);
    framebuffer.triangle(t7, t8, t4);

    // Bottom face
    framebuffer.triangle(t1, t2, t5);
    framebuffer.triangle(t2, t6, t5);
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
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

    let mut translation = Vec3::new(300.0, 200.0, 0.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut scale = 100.0f32;

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
        if window.is_key_down(Key::Q) {
            rotation.x -= PI / 10.0;
        }
        if window.is_key_down(Key::W) {
            rotation.x += PI / 10.0;
        }
        if window.is_key_down(Key::E) {
            rotation.y -= PI / 10.0;
        }
        if window.is_key_down(Key::R) {
            rotation.y += PI / 10.0;
        }
        if window.is_key_down(Key::T) {
            rotation.z -= PI / 10.0;
        }
        if window.is_key_down(Key::Y) {
            rotation.z += PI / 10.0;
        }


        // Clear the framebuffer
        framebuffer.clear();

        // Draw some points
        framebuffer.set_current_color(0xFFDDDD);
        let vertex = Vec3::new(0.0, 0.0, 0.0);
        render_cube(&mut framebuffer, vertex, translation, scale, rotation);

        // Update the window with the framebuffer contents
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
