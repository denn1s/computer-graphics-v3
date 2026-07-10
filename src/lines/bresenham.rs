// STAGE 6 -- Bresenham, the real thing: INTEGERS ONLY.
//
// Stage 5's error math was:
//
//     err += dy/dx;        if err >= 0.5 { hop; err -= 1.0; }
//
// Multiply every term by 2*dx (a positive constant, so none of the
// comparisons change direction):
//
//     err += 2*dy;         if err >= dx  { hop; err -= 2*dx; }
//
// Same decisions, same pixels -- but now every variable is an integer.
// No rounding, no drift, and cheap enough for a 1962 pen plotter, which
// is exactly what Jack Bresenham invented it for. Every GPU still
// rasterizes lines with this idea today.

use super::PlottedPixel;

pub fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) -> Vec<PlottedPixel> {
    // flip steep lines into shallow ones (see stage 5)
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    // always walk left to right
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = (y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut y = y0;
    let mut err: i32 = 0;
    let mut pixels = Vec::new();

    for x in x0..=x1 {
        let (px, py) = if steep { (y, x) } else { (x, y) };
        pixels.push(PlottedPixel::new(
            px,
            py,
            format!("err = {} (hop when err >= dx = {})", err, dx),
        ));
        err += 2 * dy;
        if err >= dx {
            y += sy;
            err -= 2 * dx;
        }
    }
    pixels
}
