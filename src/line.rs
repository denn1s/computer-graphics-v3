// line.rs
use crate::framebuffer::Framebuffer;
use nalgebra_glm::Vec3;

pub trait Line {
    fn line(&mut self, start: Vec3, end: Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start: Vec3, end: Vec3) {
        let mut x0 = start.x as i32;
        let mut y0 = start.y as i32;
        let x1 = end.x as i32;
        let y1 = end.y as i32;

        // Determine the deltas
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();

        // Determine the direction of the increment
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };

        let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

        loop {
            // Draw the current point
            self.point(x0 as usize, y0 as usize);

            // Check if the end point has been reached
            if x0 == x1 && y0 == y1 { break; }

            // Calculate error
            let e2 = err;

            // Adjust error and coordinates based on the dominant direction
            if e2 > -dx {
                err -= dy;
                x0 += sx;
            }
            if e2 < dy {
                err += dx;
                y0 += sy;
            }
        }
    }
}
