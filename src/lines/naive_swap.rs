// STAGE 2 -- Same equation, but walk along the LONGER axis.
//
// Stage 1 leaves holes because it takes exactly one sample per column,
// even when a steep line crosses several rows inside that column. The
// fix: if the line is steep (|dy| > |dx|), walk the ROWS instead and
// solve the equation for x:
//
//     x = x0 + (y - y0) * (dx/dy)
//
// FIXES:  holes on steep lines -- and even vertical lines work now.
// BREAKS: right-to-left lines (and bottom-to-top on the steep branch)
//         still draw NOTHING, because the loop range is empty.
//         And we still pay for float math on every single pixel.

use super::PlottedPixel;

pub fn line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<PlottedPixel> {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut pixels = Vec::new();

    if dy.abs() <= dx.abs() {
        // shallow: one sample per column (exactly stage 1)
        let m = dy as f32 / dx as f32;
        for x in x0..=x1 {
            let exact = y0 as f32 + m * (x - x0) as f32;
            let y = exact.round() as i32;
            pixels.push(PlottedPixel::new(
                x,
                y,
                format!("walk x: y = {:.2} -> row {}", exact, y),
            ));
        }
    } else {
        // steep: one sample per row (the roles of x and y are FLIPPED)
        let m_inv = dx as f32 / dy as f32;
        for y in y0..=y1 {
            let exact = x0 as f32 + m_inv * (y - y0) as f32;
            let x = exact.round() as i32;
            pixels.push(PlottedPixel::new(
                x,
                y,
                format!("walk y: x = {:.2} -> col {}", exact, x),
            ));
        }
    }
    pixels
}
