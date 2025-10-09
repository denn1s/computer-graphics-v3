use crate::fragment::Fragment;
use crate::vertex::Vertex;
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

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

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
                // Use a flat color for now (we'll add shading later)
                let color = Vector3::new(1.0, 1.0, 1.0); // White
                let depth = 0.0; // Depth will be added in a future lesson

                fragments.push(Fragment::new(p_x, p_y, color, depth));
            }
        }
    }

    fragments
}
