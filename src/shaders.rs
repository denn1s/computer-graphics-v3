use raylib::prelude::*;
use crate::vertex::Vertex;
use crate::fragment::Fragment;
use crate::Uniforms;

// This function manually multiplies a 4x4 matrix with a 4D vector (in homogeneous coordinates)
fn multiply_matrix_vector4(matrix: &Matrix, vector: &Vector4) -> Vector4 {
    Vector4::new(
        matrix.m0 * vector.x + matrix.m4 * vector.y + matrix.m8 * vector.z + matrix.m12 * vector.w,
        matrix.m1 * vector.x + matrix.m5 * vector.y + matrix.m9 * vector.z + matrix.m13 * vector.w,
        matrix.m2 * vector.x + matrix.m6 * vector.y + matrix.m10 * vector.z + matrix.m14 * vector.w,
        matrix.m3 * vector.x + matrix.m7 * vector.y + matrix.m11 * vector.z + matrix.m15 * vector.w,
    )
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
  // Convert vertex position to homogeneous coordinates (Vec4) by adding a w-component of 1.0
  let position_vec4 = Vector4::new(
    vertex.position.x,
    vertex.position.y,
    vertex.position.z,
    1.0
  );

  // Apply Model transformation
  let world_position = multiply_matrix_vector4(&uniforms.model_matrix, &position_vec4);

  // Apply View transformation (camera)
  let view_position = multiply_matrix_vector4(&uniforms.view_matrix, &world_position);

  // Apply Projection transformation (perspective)
  let clip_position = multiply_matrix_vector4(&uniforms.projection_matrix, &view_position);

  // Perform perspective division to get NDC (Normalized Device Coordinates)
  let ndc = if clip_position.w != 0.0 {
      Vector3::new(
          clip_position.x / clip_position.w,
          clip_position.y / clip_position.w,
          clip_position.z / clip_position.w,
      )
  } else {
      Vector3::new(clip_position.x, clip_position.y, clip_position.z)
  };

  // Apply Viewport transformation to get screen coordinates
  let ndc_vec4 = Vector4::new(ndc.x, ndc.y, ndc.z, 1.0);
  let screen_position = multiply_matrix_vector4(&uniforms.viewport_matrix, &ndc_vec4);

  let transformed_position = Vector3::new(
      screen_position.x,
      screen_position.y,
      screen_position.z,
  );

  // Create a new Vertex with the transformed position
  Vertex {
    position: vertex.position,
    normal: vertex.normal,
    tex_coords: vertex.tex_coords,
    color: vertex.color,
    transformed_position,
    transformed_normal: vertex.normal, // Note: Correct normal transformation is more complex
  }
}

// === Fragment Shader Examples ===

/// Example 1: Sine wave pattern in world space
#[allow(dead_code)]
fn shader_sine_waves(fragment: &Fragment) -> Vector3 {
    let world_pos = fragment.world_position;
    let base_color = fragment.color;

    let x_pattern = (world_pos.x * 5.0).sin() * 0.5 + 0.5;
    let y_pattern = (world_pos.y * 5.0).cos() * 0.5 + 0.5;
    let z_pattern = (world_pos.z * 5.0).sin() * 0.5 + 0.5;

    let pattern_color = Vector3::new(x_pattern, y_pattern, z_pattern);

    Vector3::new(
        base_color.x * 0.7 + pattern_color.x * 0.3,
        base_color.y * 0.7 + pattern_color.y * 0.3,
        base_color.z * 0.7 + pattern_color.z * 0.3,
    )
}

/// Example 2: Horizontal stripes based on world Y position
#[allow(dead_code)]
fn shader_horizontal_stripes(fragment: &Fragment) -> Vector3 {
    let world_pos = fragment.world_position;
    let base_color = fragment.color;

    // Create stripes by taking modulo of Y position
    let stripe_frequency = 0.5;
    let stripe = ((world_pos.y * stripe_frequency).floor() % 2.0).abs();

    // Alternate between two colors
    let stripe_color1 = Vector3::new(1.0, 0.5, 0.2); // Orange
    let stripe_color2 = Vector3::new(0.2, 0.5, 1.0); // Blue

    let stripe_color = Vector3::new(
        stripe_color1.x * stripe + stripe_color2.x * (1.0 - stripe),
        stripe_color1.y * stripe + stripe_color2.y * (1.0 - stripe),
        stripe_color1.z * stripe + stripe_color2.z * (1.0 - stripe),
    );

    // Blend with base color and lighting
    Vector3::new(
        base_color.x * stripe_color.x,
        base_color.y * stripe_color.y,
        base_color.z * stripe_color.z,
    )
}

