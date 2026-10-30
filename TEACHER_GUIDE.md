# Teacher guide: one sphere through the history of shading

**Class:** approximately 120 minutes. **Tomorrow:** students create procedural planets and stars with the supplied sphere and CPU shaders. **Branch:** `SR-20-ShaderWorkshop-v2`.

**The sentence to return to:** “What information does this fragment need, and what rule turns that information into a color?”

The class alternates images, drawings, and code. Keep the same mesh throughout: students should see that changing the surface rule can produce radically different worlds without adding geometry.

## 1. Before students arrive

### Open these applications and files

- Excalidraw, with an empty horizontal board and the reference images collected below.
- Your editor, with these tabs in this order:
  1. [`src/shaders.rs`](src/shaders.rs), at `live_shader`.
  2. [`src/shader_examples/console_styles.rs`](src/shader_examples/console_styles.rs), at `flat`.
  3. [`src/shader_examples/planets.rs`](src/shader_examples/planets.rs), at `toon`.
  4. [`src/shader_examples/noise_helpers.rs`](src/shader_examples/noise_helpers.rs), at `sample`.
  5. [`src/shader_examples/stars.rs`](src/shader_examples/stars.rs), at `sun`.
  6. [`src/shader_examples/passes.rs`](src/shader_examples/passes.rs), at `bloom`.
  7. [`gpu_demo/sun.frag`](gpu_demo/sun.frag), at `main`.
  8. [`gpu_demo/main.cpp`](gpu_demo/main.cpp), at `compile_shader`.
  9. [`gpu_demo/sphere.vert`](gpu_demo/sphere.vert), at `main`.
- One terminal in the repository root.
- This guide on a second display, if available. Use editor search for function names; line numbers change as you type.

Do not open all the renderer internals during the main lesson. The already-calculated fragment inputs let the class stay in shader files. `Fragment` and `Uniforms` are reference material if someone asks where an input comes from.

### Run both demos once

```sh
./run.sh
```

Close with Escape, then:

```sh
./run.sh --gpu
```

Check that the terminal prints your Intel renderer rather than a software renderer. This machine was verified with Intel Iris Xe / OpenGL 4.6; the example requests only OpenGL 3.3.

Both windows are now 1200 × 800. On CPU, `WINDOW_WIDTH` and `WINDOW_HEIGHT` in `src/main.rs` control the window, texture, framebuffer, projection, viewport, and exported images together. The C++ file has its own matching constants. Do not change just the texture dimensions.

### Export board images

Current generated images are in [`outputs/shader-workshop`](outputs/shader-workshop). Refresh them with:

```sh
./run.sh --gallery outputs/shader-workshop
```

Useful files to drag onto Excalidraw:

- `overview.png`: contact sheet of all presets (a preview, not the opening reveal).
- `01.png`, `02.png`, `03.png`: flat, Gouraud, and per-fragment lighting.
- `04.png`: combiner recoloring.
- `07.png`, `08.png`, `09.png`: rocky, gas, and ice worlds.
- `10.png`, `11.png`: sine-wave sun and noise star.
- `sun-scene.png`, `sun-bright.png`, `sun-blur.png`, `sun-bloom.png`: actual pass outputs.

The gallery command exports individual images; the overview is a prepared contact sheet. Re-exporting does not modify your shader source.

### Controls card: keep this visible to yourself

| Key | Result |
| --- | --- |
| `0` | Your editable `live_shader` |
| `1`, `2`, `3` | Flat, Gouraud, per-fragment lighting |
| `4` | N64-style combiner |
| `5`, `6` | Toon and animated bands |
| `7`, `8`, `9` | Rocky planet, gas giant, ice world |
| `F1`, `F2` | Sine-wave sun and noise star |
| Left / Right | Browse all presets |
| Space | Pause animation time and model rotation |
| `R` | Toggle model rotation; initially off |
| `N` | Switch smooth normals / exported Blender normals; initially smooth |
| `B` | Toggle bloom; initially off |
| WASD | Orbit camera |
| Up / Down | Zoom |
| Home | Reset camera, object angle, and time |
| Escape | Close |

**Home does not turn off bloom, pause, rotation, or normal settings.** Restarting gives a known default state. CPU shader edits require closing and rebuilding; GLSL edits can be reloaded with F5 in the GPU window. Pressing `R` in the CPU window rotates the sphere; it does not reload Rust.

### Live-code routine

1. Close the CPU window with Escape.
2. Edit only `live_shader` unless the instructions explicitly say otherwise.
3. Save the file.
4. Run `./run.sh` again; it starts in preset `0`.
5. Ask for a prediction before looking at the output.
6. If the change fails, use the corresponding prepared preset and explain the same idea there.

All “replace `live_shader`” blocks below are complete replacements for that function, not additions of a second function with the same name. Keep the preset dispatch function above it. Underscores in `_fragment` or `_uniforms` mean “currently unused”; remove the underscore when using that parameter.

## 2. Timing and emergency route

| Minutes | Segment | Main file / action |
| --- | --- | --- |
| 0–7 | Original versus remake: what changed? | Images + board |
| 7–17 | SGI, Pixar, and the programmable-surface idea | Images → first constant color |
| 17–32 | PS1: flat and Gouraud | `shaders.rs`, `console_styles.rs` |
| 32–52 | N64: choose inputs to a color combiner | `console_styles.rs`, live replacements |
| 52–62 | PS2 bridge, programmable fragments, toon shading | Timeline + `planets::toon` |
| 62–78 | Coordinates, waves, colors, and time | `live_shader` |
| 78–93 | Noise becomes a planet or star | `noise_helpers.rs`, `planets.rs`, `stars.rs` |
| 93–103 | Separate rendering/image passes | `passes.rs` + exported images |
| 103–115 | The same sun in actual GLSL | C++ / OpenGL files |
| 115–120 | Node editors and tomorrow's creative task | Screenshots + closing questions |

