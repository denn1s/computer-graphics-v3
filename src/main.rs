use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere;
mod camera;
mod light;
mod material;

use framebuffer::Framebuffer;
use ray_intersect::{Intersect, RayIntersect};
use sphere::Sphere;
use camera::Camera;
use light::Light;
use material::Material;

const ORIGIN_BIAS: f32 = 1e-4;
const SKYBOX_COLOR: Color = Color::new(68, 142, 228, 255);

fn offset_origin(intersect: &Intersect, direction: &Vector3) -> Vector3 {
    let offset = intersect.normal * ORIGIN_BIAS;
    if direction.dot(intersect.normal) < 0.0 {
        intersect.point - offset
    } else {
        intersect.point + offset
    }
}

fn reflect(incident: &Vector3, normal: &Vector3) -> Vector3 {
    *incident - *normal * 2.0 * incident.dot(*normal)
}

fn refract(incident: &Vector3, normal: &Vector3, eta_t: f32) -> Vector3 {
    let cosi = -incident.dot(*normal).max(-1.0).min(1.0);
    
    let (eta, n_normal);

    if cosi < 0.0 {
        eta = 1.0 / eta_t;
        n_normal = *normal;
    } else {
        eta = eta_t;
        n_normal = -*normal;
    }
    
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    
    if k < 0.0 {
        reflect(incident, &n_normal)
    } else {
        *incident * eta + n_normal * (eta * cosi.abs() - k.sqrt())
    }
}

fn cast_shadow(
    intersect: &Intersect,
    light: &Light,
    objects: &[Sphere],
) -> f32 {
    let light_dir = (light.position - intersect.point).normalized();
    let light_distance = (light.position - intersect.point).length();

    let shadow_ray_origin = offset_origin(intersect, &light_dir);

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting && shadow_intersect.distance < light_distance {
            return 1.0; // Hit something, full shadow
        }
    }

    0.0 // No shadow
}

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Sphere],
    light: &Light,
    depth: u32,
) -> Color {
    if depth > 3 {
        return SKYBOX_COLOR;
    }

    let mut intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let i = object.ray_intersect(ray_origin, ray_direction);
        if i.is_intersecting && i.distance < zbuffer {
            zbuffer = i.distance;
            intersect = i;
        }
    }

    if !intersect.is_intersecting {
        return SKYBOX_COLOR;
    }

    let light_dir = (light.position - intersect.point).normalized();
    let view_dir = (*ray_origin - intersect.point).normalized();
    let reflect_dir = reflect(&-light_dir, &intersect.normal).normalized();

    let shadow_intensity = cast_shadow(&intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    let diffuse_intensity = intersect.normal.dot(light_dir).max(0.0);
    let diffuse_color = intersect.material.diffuse;
    let diffuse = Color::new(
        (diffuse_color.r as f32 * diffuse_intensity * light_intensity) as u8,
        (diffuse_color.g as f32 * diffuse_intensity * light_intensity) as u8,
        (diffuse_color.b as f32 * diffuse_intensity * light_intensity) as u8,
        255,
    );

    let specular_intensity = view_dir.dot(reflect_dir).max(0.0).powf(intersect.material.specular);
    let specular_color = light.color;
    let specular = Color::new(
        (specular_color.r as f32 * specular_intensity * light_intensity) as u8,
        (specular_color.g as f32 * specular_intensity * light_intensity) as u8,
        (specular_color.b as f32 * specular_intensity * light_intensity) as u8,
        255,
    );

    let mut reflect_color = Color::BLACK;
    let reflectivity = intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let reflect_dir = reflect(ray_direction, &intersect.normal).normalized();
        let reflect_origin = offset_origin(&intersect, &reflect_dir);
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    let mut refract_color = Color::BLACK;
    let transparency = intersect.material.albedo[3];
    if transparency > 0.0 {
        let refract_dir = refract(ray_direction, &intersect.normal, intersect.material.refractive_index);
        let refract_origin = offset_origin(&intersect, &refract_dir);
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    let albedo = intersect.material.albedo;
    let base_color = Color::new(
        (diffuse.r as f32 * albedo[0] + specular.r as f32 * albedo[1]) as u8,
        (diffuse.g as f32 * albedo[0] + specular.g as f32 * albedo[1]) as u8,
        (diffuse.b as f32 * albedo[0] + specular.b as f32 * albedo[1]) as u8,
        255,
    );

    Color::new(
        (base_color.r as f32 * (1.0 - reflectivity - transparency) + reflect_color.r as f32 * reflectivity + refract_color.r as f32 * transparency) as u8,
        (base_color.g as f32 * (1.0 - reflectivity - transparency) + reflect_color.g as f32 * reflectivity + refract_color.g as f32 * transparency) as u8,
        (base_color.b as f32 * (1.0 - reflectivity - transparency) + reflect_color.b as f32 * reflectivity + refract_color.b as f32 * transparency) as u8,
        255,
    )
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
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
            
            let rotated_direction = camera.basis_change(&ray_direction);

            let pixel_color = cast_ray(&camera.eye, &rotated_direction, objects, light, 0);

            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
 
    let (mut window, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    let rubber = Material::new(
        Color::new(80, 0, 0, 255),
        1.0,
        [0.9, 0.1, 0.0, 0.0],
        0.0,
    );

    let ivory = Material::new(
        Color::new(100, 100, 80, 255),
        50.0,
        [0.6, 0.3, 0.1, 0.0],
        0.0,
    );

    let glass = Material::new(
        Color::new(255, 255, 255, 255),
        1425.0,
        [0.0, 0.5, 0.1, 0.8],
        1.5,
    );

    let objects = [
        Sphere { center: Vector3::new(0.0, 0.0, 0.0), radius: 1.0, material: rubber },
        Sphere { center: Vector3::new(-1.0, -1.0, 1.5), radius: 0.5, material: ivory },
        Sphere { center: Vector3::new(-0.3, 0.3, 1.5), radius: 0.3, material: glass },
    ];

    let mut camera = Camera::new(
        Vector3::new(0.0, 0.0, 5.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );
    let rotation_speed = PI / 100.0;

    let light = Light::new(
        Vector3::new(1.0, -1.0, 5.0),
        Color::new(255, 255, 255, 255),
        1.0,
    );

    while !window.window_should_close() {
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

        framebuffer.clear();
        render(&mut framebuffer, &objects, &camera, &light);
        framebuffer.swap_buffers(&mut window, &thread);
    }
}
