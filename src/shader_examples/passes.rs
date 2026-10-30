use raylib::prelude::*;

pub fn bright_regions(scene: &[Vector3]) -> Vec<Vector3> {
    scene
        .iter()
        .map(|color| {
            let brightness = color.x * 0.2126 + color.y * 0.7152 + color.z * 0.0722;
            *color * ((brightness - 0.45) / 0.55).max(0.0)
        })
        .collect()
}

pub fn blur(source: &[Vector3], width: usize, height: usize, horizontal: bool) -> Vec<Vector3> {
    let mut result = vec![Vector3::zero(); source.len()];
    for y in 0..height {
        for x in 0..width {
            let mut sum = Vector3::zero();
            for offset in -8..=8 {
                let sx = if horizontal {
                    (x as i32 + offset).clamp(0, width as i32 - 1) as usize
                } else {
                    x
                };
                let sy = if horizontal {
                    y
                } else {
                    (y as i32 + offset).clamp(0, height as i32 - 1) as usize
                };
                sum += source[sy * width + sx];
            }
            result[y * width + x] = sum / 17.0;
        }
    }
    result
}

pub fn bloom(scene: &[Vector3], width: usize, height: usize) -> Vec<Vector3> {
    let bright = bright_regions(scene);
    let horizontal = blur(&bright, width, height, true);
    let glow = blur(&horizontal, width, height, false);
    scene
        .iter()
        .zip(glow)
        .map(|(color, light)| *color + light * 0.8)
        .collect()
}