These are target windows, not obligations to rush students. If running late, skip the optional specular experiment and detailed C++ OBJ-loader discussion, show the prepared gas giant rather than typing it, and use screenshots for the individual bloom passes. Keep the N64 arithmetic, one animated live shader, one noise example, and the CPU/GLSL comparison.

## 3. Image shopping list and Excalidraw setup

Collect images before class. Use game titles and platform names in the search so ports, remasters, emulator enhancements, and fan remakes are not silently mixed together. Use actual gameplay for rendering discussion, not prerendered cinematics or promotional illustrations. For historical comparison, avoid emulator screenshots with added texture packs or postprocessing unless you label those changes.

| What to Google | Choose an image showing | What to look for / say | What to draw over it |
| --- | --- | --- | --- |
| `Ocarina of Time N64 Kokiri Forest original gameplay` | Link, ground, a tree or wall, and some distance | Separate the polygon silhouette from color/detail inside the surface | Circle a silhouette corner; box a texture patch; arrow toward distant scenery |
| `Ocarina of Time Switch 2 Nintendo official trailer 2026 Kokiri Forest` | The closest available matching location from official footage | Compare geometry, surface detail, lighting, and art choices separately | Match the circles/boxes from the original; put question marks on effects whose implementation is unknown |
| `Final Fantasy VII PS1 original battle Cloud model` | In-game battle model, preferably a close view with visible faces | Flat areas and interpolated color are useful visual clues; the screenshot is not proof of its exact GPU commands | Outline one polygon and mark its corners |
| `Final Fantasy VIII PS1 original gameplay character closeup` | Textured in-game characters | Textures supply detail beyond a solid face color | Draw a texture square feeding a triangle |
| `Silicon Graphics Indigo2 workstation` or `SGI Onyx workstation` | A clearly identified workstation | Graphics hardware and APIs were already a serious industry before modern shader languages | Hardware box → graphics API → image |
| `Pixar RenderMan 2001 Oscar Catmull Cook Carpenter` | Pixar's award history or the people/award | The award recognized rendering advances, not an award for a particular GPU | Film branch from the timeline, labeled “programmable appearance; offline rendering” |
| `Toy Story 1995 Woody Buzz still Pixar` | A frame with contrasting materials | Plastic, cloth, and wood need different appearance rules | Circle two materials and ask what changes in the recipe |
| `Final Fantasy X PS2 original gameplay Tidus Besaid` | A gameplay scene rather than an FMV or HD remaster | A richer image does not imply arbitrary fragment programs on the Graphics Synthesizer | Emotion Engine / vector units → Graphics Synthesizer |
| `Wind Waker GameCube Wind Waker HD Wii U comparison same scene` | The same location and character pose in both releases | Stylization existed before today's shader model; examine differences without treating the HD version as “only a shader change” | Mark broad lighting regions and bright areas; keep geometry/lighting/resolution as separate labels |
| `Final Fantasy XIII PS3 gameplay Lightning armor closeup` | Several material types in gameplay | Ask what inputs a shader might need to distinguish skin, fabric, and metal; do not claim a specific implementation from the picture | Normal + view + light + material parameters → surface color |
| `Unity Shader Graph sine time lerp tutorial graph` | A small readable graph with Time, Sine, Multiply, Lerp, and output | Students should recognize their own arithmetic | Connect the matching code line to each node |
| `Unreal Engine Material Editor sine time lerp emissive` | Material graph, not a gameplay Blueprint Event Graph | The output can be material properties; the engine supplies more shader work | Circle Emissive or Base Color and the nodes feeding it |

