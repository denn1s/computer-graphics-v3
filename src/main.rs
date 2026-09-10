use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod cube;
mod camera;
mod light;
mod material;
mod procedural;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use cube::Cube;
use camera::Camera;
use light::Light;

const ORIGIN_BIAS: f32 = 1e-4;
// 24x24 surface cubes = 576 objects, brute-forced. Bump to 32/48/64 to
// demo why acceleration structures matter (lesson hook).
const TERRAIN_SIZE: i32 = 16;

fn procedural_sky(dir: Vector3) -> Vector3 {
    let d = dir.normalized();
    let t = (d.y + 1.0) * 0.5;

    let green = Vector3::new(0.1, 0.6, 0.2);
    let white = Vector3::new(1.0, 1.0, 1.0);
    let blue = Vector3::new(0.3, 0.5, 1.0);

    if t < 0.54 {
        let k = t / 0.55;
        green * (1.0 - k) + white * k
    } else if t < 0.55 {
        white
    } else if t < 0.8 {
        let k = (t - 0.55) / 0.25;
        white * (1.0 - k) + blue * k
    } else {
        blue
    }
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Cube]) -> bool {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();
    let origin = intersect.point + intersect.normal * ORIGIN_BIAS;
    let inv = Vector3::new(
        1.0 / light_dir.x,
        1.0 / light_dir.y,
        1.0 / light_dir.z,
    );

    for object in objects {
        let hit = object.ray_intersect(&origin, &light_dir, &inv);
        if hit.is_intersecting && hit.distance < light_distance {
            return true;
        }
    }
    false
}

// No reflection / refraction: Minecraft faces are matte.
// One primary ray + one shadow ray per pixel.
pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    inv_dir: &Vector3,
    objects: &[Cube],
    light: &Light,
    light_color: &Vector3,
) -> Vector3 {
    let mut best = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction, inv_dir);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            best = i;
        }
    }

    if !best.is_intersecting {
        return procedural_sky(*ray_direction);
    }

    let light_dir = (light.position - best.point).normalized();
    let view_dir = (*ray_origin - best.point).normalized();
    let normal = best.normal;

    let in_shadow = cast_shadow(&best, light, objects);
    let light_intensity = if in_shadow { 0.0 } else { light.intensity };

    // Flat material color, no textures.
    let diffuse_color = best.material.diffuse;
    let diffuse_intensity = normal.dot(light_dir).max(0.0) * light_intensity;
    let diffuse = diffuse_color * diffuse_intensity;
    // Small ambient so shadowed faces are readable, not pitch black.
    let ambient = diffuse_color * 0.18;

    let reflect_dir = reflect(&-light_dir, &normal).normalized();
    let specular_intensity = view_dir
        .dot(reflect_dir)
        .max(0.0)
        .powf(best.material.specular)
        * light_intensity;
    let specular = *light_color * specular_intensity * 0.3;

    ambient + diffuse + specular
}

// Cheap per-pixel hash for the 50% preview dither. Frame-dependent so the
// grain shimmers while orbiting instead of sitting on a fixed checkerboard.
fn preview_hash(x: u32, y: u32, width: u32, frame: u32) -> u32 {
    let mut h = y
        .wrapping_mul(width)
        .wrapping_add(x)
        .wrapping_add(frame.wrapping_mul(0x9e3779b1));
    h ^= h >> 16;
    h = h.wrapping_mul(0x7feb352d);
    h ^= h >> 15;
    h = h.wrapping_mul(0x846ca68b);
    h ^= h >> 16;
    h
}

