// STAGE 5 -- Bresenham for EVERY line (still with a float error).
//
// Two tricks generalize stage 4 to all eight octants:
//
//   1. THE FLIP: if the line is steep, swap the roles of x and y --
//      walk along y and step x instead (same idea as stage 2). Just
//      remember to un-flip each pixel before plotting it.
//   2. DIRECTION: always walk the long axis left to right by swapping
//      the ENDPOINTS if needed, and let the short axis step with
//      sy = +1 or -1 instead of always +1.
//
// After both tricks we are always back in octant 0, and stage 4's loop
// just works. One problem left: the error is still a float.

use super::PlottedPixel;

pub fn line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32) -> Vec<PlottedPixel> {
    // trick 1: flip steep lines into shallow ones
    let steep = (y1 - y0).abs() > (x1 - x0).abs();
    if steep {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    // trick 2a: always walk left to right
    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0; // >= 0 after trick 2a
    let dy = (y1 - y0).abs(); // 0 <= dy <= dx after trick 1
    let m = dy as f32 / dx as f32; // so 0 <= m <= 1: octant 0!

    // trick 2b: hop up or down as the line demands
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut y = y0;
    let mut err: f32 = 0.0;
    let mut pixels = Vec::new();

    for x in x0..=x1 {
        // un-flip when plotting
        let (px, py) = if steep { (y, x) } else { (x, y) };
        pixels.push(PlottedPixel::new(
            px,
            py,
            format!("err = {:.2}{}", err, if steep { " (flipped)" } else { "" }),
        ));
        err += m;
        if err >= 0.5 {
            y += sy;
            err -= 1.0;
        }
    }
    pixels
}
