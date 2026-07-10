// main.rs -- Line Rasterization Lab
//
// A tiny framebuffer (GRID_W x GRID_H "logical" pixels) is scaled up by
// CELL when blitted, so every pixel becomes a fat CELL x CELL block.
// The line algorithms never know they are being magnified: they work in
// plain pixel coordinates, exactly like they would on a real screen.
//
// Controls:
//   click 2 cells  draw a line with the current algorithm
//   1-6            pick algorithm (re-rasterizes the last line in place!)
//   SPACE          step one pixel of the last line
//   R              replay the last line from zero
//   ENTER          show the last line completely
//   T              toggle the "true line" overlay
//   C              clear everything
//   BACKSPACE      undo the last line

mod framebuffer;
mod lines;

use framebuffer::Framebuffer;
use lines::Algorithm;
use raylib::prelude::*;

const CELL: i32 = 20; // screen pixels per framebuffer pixel
const GRID_W: i32 = 44; // framebuffer width  (logical pixels)
const GRID_H: i32 = 30; // framebuffer height (logical pixels)
const MARGIN: i32 = 40; // room for the axes
const HUD_H: i32 = 130; // info panel below the grid

struct DrawnLine {
    algo: Algorithm,
    start: (i32, i32),
    end: (i32, i32),
    pixels: Vec<lines::PlottedPixel>,
    shown: usize,
    color: Color,
}

impl DrawnLine {
    fn rasterize(&mut self) {
        self.pixels = self
            .algo
            .rasterize(self.start.0, self.start.1, self.end.0, self.end.1);
        self.shown = self.pixels.len();
    }
}

/// Screen position of the center of a logical pixel (for the overlay).
fn cell_center(p: (i32, i32)) -> Vector2 {
    Vector2::new(
        (MARGIN + p.0 * CELL + CELL / 2) as f32,
        (MARGIN + p.1 * CELL + CELL / 2) as f32,
    )
}

