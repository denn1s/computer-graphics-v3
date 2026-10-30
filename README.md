# CPU Shader Workshop

Teaching branch: `SR-20-ShaderWorkshop-v2`, based on `SR-17-NoiseShaders-v2`.

Run commands from the repository root. The supplied Blender sphere is used unchanged (192 triangles). The Rust renderer computes geometry, rasterization, lighting, and material colors on the CPU; Raylib only presents the finished image and handles the window.

## Run

```sh
cargo run --release
```

Start in the matching CPU sun demo:

```sh
cargo run --release -- --shader 10
```

Change `WINDOW_WIDTH` and `WINDOW_HEIGHT` near the top of `src/main.rs` to change the CPU window, framebuffer, projection, gallery exports, and UI together. The GPU demo has matching constants in `gpu_demo/main.cpp`. Bloom uses the actual framebuffer dimensions.

See [TEACHER_GUIDE.md](TEACHER_GUIDE.md) for the full classroom script, board drawings, exact live-code edits, and image-search suggestions.

## Files to show in class

- `src/shaders.rs`: the live shader and the calls to prepared examples. Edit `live_shader`, restart, and select `0`.
- `src/shader_examples/console_styles.rs`: flat, Gouraud, per-fragment lighting, and a simplified N64 combiner.
- `src/shader_examples/planets.rs`: toon bands, animated stripes, continents, gas bands, and ice veins.
- `src/shader_examples/stars.rs`: matching CPU/GLSL sine-wave sun and an OpenSimplex star.
- `src/shader_examples/noise_helpers.rs`: noise sampling and four-octave fBm.
- `src/shader_examples/passes.rs`: bright extraction, horizontal blur, vertical blur, and bloom composition.

These files intentionally contain no explanatory comments. Renderer setup and controls are already implemented elsewhere.

## Controls

| Key | Action |
| --- | --- |
| `0` | Live shader |
| `1`, `2`, `3` | Flat, Gouraud, per-fragment lighting |
| `4` | N64-style mask recoloring |
| `5`, `6` | Toon, animated bands |
| `7`, `8`, `9` | Rocky planet, gas giant, ice world |
| `F1`, `F2` | Sine-wave sun, noise star |
| Left / Right | Previous / next shader |
| Space | Pause shader time and model rotation |
| `R` | Toggle model rotation |
| `N` | Toggle generated smooth normals / exported Blender normals |
| `B` | Toggle bloom after scene rendering |
| WASD | Orbit camera |
| Up / Down | Zoom |
| Home | Reset camera, model angle, and shader time |
| Escape | Exit |

FPS, total milliseconds per frame, and CPU rendering milliseconds appear in the window title. Bloom starts disabled. Animation and rotation use elapsed time, not frame rate. `frame_count` continues advancing while animation is paused.

## Inputs available to shaders

| Input | Meaning |
| --- | --- |
| `fragment.position` | Screen sample position in pixels; Y increases downward |
| `fragment.object_position` | Original mesh position; patterns stay attached during rotation |
| `fragment.world_position` | Position after the model transform |
| `fragment.normal` | Normalized world-space interpolated normal |
| `fragment.face_normal` | World-space triangle normal |
| `fragment.uv` | Perspective-correct interpolated Blender UV coordinates |
| `fragment.color` | Unlit white base color, independent of lighting |
| `fragment.depth` | NDC depth, from -1 to 1 |
| `fragment.intensity` | Lambert lighting calculated per fragment |
| `fragment.flat_intensity` | One Lambert value calculated at the face center |
| `fragment.gouraud_intensity` | Screen-space interpolation of vertex lighting |
| `uniforms.time` | Animation time in seconds |
| `uniforms.delta_time` | Animation step in seconds, zero when paused |
| `uniforms.frame_count` | Number of previously rendered frames |
| `uniforms.resolution` | Framebuffer width and height |
| `uniforms.camera_position` | World-space camera position |
| `uniforms.light_position` | World-space point-light position |
| `uniforms.noise` | Reusable OpenSimplex generator with seed 42 |
| `uniforms.mask` | Small grayscale texture prepared once for the combiner demonstration |

