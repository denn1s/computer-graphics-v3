# From Random Numbers to Minecraft Terrain

## Lesson overview

**Duration:** 2 hours, including a 10-minute break  
**Audience:** Computer graphics students who have built a ray tracer with spheres and are now building a Minecraft-style diorama with cubes  
**Format:** Instructor-led discussion and live demonstrations; no graded exercise or homework  
**Main code shown:** `src/procedural.rs`  

### Central question

> Starting only with cubes and loops, how can we generate a world that looks intentional rather than modeled by hand?

### Learning goals

By the end of the lesson, students should be able to:

- Explain the difference between true randomness and pseudorandomness.
- Explain what a seed provides: determinism and reproducibility, not quality.
- Distinguish independent random samples (white noise) from coherent noise.
- Interpret a two-dimensional noise field as a terrain heightfield.
- Explain how frequency, amplitude, and octaves affect generated terrain.
- Describe the pipeline from coordinates to noise, height, material, and cube.
- Explain why a 2D heightfield cannot produce caves or overhangs, while a 3D density field can.
- Describe, at a high level, how Minecraft combines several procedural signals and generation passes.
- Connect procedural noise in terrain generation to future shader applications.

### Main narrative

```text
Repeated "random" number
    -> deterministic sequence
    -> seed and reproducibility
    -> random terrain that looks like static
    -> spatial coherence
    -> OpenSimplex and FBM
    -> terrain rules
    -> Minecraft's multi-noise world generation
```

## Preparation before class

- Build and run the finished project with `cargo run --release`.
- Keep `TERRAIN_SIZE` small enough that all live demonstrations render promptly.
- Prepare the alternative Rust generators below in advance. Reveal or swap them one at a time rather than typing every function from scratch.
- Use a fixed seed while comparing algorithm parameters. Change only one variable per demonstration.
- Prepare a C++23 compiler for the `std::println` demonstrations. With a recent compiler, a typical command is:

```bash
g++ -std=c++23 random_demo.cpp -o random_demo
```

  `std::println` is declared in `<print>` and requires C++23 library support. Test the classroom machine beforehand; some otherwise recent compilers may not yet ship `<print>`.