**Ocarina note for this class date:** Nintendo currently lists the Switch 2 remake for November 5, 2026. On October 29, use official preview footage and call it upcoming. The 2011 Nintendo 3DS remake is a different version; label it explicitly if included. Do not imply that the 3DS had the same fully programmable fragment stage as a modern desktop GPU. [Nintendo's current game page](https://www.nintendo.com/en-gb/Games/Nintendo-Switch-2-games/The-Legend-of-Zelda-Ocarina-of-Time-3115664.html), [Nintendo's explanation of the 3DS graphics approach](https://www.nintendo.co.jp/ir/en/events/101029qa/03.html).

Official editor screenshots are available in [Unity's Shader Graph documentation](https://docs.unity.com/en-us/engine/6000.3/manual/materials-and-shaders/shaders/writing-custom/shader-graph) and [Epic's Material Editor UI documentation](https://dev.epicgames.com/documentation/en-us/unreal-engine/unreal-engine-material-editor-ui). Nintendo's [Wind Waker HD developer interview](https://www.nintendo.com/en-gb/Iwata-Asks/Iwata-Asks-The-Legend-of-Zelda-The-Wind-Waker-HD/The-Legend-of-Zelda-The-Wind-Waker-HD/5-Over-designed/5-Over-designed-807330.html) is a useful source for that comparison.

### Board layout

Make four horizontal frames and move left to right as the class develops:

1. **“What changed?”** Original/remake images at top, student observations below.
2. **“One triangle, one fragment.”** Pixel grid and the evolving color-computation box.
3. **“Write the rule.”** Coordinates, wave, remapping, noise, and time.
4. **“Whole images and real hardware.”** Pass chain, CPU/GLSL side-by-side, node graph.

Keep a narrow timeline at the bottom. Use blue for data, orange for configurable hardware operations, and green for functions students can write. Keep arrows simple. The important edit is replacing an orange recipe box with a green function while the surrounding inputs remain recognizable.

## 4. Opening: make the image a question (0–7 minutes)

**Screen:** Ocarina original and official remake imagery.

**Say:**

> “Same world, different generations of graphics. What changed? Point to something specific rather than just saying ‘better graphics.’”

Take three answers. Sort them into **geometry**, **surface detail**, and **lighting/appearance**. Add **resolution** or **art direction** when appropriate.

> “Today we'll keep the geometry almost fixed. We will change the rules that color it. By tomorrow, this one rough sphere can become several worlds.”

**Board:** Draw a triangle on a pixel grid. Mark a sample near its center and write:

```text
triangle → covered samples + interpolated values → fragments → color → tests/blending → image
```

**Say:**

> “A fragment is a candidate contribution to the image. It has more information than a color: coordinates, depth, and interpolated attributes. Several triangles can produce candidates for the same pixel. A candidate may fail the depth test.”

Use one sample per pixel for today's mental model. Do not promise a universal one-fragment/one-pixel relationship. Hardware can perform some tests early; our software renderer depth-tests before calling the material function when possible.

**Question:** “If I know which triangle covers this sample, do I already know its final color?”

**Expected answer:** No; a color rule and its inputs are still needed.

## 5. Origins, then the first tiny shader (7–17 minutes)

**Screen:** SGI workstation, then Pixar image/award.

**Say:**

> “SGI helped establish the workstation graphics world. OpenGL grew out of that ecosystem; its first specification dates to 1992. An API lets software request graphics operations without wiring the hardware itself.”

OpenGL originated as an open alternative to SGI's Iris GL. Keep this to one minute. [Khronos OpenGL history](https://wikis.khronos.org/opengl/History_of_OpenGL).

> “Film rendering gives us a parallel idea: the surface has a programmable description. Pixar's RenderMan is a major part of that story. In 2001, Ed Catmull, Loren Carpenter, and Rob Cook received an Academy Award of Merit for rendering advances exemplified by RenderMan.”

Say the names slowly if you want students to remember them. This was a technical Academy Award, not the Best Animated Feature award and not an award for inventing modern GPU fragment shaders. Film rendering and real-time hardware evolved under different constraints. [Pixar's award history](https://renderman.pixar.com/about).

**Board:** Two short branches: `film → programmable appearance` and `interactive hardware → fast, constrained operations`. They will reconnect at programmable real-time shading.

### Code checkpoint A: one function returns one color

**Open:** `src/shaders.rs`, function `live_shader` at the bottom.

The existing imports already provide `Vector3`, `Fragment`, and `Uniforms`. No import changes yet.

**Say:**

> “This is our surface rule. The renderer has already done the setup. For each visible candidate, it asks this function for a color. We can ignore all the inputs and return orange.”

The starting function is:

```rust
pub fn live_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    Vector3::new(1.0, 0.4, 0.1)
}
```

Change just the last line to `Vector3::new(0.1, 0.7, 1.0)`. Save, run `./run.sh`, and show the flat-color silhouette. Then restore orange if preferred.

**Ask:** “Did I change any vertices?” Answer: no. “Why does the silhouette still look rough?” Answer: it is still the same 192-triangle mesh.

**Board:** Add a green `f(fragment, uniforms) → RGB` box. Explain that our Rust function executes on the CPU. We will move a matching rule to the GPU later.

## 6. PS1: choose the operation, supply the data (17–32 minutes)

**Screen:** PS1 gameplay image; then our sphere.

**Say:**

> “The original PlayStation could fill flat-colored polygons, interpolate vertex colors, and combine texture samples with color. Developers selected modes and supplied data. We are recreating those ideas in Rust, not emulating the entire console.”

The GPU's flat/Gouraud selection concerns color interpolation. Lighting could be computed before submitting the polygon; do not say that the GPU's flat-shading mode automatically computed a face normal or light. [PS1 GPU command and shading reference](https://psx-spx.consoledev.net/graphicsprocessingunitgpu/).

### Code checkpoint B: one lighting value for the face

**Open:** `src/shader_examples/console_styles.rs`, `flat`. Show the one return expression. Then return to `src/shaders.rs` and replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let color = Vector3::new(0.7, 0.8, 1.0);
    color * (0.12 + fragment.flat_intensity * 0.88)
}
```

No new imports. Save and run.

**Say:**

> “The renderer calculated one light value using the face normal and face center. Every fragment in that face receives that same value. Here we choose to use it. The small 0.12 floor keeps the unlit side visible; it is an artistic ambient approximation.”

**Board:** One normal arrow in the center of a triangle. Write `one face → one light value`. A small side note can show `max(N · L, 0)`.

**Look for:** Flat patches with abrupt changes between faces. Prepared fallback: key `1`.

### Code checkpoint C: light the vertices, interpolate colors

Change exactly `fragment.flat_intensity` to `fragment.gouraud_intensity`:

```rust
pub fn live_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let color = Vector3::new(0.7, 0.8, 1.0);
    color * (0.12 + fragment.gouraud_intensity * 0.88)
}
```

Save and run. Use generated smooth normals, the default. If the result is unexpectedly faceted, press `N` once or restart.

**Say:**

> “Now lighting is evaluated at the vertices. Inside the triangle, we interpolate those resulting values. That is the central Gouraud idea. With a constant base color, interpolating the intensity gives the same color interpolation we need for this example.”

**Board:** Three corner normal arrows; label colors `C0`, `C1`, `C2`. Write:

```text
C = w0 C0 + w1 C1 + w2 C2
w0 + w1 + w2 = 1
```

Shade the triangle gradually. Circle the unchanged silhouette of the sphere.

**Question:** “Did making the inside smoother make the outside less polygonal?” Answer: no.

Prepared fallback: key `2`. Toggle `1` / `2` a few times without moving the camera.

### A brief look forward: per-fragment lighting

Change only `fragment.gouraud_intensity` to `fragment.intensity`, or press `3`.

**Say:**

> “This third version interpolates the normal and evaluates the lighting equation at the fragment. It is a different place to perform the calculation. Our simple diffuse sphere may look only subtly different; the change is easier to notice with a small specular highlight.”

Do not claim per-fragment lighting is required for all smooth-looking surfaces. Do not call normal interpolation Gouraud shading. The optional specular demonstration is near the end of this guide if students want a stronger contrast.

**Transition:** “Even before arbitrary fragment programs, hardware could offer more expressive color recipes. Let's take apart one of them.”

## 7. N64: the combiner becomes a small puzzle (32–52 minutes)

**Screen:** Original Ocarina image. Briefly point out that its appearance combines several systems: geometry, textures, shading, fog, and other effects. We are isolating one hardware block, not reconstructing this game's exact rendering state.

### Start on the board, before code

Draw:

```text
texture sample ──┐
shade color ─────┤
constant colors ┤ → choose A, B, C, D → (A − B) × C + D → RGB
0 and 1 ────────┘
```

**Say:**

> “This expression is built into the combiner. You choose the sources connected to its inputs. The calculation is performed for red, green, and blue separately. Different settings produce different effects, even though the arithmetic structure stays the same.”

The actual N64 has restrictions on which sources can connect to each slot and has a corresponding alpha calculation. Today's function uses convenient floating-point RGB values; it does not emulate fixed-point precision or every legal hardware setting. [Nintendo tutorial: combiner architecture](https://jrra.zone/n64/doc/tutorial/graphics/4/4_1.htm).

Define the four names you will use:

| Name | Classroom meaning |
| --- | --- |
| Texel | A sampled texture color; our demo uses a grayscale mask |
| Shade | Interpolated shading/color from the geometry stage |
| Primitive color | A constant set by the application |
| Environment color | Another constant set by the application; the name does not mean “environment map” |

In two-cycle mode, the second calculation can use the first result as **combined color**. [Nintendo tutorial: source colors and alpha](https://jrra.zone/n64/doc/tutorial/graphics/4/4_2.htm).

### Open the prepared arithmetic

**Open:** `src/shader_examples/console_styles.rs`, `combine`.

Read the red channel aloud:

```text
(a.x − b.x) × c.x + d.x
```

Point to the green and blue lines: “Same operation, different component.” Do not spend time on matrix transforms here.

### Code checkpoint D: show the mask first

**Open:** `src/shaders.rs`. Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    uniforms.mask.sample(fragment.uv)
}
```

No new imports. Save and run.

**Say:**

> “This grayscale pattern was prepared once as a small texture. The shader is looking up a value using UV coordinates. We are not asking the historical combiner to calculate sine or noise.”

If asked, the texture is generated in `src/mask.rs`, `Mask::new`; that setup is outside the fragment function. It is fine to answer without opening that file.

**Board:** Draw a black-to-white square with axes U and V. Connect it to a sphere. Clarify that UVs are coordinates on a texture and object position is a 3D coordinate in the model; they are different inputs.

### Code checkpoint E: recolor the mask

Keep the existing `console_styles` import at the top of `shaders.rs`. It already exists; do not import it twice.

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let texel = uniforms.mask.sample(fragment.uv);
    let primitive = Vector3::new(1.0, 0.3, 0.02);
    let environment = Vector3::new(0.03, 0.04, 0.22);
    console_styles::combine(primitive, environment, texel, environment)
}
```

**Before running, ask:** “Which color will a black part of the mask become? Which color will a white part become?”

**Board calculation:** Call the orange color `P`, the blue color `E`, and the grayscale mask `m`:

```text
(P − E)m + E
m = 0 → E
m = 1 → P
m = 0.5 → halfway between E and P
```

For a scalar example, use `P = 0.8`, `E = 0.2`, `m = 0.25`: `(0.8 − 0.2) × 0.25 + 0.2 = 0.35`.

**Say:**

> “We've used a fixed expression to interpolate between two colors. The mask decides where each color appears. Changing two constants can recolor the same pattern without replacing the texture.”

Save and run. Change `primitive` to `Vector3::new(0.1, 1.0, 0.4)` and predict the new result. Restore or keep it; the rest of the lesson does not depend on the exact palette.

### Code checkpoint F: combine a second time

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let texel = uniforms.mask.sample(fragment.uv);
    let primitive = Vector3::new(1.0, 0.3, 0.02);
    let environment = Vector3::new(0.03, 0.04, 0.22);
    let recolored = console_styles::combine(primitive, environment, texel, environment);
    let shade = Vector3::new(1.0, 1.0, 1.0) * (0.15 + fragment.gouraud_intensity * 0.85);
    console_styles::combine(recolored, Vector3::zero(), shade, Vector3::zero())
}
```

**Board:** Add a second small combiner box. Label the result of the first `combined`. The second performs `(combined − 0) × shade + 0`.

**Say:**

> “The first calculation chooses the material colors; the second modulates them by the shade value. These are two color-combiner calculations for the current fragment. We have not rendered the scene twice.”

The two-cycle distinction is described in the [SGI/Nintendo RDP command reference](https://hcs64.com/files/RDP_COMMANDS.pdf).

### Show the prepared animated version

**Open:** `console_styles.rs`, `n64`. Run preset `4`.

Explain the lines in order:

1. `mask.sample(fragment.uv)` reads the grayscale pattern.
2. `primitive = mix(...)` animates a constant palette color in our software example.
3. `environment` provides the second endpoint color.
4. `recolored` applies the first expression.
5. `shade` uses the already-calculated Gouraud value.
6. The last call applies the second expression.

**Historical qualification:** The sine-based animation is our host-side/software demonstration of varying a constant. On the actual console, software could update a color register between draws/frames; the combiner did not execute our Rust `sin` call.

**Question:** “Can I change the palette without changing the mesh or texture?” Answer: yes. “Can I put any arbitrary program inside this box?” Answer: not with this fixed combiner.

**Fallback if this section runs long:** The mask → recolor demonstration is enough. Show the final prepared `n64` function instead of typing the second cycle.

## 8. PS2 bridge and programmable fragments (52–62 minutes)

**Screen:** Original FFX gameplay, then a short timeline. Use the Emotion Engine story you know, but keep the key architectural distinction visible.

**Board:**

```text
Emotion Engine / programmable vector units → geometry + data → Graphics Synthesizer
```

**Say:**

> “Programmability is not one yes/no property of the entire console. The PS2 had programmable vector processing in the Emotion Engine. That is different from an arbitrary modern fragment program running in its pixel pipeline.”

The hardware designers describe VU1's role in geometry and lighting in their [Emotion Engine paper](https://www.eecg.toronto.edu/~moshovos/ACA07/readings/EmotionEngine.pdf). Avoid presenting “no modern fragment shader” as “no programmability anywhere.”

### Selected milestones, not a ranking of artistic quality

| Platform | What to say about fragment processing |
| --- | --- |
| PS1 (1994) | Fixed polygon drawing modes: flat/Gouraud color, texture operations; software supplies data and selects modes |
| N64 (1996) | Configurable color combiner and blender; the geometry processor's programmability is a separate topic |
| PS2 (2000) | Programmable vector units upstream; Graphics Synthesizer is not a modern general fragment-program stage |
| Xbox (2001) | Early pixel-shader-era flexibility with strong limitations, unlike current GLSL |
| GameCube (2001), Wii (2006) | Configurable TEV stages; a useful reminder that impressive material effects do not require today's general shader model |
| PS3 (2006) | RSX programmable vertex/fragment pipelines; a clear PlayStation milestone for this lesson |
| Wii U (2012) | Fully programmable shader pipeline; useful context for Wind Waker HD |

For Xbox's limited early fragment capabilities, use [NVIDIA's contemporary Cg tutorial](https://download.nvidia.com/developer/cg/Cg_Tutorial/Chapter_1.pdf). For GameCube/Wii TEV, use [Dolphin's technical account](https://dolphin-emu.org/blog/2017/07/30/ubershaders/). For PS3, [Sony's hardware announcement](https://sonyinteractive.com/en/press-releases/2005/sony-computer-entertainment-inc-to-launch-its-next-generation-computer-entertainment-system-playstation-3-in-spring-2006/) describes programmable shader pipelines; its planned spring release is not the actual launch date. [Cemu's developer documentation](https://wiki.cemu.info/wiki/Graphic_packs_creation) describes Wii U's programmable pipeline.

**Say:**

> “Now replace the combiner box with a function whose arithmetic we can write. We can evaluate lighting, thresholds, waves, texture lookups, and procedural patterns in that function.”

### Toon example: use a programmable rule for an artistic look

**Open:** `src/shader_examples/planets.rs`, `toon`; run key `5`.

Read the thresholds: above 0.7 returns 1.0, above 0.3 returns 0.6, otherwise 0.2. Explain that those are selected brightness levels, not extra polygons.

**Board:** Draw a continuous brightness ramp and a three-step staircase beside it.

**Say:**

> “The input lighting is continuous. Our rule groups it into a few bands. This is one simple way to make toon-like lighting.”

Show Wind Waker imagery here. **Do not say this Rust threshold function is Wind Waker's original implementation.** It is a modern teaching recipe for a visual idea that older hardware also achieved through other mechanisms.

## 9. Coordinates, waves, and animation (62–78 minutes)

### Prepare one additional import

**Open:** `src/shaders.rs`. Under the existing imports, add exactly once:

```rust
use crate::shader_examples::mix;
```

`mix` is a small helper from `src/shader_examples/mod.rs`. If asked, show its expression `a * (1.0 - amount) + b * amount`. This is the same interpolation idea we just discovered in the combiner.

### Code checkpoint G: visualize a coordinate

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let value = (fragment.object_position.y + 1.0) * 0.5;
    Vector3::new(value, value, value)
}
```

**Say:**

> “Instead of sampling an image, I am using the surface's original Y coordinate as data. Each fragment receives a different position, so the same function produces different outputs.”

Our mesh is centered near zero but is not a mathematically exact unit sphere. This formula gives a useful ramp without implying its top and bottom must reach exactly 1 and 0.

**Board:** A vertical axis through the sphere; dark at the bottom, light at the top.

**Optional 30-second coordinate contrast:** Replace the `value` line with:

```rust
let value = fragment.position.y / uniforms.resolution.y;
```

Also rename `_uniforms` to `uniforms`. That produces a screen-space ramp; screen Y increases downward. Undo that experiment before proceeding. Object-space patterns rotate with the model; screen-space patterns stay tied to the display. Use `R` to compare with the later stripes.

### Code checkpoint H: a wave is already a material mask

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let wave = (fragment.object_position.y * 22.0).sin();
    let amount = wave * 0.5 + 0.5;
    Vector3::new(amount, amount, amount)
}
```

**Board:** `position → × frequency → sin → × 0.5 + 0.5`. Label the ranges `−1…1` and `0…1`.

**Say:**

> “The wave gives us a repeating pattern. Remapping it gives the same kind of black-to-white mask we previously stored in a texture.”

Ask what changing 22 to 8 would do before making the edit. Answer: fewer, wider bands. Restore 22 if you want the prepared result.

### Code checkpoint I: mix two colors, then light them

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, _uniforms: &Uniforms) -> Vector3 {
    let wave = (fragment.object_position.y * 22.0).sin();
    let amount = wave * 0.5 + 0.5;
    let color = mix(
        Vector3::new(0.2, 0.04, 0.4),
        Vector3::new(0.9, 0.4, 0.7),
        amount,
    );
    color * (0.12 + fragment.intensity * 0.88)
}
```

**Say:**

> “The pattern decides the material color. The lighting controls how bright it appears. Keeping those ideas separate makes the shader easier to change.”

Temporarily replace the last line with `color` to show the unlit version. Restore the lighting line. Avoid multiplying an already-lit color by lighting twice.

### Code checkpoint J: time is just another input

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let wave = (fragment.object_position.y * 22.0 + uniforms.time).sin();
    let amount = wave * 0.5 + 0.5;
    let color = mix(
        Vector3::new(0.2, 0.04, 0.4),
        Vector3::new(0.9, 0.4, 0.7),
        amount,
    );
    color * (0.12 + fragment.intensity * 0.88)
}
```

