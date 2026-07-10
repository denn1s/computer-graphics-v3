// STAGE 4 -- Bresenham's idea, smallest possible version.
//
// Observation: for a gentle line going right (0 <= m <= 1), every step
// moves one column right, and y either STAYS or moves DOWN BY ONE.
// So don't recompute y = mx + b. Keep a running "error": how far the
// true line has drifted below the row we are currently painting. Each
// column the drift grows by m; when it passes half a pixel, hop down
// one row and pay the drift back.
//
// WORKS:  ONLY for 0 <= m <= 1, drawn left to right (octant 0).
// BREAKS: - steep lines: y can hop at most ONE row per column -> holes.
//         - uphill lines (m < 0): err only shrinks, we never hop UP,
//           the line comes out flat and wrong.
//         - right to left: empty loop, nothing drawn.
//         - vertical: m = dy/0 = infinity again.

use super::PlottedPixel;

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<PlottedPixel> {
    let m = (y1 - y0) as f32 / (x1 - x0) as f32;

    let mut y = y0;
    let mut err: f32 = 0.0;
    let mut pixels = Vec::new();

    for x in x0..=x1 {
        pixels.push(PlottedPixel::new(x, y, format!("err = {:.2}", err)));
        err += m;
        if err >= 0.5 {
            y += 1; // hop one row down...
            err -= 1.0; // ...and pay the drift back
        }
    }
    pixels
}
