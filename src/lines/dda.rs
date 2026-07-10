// STAGE 3 -- DDA: don't solve the equation, WALK it.
//
// Take steps = max(|dx|, |dy|), so the longer axis advances exactly one
// pixel per step, then move BOTH coordinates by a constant increment
// each step. Direction is baked into the sign of the increments, so
// every octant works with no special cases.
//
// FIXES: all directions, all slopes. This is a correct line!
// COST:  two float additions and two roundings PER PIXEL. In 1965 that
//        was real money, and it still is inside a GPU today. Floats can
//        also accumulate rounding drift on very long lines.

use super::PlottedPixel;

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<PlottedPixel> {
    let dx = (x1 - x0) as f32;
    let dy = (y1 - y0) as f32;
    let steps = dx.abs().max(dy.abs()) as i32;

    if steps == 0 {
        return vec![PlottedPixel::new(x0, y0, "start == end".to_string())];
    }

    let x_inc = dx / steps as f32;
    let y_inc = dy / steps as f32;

    let mut x = x0 as f32;
    let mut y = y0 as f32;
    let mut pixels = Vec::new();
    for i in 0..=steps {
        pixels.push(PlottedPixel::new(
            x.round() as i32,
            y.round() as i32,
            format!("step {}: x = {:.2}, y = {:.2}", i, x, y),
        ));
        x += x_inc;
        y += y_inc;
    }
    pixels
}