The only conceptual change is adding time to the wave's phase. Save and run. Press Space to freeze; press it again to resume. Keep rotation off initially so movement is clearly coming from the material.

**Say:**

> “The geometry is stationary. The function changes because its time input changes. A uniform is shared by the fragments of this draw; the position varies across them.”

**Ask:** “Would using frame count instead of elapsed seconds give the same speed at 30 FPS and 120 FPS?” Answer: no, unless you compensate. Our time uses elapsed seconds.

Prepared fallback: key `6`, then open `planets::bands` and identify the same operations.

## 10. Noise: reuse what they know to make worlds (78–93 minutes)

### Add the noise imports

In `src/shaders.rs`, add this once, leaving the earlier `mix` import:

```rust
use crate::shader_examples::noise_helpers::{fbm, sample};
```

`sample` wraps the noise crate; no direct `NoiseFn` import is needed in `shaders.rs` when using this helper. The existing `Uniforms` contains a reusable `OpenSimplex` generator.

### Code checkpoint K: see the raw signal

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let value = sample(&uniforms.noise, fragment.object_position * 4.0);
    let amount = value * 0.5 + 0.5;
    Vector3::new(amount, amount, amount)
}
```

**Say:**

> “You already used noise to control terrain height. Today the output controls appearance. We have not displaced a vertex.”

**Board:** Branch one noise value into `terrain height` and `material mask`. Cross out neither; both are valid uses.

**Open:** `src/shader_examples/noise_helpers.rs`, `sample`.

Explain: it samples a 3D field at x, y, z; `f64` conversions satisfy the library API, and the helper returns `f32` for our renderer. Treat the sample as a roughly signed noise signal and remap it before using it as a blend amount.

Change the frequency 4.0 to 12.0 if there is time. Higher frequency means smaller features, not automatically a better planet. Very fine patterns can alias or shimmer because pixels sample finite points.

### Code checkpoint L: noise controls land and sea

Replace `live_shader`:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let height = fbm(&uniforms.noise, fragment.object_position * 3.0);
    let color = if height > 0.5 {
        Vector3::new(0.2, 0.5, 0.15)
    } else {
        Vector3::new(0.02, 0.1, 0.35)
    };
    color * (0.12 + fragment.intensity * 0.88)
}
```

