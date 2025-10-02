// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;

use framebuffer::Framebuffer;
use vertex::Vertex;
use triangle::triangle;
use shaders::vertex_shader;
use obj::Obj;
use raylib::prelude::*;
use std::thread;
use std::time::Duration;
use std::f32::consts::PI;

pub struct Uniforms {
    pub model_matrix: Matrix,
}

fn create_model_matrix(translation: Vector3, scale: f32, rotation: Vector3) -> Matrix {
    let rotation_matrix_x = Matrix::rotate_x(rotation.x);
    let rotation_matrix_y = Matrix::rotate_y(rotation.y);
    let rotation_matrix_z = Matrix::rotate_z(rotation.z);
    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let scale_matrix = Matrix::scale(scale, scale, scale);
    let translation_matrix = Matrix::translate(translation.x, translation.y, translation.z);

    translation_matrix * rotation_matrix * scale_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Log the first 3 transformed vertices for debugging
    println!("--- Transformed Vertices (first 3) ---");
    for i in 0..3.min(transformed_vertices.len()) {
        println!("Vertex {}: {:?}", i, transformed_vertices[i].transformed_position);
    }

    // Primitive Assembly Stage
    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    // Rasterization Stage
    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    // Fragment Processing Stage
    for fragment in fragments {
        framebuffer.point(
            fragment.position.x as i32,
            fragment.position.y as i32,
            fragment.color
        );
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Rust Graphics - Renderer Example")
        .log_level(TraceLogLevel::LOG_WARNING) // Suppress INFO messages
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);
    framebuffer.set_background_color(Vector3::new(0.2, 0.2, 0.4)); // Dark blue-ish

    // Initialize the texture inside the framebuffer
    framebuffer.init_texture(&mut window, &thread);

    let mut translation = Vector3::new(400.0, 300.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.2f32; // Set scale to 1.2

    let obj = Obj::load("assets/models/anya.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    while !window.window_should_close() {
        handle_input(&mut window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms = Uniforms { model_matrix };

        render(&mut framebuffer, &uniforms, &vertex_array);

        // Call the encapsulated swap_buffers function
        framebuffer.swap_buffers(&mut window, &thread);

        thread::sleep(Duration::from_millis(16));
    }
}

fn handle_input(window: &mut RaylibHandle, translation: &mut Vector3, rotation: &mut Vector3, scale: &mut f32) {
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        translation.x += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        translation.x -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_UP) {
        translation.y -= 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) {
        translation.y += 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_S) {
        *scale += 0.1;
    }
    if window.is_key_down(KeyboardKey::KEY_A) {
        *scale -= 0.1;
    }
    if window.is_key_down(KeyboardKey::KEY_Q) {
        rotation.x -= PI / 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_W) {
        rotation.x += PI / 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_E) {
        rotation.y -= PI / 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_R) {
        rotation.y += PI / 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_T) {
        rotation.z -= PI / 10.0;
    }
    if window.is_key_down(KeyboardKey::KEY_Y) {
        rotation.z += PI / 10.0;
    }
}
