// STAGE 1 -- The line from math class: y = m*x + b
//
// Walk x from x0 to x1, one column at a time, and paint the y that the
// equation gives us for that column.
//
// WORKS:  gentle lines drawn left to right (|m| <= 1).
// BREAKS: - steep lines (|m| > 1): consecutive columns land more than one
//           row apart and nobody paints the rows in between -> HOLES.
//         - vertical lines: dx = 0, so m = dy/0 = infinity. The equation
//           cannot even describe this line; it just vanishes.
//         - right to left: the range x0..=x1 is empty when x1 < x0,
//           so nothing is drawn at all.

use super::PlottedPixel;

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<PlottedPixel> {
    let m = (y1 - y0) as f32 / (x1 - x0) as f32;
    let b = y0 as f32 - m * x0 as f32;

    let mut pixels = Vec::new();
    for x in x0..=x1 {
        let exact = m * x as f32 + b;
        let y = exact.round() as i32;
        pixels.push(PlottedPixel::new(
            x,
            y,
            format!("y = {:.2}*{} + {:.2} = {:.2} -> row {}", m, x, b, exact, y),
        ));
    }
    pixels
}