**Say:**

> “I'm naming this value height because it is useful to imagine a terrain field. But here it only selects color. Land and water still occupy the same original surface.”

Ask what changing the threshold to 0.6 does. Expected answer: less land. This is a good student suggestion to try live.

**Open:** `noise_helpers.rs`, `fbm`.

Trace just four operations: sample, add weighted value, double frequency, halve amplitude. The final division normalizes the accumulated weights; then the result is remapped. More octaves add detail and more work. Do not construct a new generator for every fragment; this code reuses `uniforms.noise`.

### Prepared tour: do not type these from scratch

| File / function | Run | Point to these lines / expressions | Say |
| --- | --- | --- | --- |
| `planets.rs` → `rocky` | `7` | `height`, ocean/land colors, `smoothstep`, snow mix | “Same noise field, a richer palette, and soft transitions around thresholds.” |
| `planets.rs` → `gas_giant` | `8` | `drift`, `turbulence`, the sine argument | “Noise bends the bands instead of replacing them. Time moves the sampled field.” |
| `planets.rs` → `ice` | `9` | absolute noise, `smoothstep`, `view`, `rim` | “Values near a noise zero-crossing make dark veins. The normal and view direction add a stylized rim.” |
| `stars.rs` → `sun` | `F1` | `warp`, `wave`, `cells`, `heat`, final `mix` | “Several inexpensive waves produce a complex pattern. This star ignores the external light intensity.” |
| `stars.rs` → `noise_star` | `F2` | sampling position plus `drift`, clamped heat, color mix | “Layered noise gives an alternative star surface. Animation moves through the field.” |

