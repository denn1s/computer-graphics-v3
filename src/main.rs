 // main.rs

mod framebuffer;
mod line;
mod bmp;

use framebuffer::Framebuffer;
use line::Line;
use bmp::WriteBmp;

fn main() {
    let width = 800;
    let height = 600;
    let mut framebuffer = Framebuffer::new(width, height);

    // Clear the framebuffer with a white background
    framebuffer.set_background_color(0xFFFFFF);
    framebuffer.clear();

    // Set the current drawing color to black
    framebuffer.set_current_color(0x000000);

    // Draw some lines using Bresenham's algorithm
    framebuffer.line(100, 100, 700, 500);
    framebuffer.line(700, 100, 100, 500);
    framebuffer.line(400, 50, 400, 550);
    framebuffer.line(50, 300, 750, 300);

    // Save the framebuffer as a BMP file
    let output_file = "lines.bmp";
    match framebuffer.render_buffer(output_file) {
        Ok(_) => println!("Image saved as {}", output_file),
        Err(e) => eprintln!("Error saving image: {}", e),
    }
} 
