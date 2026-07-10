// lines/mod.rs
//
// Every line algorithm in this module has the same shape:
//
//     fn line(x0, y0, x1, y1) -> Vec<PlottedPixel>
//
// It does NOT draw anything. It only decides WHICH pixels the line
// touches, and returns them in order. Drawing them (all at once, or one
// by one while stepping) is someone else's job. Keeping the algorithm
// separate from the rendering is the whole trick.

pub mod naive;
pub mod naive_swap;
pub mod dda;
pub mod bresenham_simple;
pub mod bresenham_float;
pub mod bresenham;

/// One pixel produced by a line algorithm, plus a note about how the
/// algorithm decided to paint it (shown in the HUD while stepping).
pub struct PlottedPixel {
    pub x: i32,
    pub y: i32,
    pub info: String,
}

impl PlottedPixel {
    pub fn new(x: i32, y: i32, info: String) -> Self {
        PlottedPixel { x, y, info }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Algorithm {
    Naive,
    NaiveSwap,
    Dda,
    BresenhamSimple,
    BresenhamFloat,
    Bresenham,
}

impl Algorithm {
    pub fn name(self) -> &'static str {
        match self {
            Algorithm::Naive => "1. Naive: y = mx + b",
            Algorithm::NaiveSwap => "2. Naive, walking the long axis",
            Algorithm::Dda => "3. DDA (parametric, floats)",
            Algorithm::BresenhamSimple => "4. Bresenham, octant 0 only",
            Algorithm::BresenhamFloat => "5. Bresenham, all octants (float error)",
            Algorithm::Bresenham => "6. Bresenham, pure integers",
        }
    }

    pub fn rasterize(self, x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<PlottedPixel> {
        match self {
            Algorithm::Naive => naive::line(x0, y0, x1, y1),
            Algorithm::NaiveSwap => naive_swap::line(x0, y0, x1, y1),
            Algorithm::Dda => dda::line(x0, y0, x1, y1),
            Algorithm::BresenhamSimple => bresenham_simple::line(x0, y0, x1, y1),
            Algorithm::BresenhamFloat => bresenham_float::line(x0, y0, x1, y1),
            Algorithm::Bresenham => bresenham::line(x0, y0, x1, y1),
        }
    }
}