fn main() {
    let window_width = MARGIN + GRID_W * CELL + 10;
    let window_height = MARGIN + GRID_H * CELL + HUD_H;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Line Rasterization Lab")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(GRID_W as u32, GRID_H as u32);
    framebuffer.set_background_color(Color::new(18, 18, 32, 255));

    let palette = [
        Color::YELLOW,
        Color::SKYBLUE,
        Color::ORANGE,
        Color::LIME,
        Color::VIOLET,
        Color::PINK,
    ];

    let mut algorithm = Algorithm::Naive;
    let mut drawn: Vec<DrawnLine> = Vec::new();
    let mut pending: Option<(i32, i32)> = None;
    let mut overlay = true;
    let mut color_idx = 0usize;

    while !window.window_should_close() {
        // ------------------------------------------------ input
        let algo_keys = [
            (KeyboardKey::KEY_ONE, Algorithm::Naive),
            (KeyboardKey::KEY_TWO, Algorithm::NaiveSwap),
            (KeyboardKey::KEY_THREE, Algorithm::Dda),
            (KeyboardKey::KEY_FOUR, Algorithm::BresenhamSimple),
            (KeyboardKey::KEY_FIVE, Algorithm::BresenhamFloat),
            (KeyboardKey::KEY_SIX, Algorithm::Bresenham),
        ];
        for (key, algo) in algo_keys {
            if window.is_key_pressed(key) {
                algorithm = algo;
                // re-rasterize the last line so the class can compare
                // algorithms on the exact same endpoints
                if let Some(last) = drawn.last_mut() {
                    last.algo = algo;
                    last.rasterize();
                }
            }
        }

        if window.is_key_pressed(KeyboardKey::KEY_SPACE) {
            if let Some(last) = drawn.last_mut() {
                if last.shown < last.pixels.len() {
                    last.shown += 1;
                }
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_R) {
            if let Some(last) = drawn.last_mut() {
                last.shown = 0;
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_ENTER) {
            if let Some(last) = drawn.last_mut() {
                last.shown = last.pixels.len();
            }
        }
        if window.is_key_pressed(KeyboardKey::KEY_T) {
            overlay = !overlay;
        }
        if window.is_key_pressed(KeyboardKey::KEY_C) {
            drawn.clear();
            pending = None;
        }
        if window.is_key_pressed(KeyboardKey::KEY_BACKSPACE) {
            drawn.pop();
        }

        let mouse = window.get_mouse_position();
        let hover = {
            let gx = (mouse.x as i32 - MARGIN).div_euclid(CELL);
            let gy = (mouse.y as i32 - MARGIN).div_euclid(CELL);
            if gx >= 0 && gx < GRID_W && gy >= 0 && gy < GRID_H {
                Some((gx, gy))
            } else {
                None
            }
        };

        if window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(cell) = hover {
                match pending.take() {
                    None => pending = Some(cell),
                    Some(start) => {
                        let mut line = DrawnLine {
                            algo: algorithm,
                            start,
                            end: cell,
                            pixels: Vec::new(),
                            shown: 0,
                            color: palette[color_idx % palette.len()],
                        };
                        color_idx += 1;
                        line.rasterize();
                        drawn.push(line);
                    }
                }
            }
        }

        // ------------------------------------- rasterize into framebuffer
        framebuffer.clear();
        for line in &drawn {
            framebuffer.set_current_color(line.color);
            for px in &line.pixels[..line.shown] {
                if px.x >= 0 && px.y >= 0 {
                    framebuffer.set_pixel(px.x as u32, px.y as u32);
                }
            }
        }

        // ------------------------------------------------ draw
        let texture = window
            .load_texture_from_image(&raylib_thread, &framebuffer.color_buffer)
            .unwrap();

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::new(10, 10, 18, 255));

        // the framebuffer, scaled so 1 logical pixel = CELL x CELL block
        d.draw_texture_ex(
            &texture,
            Vector2::new(MARGIN as f32, MARGIN as f32),
            0.0,
            CELL as f32,
            Color::WHITE,
        );

        // grid lines (brighter every 5 pixels)
        let grid_right = MARGIN + GRID_W * CELL;
        let grid_bottom = MARGIN + GRID_H * CELL;
        for i in 0..=GRID_W {
            let x = MARGIN + i * CELL;
            let c = if i % 5 == 0 {
                Color::new(255, 255, 255, 60)
            } else {
                Color::new(255, 255, 255, 20)
            };
            d.draw_line(x, MARGIN, x, grid_bottom, c);
        }
        for j in 0..=GRID_H {
            let y = MARGIN + j * CELL;
            let c = if j % 5 == 0 {
                Color::new(255, 255, 255, 60)
            } else {
                Color::new(255, 255, 255, 20)
            };
            d.draw_line(MARGIN, y, grid_right, y, c);
        }

        // axis tick labels every 5 cells (centered on the cell they name)
        for i in (0..GRID_W).step_by(5) {
            d.draw_text(
                &format!("{}", i),
                MARGIN + i * CELL + CELL / 2 - 4,
                MARGIN - 18,
                14,
                Color::LIGHTGRAY,
            );
        }
        for j in (0..GRID_H).step_by(5) {
            d.draw_text(
                &format!("{}", j),
                MARGIN - 24,
                MARGIN + j * CELL + CELL / 2 - 7,
                14,
                Color::LIGHTGRAY,
            );
        }

        // x axis arrow (along the top)
        d.draw_line_ex(
            Vector2::new(MARGIN as f32, 8.0),
            Vector2::new((MARGIN + 70) as f32, 8.0),
            2.0,
            Color::YELLOW,
        );
        d.draw_line_ex(
            Vector2::new((MARGIN + 70) as f32, 8.0),
            Vector2::new((MARGIN + 62) as f32, 4.0),
            2.0,
            Color::YELLOW,
        );
        d.draw_line_ex(
            Vector2::new((MARGIN + 70) as f32, 8.0),
            Vector2::new((MARGIN + 62) as f32, 12.0),
            2.0,
            Color::YELLOW,
        );
        d.draw_text("x", MARGIN + 78, 1, 16, Color::YELLOW);

        // y axis arrow (down the left side -- yes, y grows DOWNWARD!)
        d.draw_line_ex(
            Vector2::new(8.0, MARGIN as f32),
            Vector2::new(8.0, (MARGIN + 70) as f32),
            2.0,
            Color::YELLOW,
        );
        d.draw_line_ex(
            Vector2::new(8.0, (MARGIN + 70) as f32),
            Vector2::new(4.0, (MARGIN + 62) as f32),
            2.0,
            Color::YELLOW,
        );
        d.draw_line_ex(
            Vector2::new(8.0, (MARGIN + 70) as f32),
            Vector2::new(12.0, (MARGIN + 62) as f32),
            2.0,
            Color::YELLOW,
        );
        d.draw_text("y", 3, MARGIN + 76, 16, Color::YELLOW);

        // origin
        d.draw_text("(0,0)", 3, 22, 13, Color::YELLOW);

        // hovered cell: highlight + live coordinates next to the cursor
        if let Some((hx, hy)) = hover {
            d.draw_rectangle(
                MARGIN + hx * CELL,
                MARGIN + hy * CELL,
                CELL,
                CELL,
                Color::new(255, 255, 255, 30),
            );
            let label = format!("({}, {})", hx, hy);
            d.draw_text(&label, mouse.x as i32 + 15, mouse.y as i32 - 19, 16, Color::BLACK);
            d.draw_text(&label, mouse.x as i32 + 14, mouse.y as i32 - 20, 16, Color::WHITE);
        }

        // first click marker
        if let Some(p) = pending {
            d.draw_rectangle_lines(MARGIN + p.0 * CELL, MARGIN + p.1 * CELL, CELL, CELL, Color::WHITE);
        }

        // the "true" mathematical line, drawn from cell center to cell center
        if overlay {
            for line in &drawn {
                let a = cell_center(line.start);
                let b = cell_center(line.end);
                d.draw_line_ex(a, b, 3.0, Color::new(255, 255, 255, 150));
                d.draw_circle_v(a, 4.0, Color::WHITE);
                d.draw_circle_v(b, 4.0, Color::WHITE);
            }
        }

        // ------------------------------------------------ HUD
        let hud_y = grid_bottom + 12;
        d.draw_text(
            &format!("ALGORITHM: {}", algorithm.name()),
            MARGIN,
            hud_y,
            20,
            Color::WHITE,
        );
        d.draw_text(
            if overlay { "overlay: ON [T]" } else { "overlay: OFF [T]" },
            window_width - 160,
            hud_y,
            16,
            Color::GRAY,
        );

        if let Some(last) = drawn.last() {
            let (x0, y0) = last.start;
            let (x1, y1) = last.end;
            let dx = x1 - x0;
            let dy = y1 - y0;
            let slope = if dx == 0 {
                format!("m = {}/0 = undefined (vertical!)", dy)
            } else {
                format!("m = dy/dx = {}/{} = {:.2}", dy, dx, dy as f32 / dx as f32)
            };
            d.draw_text(
                &format!(
                    "P0 = ({}, {})   P1 = ({}, {})   dx = {}   dy = {}   {}",
                    x0, y0, x1, y1, dx, dy, slope
                ),
                MARGIN,
                hud_y + 28,
                18,
                Color::new(200, 200, 220, 255),
            );

            let step_text = if last.pixels.is_empty() {
                "0 pixels produced -- the algorithm gave up on this line!".to_string()
            } else if last.shown == 0 {
                format!("0/{} pixels -- press SPACE to step", last.pixels.len())
            } else {
                let px = &last.pixels[last.shown - 1];
                format!(
                    "pixel {}/{}: ({}, {})   {}",
                    last.shown,
                    last.pixels.len(),
                    px.x,
                    px.y,
                    px.info
                )
            };
            d.draw_text(&step_text, MARGIN, hud_y + 52, 18, last.color);
        } else {
            d.draw_text(
                "Click two cells to draw a line.",
                MARGIN,
                hud_y + 28,
                18,
                Color::GRAY,
            );
        }

        d.draw_text(
            "click 2 cells: line   1-6: algorithm (re-runs last line)   SPACE: step   R: replay   ENTER: finish",
            MARGIN,
            hud_y + 82,
            14,
            Color::GRAY,
        );
        d.draw_text(
            "T: true-line overlay   C: clear   BACKSPACE: undo last",
            MARGIN,
            hud_y + 100,
            14,
            Color::GRAY,
        );
    }
}