Transformation matrices and the selected shader index are also in `Uniforms`. Return an RGB `Vector3`; display conversion clamps its components to 0–1. A star can ignore lighting entirely. A planet can multiply its procedural color by `0.12 + fragment.intensity * 0.88` to retain some visibility on the dark side.

Generated smooth normals are enabled initially so the flat/Gouraud/per-fragment comparison is visible. They average adjacent face normals without modifying positions. Press `N` to restore the exported faceted normals. The silhouette is identical in both modes.

## Suggested live progression

1. Change the constant color in `live_shader`.
2. Return `fragment.normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)` to visualize normals.
3. Compare presets 1–3 and inspect their different intensity inputs.
4. Demonstrate `(A - B) * C + D` with preset 4.
5. Write a sine wave from `fragment.object_position.y`, remap it to 0–1, and mix two colors.
6. Add `uniforms.time`, then apply lighting.
7. Replace or distort the wave with OpenSimplex. Explore the prepared planets and stars.
8. Enable bloom and inspect the separate image passes.
9. Open the GLSL sun and compare its math with `stars::sun`.

The console examples illustrate shading ideas, not complete console emulation. The N64 example samples a precomputed texture, recolors it with one combiner calculation, then applies Gouraud shade with another. Its float implementation omits the real hardware's input-routing restrictions and fixed-point rules. Two combiner cycles are not two scene render passes. References: [Nintendo's combiner tutorial](https://jrra.zone/n64/doc/tutorial/graphics/4/4_1.htm), [SGI/Nintendo RDP command reference](https://hcs64.com/files/RDP_COMMANDS.pdf).

The workshop's normal transform assumes rotation and uniform scaling. Camera controls keep the supplied sphere in front of the near plane; this is not a general homogeneous polygon-clipping implementation. Procedural materials use perspective-correct surface interpolation; the historical Gouraud demonstration intentionally uses screen-space color/intensity interpolation.

## C++ / OpenGL comparison

Requires a C++17 compiler, CMake, GLFW, GLEW, GLM, and OpenGL 3.3 or later.

```sh
./run.sh --gpu
```

- `gpu_demo/main.cpp`: window, sphere upload, shader compilation/linking, uniforms, draw call.
- `gpu_demo/sphere.vert`: transformations and interpolated object coordinates.
- `gpu_demo/sun.frag`: the same math as the CPU sine-wave sun.

Space pauses, `R` toggles rotation, Home resets time/angle, and `F5` reloads GLSL files. Compilation failures print a diagnostic and keep the previous working program. Shader edits in Rust require rebuilding/restarting.

The GPU renderer prints the actual renderer and OpenGL version at startup. Both demos use the same OBJ, initial camera, 1200×800 framebuffer, sun equations, and no intentional frame limiter. GPU VSync is explicitly disabled; a compositor may still limit presentation. Compare CPU preset 10 with bloom off and the default camera. The displayed FPS measures each complete application, including presentation; it is not a GPU execution timer or a Rust-versus-C++ benchmark. CPU uploads and UI also contribute to its total time.

## Export images for the board

```sh
cargo run --release -- --gallery /tmp/shader-gallery
```

This works without a window. It exports all twelve presets at time 2 seconds and the sun's scene, bright extraction, blur, and bloom images. The mask is a teaching aid; the planet/star shaders generate their appearances from coordinates and noise rather than loaded surface images.

Capture the matching GPU frame as a PPM image:

```sh
./target/gpu-demo/shader_workshop --frames 1 --time 2 --capture /tmp/gpu-sun.ppm
```

For a bounded window smoke check:

```sh
cargo run --release -- --shader 10 --frames 3
```

Run `cargo test` to check combiner endpoints, perspective interpolation, all preset outputs, and bloom spreading.
