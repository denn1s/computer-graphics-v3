# Computer Graphics with Rust: Framebuffer Lesson

## Introduction

Welcome to the Computer Graphics course with Rust! In this lesson, we'll dive into the fundamental concept of a framebuffer and how to implement it using Rust programming language. We'll be using the `minifb` library for window management and pixel manipulation, and the `nalgebra_glm` library for potential future use with linear algebra operations.

## What is a Framebuffer?

A framebuffer is a data structure that represents the pixels displayed on a computer screen. It holds information about the color and intensity of each pixel on the screen. Think of it as a canvas where you can draw graphics, text, and images.


### Core Changes

1. **New Framebuffer Class**: A new `Framebuffer` class was introduced, replacing the previous implementation.

2. **Dependency Addition**: The `raylib` library was added as a dependency.

3. **Structural Changes**: The `Framebuffer` now uses `raylib::Image` instead of a raw buffer array.

4. **Method Renamings**: Several methods were renamed for clarity and consistency.

5. **Color Representation**: Colors are now represented using `raylib::Color` instead of `u32`.

### Direct Impact

1. **Simplified API**: The new `Framebuffer` class provides a more intuitive and easier-to-use interface.

2. **Improved Performance**: Using `raylib::Image` likely improves performance compared to manual buffer management.

3. **Enhanced Features**: The addition of `raylib` brings additional functionality and better graphics capabilities.

4. **Compatibility Issues**: Code using the old `Framebuffer` might need updates to work with the new implementation.

5. **Learning Curve**: Developers familiar with the old implementation may need time to adapt to the new `raylib`-based approach.
