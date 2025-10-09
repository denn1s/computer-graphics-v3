use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::light::Light;
use raylib::prelude::Vector3;

/// Calculate barycentric coordinates for a point P with respect to triangle (A, B, C)
/// Returns (w1, w2, w3) where P = w1*A + w2*B + w3*C
/// If the point is outside the triangle, one or more weights will be negative
fn barycentric_coordinates(p_x: f32, p_y: f32, a: &Vertex, b: &Vertex, c: &Vertex) -> (f32, f32, f32) {
    let a_x = a.transformed_position.x;
    let a_y = a.transformed_position.y;
    let b_x = b.transformed_position.x;
    let b_y = b.transformed_position.y;
    let c_x = c.transformed_position.x;
    let c_y = c.transformed_position.y;

    // Calculate the area of the parallelogram formed by vectors AB and AC
    let denom = (b_y - c_y) * (a_x - c_x) + (c_x - b_x) * (a_y - c_y);

    // Avoid division by zero for degenerate triangles
    if denom.abs() < 1e-10 {
        return (-1.0, -1.0, -1.0);
    }

    // Calculate barycentric coordinates
    let w1 = ((b_y - c_y) * (p_x - c_x) + (c_x - b_x) * (p_y - c_y)) / denom;
    let w2 = ((c_y - a_y) * (p_x - c_x) + (a_x - c_x) * (p_y - c_y)) / denom;
    let w3 = 1.0 - w1 - w2;

    (w1, w2, w3)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex, light: &Light) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Assign RGB colors to the three vertices for interpolation demonstration
    let color1 = Vector3::new(1.0, 0.0, 0.0); // Red
    let color2 = Vector3::new(0.0, 0.0, 1.0); // Blue
    let color3 = Vector3::new(0.0, 1.0, 0.0); // Green

    // Calculate face normal using cross product of two edges
    // Edge 1: v2 - v1
    let edge1 = Vector3::new(
        v2.position.x - v1.position.x,
        v2.position.y - v1.position.y,
        v2.position.z - v1.position.z,
    );

    // Edge 2: v3 - v1
    let edge2 = Vector3::new(
        v3.position.x - v1.position.x,
        v3.position.y - v1.position.y,
        v3.position.z - v1.position.z,
    );

    // Normal = edge1 × edge2 (cross product)
    let mut normal = Vector3::new(
        edge1.y * edge2.z - edge1.z * edge2.y,
        edge1.z * edge2.x - edge1.x * edge2.z,
        edge1.x * edge2.y - edge1.y * edge2.x,
    );

    // Normalize the normal vector
    let normal_length = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    if normal_length > 0.0 {
        normal.x /= normal_length;
        normal.y /= normal_length;
        normal.z /= normal_length;
    }

    // Calculate centroid of the triangle in world space
    let centroid = Vector3::new(
        (v1.position.x + v2.position.x + v3.position.x) / 3.0,
        (v1.position.y + v2.position.y + v3.position.y) / 3.0,
        (v1.position.z + v2.position.z + v3.position.z) / 3.0,
    );

    // Light direction (from surface to light)
    let mut light_dir = Vector3::new(
        light.position.x - centroid.x,
        light.position.y - centroid.y,
        light.position.z - centroid.z,
    );

    // Normalize light direction
    let light_length = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
    if light_length > 0.0 {
        light_dir.x /= light_length;
        light_dir.y /= light_length;
        light_dir.z /= light_length;
    }

    // Calculate lighting intensity using dot product (Lambertian shading)
    // Keep this flat for now (same across entire triangle)
    let intensity = (normal.x * light_dir.x + normal.y * light_dir.y + normal.z * light_dir.z).max(0.0);

    // Get the bounding box of the triangle
    let min_x = v1.transformed_position.x.min(v2.transformed_position.x).min(v3.transformed_position.x).floor() as i32;
    let max_x = v1.transformed_position.x.max(v2.transformed_position.x).max(v3.transformed_position.x).ceil() as i32;
    let min_y = v1.transformed_position.y.min(v2.transformed_position.y).min(v3.transformed_position.y).floor() as i32;
    let max_y = v1.transformed_position.y.max(v2.transformed_position.y).max(v3.transformed_position.y).ceil() as i32;

    // Iterate over each pixel in the bounding box
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p_x = x as f32 + 0.5; // Sample at pixel center
            let p_y = y as f32 + 0.5;

            // Calculate barycentric coordinates
            let (w1, w2, w3) = barycentric_coordinates(p_x, p_y, v1, v2, v3);

            // Check if point is inside the triangle
            if w1 >= 0.0 && w2 >= 0.0 && w3 >= 0.0 {
                // Interpolate color using barycentric coordinates
                let interpolated_color = Vector3::new(
                    w1 * color1.x + w2 * color2.x + w3 * color3.x,
                    w1 * color1.y + w2 * color2.y + w3 * color3.y,
                    w1 * color1.z + w2 * color2.z + w3 * color3.z,
                );

                // Apply flat shading (same intensity across entire triangle)
                let shaded_color = Vector3::new(
                    interpolated_color.x * intensity,
                    interpolated_color.y * intensity,
                    interpolated_color.z * intensity,
                );

                // Interpolate depth using barycentric coordinates
                let depth = w1 * v1.transformed_position.z
                          + w2 * v2.transformed_position.z
                          + w3 * v3.transformed_position.z;

                fragments.push(Fragment::new(p_x, p_y, shaded_color, depth));
            }
        }
    }

    fragments
}