- Open the FastNoiseLite playground or have screenshots ready showing white noise, coherent noise, and different frequencies.
- Optionally open selected illustrations from [How Minecraft Generates Worlds You Want to Explore](https://dawnosaur.substack.com/p/how-minecraft-generates-worlds-you).

## Timeline at a glance

| Time | Segment | Main idea |
|---:|---|---|
| 0:00-0:08 | Final-world reveal | The ray tracer only sees cubes; generation constructs the scene |
| 0:08-0:25 | The surprising random number | PRNGs are deterministic state machines |
| 0:25-0:35 | Seeds and `time()` | Seeds provide reproducibility; time is convenient but predictable |
| 0:35-0:58 | Generators that do not work | Flat, random-height, and random-volume worlds reveal the need for coherence |
| 0:58-1:08 | Break | — |
| 1:08-1:28 | From number sequences to noise fields | Coordinates, coherent noise, scale, and visualization |
| 1:28-1:45 | Read `procedural.rs` | Coordinates become heights, materials, and cubes |
| 1:45-1:55 | Minecraft's larger system | Multiple fields, selectors, density, biomes, and passes |
| 1:55-2:00 | Shader bridge and closing | Noise is a general-purpose signal, not a terrain algorithm |

---

## 0:00-0:08 — Reveal the finished world

Run the finished terrain before showing its implementation. Orbit the camera and regenerate several worlds.

Ask:

- “How many cubes did I position manually?”
- “When I regenerate, what changes?”
- “What remains unchanged?”
- “Does the ray tracer need to know that these cubes represent terrain?”

Land on this distinction:

> The ray tracer does not know that this is terrain. It receives a collection of cubes and traces rays against them. Procedural generation changes how the scene is constructed, not how the scene is rendered.

Show only the conceptual call from `main.rs`:

```rust
let objects = generate_terrain(width, depth, max_height, scale, seed, &palette);
```

Do not reveal the cube intersection implementation. For this lesson, a cube can remain an abstraction that accepts a position, size, and material.

---

## 0:08-0:25 — The surprising random number

### Demonstration 1: An unseeded C random generator

Write and run this small C++23 program. The printing API is modern C++, but `std::rand` deliberately demonstrates the older C-style random generator and its implicit default seed.

```cpp
#include <cstdlib>
#include <print>

int main() {
    std::println("{}", std::rand());
}
```

Before running it a second time, ask:

> “What number will appear when I run the program again?”

The first value should repeat across separate executions because the program never calls `std::srand`; the implementation begins from the same default state.

Possible follow-up:

```cpp
#include <cstdlib>
#include <print>

int main() {
    std::println("{}", std::rand());
    std::println("{}", std::rand());
    std::println("{}", std::rand());
}
```

The values differ within one run, but the whole sequence repeats on the next run.

### Board model: a deterministic state machine

Draw:

```text
seed -> initial state -> value -> next state -> value -> next state -> ...
```

Then use the familiar line analogy:

```text
... 41, 7, 93, 12, 65, 28, ...
          ^
        state
```

Explain the limitation of the drawing:

> This is a model of the behavior, not usually the implementation. The values are normally computed rather than stored in a large table. The seed initializes the generator's state; it is not literally a pointer into a precomputed list.

Key statement:

> A pseudorandom generator is a deterministic program that produces a sequence designed to look random for a particular purpose.

### Demonstration 2: Manually choose a seed

```cpp
#include <cstdlib>
#include <print>

int main() {
    unsigned int seed = 42;
    std::srand(seed);

    std::println("seed: {}", seed);
    std::println("{}", std::rand());
    std::println("{}", std::rand());
    std::println("{}", std::rand());
}
```

Change `42` to another number and run it. Return to `42` and recover the original sequence.

Ask:

- “Did the seed make the numbers more random?”
- “What did the seed actually give us?”

Answer: it gave us a repeatable choice of sequence.

---

## 0:25-0:35 — How should we choose a seed?

Invite suggestions. A student will often suggest the current time.

### Demonstration 3: Seed with time

```cpp
#include <cstdlib>
#include <ctime>
#include <print>

int main() {
    auto seed = static_cast<unsigned int>(std::time(nullptr));
    std::srand(seed);

    std::println("seed: {}", seed);
    std::println("random value: {}", std::rand());
}
```

Discuss why this is useful:

- Normal runs begin with different sequences.
- It is simple and often sufficient for a toy or visual effect.

Then discuss where it can fail:

- `std::time(nullptr)` normally has one-second resolution.
- Programs started in the same second can receive the same seed.
- A person who knows the approximate start time has only a small range of seeds to try.
- A failure is difficult to reproduce if the program does not print or save its seed.
- This is unsuitable for passwords, tokens, or cryptographic secrets.

Key statement:

> A time seed changes the deterministic sequence; it does not create true randomness or cryptographic security.

Connect this directly to games:

- Generate a fresh seed for a new world.
- Display and store that seed.
- Reuse it to share the world or reproduce a bug.

Compact model:

```text
seed + coordinates + rules = reproducible world
```

---

## 0:35-0:58 — Terrain generators that do not work

Use the same rhythm for every generator:

1. Ask students for the next idea.
2. Ask them to predict the image.
3. Run it.
4. Ask what worked.
5. Ask which property is still missing.

All snippets below use the project's existing `TerrainPalette`, `Cube`, and `Material` types.

### Generator 1: A flat plane

```rust
pub fn generate_flat_terrain(
    width: i32,
    depth: i32,
    material: Material,
) -> Vec<Cube> {
    let mut cubes = Vec::with_capacity((width * depth) as usize);

    for x in 0..width {
        for z in 0..depth {
            cubes.push(Cube::new(
                Vector3::new(
                    x as f32 - width as f32 * 0.5,
                    0.0,
                    z as f32 - depth as f32 * 0.5,
                ),
                1.0,
                material,
            ));
        }
    }

    cubes
}
```

Ask:

> “Is this procedural generation?”

Yes. It is generated by rules rather than modeled manually. Procedural does not necessarily mean random, complex, or realistic.

The function establishes the fundamental heightfield question:

```text
For every horizontal coordinate (x, z), choose a height y.
```

### Generator 2: An independently random height at every column

Add these imports:

```rust
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
```

Then use:

```rust
pub fn generate_random_height_terrain(
    width: i32,
    depth: i32,
    max_height: i32,
    seed: u64,
    material: Material,
) -> Vec<Cube> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut cubes = Vec::with_capacity((width * depth) as usize);

    for x in 0..width {
        for z in 0..depth {
            let y = rng.gen_range(0..=max_height);

            cubes.push(Cube::new(
                Vector3::new(
                    x as f32 - width as f32 * 0.5,
                    y as f32,
                    z as f32 - depth as f32 * 0.5,
                ),
                1.0,
                material,
            ));
        }
    }

    cubes
}
```

Ask students to predict the result before running it. It should resemble spikes or television static rather than geography.

Important vocabulary:

- These independent samples are still noise: approximately **white noise**.
- The visual problem is the absence of **spatial coherence**.
- Nearby positions have no reason to produce nearby heights.

Ask:

> “What relationship should two neighboring terrain columns normally have?”

Desired conclusion:

> Nearby terrain positions should usually have related values.

Changing the seed reproduces a different bad terrain. This proves that determinism and visual quality are separate properties.

### Generator 3: Random occupancy inside a 3D volume

This creates a visually interesting voxel cloud. Keep the dimensions small because it can generate many cubes and the ray tracer tests cubes by brute force.

```rust
pub fn generate_random_voxel_cloud(
    width: i32,
    height: i32,
    depth: i32,
    fill_probability: f64,
    seed: u64,
    material: Material,
) -> Vec<Cube> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut cubes = Vec::new();
    let probability = fill_probability.clamp(0.0, 1.0);

    for x in 0..width {
        for y in 0..height {
            for z in 0..depth {
                if rng.gen_bool(probability) {
                    cubes.push(Cube::new(
                        Vector3::new(
                            x as f32 - width as f32 * 0.5,
                            y as f32,
                            z as f32 - depth as f32 * 0.5,
                        ),
                        1.0,
                        material,
                    ));
                }
            }
        }
    }

    cubes
}
```

A starting call might use a low occupancy:

```rust
let objects = generate_random_voxel_cloud(12, 7, 12, 0.18, 42, palette.stone);
```

Ask:

- “Is it visually interesting?”
- “Is it terrain?”
- “Could a player reliably walk across it?”
- “What can this representation express that a heightfield cannot?”

Introduce the representational distinction:

```text
Heightfield:      f(seed, x, z)    -> height
Voxel density:   f(seed, x, y, z) -> solid or empty
```

The three-dimensional representation is not the mistake. It can express caves, arches, overhangs, and floating islands. Independent randomness is the problem because it lacks spatial structure.

Foreshadow the later callback:

> The random cube cloud is the right kind of domain for caves, but the wrong kind of randomness.

### Optional failed repair: smooth after generating

If time permits, draw rather than implement a scheme that averages neighboring random heights or prevents each column from differing too much from the previous column.

Ask what might go wrong:

- It may depend on iteration order.
- Smoothing along one axis can create directional artifacts.
- Changing an early value can affect many later values.
- Independently generating adjacent chunks becomes difficult.

State the new requirement:

> We need to evaluate any coordinate independently while still obtaining related values at nearby coordinates.

That is the entrance to coherent noise.

---

## 0:58-1:08 — Break

Leave the random voxel cloud or random-height terrain visible during the break.

---

## 1:08-1:28 — From a sequence to a spatial field

### Add dimensions to the mental model

Start with the PRNG sequence:

```text
state -> value -> next state -> value -> ...
```

Move to a coordinate-based function:

```text
f(seed, x)       -> value on a line
f(seed, x, z)    -> value on a plane
f(seed, x, y, z) -> value in a volume
```

Clarify an important distinction:

- A stateful PRNG produces the next value in a sequence; call order matters.
- A noise function is sampled at coordinates; the same seed and coordinates reproduce the same value.

This is why a coordinate-based function is useful for large worlds and chunks. A game can ask for a distant location without generating every preceding coordinate first.

### Show noise as an image

Use the FastNoiseLite playground or prepared images.

Show independent white noise first, then coherent noise. Ask:

> “Where is the terrain in this image?”

Answer: there is no terrain yet. There are only values. We choose their meaning.

For a grayscale image:

```text
black -> low value
gray  -> middle value
white -> high value
```

Interpret it as height:

```text
noise value -> terrain height
```

Then show that the same signal could instead mean:

```text
noise value -> temperature
noise value -> humidity
noise value -> material blend
noise value -> cloud opacity
noise value -> surface roughness
```

### Frequency, amplitude, and octaves

Introduce only the vocabulary students need:

- **Frequency:** how quickly values change across space; it controls feature size.
- **Amplitude:** how strongly a layer contributes.
- **Octave:** one layer of coherent noise at a particular frequency and amplitude.
- **FBM:** several octaves combined, commonly with increasing frequency and decreasing amplitude.

Board sketch:

```text
broad, strong hills
    + medium variation
    + small, weak detail
    = fractal Brownian motion (FBM)
```

Do not derive OpenSimplex internals. Treat it as a deterministic coherent function:

```text
OpenSimplex(seed, x, z) -> smoothly varying value
```

Useful distinction:

> OpenSimplex supplies coherent samples. FBM combines several scales of those samples.

---

## 1:28-1:45 — Reveal and read `procedural.rs`

The students now know why each stage exists. Read `generate_terrain` as a transformation pipeline, not as isolated Rust syntax.

### Stage 1: Select the field

```rust
let noise_fn: Fbm<OpenSimplex> = Fbm::new(seed);
```

The seed selects the reproducible world. FBM indicates that several detail scales are combined.

### Stage 2: Visit the horizontal domain

```rust
for x in 0..width {
    for z in 0..depth {
```

This generator is a heightfield: each `(x, z)` receives exactly one surface cube.

### Stage 3: Scale the input coordinates

```rust
let nx = x as f64 * scale;
let nz = z as f64 * scale;
```

Ask for predictions:

- Smaller `scale` samples nearby points in noise space and creates broader, smoother features.
- Larger `scale` moves farther between samples and creates smaller, rougher features.

Although the parameter is named `scale`, it behaves like an input frequency control. It changes horizontal feature size, not vertical height.

### Stage 4: Sample, remap, and quantize

```rust
let h = noise_fn.get([nx, nz]);
let y = (((h * 0.5 + 0.5) * max_height as f64).floor() as i32)
    .clamp(0, max_height);
```

Walk through the transformation:

```text
h                     approximately [-1, 1]
h * 0.5 + 0.5         approximately [ 0, 1]
... * max_height      approximately [ 0, max_height]
floor                  integer block level
clamp                  safe final bounds
```

Mention that coherent-noise implementations do not necessarily visit their theoretical extreme values frequently. Consequently, some seeds may have few cubes in the very lowest or highest bands.

Key statement:

> Noise creates continuous variation; `floor` is where the smooth mathematical signal becomes block terrain.

### Stage 5: Classify the value

```rust
let material = if y >= max_height - 1 {
    palette.snow
} else if y >= (max_height as f32 * 0.65) as i32 {
    palette.stone
} else if y >= (max_height as f32 * 0.35) as i32 {
    palette.grass
} else if y >= 1 {
    palette.dirt
} else {
    palette.sand
};
```

Key statement:

> Noise gives us numbers. Rules give those numbers meaning.

Ask what additional signals could improve material selection:

- Temperature
- Humidity
- Slope
- Distance from water
- Biome identity
- A separate patchiness field

With `max_height = 6`, percentage thresholds collapse into only a few integer levels. Use this as a small example of how quantization changes a continuous design.

### Stage 6: Turn the decision into geometry

```rust
cubes.push(Cube::new(
    Vector3::new(
        x as f32 - width as f32 * 0.5,
        y as f32,
        z as f32 - depth as f32 * 0.5,
    ),
    1.0,
    material,
));
```

The subtraction centers the generated grid around the world origin. The result is an ordinary `Vec<Cube>` consumed by the existing ray tracer.

Summarize the complete pipeline:

```text
(x, z)
  -> scaled coordinates
  -> FBM/OpenSimplex sample
  -> normalized value
  -> scaled height
  -> integer block level
  -> material band
  -> cube
```

### Controlled live changes

Keep one seed fixed and change only one item at a time:

1. Run the same seed twice to prove determinism.
2. Change only the seed.
3. Change `scale` from `0.08` to `0.03`.
4. Change `scale` from `0.08` to `0.20`.
5. Increase `max_height`.
6. Change one material threshold.

Ask students to predict every result before running it.

### Performance aside

The implementation creates only the surface cube at each `(x, z)`. Solid columns would look better from the side, but would multiply the object count by roughly the average column height.

Terrain area also grows quadratically:

```text
16 x 16 ->   256 surface cubes
32 x 32 -> 1,024 surface cubes
64 x 64 -> 4,096 surface cubes
```

In this project, primary and shadow rays test cubes by brute force. Use this only as a teaser for chunks, spatial partitioning, voxel traversal, or BVHs—not as a second full lesson.

---

## 1:45-1:55 — How Minecraft grows the idea

Frame this section carefully:

> Our program is Minecraft-inspired. It demonstrates a foundational idea, not Minecraft's complete generator.

### From one field to several jobs

At a high level, modern Minecraft-style generation evaluates multiple spatial parameters. Conceptual roles include:

- **Continentalness:** roughly distinguishes oceans, coasts, and inland areas.
- **Erosion:** influences whether terrain is dramatic or comparatively flat.
- **Peaks and valleys:** contributes to mountainous and valley-shaped regions.
- **Temperature and humidity:** help select biomes.
- **Weirdness:** contributes additional variation and distinctions.

Emphasize that these names describe design roles, not literal physical simulations.

Show a selector-style example:

```text
continentalness: inland
erosion:         rugged
peaks/valleys:   peak
temperature:     cold
                         -> snowy mountain region
```

The important advance is not merely adding every noise sample together. One field can change how another field is interpreted, or select a rule or biome.

### Multiple generation passes

Present a simplified pipeline:

```text
base terrain
    -> biome selection
    -> surface blocks
    -> caves and aquifers
    -> structures
    -> vegetation, ores, and other features
```

Minecraft's exact implementation differs by edition and version. Keep this as a design overview.

### Return to the voxel cloud

Recall:

```text
f(seed, x, y, z) -> density or solid/empty decision
```

If nearby three-dimensional samples vary coherently, the result can express caves, arches, overhangs, and floating formations.

Callback:

> The random voxel cloud used the right domain for caves, but the wrong relationship between neighboring values.

### Historical wording

Avoid claiming that Minecraft's first generator was exactly one noise layer identical to this project. A safe summary is:

> Some early Minecraft terrain used a much simpler, 2D height-oriented noise approach. Over time, world generation grew into combinations of 3D density, multiple noise fields, biome parameters, carving, and generation passes.

---

## 1:55-2:00 — Shader bridge and closing

Return to the grayscale noise image.

Today:

```text
sample once per terrain column -> decide geometry
```

Later, in shaders:

```text
sample once per vertex or fragment -> decide color, displacement,
                                      normal, roughness, or opacity
```

Close with:

> Noise is not a terrain generator. It is a reproducible spatial signal. Terrain generation is one interpretation of that signal.

Final discussion question:

> “If you could add one more noise field to this world, what would you make it control?”

Possible student answers include forests, rainfall, ore deposits, rivers, snow, cloud density, cave placement, or enemy spawning. No implementation or homework is required.

---

## Instructor reference: central distinctions

### Randomness versus determinism

| Concept | What it means |
|---|---|
| Pseudorandom sequence | Deterministic state transitions that produce apparently irregular values |
| Seed | Initializes or selects generator state/function |
| Same seed | Reproduces results when algorithm and inputs are unchanged |
| Time-based seed | Convenient variation, but predictable and possibly duplicated |
| Cryptographic randomness | A different requirement; ordinary game PRNGs are not appropriate for secrets |

### White noise versus coherent noise

| White noise | Coherent noise |
|---|---|
| Neighboring values are unrelated | Neighboring values are correlated |
| Good for independent decisions | Good for continuous spatial patterns |
| Terrain resembles static or spikes | Terrain forms hills and valleys |

### Heightfield versus density field

| Heightfield | Density field |
|---|---|
| `f(x, z) -> height` | `f(x, y, z) -> density` |
| One surface level per horizontal coordinate | Every position can be solid or empty |
| Simple and relatively cheap | More expressive and more expensive |
| Cannot represent caves or overhangs | Can represent caves, arches, and overhangs |

## References

- [Microsoft: World Generation Overview](https://learn.microsoft.com/en-us/minecraft/creator/documents/world-generation?view=minecraft-bedrock-stable) — high-level description of seeds, terrain, biome, structure, and feature passes.
- [Minecraft: Caves & Cliffs Part II Developer Q&A](https://www.minecraft.net/de-de/article/caves---cliffs-update--part-ii-dev-q-a) — developer discussion of the 1.18 rewrite, density values, multinoise, caves, aquifers, and visualization tools.
- [How Minecraft Generates Worlds You Want to Explore](https://dawnosaur.substack.com/p/how-minecraft-generates-worlds-you) — accessible illustrated overview of coherent noise, fractal noise, multi-noise, splines, 3D noise, and biome mapping. Treat it as a simplified secondary explanation.
- [Minecraft Wiki: Noise Generator](https://theminecraftwiki.com/wiki/noise-generator/) — historical detail about 2D and 3D noise use across versions; useful for instructor preparation rather than a full classroom walkthrough.
- [FastNoiseLite](https://github.com/Auburn/FastNoiseLite) — noise library and visual playground for comparing noise types and parameters.

## Final teaching note

The unsuccessful generators are not detours. Each one creates the need for the next concept:

```text
flat plane          -> needs variation
random heights      -> needs spatial coherence
random voxel cloud  -> needs coherent 3D structure
single noise scale  -> needs detail at several scales
height alone        -> needs classification and additional fields
```

Let students name the missing property before presenting its solution. That keeps the class exploratory even though it has no assigned exercise.
