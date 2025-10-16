# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a 3D software renderer written in Rust that implements the complete graphics pipeline from scratch. The project uses Raylib for window management and display, but all rendering logic (vertex transformation, rasterization, shading) is custom-implemented.

**This is an educational codebase designed for students learning computer graphics.** Code readability and clarity must always be prioritized over performance optimizations or clever shortcuts. When making changes:
- Use descriptive variable names that clearly indicate purpose
- Add comments explaining the "why" behind complex graphics concepts
- Keep functions focused and understandable
- Prefer explicit, readable code over terse or overly abstracted solutions
- Document mathematical concepts and transformations clearly

## Build and Run Commands

```bash
# Build and run (development with optimizations)
cargo run

# Build and run release version
./run.sh
# or
cargo build --release && ./target/release/computer-graphics-v3

# Build only
cargo build
```

Note: The dev profile has `opt-level = 3` and `debug = false` for performance during development.

## Branch Organization

**This repository uses branches as a lesson progression system, not for traditional feature development.**

Each branch represents a complete lesson/checkpoint in the graphics curriculum:
- **SR-XX** branches: Software Rasterizer lessons (this codebase)
  - SR-01 through SR-19 cover topics from basic framebuffer to skybox rendering
  - Currently on SR-13-Interpolation-v2 (Lambert shading with normal interpolation)
- **RT-XX** branches: Ray Tracing lessons (different curriculum track)
  - RT-01 through RT-11 cover ray tracing concepts
- **RC-XX** branches: Raycasting lessons (another curriculum track)
  - RC-01 through RC-08 cover raycasting/2.5D rendering

**Important branching guidelines:**
- DO NOT create feature branches or use typical Git Flow
- Each lesson branch contains the complete, working state after that lesson
- Students checkout specific branches to see the codebase at that learning stage
- The `main` branch may be a stable reference point or latest curriculum state
- When making changes, ensure they fit within the current lesson's scope
- If implementing features from future lessons, consider whether you're on the right branch

## Architecture

### Graphics Pipeline Flow

The renderer implements a standard 3D graphics pipeline in `src/main.rs`:

1. **Vertex Shader Stage** (`src/shaders.rs:vertex_shader`): Transforms vertices through Model → View → Projection → Viewport matrices
2. **Primitive Assembly**: Groups transformed vertices into triangles (every 3 vertices form a triangle)
3. **Rasterization Stage** (`src/triangle.rs:triangle`): Converts triangles to fragments using barycentric coordinates
4. **Fragment Processing**: Writes fragments to framebuffer with depth testing

### Core Systems

**Framebuffer** (`src/framebuffer.rs`):
- Manages a software framebuffer using `raylib::Image` for pixel storage
- Implements depth buffer for Z-testing (closer fragments occlude farther ones)
- `point()` method performs depth test before writing pixels
- `swap_buffers()` uploads the software framebuffer to GPU texture for display

**Matrix Transformations** (`src/matrix.rs`):
- All matrices are stored in **column-major order** (Raylib convention)
- Helper function `new_matrix4()` accepts **row-major** input and transposes automatically
- Key transformations:
  - Model matrix: Translation → Rotation (Z→Y→X order) → Scale
  - View matrix: LookAt implementation (eye, target, up vectors)
  - Projection matrix: Perspective projection with FOV, aspect ratio, near/far planes
  - Viewport matrix: NDC to screen space conversion

**Camera System** (`src/camera.rs`):
- Orbit camera with spherical coordinates (yaw, pitch, distance)
- Controls:
  - W/S: Pitch up/down
  - A/D: Yaw left/right
  - Up/Down arrows: Zoom in/out
  - Q/E or Left/Right arrows: Pan horizontally
  - R/F: Pan vertically

**Shading** (`src/triangle.rs`):
- Lambert (diffuse) shading with per-fragment lighting
- Normal vectors are interpolated across triangle faces using barycentric coordinates
- Light direction computed per-fragment for accurate shading
- Commented-out code shows RGB color interpolation demo (lines 36-40, 62-68)

**OBJ Loading** (`src/obj.rs`):
- Uses `tobj` crate to load .obj model files
- Note: Y-axis is inverted during loading (`-y`) to match coordinate system
- `get_vertex_array()` flattens indexed mesh into triangle list

### Coordinate Systems

- **World Space**: Model positioned at origin, transformations applied via model matrix
- **View Space**: Camera-relative coordinates via view matrix
- **Clip Space**: After projection matrix, before perspective divide
- **NDC (Normalized Device Coordinates)**: After perspective divide (w-component)
- **Screen Space**: Final pixel coordinates after viewport transform

### Data Structures

**Vertex** (`src/vertex.rs`):
- Stores both original (`position`, `normal`) and transformed (`transformed_position`, `transformed_normal`) data
- Contains texture coordinates and color fields

**Fragment** (`src/fragment.rs`):
- Represents a potential pixel with position, color, and depth

**Uniforms** (`src/main.rs:27-32`):
- Container for transformation matrices passed to shaders
- Includes model, view, projection, and viewport matrices

## Important Technical Details

### Matrix Multiplication Order
The transformation chain is: `Viewport × Projection × View × Model × Vertex`

### Depth Buffer
- Initialized to `f32::INFINITY` (far plane)
- Smaller depth values = closer to camera
- Depth test in `framebuffer.rs:49`: only draws if `depth < depth_buffer[index]`

### Barycentric Interpolation
The triangle rasterizer uses barycentric coordinates to interpolate:
- Vertex normals (for smooth shading)
- Depth values (for depth testing)
- Colors (optional, see commented demo code)

### Performance
The project uses Rust edition 2024 with aggressive optimization (`opt-level = 3`) even in dev builds to maintain acceptable frame rates for software rendering.
