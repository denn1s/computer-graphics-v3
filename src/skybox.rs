use raylib::prelude::{Vector2, Vector3};
use crate::vertex::Vertex;

/// Generate a sky sphere with vertices
/// The sphere is created using latitude/longitude subdivisions
pub fn create_sky_sphere(radius: f32, subdivisions: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();

    // Generate vertices for sphere using spherical coordinates
    for lat in 0..=subdivisions {
        let theta = lat as f32 * std::f32::consts::PI / subdivisions as f32;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        for lon in 0..=subdivisions {
            let phi = lon as f32 * 2.0 * std::f32::consts::PI / subdivisions as f32;
            let sin_phi = phi.sin();
            let cos_phi = phi.cos();

            // Calculate position on sphere
            let x = radius * sin_theta * cos_phi;
            let y = radius * cos_theta;
            let z = radius * sin_theta * sin_phi;

            let position = Vector3::new(x, y, z);

            // Normal points inward (we're inside the sphere looking out)
            let normal = Vector3::new(-x / radius, -y / radius, -z / radius);

            // Texture coordinates (not used for procedural skybox, but required by Vertex)
            let u = lon as f32 / subdivisions as f32;
            let v = lat as f32 / subdivisions as f32;
            let tex_coords = Vector2::new(u, v);

            vertices.push(Vertex::new(position, normal, tex_coords));
        }
    }

    // Generate triangles from the grid
    let mut triangles = Vec::new();
    for lat in 0..subdivisions {
        for lon in 0..subdivisions {
            let first = (lat * (subdivisions + 1) + lon) as usize;
            let second = first + subdivisions as usize + 1;

            // First triangle of quad
            triangles.push(vertices[first].clone());
            triangles.push(vertices[second].clone());
            triangles.push(vertices[first + 1].clone());

            // Second triangle of quad
            triangles.push(vertices[second].clone());
            triangles.push(vertices[second + 1].clone());
            triangles.push(vertices[first + 1].clone());
        }
    }

    triangles
}

/// Hash function for procedural star generation
/// Takes a direction vector and returns a pseudo-random value
fn hash(v: Vector3) -> f32 {
    // Simple hash based on position
    let p = Vector3::new(
        (v.x * 127.1).sin(),
        (v.y * 311.7).sin(),
        (v.z * 74.7).sin(),
    );

    let hash_value = (p.x + p.y + p.z) * 43758.5453;
    (hash_value - hash_value.floor()).abs()
}

/// Generate star color and intensity based on world position
/// Returns a color with stars procedurally placed
pub fn generate_stars(world_pos: Vector3) -> Vector3 {
    // Normalize the position to get direction
    let len = (world_pos.x * world_pos.x + world_pos.y * world_pos.y + world_pos.z * world_pos.z).sqrt();
    if len < 0.001 {
        return Vector3::new(0.0, 0.0, 0.1); // Dark blue background
    }

    let dir = Vector3::new(
        world_pos.x / len,
        world_pos.y / len,
        world_pos.z / len,
    );

    // Create multiple layers of stars at different scales
    let mut star_intensity = 0.0;

    // Small frequent stars
    let scale1 = 50.0;
    let grid_pos1 = Vector3::new(
        (dir.x * scale1).floor(),
        (dir.y * scale1).floor(),
        (dir.z * scale1).floor(),
    );
    let hash1 = hash(grid_pos1);
    if hash1 > 0.98 {
        star_intensity += (hash1 - 0.98) * 50.0;
    }

    // Medium stars
    let scale2 = 30.0;
    let grid_pos2 = Vector3::new(
        (dir.x * scale2).floor(),
        (dir.y * scale2).floor(),
        (dir.z * scale2).floor(),
    );
    let hash2 = hash(grid_pos2);
    if hash2 > 0.97 {
        star_intensity += (hash2 - 0.97) * 33.0;
    }

    // Large bright stars
    let scale3 = 15.0;
    let grid_pos3 = Vector3::new(
        (dir.x * scale3).floor(),
        (dir.y * scale3).floor(),
        (dir.z * scale3).floor(),
    );
    let hash3 = hash(grid_pos3);
    if hash3 > 0.95 {
        star_intensity += (hash3 - 0.95) * 20.0;
    }

    star_intensity = star_intensity.min(1.0);

    // Base sky color (dark blue)
    let sky_color = Vector3::new(0.0, 0.0, 0.1);

    // Star color (white/yellow)
    let star_color = Vector3::new(1.0, 1.0, 0.9);

    // Blend sky and stars
    Vector3::new(
        sky_color.x * (1.0 - star_intensity) + star_color.x * star_intensity,
        sky_color.y * (1.0 - star_intensity) + star_color.y * star_intensity,
        sky_color.z * (1.0 - star_intensity) + star_color.z * star_intensity,
    )
}
