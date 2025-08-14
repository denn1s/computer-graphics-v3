# Raytracer in Rust

This project is a simple raytracer written in Rust using the `raylib` library. It is intended as an educational tool for students learning about computer graphics.

## What is a Raycaster?

A raycaster is a rendering technique that creates a 3D perspective in a 2D map. The basic idea is to trace rays from the "eye" of the viewer through each pixel on the screen. The color of the pixel is then determined by what the ray hits in the scene.

This project implements a simple ray-sphere intersection algorithm to render a sphere on the screen.

## The Orbit Camera

This project uses an orbit camera to view the 3D scene. An orbit camera is a type of camera that rotates around a fixed central point, always keeping its focus on that point. This is different from a free-look camera, which can move and look in any direction independently. Orbit cameras are common in 3D modeling software and games where the user needs to inspect a central object from various angles.

### Core Concepts

The orbit camera implementation in `src/camera.rs` relies on several key concepts from linear algebra and trigonometry, which are fundamental to computer graphics.

#### Linear Algebra

*   **Vectors**: The camera's state is defined by 3D vectors:
    *   `eye`: The position of the camera in world space.
    *   `center`: The point the camera is looking at. This is the pivot point for the orbit.
    *   `up`: A vector that indicates the "up" direction for the camera, used to orient it correctly.

*   **Orthonormal Basis**: To properly orient the camera, we create a new coordinate system relative to the camera's view, called an orthonormal basis. This basis consists of three mutually perpendicular unit vectors:
    1.  `forward`: The direction the camera is looking. Calculated by normalizing the vector from the `eye` to the `center` (`center - eye`).
    2.  `right`: The direction to the right of the camera's view. Calculated using the **cross product** of the `forward` vector and the world's `up` vector.
    3.  `up` (camera's up): The "true" up direction for the camera. It's recalculated by taking the **cross product** of the `right` and `forward` vectors. This ensures the basis is perfectly orthogonal.

*   **Change of Basis**: When we cast rays from the camera, we define them in camera space and then transform them into world space. The `basis_change` function performs this transformation. It takes a vector defined in the camera's coordinate system (using `right`, `up`, `forward` as axes) and converts it to the world coordinate system, allowing us to correctly trace its path in the scene.

#### Trigonometry and Spherical Coordinates

*   **Rotation**: To make the camera "orbit", we need to rotate its `eye` position around the `center`. While this can be done with rotation matrices, a more intuitive way is to use spherical coordinates.
*   **Spherical Coordinates**: We can represent the camera's position by its `(radius, yaw, pitch)` relative to the `center`.
    *   `radius`: The distance from the `eye` to the `center`. This remains constant in an orbit camera.
    *   `yaw`: The horizontal angle (rotation around the vertical axis).
    *   `pitch`: The vertical angle (elevation from the horizontal plane).
*   **Conversion**: The `orbit` function works by:
    1.  Calculating the current `yaw` and `pitch` from the `eye`'s Cartesian coordinates.
    2.  Adding the user's input to these angles.
    3.  Converting the new `(radius, yaw, pitch)` back into Cartesian coordinates to get the new `eye` position.
    4.  The `pitch` is clamped to prevent the camera from flipping over and to avoid **gimbal lock**, a phenomenon that can cause loss of rotational control.

## How to run this code

To run this code, you will need to have Rust installed. You can find instructions on how to install Rust [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and run the following command in the root directory of the project:

```bash
cargo run --release
```

This will compile and run the project. A window should appear with a rendered sphere.

## File Structure

The project is organized into the following files:

-   `src/main.rs`: The main entry point of the program. It initializes `raylib`, creates a window, and contains the main render loop.
-   `src/camera.rs`: Implements the orbit camera, including its orientation and movement logic.
-   `src/framebuffer.rs`: This file contains the `Framebuffer` struct, which is used to store the rendered image before it is displayed on the screen.
-   `src/ray_intersect.rs`: This file defines the `RayIntersect` trait, which is used to check if a ray intersects with an object in the scene.
-   `src/sphere.rs`: This file contains the `Sphere` struct and its implementation of the `RayIntersect` trait.
