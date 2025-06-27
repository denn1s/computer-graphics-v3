# Lesson 2: Windows, Window Managers, Operating Systems, and Video Drivers

## Introduction

In this lesson, we'll explore the concepts of windows, window managers, operating systems, and video drivers, and how they relate to each other in the context of computer graphics.

## Definitions

*Window:* A rectangular area on the screen where a program displays its output and receives user input. It's a fundamental building block of graphical user interfaces (GUIs).
*Window Manager:* A system software component responsible for managing the placement, sizing, and appearance of windows within a windowing system. It controls the layout and decoration of windows.
*Operating System (OS):* The software that manages computer hardware, software resources, and provides common services for computer programs. Examples include Windows, macOS, and Linux.
*Video Driver:* A software component that enables communication between the operating system and the video hardware, such as the graphics card. It provides a standardized interface for programs to access video hardware capabilities.

## The Abstraction of Windows

The operating system provides an abstraction layer to create the illusion of windows in the framebuffer, which is essentially an array in memory. Here's how the process works:

### 1. The operating system communicates with the video driver to manage the framebuffer and access the video hardware.
### 2. Programs interact with the operating system's API to create and manage windows. They specify the size, position, and content of the windows.
### 3. The window manager, which is a part of the operating system or a separate program running on top of it, receives the window creation requests and manages the layout and appearance of the windows.
### 4. When a program wants to update the content of its window, it renders the graphics using APIs like OpenGL, DirectX, or Vulkan, which communicate with the video driver to access the video hardware.
### 5. The window manager composes the individual windows' contents into a single image that represents the entire screen. This composition process involves combining the framebuffers of each window according to their positions and stacking order.
### 6. The final composed image is then sent to the video driver, which displays it on the screen.

## Window Managers in Different Operating Systems

### - Windows: On Microsoft Windows, the window manager is an integral part of the operating system. It is called the Desktop Window Manager (DWM) and is responsible for compositing windows and providing visual effects.
### - macOS: macOS uses a window manager called Quartz Compositor, which is part of the Cocoa framework. It handles window compositing and provides a consistent user interface experience.
### - Linux: Linux has various window managers available, such as i3, a popular tiling window manager. Other common window managers include Mutter (GNOME), KWin (KDE), and Xfwm (Xfce). These window managers run on top of the X Window System (X11) or the newer Wayland protocol.

The interaction between the operating system, window manager, and video driver creates the seamless experience of windows on the screen. The operating system provides the necessary abstractions and APIs for programs to create and manage windows, while the window manager handles the composition and layout of the windows. The video driver ensures that the final composed image is displayed correctly on the screen.

Understanding this abstraction process and the role of each component is essential for developing graphical app

Core Changes:

1. Updated `.gitignore`:
   - Removed `*.png` from ignored files.

2. Modified `Cargo.toml`:
   - Changed Rust edition from "2021" to "2024".
   - Added a `[profile.dev]` section with `opt-level = 3` and `debug = false`.

3. Updated README.md:
   - Changed the title and introduction to focus on windows, window managers, operating systems, and video drivers.
   - Reorganized content to explain the abstraction of windows.
   - Added sections on window managers in different operating systems.

4. Modified `src/framebuffer.rs`:
   - Introduced a new `_render_to_file` method (renamed from `render_to_file`).
   - Added a new `swap_buffers` method that interacts with Raylib for rendering.

5. Updated `src/main.rs`:
   - Imported `framebuffer` module.
   - Initialized a Raylib window and thread.
   - Implemented a main loop that calls `swap_buffers` continuously.

Direct Impacts:

1. Simplified API: The new `Framebuffer` class provides a more intuitive interface.

2. Improved Performance: Using `raylib::Image` likely enhances performance compared to manual buffer management.

3. Enhanced Features: The addition of `raylib` brings additional functionality and better graphics capabilities.

4. Compatibility Issues: Code using the old `Framebuffer` implementation may require updates to work with the new `raylib`-based approach.

5. Learning Curve: Developers familiar with the old implementation will need time to adapt to the new `raylib`-based approach.

6. New Rendering Approach: The `swap_buffers` method introduces a new rendering mechanism using Raylib, which may require adjustments in how graphics are handled.

7. Window Management Integration: The changes allow for better integration with window management systems through Raylib.

8. Continuous Rendering: The main loop now continuously updates the framebuffer and renders it to the window, enabling smoother animations and interactions.
