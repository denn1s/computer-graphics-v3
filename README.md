# Lesson 4: Drawing Your First Triangle

In this lesson, we will be drawing triangles on the screen.

## Key Definitions

* **Triangle:** A polygon with three edges and three vertices, forming the simplest possible polygon.
* **Vertex:** A point in 2D or 3D space, used to define the corners of a polygon.
* **Primitive:** The basic shape used to construct more complex graphics. In computer graphics, triangles are the most commonly used primitive.
* **Rasterization:** The process of converting vector graphics (like triangles) into pixels on the screen.
* **Fragment:** A potential pixel in rasterized graphics, containing data needed to compute the final color and depth.

## Why Triangles?

Triangles are the simplest polygon that can define a surface in both 2D and 3D space, making them highly efficient for computer graphics. Here are a few reasons why triangles are preferred:

- **Simplicity:** With only three vertices, a triangle is the simplest shape that can define a flat surface, ensuring that any other polygon can be broken down into triangles.
- **Stability:** Triangles are always flat (planar) in 3D space, which avoids issues like concavity or deformation that can occur with more complex polygons.
- **Performance:** Modern GPUs are optimized for triangle processing, making rendering faster and more efficient.
- **Versatility:** Any 3D shape can be approximated by a mesh of triangles, allowing for complex models and detailed surfaces.

## Main Steps to Draw a Triangle

### - Vertex Specification:
        Define the vertices of the triangle in either 2D or 3D space.

### - Buffer Setup:
        Create and configure a buffer to store vertex data and manage graphics memory.

### - Drawing Command:
        Issue a command to draw the triangle using the specified vertices.

### - Rendering Pipeline:
        Transform and rasterize the triangle, converting vertex data to fragments on the screen.

