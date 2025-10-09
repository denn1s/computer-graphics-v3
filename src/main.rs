// main.rs

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod fragment;
mod shaders;
mod obj;
mod matrix;

use crate::matrix::{create_model_matrix, create_view_matrix, create_projection_matrix, create_viewport_matrix};
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
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]) {
    // Vertex Shader Stage
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    // Log the first 3 transformed vertices for debugging
    // println!("--- Transformed Vertices (first 3) ---");
    // for i in 0..3.min(transformed_vertices.len()) {
    //     println!("Vertex {}: {:?}", i, transformed_vertices[i].transformed_position);
    // }

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

    let mut translation = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation = Vector3::new(0.0, 0.0, 0.0);
    let mut scale = 1.0f32;

    // Camera setup
    let camera_position = Vector3::new(0.0, 0.0, 5.0);
    let camera_target = Vector3::new(0.0, 0.0, 0.0);
    let camera_up = Vector3::new(0.0, 1.0, 0.0);

    // Projection setup
    let fov_y = PI / 3.0; // 60 degrees
    let aspect = window_width as f32 / window_height as f32;
    let near = 0.1;
    let far = 100.0;

    let obj = Obj::load("assets/models/anya.obj").expect("Failed to load obj");
    let vertex_array = obj.get_vertex_array();

    while !window.window_should_close() {
        handle_input(&mut window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let view_matrix = create_view_matrix(camera_position, camera_target, camera_up);
        let projection_matrix = create_projection_matrix(fov_y, aspect, near, far);
        let viewport_matrix = create_viewport_matrix(0.0, 0.0, window_width as f32, window_height as f32);

        let uniforms = Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
        };

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