For the ice rim, do not describe the added color as a physically simulated atmosphere. For the star, explain that an unlit/emissive-looking surface does not automatically illuminate other objects; our renderer has not implemented that light transport.

**Board:** A recipe students can reuse tomorrow:

```text
coordinates → pattern/noise → mask → palette → optional lighting → color
                                  ↑
                         time or thresholds
```

**Question:** “Which input would you change for bigger continents? More ocean? A faster-moving gas band?”

## 11. Passes: when the next input is an image (93–103 minutes)

**Run:** Preset `F1`, then press `B` to toggle bloom. Turn it off after this section so the GPU comparison uses matching workloads.

**Say:**

> “Until now, we combined calculations for one fragment. Now we'll complete an image and use that image as input to another stage.”

**Board:** Place the four exported sun images in order:

```text
scene ─────────────────────────────────────────────────┐
  └→ bright extraction → horizontal blur → vertical blur ┴→ add → final
```

Use `sun-scene.png`, `sun-bright.png`, `sun-blur.png`, and `sun-bloom.png` from `outputs/shader-workshop`. The blur export shows the result after both blur directions.

**Open:** `src/shader_examples/passes.rs`, `bloom` first.

Explain these exact statements:

1. `bright_regions(scene)` produces a new image containing stronger contributions from bright pixels.
2. `blur(..., true)` reads horizontal neighbors from that image into another image.
3. `blur(..., false)` reads vertical neighbors from the horizontal result.
4. `scene.iter().zip(glow)` combines the original image with the blurred light.

Then open `bright_regions`: the weighted RGB sum estimates brightness; 0.45 is our chosen threshold. This simple LDR demonstration is not a full HDR/tone-mapped bloom pipeline.

Then open `blur`: show the loop from −8 to 8 and the division by 17. It is a separable box blur. Explain why we read from one image and write another: in-place updates would make later pixels depend on already-changed neighbors.

**Say:**

> “On a GPU these stages can render to intermediate textures. In this CPU exercise, the intermediate images are vectors of colors. The dependency is the same: finish one result, then use it as input.”

**Distinction to repeat:** N64 combiner cycles and multiple helper calls inside `fragment_shader` are not the same as these full-image passes.

