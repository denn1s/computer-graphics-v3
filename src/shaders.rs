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

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    // Access fragment properties (all interpolated values)
    let _screen_position = fragment.position;  // Screen-space position
    let base_color = fragment.color;           // Interpolated color from triangle
    let _depth = fragment.depth;               // Interpolated depth
    let world_pos = fragment.world_position;   // Interpolated world-space position

    // Access uniforms (non-interpolated global values)
    let _model = &uniforms.model_matrix;
    let _view = &uniforms.view_matrix;
    let _projection = &uniforms.projection_matrix;
    let _viewport = &uniforms.viewport_matrix;

    // Create a colorful pattern based on WORLD-SPACE position
    // This pattern will now move with the model as it rotates!
    // The pattern is "painted" onto the model's surface in world coordinates
    let x_pattern = (world_pos.x * 5.0).sin() * 0.5 + 0.5;
    let y_pattern = (world_pos.y * 5.0).cos() * 0.5 + 0.5;
    let z_pattern = (world_pos.z * 5.0).sin() * 0.5 + 0.5;

    // Mix the base color with the pattern
    let pattern_color = Vector3::new(
        x_pattern,
        y_pattern,
        z_pattern,
    );

    // Blend pattern with base color (70% base, 30% pattern for subtle effect)
    Vector3::new(
        base_color.x * 0.7 + pattern_color.x * 0.3,
        base_color.y * 0.7 + pattern_color.y * 0.3,
        base_color.z * 0.7 + pattern_color.z * 0.3,
    )
}