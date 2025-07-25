use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere;

use framebuffer::Framebuffer;
use ray_intersect::RayIntersect;
use sphere::Sphere;

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Sphere],
) -> Color {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return Color::new(157, 165, 189, 255);
        }
    }
    Color::new(4, 12, 36, 255)
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;
    let fov = PI / 3.0;
    let perspective_scale = (fov * 0.5).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, -1.0).normalized();
            let ray_origin = Vector3::new(0.0, 0.0, 0.0);

            let pixel_color = cast_ray(&ray_origin, &ray_direction, objects);

            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Raytracer")
        .build();

    let mut framebuffer = Framebuffer::new(800, 600);

    let objects = [Sphere {
        center: Vector3::new(0.0, 0.0, -5.0),
        radius: 1.0,
    }];

    while !rl.window_should_close() {
        render(&mut framebuffer, &objects);

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        let texture = d
            .load_texture_from_image(&thread, &framebuffer.color_buffer)
            .unwrap();
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}