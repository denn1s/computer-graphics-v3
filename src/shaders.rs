use nalgebra_glm::{Vec3, Vec4, Mat3, dot, mat4_to_mat3, mix};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;
use std::f32::consts::PI;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Transform position
  let position = Vec4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );
  let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

  // Perform perspective division
  let w = transformed.w;
  let ndc_position = Vec4::new(
    transformed.x / w,
    transformed.y / w,
    transformed.z / w,
    1.0
  );

  // apply viewport matrix
  let screen_position = uniforms.viewport_matrix * ndc_position;

  // Transform normal
  let model_mat3 = mat4_to_mat3(&uniforms.model_matrix); 
  let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

  let transformed_normal = normal_matrix * vertex.normal;

  // Create a new Vertex with transformed attributes
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
    transformed_normal,
  }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  // cloud_shader(fragment, uniforms)
  // cellular_shader(fragment, uniforms)
  // cracked_ground_shader(fragment, uniforms)
  lava_shader(fragment, uniforms)
}

fn cloud_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 100.0;  // to move our values 
  let ox = 100.0; // offset x in the noise map
  let oy = 100.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;
  let t = uniforms.time as f32 * 0.5;

  let noise_value = uniforms.noise.get_noise_2d(x * zoom + ox + t, y * zoom + oy);

  // Define cloud threshold and colors
  let cloud_threshold = 0.5; // Adjust this value to change cloud density
  let cloud_color = Color::new(255, 255, 255); // White for clouds
  let sky_color = Color::new(30, 97, 145); // Sky blue

  // Determine if the pixel is part of a cloud or sky
  let noise_color = if noise_value > cloud_threshold {
    cloud_color
  } else {
    sky_color
  };

  noise_color * fragment.intensity
}

fn cellular_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 30.0;  // Zoom factor to adjust the scale of the cell pattern
  let ox = 50.0;    // Offset x in the noise map
  let oy = 50.0;    // Offset y in the noise map
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Use a cellular noise function to create the plant cell pattern
  let cell_noise_value = uniforms.noise.get_noise_2d(x * zoom + ox, y * zoom + oy).abs();

  // Define different shades of green for the plant cells
  let cell_color_1 = Color::new(85, 107, 47);   // Dark olive green
  let cell_color_2 = Color::new(124, 252, 0);   // Light green
  let cell_color_3 = Color::new(34, 139, 34);   // Forest green
  let cell_color_4 = Color::new(173, 255, 47);  // Yellow green

  // Use the noise value to assign a different color to each cell
  let final_color = if cell_noise_value < 0.15 {
    cell_color_1
  } else if cell_noise_value < 0.7 {
    cell_color_2
  } else if cell_noise_value < 0.75 {
    cell_color_3
  } else {
    cell_color_4
  };

  // Adjust intensity to simulate lighting effects (optional)
  final_color * fragment.intensity
}

fn lava_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
  let zoom = 10.0;
  let x = fragment.vertex_position.x;
  let y = fragment.vertex_position.y;

  // Get the base FBm noise value at this point
  let noise_value = uniforms.noise.get_noise_2d(x * zoom, y * zoom);

  // Edge detection: Calculate noise values at neighboring points
  let noise_dx = uniforms.noise.get_noise_2d((x + 1.0) * zoom, y * zoom) - noise_value;
  let noise_dy = uniforms.noise.get_noise_2d(x * zoom, (y + 1.0) * zoom) - noise_value;

  // Compute the gradient magnitude (edge detection)
  let edge_value = (noise_dx * noise_dx + noise_dy * noise_dy).sqrt();

  // Define threshold for detecting edges (cracks)
  let crack_min_threshold = 0.10;  // Minimum threshold for edge detection (inside crack)
  let crack_max_threshold = 0.12;  // Maximum threshold for edge detection (outside crack)

  // Determine if the current point is within the crack line (thin band)
  let is_crack = edge_value > crack_min_threshold && edge_value < crack_max_threshold;

  // Define colors for the ground and cracks
  let ground_color = Color::new(110, 80, 40);  // Light brown (dried ground)
  let crack_color = Color::new(51, 17, 03);     // Dark brown (cracks)

  // Blend between ground and crack color based on edge detection
  let final_color = if is_crack {
    crack_color
  } else {
    ground_color
  };

  final_color * fragment.intensity
}

fn pulsating_sun_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
      // Base colors for the pulsating effect
    let bright_color = Vec3::new(1.0, 0.6, 0.0); // Bright orange (lava-like)
    let dark_color = Vec3::new(0.8, 0.2, 0.0);   // Darker red-orange

    // Get fragment position
    let position = Vec3::new(
        fragment.position.x,
        fragment.position.y,
        fragment.depth
    );

    let radius = position.magnitude();

    // Spherical coordinates for texture mapping
    let lon = position.x.atan2(position.z);
    let lat = (position.y / radius).acos();

    // Create direction vector from spherical coordinates
    let dir = Vec3::new(
        lat.sin() * lon.cos(),
        lat.sin() * lon.sin(),
        lat.cos()
    );
    let dir = dir.normalize();

    // Animate frequency to create pulsating effect
    let base_frequency = 0.02;
    let pulsating_freq = base_frequency + (10.0 - ((uniforms.time as f32 / 10.0).sin() * 10.0).abs()) / 2000.0;

    // Set noise frequency dynamically
    // uniforms.noise.set_frequency(Some(pulsating_freq));

    // Apply noise to UV coordinates with a large zoom for texture
    let zoom = 1000.0;
    let noise_value1 = uniforms.noise.get_noise_3d(position.x * zoom, position.y * zoom, position.z * zoom);
    let noise_value2 = uniforms.noise.get_noise_3d(position.x * zoom + 1000.0, position.y * zoom + 1000.0, position.z * zoom + 1000.0);
    let noise_value = (noise_value1 + noise_value2) * 0.5;  // Averaging noise for smoother transitions

    // Edge factor to make alpha vary (optional visual effect)
    let edge_factor_y = dir.y.sin();
    let edge_factor_x = dir.x.sin();
    let edge_factor = edge_factor_x * edge_factor_y;
    let alpha = 1.0;

    // Mix bright and dark colors based on noise value
    let color_vec = mix(&bright_color, &dark_color, noise_value);

    // Final color applied to the fragment
    Color::from_float(color_vec.x, color_vec.y, color_vec.z) * fragment.intensity
}