/// Example 3: Vertical stripes based on world X position
#[allow(dead_code)]
fn shader_vertical_stripes(fragment: &Fragment) -> Vector3 {
    let world_pos = fragment.world_position;
    let base_color = fragment.color;

    let stripe_frequency = 1.0;
    let stripe = ((world_pos.x * stripe_frequency).floor() % 2.0).abs();

    let stripe_color1 = Vector3::new(1.0, 0.0, 0.5); // Pink
    let stripe_color2 = Vector3::new(0.0, 1.0, 0.5); // Cyan

    let stripe_color = Vector3::new(
        stripe_color1.x * stripe + stripe_color2.x * (1.0 - stripe),
        stripe_color1.y * stripe + stripe_color2.y * (1.0 - stripe),
        stripe_color1.z * stripe + stripe_color2.z * (1.0 - stripe),
    );

    Vector3::new(
        base_color.x * stripe_color.x,
        base_color.y * stripe_color.y,
        base_color.z * stripe_color.z,
    )
}

/// Example 4: Checkerboard pattern
#[allow(dead_code)]
fn shader_checkerboard(fragment: &Fragment) -> Vector3 {
    let world_pos = fragment.world_position;
    let base_color = fragment.color;

    let scale = 1.0;
    let x_check = (world_pos.x * scale).floor() % 2.0;
    let z_check = (world_pos.z * scale).floor() % 2.0;

    // XOR pattern for checkerboard
    let checker = ((x_check + z_check) % 2.0).abs();

    let color1 = Vector3::new(0.9, 0.9, 0.9); // Light gray
    let color2 = Vector3::new(0.2, 0.2, 0.2); // Dark gray

    let checker_color = Vector3::new(
        color1.x * checker + color2.x * (1.0 - checker),
        color1.y * checker + color2.y * (1.0 - checker),
        color1.z * checker + color2.z * (1.0 - checker),
    );

    Vector3::new(
        base_color.x * checker_color.x,
        base_color.y * checker_color.y,
        base_color.z * checker_color.z,
    )
}

/// Example 5: Distance-based gradient from origin
#[allow(dead_code)]
fn shader_radial_gradient(fragment: &Fragment) -> Vector3 {
    let world_pos = fragment.world_position;
    let base_color = fragment.color;

    // Calculate distance from origin
    let distance = (world_pos.x * world_pos.x + world_pos.y * world_pos.y + world_pos.z * world_pos.z).sqrt();

    // Create gradient based on distance
    let gradient = (distance * 2.0).sin() * 0.5 + 0.5;

    let gradient_color = Vector3::new(
        gradient,
        1.0 - gradient,
        (gradient * 2.0) % 1.0,
    );

    Vector3::new(
        base_color.x * 0.5 + gradient_color.x * 0.5,
        base_color.y * 0.5 + gradient_color.y * 0.5,
        base_color.z * 0.5 + gradient_color.z * 0.5,
    )
}

/// Example 6: Screen-space effect (doesn't move with model)
#[allow(dead_code)]
fn shader_screen_space_pattern(fragment: &Fragment) -> Vector3 {
    let screen_pos = fragment.position;
    let base_color = fragment.color;

    let x_pattern = (screen_pos.x / 50.0).sin() * 0.5 + 0.5;
    let y_pattern = (screen_pos.y / 50.0).cos() * 0.5 + 0.5;

    let pattern_color = Vector3::new(
        x_pattern,
        y_pattern,
        (x_pattern + y_pattern) / 2.0,
    );

    Vector3::new(
        base_color.x * 0.5 + pattern_color.x * 0.5,
        base_color.y * 0.5 + pattern_color.y * 0.5,
        base_color.z * 0.5 + pattern_color.z * 0.5,
    )
}

/// Example 7: Just pass through the base color (standard lighting only)
#[allow(dead_code)]
fn shader_base_color(fragment: &Fragment) -> Vector3 {
    fragment.color
}

// === Main Fragment Shader ===
pub fn fragment_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    // Uncomment one of the shader examples below to see different effects!
    // Try each one and observe how they behave as the model rotates

    // shader_sine_waves(fragment)
    // shader_horizontal_stripes(fragment)
    // shader_vertical_stripes(fragment)
    // shader_checkerboard(fragment)
    // shader_radial_gradient(fragment)
    // shader_screen_space_pattern(fragment)
    shader_base_color(fragment) // Default: just show the lighting
}