pub fn render(
    pixels: &mut [u8],
    width: u32,
    height: u32,
    objects: &[Cube],
    camera: &Camera,
    light: &Light,
    light_color: &Vector3,
    // None = full quality. Some(frame) = shade a random ~50% of pixels,
    // leave the rest black: grainy but ~2x faster while the camera moves.
    preview: Option<u32>,
) {
    let w = width as f32;
    let h = height as f32;
    let aspect_ratio = w / h;
    let perspective_scale = (PI / 3.0 * 0.5).tan();

    // Snapshot camera basis so threads only read Copy values.
    let eye = camera.eye;
    let right = camera.right;
    let up = camera.up;
    let forward = camera.forward;

    let n_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(height as usize)
        .max(1);
    let rows_per_thread = (height as usize + n_threads - 1) / n_threads;
    let stride = width as usize * 4;

    std::thread::scope(|s| {
        for (band_idx, band) in pixels.chunks_mut(rows_per_thread * stride).enumerate() {
            let y_start = band_idx * rows_per_thread;
            // References captured by the scope; no Send bounds on raw pointers needed.
            let objects = objects;
            let light = light;
            let light_color = light_color;
            s.spawn(move || {
                for (row, line) in band.chunks_mut(stride).enumerate() {
                    let y = y_start + row;
                    if y >= height as usize {
                        break;
                    }
                    let sy = (-(2.0 * y as f32) / h + 1.0) * perspective_scale;
                    for x in 0..width as usize {
                        let o = x * 4;
                        if let Some(frame) = preview {
                            if preview_hash(x as u32, y as u32, width, frame) & 1 == 0 {
                                line[o] = 0;
                                line[o + 1] = 0;
                                line[o + 2] = 0;
                                line[o + 3] = 255;
                                continue;
                            }
                        }

                        let sx =
                            ((2.0 * x as f32) / w - 1.0) * aspect_ratio * perspective_scale;

                        // Camera space (sx, sy, -1) -> world. basis_change inlined:
                        // world = right*sx + up*sy - forward*(-1)
                        let mut dir = Vector3::new(
                            right.x * sx + up.x * sy + forward.x,
                            right.y * sx + up.y * sy + forward.y,
                            right.z * sx + up.z * sy + forward.z,
                        )
                        .normalized();
                        // Keep sky gradient stable if basis drifts from perfect orthonormal.
                        dir = dir.normalized();
                        let inv = Vector3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);

                        let c = cast_ray(&eye, &dir, &inv, objects, light, light_color);

                        line[o] = (c.x.clamp(0.0, 1.0) * 255.0) as u8;
                        line[o + 1] = (c.y.clamp(0.0, 1.0) * 255.0) as u8;
                        line[o + 2] = (c.z.clamp(0.0, 1.0) * 255.0) as u8;
                        line[o + 3] = 255;
                    }
                }
            });
        }
    });
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Minecraft Raytracer")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer =
        Framebuffer::new(&mut window, &thread, window_width as u32, window_height as u32);

    let palette = procedural::TerrainPalette::minecraft();
    // Random seed on load so every run shows new terrain; press R for more.
    let mut seed: u32 = rand::random();
    let mut objects =
        procedural::generate_terrain(TERRAIN_SIZE, TERRAIN_SIZE, 6, 0.08, seed, &palette);

    let mut camera = Camera::new(
        Vector3::new(18.0, 15.0, 22.0),
        Vector3::new(0.0, 2.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = PI / 100.0;
    let zoom_speed = 0.5;

    let light = Light::new(
        Vector3::new(10.0, 15.0, 8.0),
        Color::new(255, 255, 255, 255),
        1.1,
    );
    let light_color = Vector3::new(1.0, 1.0, 1.0);

    let fb_w = framebuffer.width;
    let fb_h = framebuffer.height;
    render(
        framebuffer.pixels_mut(),
        fb_w,
        fb_h,
        &objects,
        &camera,
        &light,
        &light_color,
        None,
    );
    // Consume the initial "changed" flag so we don't render twice.
    camera.is_changed();
    let mut hud = format!(
        "Seed: {} | Cubes: {} | [R] regenerate",
        seed,
        objects.len()
    );
    let mut frame: u32 = 0;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_R) || window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            seed = rand::random();
            objects =
                procedural::generate_terrain(TERRAIN_SIZE, TERRAIN_SIZE, 6, 0.08, seed, &palette);
            hud = format!(
                "Seed: {} | Cubes: {} | [R] regenerate",
                seed,
                objects.len()
            );
            let fb_w = framebuffer.width;
            let fb_h = framebuffer.height;
            render(
                framebuffer.pixels_mut(),
                fb_w,
                fb_h,
                &objects,
                &camera,
                &light,
                &light_color,
                None,
            );
        }
        if window.is_key_down(KeyboardKey::KEY_LEFT) {
            camera.orbit(rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_RIGHT) {
            camera.orbit(-rotation_speed, 0.0);
        }
        if window.is_key_down(KeyboardKey::KEY_UP) {
            camera.orbit(0.0, -rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_DOWN) {
            camera.orbit(0.0, rotation_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_W) {
            camera.zoom(zoom_speed);
        }
        if window.is_key_down(KeyboardKey::KEY_S) {
            camera.zoom(-zoom_speed);
        }

        // While orbiting/zooming: grainy 50% preview (~2x faster).
        // On release: one full-quality pass.
        let moving = window.is_key_down(KeyboardKey::KEY_LEFT)
            || window.is_key_down(KeyboardKey::KEY_RIGHT)
            || window.is_key_down(KeyboardKey::KEY_UP)
            || window.is_key_down(KeyboardKey::KEY_DOWN)
            || window.is_key_down(KeyboardKey::KEY_W)
            || window.is_key_down(KeyboardKey::KEY_S);
        if moving {
            frame = frame.wrapping_add(1);
            let fb_w = framebuffer.width;
            let fb_h = framebuffer.height;
            render(
                framebuffer.pixels_mut(),
                fb_w,
                fb_h,
                &objects,
                &camera,
                &light,
                &light_color,
                Some(frame),
            );
            // NOTE: do NOT drain camera.is_changed() here. The flag stays set
            // so the first still frame below runs one full-quality pass.
        } else if camera.is_changed() {
            let fb_w = framebuffer.width;
            let fb_h = framebuffer.height;
            render(
                framebuffer.pixels_mut(),
                fb_w,
                fb_h,
                &objects,
                &camera,
                &light,
                &light_color,
                None,
            );
        }

        framebuffer.present(&mut window, &thread, &hud, moving);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_terrain_renders_cubes() {
        // No window needed: exercises slab test, OpenSimplex terrain, lighting.
        let palette = procedural::TerrainPalette::minecraft();
        let objects = procedural::generate_terrain(16, 16, 6, 0.08, 1337, &palette);
        assert_eq!(objects.len(), 256);

        // A ray straight down over the terrain center must hit a cube.
        let origin = Vector3::new(0.0, 20.0, 0.0);
        let dir = Vector3::new(0.0, -1.0, 0.0).normalized();
        let inv = Vector3::new(1.0 / dir.x, 1.0 / dir.y, 1.0 / dir.z);
        let light = Light::new(
            Vector3::new(10.0, 15.0, 8.0),
            Color::new(255, 255, 255, 255),
            1.1,
        );
        let light_color = Vector3::new(1.0, 1.0, 1.0);
        let c = cast_ray(&origin, &dir, &inv, &objects, &light, &light_color);
        assert!(c.x > 0.0 || c.y > 0.0 || c.z > 0.0);

        // Full small-frame render: all alpha bytes must stay opaque,
        // and a healthy fraction of pixels must be terrain (not sky).
        let w = 160u32;
        let h = 120u32;
        let mut pixels = vec![0u8; (w * h * 4) as usize];
        let camera = Camera::new(
            Vector3::new(18.0, 15.0, 22.0),
            Vector3::new(0.0, 2.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
        );
        render(&mut pixels, w, h, &objects, &camera, &light, &light_color, None);
        assert!(pixels.chunks_exact(4).all(|px| px[3] == 255));
        let lit = pixels
            .chunks_exact(4)
            .filter(|px| px[0] > 10 || px[1] > 10 || px[2] > 10)
            .count();
        assert!(lit > pixels.len() / 4 / 10, "frame looks empty");

        // Preview mode shades roughly half the pixels; the rest stay black.
        // Shaded pixels are never pure black (ambient lift), so black == skipped.
        let mut pv = vec![0u8; (w * h * 4) as usize];
        render(&mut pv, w, h, &objects, &camera, &light, &light_color, Some(7));
        let total = (w * h) as usize;
        let skipped = pv
            .chunks_exact(4)
            .filter(|px| px[0] == 0 && px[1] == 0 && px[2] == 0)
            .count();
        assert!(
            skipped > total * 35 / 100 && skipped < total * 65 / 100,
            "preview should skip ~50%, skipped {skipped}/{total}"
        );
    }
}
