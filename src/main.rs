mod fragment;
mod framebuffer;
mod mask;
mod matrix;
mod obj;
mod pipeline;
mod shader_examples;
mod shaders;
mod triangle;
mod vertex;

#[cfg(test)]
mod tests;

use framebuffer::Framebuffer;
use noise::OpenSimplex;
use raylib::prelude::*;
use std::{f32::consts::PI, time::Instant};

const WINDOW_WIDTH: usize = 1200;
const WINDOW_HEIGHT: usize = 800;

#[allow(dead_code)]
pub struct Uniforms {
    pub model_matrix: Matrix,
    pub view_matrix: Matrix,
    pub projection_matrix: Matrix,
    pub viewport_matrix: Matrix,
    pub time: f32,
    pub delta_time: f32,
    pub frame_count: u64,
    pub resolution: Vector2,
    pub camera_position: Vector3,
    pub light_position: Vector3,
    pub shader_index: usize,
    pub smooth_normals: bool,
    pub noise: OpenSimplex,
    pub mask: mask::Mask,
}

impl Uniforms {
    fn new(width: usize, height: usize) -> Self {
        let camera_position = Vector3::new(0.0, 0.0, 3.2);
        Self {
            model_matrix: matrix::create_model_matrix(Vector3::zero(), 1.0, Vector3::zero()),
            view_matrix: matrix::create_view_matrix(
                camera_position,
                Vector3::zero(),
                Vector3::new(0.0, 1.0, 0.0),
            ),
            projection_matrix: matrix::create_projection_matrix(
                PI / 3.0,
                width as f32 / height as f32,
                0.1,
                100.0,
            ),
            viewport_matrix: matrix::create_viewport_matrix(0.0, 0.0, width as f32, height as f32),
            time: 0.0,
            delta_time: 0.0,
            frame_count: 0,
            resolution: Vector2::new(width as f32, height as f32),
            camera_position,
            light_position: Vector3::new(3.0, 4.0, 5.0),
            shader_index: 0,
            smooth_normals: true,
            noise: OpenSimplex::new(42),
            mask: mask::Mask::new(),
        }
    }
}

fn render(framebuffer: &mut Framebuffer, vertices: &[vertex::Vertex], uniforms: &Uniforms) {
    framebuffer.clear();
    let transformed: Vec<_> = vertices
        .iter()
        .map(|v| pipeline::vertex_shader(v, uniforms))
        .collect();
    for vertices in transformed.chunks_exact(3) {
        for fragment in triangle::triangle(vertices, uniforms) {
            let index =
                fragment.position.y as usize * framebuffer.width + fragment.position.x as usize;
            if fragment.depth < framebuffer.depths[index] {
                framebuffer.depths[index] = fragment.depth;
                framebuffer.colors[index] = shaders::fragment_shader(&fragment, uniforms);
            }
        }
    }
}

fn gallery(directory: &str, vertices: &[vertex::Vertex]) {
    std::fs::create_dir_all(directory).expect("Cannot create gallery directory");
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut uniforms = Uniforms::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    uniforms.time = 2.0;
    for index in 0..shaders::NAMES.len() {
        uniforms.shader_index = index;
        render(&mut framebuffer, vertices, &uniforms);
        assert!(framebuffer.depths.iter().any(|depth| depth.is_finite()));
        assert!(
            framebuffer
                .colors
                .iter()
                .all(|c| c.x.is_finite() && c.y.is_finite() && c.z.is_finite())
        );
        framebuffer.save(&format!("{directory}/{index:02}.png"));
    }
    uniforms.shader_index = 10;
    render(&mut framebuffer, vertices, &uniforms);
    let scene = framebuffer.colors.clone();
    framebuffer.save(&format!("{directory}/sun-scene.png"));
    framebuffer.colors = shader_examples::passes::bright_regions(&scene);
    framebuffer.save(&format!("{directory}/sun-bright.png"));
    framebuffer.colors = shader_examples::passes::blur(
        &framebuffer.colors,
        framebuffer.width,
        framebuffer.height,
        true,
    );
    framebuffer.colors = shader_examples::passes::blur(
        &framebuffer.colors,
        framebuffer.width,
        framebuffer.height,
        false,
    );
    framebuffer.save(&format!("{directory}/sun-blur.png"));
    framebuffer.colors =
        shader_examples::passes::bloom(&scene, framebuffer.width, framebuffer.height);
    framebuffer.save(&format!("{directory}/sun-bloom.png"));
}