**Question:** “Can a fragment function that knows only its own position and color blur the whole scene?” Expected answer: it needs access to neighboring image samples, which our current fragment interface does not provide.

## 12. Real GPU shaders, using the same sun (103–115 minutes)

### Set up a controlled comparison

Close the CPU window and run:

```sh
./run.sh --shader 10
```

The fresh launch has bloom and rotation off and the default camera. Note the title's FPS and milliseconds/frame. Close it, then run:

```sh
./run.sh --gpu
```

Both use the same mesh, initial camera, resolution, and sun equations. Read the terminal's renderer string. Their animation phases start independently, so moving patterns need not line up frame-for-frame in two live windows.

**Say:**

> “This comparison is CPU software rendering versus a GPU rendering pipeline. It is not a Rust-versus-C++ benchmark. C++ is submitting the work; GLSL is the language of the shader we send to OpenGL.”

The CPU includes software transformation/rasterization and a framebuffer upload. Both include presentation overhead. VSync is disabled in the C++ app, but a compositor can still constrain visible frame delivery. Do not promise a particular speedup or call the window-title number a GPU-only execution timer. Compare one running window at a time if you want to avoid simultaneous competition for resources.

### File 1: the code they already understand

**Open side by side:** `src/shader_examples/stars.rs` → `sun`, and `gpu_demo/sun.frag` → `main`.

Point to the matching `warp`, `wave`, `cells`, `heat`, and `mix` expressions.

**Say:**

> “Look how little the mathematics changed. Rust's `Vector3` becomes GLSL's `vec3`. The shader receives an interpolated position and a shared time value. It calculates one fragment's output. GPU hardware schedules many invocations of this work.”

Explain `vec4(color, 1.0)`: RGB plus opaque alpha. GLSL `mix` is built in; our Rust version was a helper. `#version 330 core` selects the GLSL language version.

### File 2: how source code reaches OpenGL

**Open:** `gpu_demo/main.cpp`, `compile_shader`.

Read only this sequence:

```cpp
GLuint shader = glCreateShader(type);
glShaderSource(shader, 1, &text, nullptr);
glCompileShader(shader);
```

**Say:**

> “The application reads the file as text, gives that text to OpenGL, and requests compilation. The driver handles the implementation-specific compilation work. A compile failure is why we retrieve and print the shader log.”

Then open `create_program` and point to `glAttachShader` and `glLinkProgram`.

> “We attach the vertex and fragment stages and link their interfaces into a program. That is separate from compiling the C++ executable.”

