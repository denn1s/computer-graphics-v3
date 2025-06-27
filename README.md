Rust Raycaster with Raylib

This project is a simple raycasting engine implemented in Rust using the raylib library for graphics and input handling. It demonstrates fundamental concepts of 3D rendering using 2D raycasting, inspired by classic games like Wolfenstein 3D.

Features

- 2D Maze Loading: Load maze layouts from text files to define walls and open spaces.
- Raycasting Rendering: Render a 3D-like view of the maze by casting rays and drawing vertical wall slices.
- Textured Walls: Apply textures to walls for a more immersive visual experience.
- Sprite Rendering: Render sprites (e.g., enemies) within the 3D world using raycasting techniques.
- UI Rendering: Display 2D UI elements using textures.
- Player Movement and Rotation: Control the player with keyboard input, including smooth rotation and movement.
- Fisheye Correction: Correct the distortion caused by ray angles to produce realistic visuals.

Getting Started

Prerequisites

- Rust toolchain installed (recommended via rustup)
- raylib installed on your system
- raylib-rs crate dependencies handled via Cargo

Project Structure

- main.rs: Entry point, initializes window, loads maze and textures, runs the main loop.
- framebuffer.rs: Manages pixel buffer for drawing.
- maze.rs: Loads maze layout from files.
- caster.rs: Contains raycasting logic.
- player.rs: Handles player state and input processing.
- texture_manager.rs: Loads and manages textures and images.
- line.rs: Implements line drawing algorithms.
- assets/: Contains textures and maze files.

Technical Details

- Uses raylib for window creation, input, and texture management.
- Implements Bresenham's line algorithm for drawing lines.
- Uses trigonometric functions and vector math for raycasting calculations.
- Applies fisheye correction by adjusting ray distances with cosine of angle difference.
- Manages textures efficiently by loading once and accessing pixel data in RAM.

License

This project is licensed under the MIT License.