fn main() {
    let vertices = obj::load("assets/models/sphere.obj")
        .expect("Run from the repository root; sphere.obj must exist");
    println!("Sphere: {} triangles", vertices.len() / 3);
    let arguments: Vec<String> = std::env::args().collect();
    if let Some(index) = arguments.iter().position(|arg| arg == "--gallery") {
        gallery(
            arguments
                .get(index + 1)
                .expect("--gallery needs a directory"),
            &vertices,
        );
        return;
    }
    let frame_limit = arguments
        .iter()
        .position(|arg| arg == "--frames")
        .map(|index| {
            arguments[index + 1]
                .parse::<u64>()
                .expect("Invalid frame count")
        });
    let initial_shader = arguments
        .iter()
        .position(|arg| arg == "--shader")
        .map(|index| {
            arguments[index + 1]
                .parse::<usize>()
                .expect("Invalid shader index")
        })
        .unwrap_or(0)
        .min(shaders::NAMES.len() - 1);
    let (mut window, thread) = raylib::init()
        .size(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .title("CPU Shader Workshop")
        .build();
    let image = Image::gen_image_color(WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32, Color::BLACK);
    let mut texture = window.load_texture_from_image(&thread, &image).unwrap();
    let mut framebuffer = Framebuffer::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    let mut uniforms = Uniforms::new(WINDOW_WIDTH, WINDOW_HEIGHT);
    uniforms.shader_index = initial_shader;
    let mut paused = false;
    let mut rotating = false;
    let mut bloom = false;
    let mut rotation = 0.0;
    let mut yaw: f32 = 0.0;
    let mut pitch: f32 = 0.0;
    let mut distance: f32 = 3.2;
    let mut last_frame = Instant::now();
    let mut report_time = Instant::now();
    let mut report_frames = 0;
    let mut render_total = 0.0;

    while !window.window_should_close() {
        let delta = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            paused = !paused;
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            rotating = !rotating;
        }
        if window.is_key_pressed(KeyboardKey::KEY_N) {
            uniforms.smooth_normals = !uniforms.smooth_normals;
        }
        if window.is_key_pressed(KeyboardKey::KEY_B) {
            bloom = !bloom;
        }
        if window.is_key_pressed(KeyboardKey::KEY_HOME) {
            yaw = 0.0;
            pitch = 0.0;
            distance = 3.2;
            rotation = 0.0;
            uniforms.time = 0.0;
        }
        if window.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            uniforms.shader_index = (uniforms.shader_index + 1) % shaders::NAMES.len();
        }
        if window.is_key_pressed(KeyboardKey::KEY_LEFT) {
            uniforms.shader_index =
                (uniforms.shader_index + shaders::NAMES.len() - 1) % shaders::NAMES.len();
        }
        let keys = [
            KeyboardKey::KEY_ZERO,
            KeyboardKey::KEY_ONE,
            KeyboardKey::KEY_TWO,
            KeyboardKey::KEY_THREE,
            KeyboardKey::KEY_FOUR,
            KeyboardKey::KEY_FIVE,
            KeyboardKey::KEY_SIX,
            KeyboardKey::KEY_SEVEN,
            KeyboardKey::KEY_EIGHT,
            KeyboardKey::KEY_NINE,
        ];
        for (index, key) in keys.iter().enumerate() {
            if window.is_key_pressed(*key) {
                uniforms.shader_index = index;
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_F1) {
            uniforms.shader_index = 10;
        }
        if window.is_key_pressed(KeyboardKey::KEY_F2) {
            uniforms.shader_index = 11;
        }
        if window.is_key_down(KeyboardKey::KEY_A) {
            yaw -= delta;
        }
        if window.is_key_down(KeyboardKey::KEY_D) {
            yaw += delta;
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            pitch += delta;
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            pitch -= delta;
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            distance -= delta;
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            distance += delta;
        }
        pitch = pitch.clamp(-1.3, 1.3);
        distance = distance.clamp(2.0, 8.0);
        uniforms.delta_time = if paused { 0.0 } else { delta };
        uniforms.time += uniforms.delta_time;
        if rotating {
            rotation += uniforms.delta_time * 0.35;
        }
        uniforms.camera_position = Vector3::new(
            distance * yaw.sin() * pitch.cos(),
            distance * pitch.sin(),
            distance * yaw.cos() * pitch.cos(),
        );
        uniforms.view_matrix = matrix::create_view_matrix(
            uniforms.camera_position,
            Vector3::zero(),
            Vector3::new(0.0, 1.0, 0.0),
        );
        uniforms.model_matrix =
            matrix::create_model_matrix(Vector3::zero(), 1.0, Vector3::new(0.0, rotation, 0.0));
        let start = Instant::now();
        render(&mut framebuffer, &vertices, &uniforms);
        if bloom {
            framebuffer.colors = shader_examples::passes::bloom(
                &framebuffer.colors,
                framebuffer.width,
                framebuffer.height,
            );
        }
        render_total += start.elapsed().as_secs_f64() * 1000.0;
        texture.update_texture(&framebuffer.bytes()).unwrap();
        {
            let mut drawing = window.begin_drawing(&thread);
            drawing.clear_background(Color::BLACK);
            drawing.draw_texture(&texture, 0, 0, Color::WHITE);
            drawing.draw_text(
                &format!(
                    "{}: {}",
                    uniforms.shader_index,
                    shaders::NAMES[uniforms.shader_index]
                ),
                18,
                16,
                22,
                Color::RAYWHITE,
            );
            drawing.draw_text(
                "0-9 / F1-F2: shader   Left/Right: browse   Space: pause",
                18,
                WINDOW_HEIGHT as i32 - 52,
                16,
                Color::LIGHTGRAY,
            );
            drawing.draw_text(
                "WASD: orbit   Up/Down: zoom   R: rotate   N: normals   B: bloom   Home: reset",
                18,
                WINDOW_HEIGHT as i32 - 26,
                16,
                Color::LIGHTGRAY,
            );
            if paused {
                drawing.draw_text("PAUSED", WINDOW_WIDTH as i32 - 110, 18, 18, Color::GOLD);
            }
            if bloom {
                drawing.draw_text("BLOOM", WINDOW_WIDTH as i32 - 110, 42, 18, Color::GOLD);
            }
        }
        uniforms.frame_count += 1;
        report_frames += 1;
        if report_time.elapsed().as_secs_f64() >= 0.5 {
            let elapsed = report_time.elapsed().as_secs_f64();
            window.set_window_title(
                &thread,
                &format!(
                    "CPU | {} | {:.0} FPS | {:.2} ms/frame | render {:.2} ms",
                    shaders::NAMES[uniforms.shader_index],
                    report_frames as f64 / elapsed,
                    elapsed * 1000.0 / report_frames as f64,
                    render_total / report_frames as f64
                ),
            );
            report_time = Instant::now();
            report_frames = 0;
            render_total = 0.0;
        }
        if frame_limit.is_some_and(|limit| uniforms.frame_count >= limit) {
            break;
        }
    }
}