This is the conventional source-based OpenGL path used by this demo; do not imply every graphics API or console ships raw GLSL source. [Khronos: shader compilation and linking](https://wikis.khronos.org/opengl/Shader_Compilation).

**Board:**

```text
sun.frag text → OpenGL driver compilation ┐
sphere.vert text → compilation ──────────┴→ linked program
mesh data → GPU buffer                     ↓
time/matrices → uniforms → select program → draw
```

### File 3: geometry still has a stage

**Open:** `gpu_demo/sphere.vert`, `main`.

Point to:

```glsl
object_position = position;
gl_Position = projection * view * model * vec4(position, 1.0);
```

**Say:**

> “The vertex stage positions the geometry. It also passes the original coordinates onward. Rasterization interpolates those coordinates for the fragment shader. We did not write a loop over screen pixels in GLSL.”

Keep vertex processing to two minutes; this class focuses on fragments.

### File 4: binding data and drawing

**Return to:** `gpu_demo/main.cpp`, `run`.

Use editor Find for these calls, in order:

| Find | What to say |
| --- | --- |
| `glBufferData` | “Upload the sphere positions once.” |
| `glVertexAttribPointer` | “Describe how the input buffer is laid out.” |
| `glUseProgram` | “Select the program for this draw.” |
| `glUniformMatrix4fv` | “Supply the transformations.” |
| `glUniform1f` | “Update the time parameter.” |
| `glDrawArrays` | “Submit triangles; the GPU runs the selected stages.” |

Skip `load_sphere`, the screenshot writer, detailed window creation, and input handling unless asked. GLFW supplies the window/context, GLEW supplies function access, and GLM supplies matrix helpers. They are infrastructure, not today's material lesson.

### One quick GLSL edit

**Open:** `gpu_demo/sun.frag`. In the final `mix`, change:

```glsl
vec3(1.0, 0.9, 0.25)
```

to:

```glsl
vec3(0.2, 0.8, 1.0)
```

Save, focus the GPU window, and press **F5**. No C++ rebuild is needed for this text-file change. Restore the original color and press F5 again before any visual/performance comparison.

If compilation fails, read the terminal. The demo retains the previous working program. That can make an unsuccessful edit appear to have done nothing; check the log rather than repeatedly pressing reload.

## 13. Node editors and tomorrow's planets (115–120 minutes)

**Screen:** Small Unity Shader Graph and Unreal Material Editor screenshots from the image list.

**Board:** Draw your own tiny graph, even if the screenshots are visually busy:

```text
Position.Y → Multiply(22) → Add(Time) → Sine → Multiply(0.5) → Add(0.5) → Lerp(Color A, Color B)
```

Time gets its own incoming arrow at Add. Both colors get arrows into Lerp. The remapped wave is Lerp's blend factor.

**Say:**

> “These boxes are operations you already wrote. The editor builds shader code from the graph. A material graph may output properties such as base color, normal, roughness, or emission, and the engine adds the surrounding lighting and rendering code.”

A graph node is not automatically a render pass. A material graph is not the same as Unreal's gameplay Blueprint Event Graph. This prevents two common confusions without requiring either engine to be installed.

### Explain tomorrow's direction, not a new engine requirement

> “Tomorrow you will use this one supplied sphere and CPU shaders to create several different planets or stars. Geometry stays the same. Change the patterns, palettes, lighting choices, and animation. This is the beginning of your fantasy solar system; you do not need to build orbits or a whole system yet.”

Suggested practice target, to confirm when assigning the lab: one striped/banded world, one noise-based world, and one self-lit animated star. Require students to explain at least one meaningful parameter for each. The N64 texture is a historical teaching aid; it does not mean the planet assignment needs external texture images.

Keep the focus on the sphere and material function. Bloom is optional and outside the single-fragment shader; do not quietly turn it into a requirement. No OpenGL work is required for tomorrow's CPU exercise.

**Three exit questions:**

1. “What is shared across the fragments, and what varies?” — Uniform time/light versus interpolated position/normal/UV.
2. “How can a stationary mesh look animated?” — The appearance rule depends on time.
3. “What changed when we moved the sun to GLSL?” — Who executes the rule and the surrounding pipeline; the core math is recognizably the same.

## 14. Optional extensions if time remains

### Evaluate lighting explicitly inside the shader

This helps explain the difference between reading a precalculated intensity and owning the lighting equation. No new imports.

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let light = (uniforms.light_position - fragment.world_position).normalized();
    let diffuse = fragment.normal.dot(light).max(0.0);
    Vector3::new(0.7, 0.8, 1.0) * (0.12 + diffuse * 0.88)
}
```

**Say:** “Our renderer normally gives us this value for convenience. Here we calculate it ourselves from the normal and light direction. This is the kind of choice programmable fragments give us.”

### Add a small specular highlight

Replace `live_shader` with:

```rust
pub fn live_shader(fragment: &Fragment, uniforms: &Uniforms) -> Vector3 {
    let light = (uniforms.light_position - fragment.world_position).normalized();
    let view = (uniforms.camera_position - fragment.world_position).normalized();
    let halfway = (light + view).normalized();
    let diffuse = fragment.normal.dot(light).max(0.0);
    let specular = if diffuse > 0.0 {
        fragment.normal.dot(halfway).max(0.0).powf(64.0)
    } else {
        0.0
    };
    Vector3::new(0.2, 0.35, 0.6) * (0.12 + diffuse * 0.88)
        + Vector3::new(1.0, 1.0, 1.0) * specular
}
```

Use WASD to move the viewpoint. Increase 64 to 128 for a narrower highlight. Explain why view direction is needed. This is a simple Blinn–Phong-style demonstration, not a physically complete material.

Do not claim this directly compares per-vertex specular against per-fragment specular: the supplied Gouraud preset contains diffuse lighting only. To discuss missed highlights at vertices, draw a highlight inside a triangle that none of its corners sample.

### Normal-color visualization

Replace the last expression in a simple shader with:

```rust
fragment.normal * 0.5 + Vector3::new(0.5, 0.5, 0.5)
```

Map signed XYZ to RGB. Toggle `N` to reveal the distinction between face-like exported normals and generated smooth normals. The mesh positions remain unchanged.

## 15. Recovery card: when something goes wrong

| Symptom | Fast action |
| --- | --- |
| My code change is invisible | Close Rust app, save, rerun `./run.sh`, select `0`; check that you edited `live_shader`, not an inactive preset |
| Rust says `mix`, `sample`, or `fbm` is missing | Add the exact helper imports from sections 9 and 10 once at the top of `src/shaders.rs` |
| Rust says `uniforms` is unknown | Rename `_uniforms` to `uniforms` in that function's parameter list |
| “Name defined multiple times” | Replace the existing function rather than pasting another; remove duplicate imports |
| Motion stopped | Space may be paused; a fresh launch restores running time |
| Sphere unexpectedly turns | Press `R`; shader animation and model rotation are separate |
| Sphere is faceted in every lighting mode | Toggle `N` or restart; exported Blender normals are faceted |
| Sphere disappeared or camera feels wrong | Home resets camera and object angle; restart to reset all toggles |
| Material looks much too dark | Check whether lighting was multiplied twice; temporarily return the material color alone |
| Palette looks clipped | Our display clamps channels to 0–1; temporarily return the mask as grayscale |
| Bloom is slow | Turn off `B`; use the exported pass images |
| GPU edit did nothing | Save `sun.frag`, focus GPU window, press F5, and read terminal compilation errors |
| Both FPS counters are near a display limit | Explain presentation/compositor limits; compare the math and execution model rather than inventing a speedup |
| Rust sphere is off-center after changing dimensions | Change only `WINDOW_WIDTH` and `WINDOW_HEIGHT`; window, viewport, framebuffer, and texture must agree |
| Need a known-good CPU demo immediately | `./run.sh --shader 8` for gas giant, or `./run.sh --shader 10` for the sun |

## 16. Technical reminders for the instructor

- The supplied sphere is the user's Blender subdivided cube, not a perfect mathematical sphere. Geometry is untouched by these demos.
- Flat/Gouraud/per-fragment intensity values are precomputed to keep the live class inside shader files. For questions afterward: `pipeline::vertex_shader` computes vertex lighting; `triangle::triangle` computes face lighting and interpolated/per-fragment data.
- `object_position` really is the original position. `world_position` includes the model transform. UVs and surface coordinates use perspective-correct interpolation; the historical Gouraud intensity intentionally uses screen-space interpolation.
- `fragment.color` is currently an unlit white base value. It is not a hidden texture or an already-lit material.
- The current normal transform assumes rotation and uniform scale. The demonstration camera is constrained so the supplied sphere stays away from the near clipping plane. This is a teaching renderer, not a general clipped scene renderer.
- The GPU example uses no OpenSimplex. Its sun matches the sine-wave CPU preset exactly in arithmetic; the noise star is a separate example.
- The bloom code stores distinct intermediate arrays. It is a real sequence of image-processing passes even though it runs on the CPU.
- Native game screenshots are visual prompts. A screenshot alone does not establish a proprietary game's exact shader, number of passes, or lighting algorithm.
- The class story is about increasing control over appearance. Different architectures overlap in time; it is not a single straight ladder from “bad graphics” to “good graphics.”
