# Raytracer in Rust

This project is a simple raytracer written in Rust using the `raylib` library. It is intended as an educational tool for students learning about computer graphics.

## What is a Raycaster?

A raycaster is a rendering technique that creates a 3D perspective in a 2D map. The basic idea is to trace rays from the "eye" of the viewer through each pixel on the screen. The color of the pixel is then determined by what the ray hits in the scene.

This project implements a simple ray-sphere intersection algorithm to render a sphere on the screen.

## How to run this code

To run this code, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and run the following command in the root directory of the project:

```bash
cargo run --release
```

This will compile and run the project. A window should appear with a rendered sphere.

## File Structure

The project is organized into the following files:

-   `src/main.rs`: This is the main entry point of the program. It initializes `raylib`, creates a window, and contains the main render loop.
-   `src/framebuffer.rs`: This file contains the `Framebuffer` struct, which is used to store the rendered image before it is displayed on the screen.
-   `src/ray_intersect.rs`: This file defines the `RayIntersect` trait, which is used to check if a ray intersects with an object in the scene.
-   `src/sphere.rs`: This file contains the `Sphere` struct and its implementation of the `RayIntersect` trait.