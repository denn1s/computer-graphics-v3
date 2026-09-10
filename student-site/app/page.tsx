"use client";

import { useMemo, useState } from "react";

type TerrainMode = "flat" | "random" | "coherent";

const rustCode = `let nx = x as f64 * scale;
let nz = z as f64 * scale;

let h = noise_fn.get([nx, nz]);

let y = (((h * 0.5 + 0.5)
    * max_height as f64)
    .floor() as i32)
    .clamp(0, max_height);`;

function hash(seed: number, x: number) {
  let n = (seed ^ Math.imul(x + 1, 0x45d9f3b)) | 0;
  n = Math.imul(n ^ (n >>> 16), 0x45d9f3b);
  n = Math.imul(n ^ (n >>> 16), 0x45d9f3b);
  return ((n ^ (n >>> 16)) >>> 0) / 4294967295;
}
function smoothstep(t: number) { return t * t * (3 - 2 * t); }
function valueNoise(seed: number, x: number) {
  const left = Math.floor(x);
  const t = smoothstep(x - left);
  return hash(seed, left) * (1 - t) + hash(seed, left + 1) * t;
}
function fbm(seed: number, x: number) {
  let value = 0, amplitude = 1, frequency = 1, total = 0;
  for (let octave = 0; octave < 4; octave += 1) {
    value += valueNoise(seed + octave * 101, x * frequency) * amplitude;
    total += amplitude; amplitude *= 0.5; frequency *= 2;
  }
  return value / total;
}

export default function Home() {
  const [mode, setMode] = useState<TerrainMode>("flat");
  const [seed, setSeed] = useState(42);
  const [scale, setScale] = useState(32);
  const heights = useMemo(() => Array.from({ length: 36 }, (_, x) => {
    if (mode === "flat") return 4;
    if (mode === "random") return 1 + Math.floor(hash(seed, x) * 10);
    return 1 + Math.floor(fbm(seed, x / (scale / 4)) * 10);
  }), [mode, seed, scale]);

  return <main>
    <nav className="topbar" aria-label="Lesson sections">
      <a className="brand" href="#top"><span className="brand-cube" />WORLD//FUNCTION</a>
      <div className="nav-links"><a href="#random">Random</a><a href="#terrain">Terrain lab</a><a href="#noise">Noise</a><a href="#minecraft">Minecraft</a></div>
    </nav>

    <section className="hero" id="top">
      <div className="hero-copy"><p className="kicker">Computer graphics field notes</p><h1>How do numbers<br />become a world?</h1><p className="hero-lead">A journey from a suspiciously predictable random number to terrain, caves, biomes, and the signals that shape Minecraft.</p><a className="start-link" href="#random">Start with a surprise <span aria-hidden="true">↓</span></a></div>
      <div className="hero-terrain" aria-hidden="true">{[3,4,4,5,6,8,9,8,7,6,6,5,7,9,11,10,8,7,5,4].map((h,i)=><div className="hero-column" key={i} style={{height:`${h*22}px`}}><span className={h>9?"snow":h<5?"sand":h>7?"stone":"grass"}/></div>)}</div>
      <div className="coordinate-note" aria-hidden="true">f(seed, x, z) → height</div>
    </section>

    <section className="chapter dark" id="random"><div className="chapter-number">01</div><div className="chapter-copy">
      <h2>The random number that remembers</h2><p>Run a program containing <code>std::rand()</code> without choosing a seed. Close it. Run it again. The first number repeats.</p>
      <div className="terminal" aria-label="C++ random number example"><div className="terminal-bar"><span/><span/><span/><b>random_demo.cpp</b></div><pre><code>{`#include <cstdlib>
#include <print>

int main() {
    std::println("{}", std::rand());
}`}</code></pre><div className="terminal-output"><span>$ ./random_demo</span><strong>1804289383</strong><span>$ ./random_demo</span><strong>1804289383</strong></div></div>
      <blockquote>“Random” is often a deterministic machine whose output only looks irregular.</blockquote>
    </div></section>

    <section className="seed-section"><div><p className="kicker">A useful mental model</p><h2>The seed chooses a journey</h2><p>A seed initializes the generator’s state. The same seed recreates the same sequence—useful for sharing worlds, reproducing bugs, and writing tests.</p></div><div className="sequence" aria-label="Pseudorandom sequence diagram"><span className="seed-tag">seed 42</span><i/>{[41,7,93,12,65].map((n,i)=><span className={i===0?"active":""} key={n}>{n}</span>)}</div><div className="warning-strip"><strong>What about <code>time()</code>?</strong><p>Convenient for variety, poor for secrets. Two programs started in the same second may choose the same seed, and an approximate start time leaves few possibilities to test.</p></div></section>

    <section className="terrain-lab" id="terrain"><div className="lab-heading"><div><p className="kicker">Try the failed ideas</p><h2>The terrain laboratory</h2></div><p>Every failed generator reveals a requirement. Switch algorithms and inspect what changes.</p></div><div className="lab-shell">
      <div className="lab-controls"><fieldset><legend>Generator</legend>{(["flat","random","coherent"] as TerrainMode[]).map(item=><button key={item} className={mode===item?"selected":""} onClick={()=>setMode(item)} aria-pressed={mode===item}><span>{item==="flat"?"Flat plane":item==="random"?"Random heights":"Coherent noise"}</span><small>{item==="flat"?"y = constant":item==="random"?"random y per x":"nearby x, related y"}</small></button>)}</fieldset><label>Seed <output>{seed}</output><input type="range" min="1" max="99" value={seed} onChange={e=>setSeed(Number(e.target.value))}/></label><label className={mode!=="coherent"?"disabled":""}>Feature size <output>{scale}</output><input disabled={mode!=="coherent"} type="range" min="12" max="60" value={scale} onChange={e=>setScale(Number(e.target.value))}/></label><button className="new-seed" onClick={()=>setSeed((seed*17+23)%99||1)}>Choose another seed</button></div>
      <div className="terrain-stage"><div className="stage-grid"/><div className="terrain-columns" aria-label={`${mode} terrain visualization`}>{heights.map((h,i)=><div key={i} className="terrain-column" style={{height:`${h*24}px`}}><span className={h>9?"snow":h>7?"stone":h<3?"sand":"grass"}/></div>)}</div><p className="stage-caption">{mode==="flat"?"Procedural? Yes. Interesting? Not yet.":mode==="random"?"Variation without relationships becomes static.":"Nearby inputs produce related heights: terrain emerges."}</p></div>
    </div></section>

    <section className="chapter noise-section" id="noise"><div className="chapter-number">02</div><div className="noise-intro"><p className="kicker">The missing relationship</p><h2>Noise is a field of possibilities</h2><p>White noise makes independent decisions. Coherent noise makes nearby coordinates produce related values. The image is not terrain—it is a field we can interpret.</p></div><div className="noise-comparison"><div><div className="noise-map white-noise"/><h3>White noise</h3><p>Unrelated neighbors. Useful for independent events; terrible for hills.</p></div><div><div className="noise-map smooth-noise"/><h3>Coherent noise</h3><p>Related neighbors. Bright regions can become peaks; dark regions can become valleys.</p></div></div></section>

    <section className="octaves"><div className="octave-copy"><p className="kicker">Fractal Brownian motion</p><h2>Build detail in layers</h2><p>FBM combines several octaves. Each layer gets finer, and usually weaker. Your terrain uses FBM with OpenSimplex as its source.</p></div><div className="octave-equation"><div><span className="wave broad"/><b>Broad forms</b><small>low frequency · strong</small></div><strong>+</strong><div><span className="wave medium"/><b>Hills</b><small>medium frequency</small></div><strong>+</strong><div><span className="wave fine"/><b>Details</b><small>high frequency · weak</small></div></div></section>

    <section className="code-story"><div className="code-explain"><p className="kicker">Inside procedural.rs</p><h2>One small pipeline</h2><ol><li><b>Scale</b><span>Choose how quickly the field changes.</span></li><li><b>Sample</b><span>Ask the noise function about this coordinate.</span></li><li><b>Remap</b><span>Move values from about −1…1 into 0…1.</span></li><li><b>Quantize</b><span><code>floor()</code> turns a smooth signal into blocks.</span></li><li><b>Classify</b><span>Height bands become sand, grass, stone, or snow.</span></li></ol></div><pre className="rust-code"><code>{rustCode}</code></pre></section>

    <section className="dimensions"><h2>Two maps, two kinds of world</h2><div className="dimension-grid"><article><div className="dimension-mark">2D</div><code>f(seed, x, z) → height</code><h3>Heightfield</h3><p>Simple and efficient. One surface level per horizontal position—but no caves, arches, or overhangs.</p></article><article><div className="dimension-mark">3D</div><code>f(seed, x, y, z) → density</code><h3>Density field</h3><p>Every position can be solid or empty. Coherent 3D samples can form caves, floating islands, and tunnels.</p></article></div><p className="callback">The random voxel cloud had the right domain for caves—and the wrong relationship between neighbors.</p></section>

    <section className="minecraft" id="minecraft"><div className="minecraft-heading"><p className="kicker">Beyond our tiny generator</p><h2>Minecraft gives different fields different jobs</h2><p>This is a high-level design model, not Minecraft source code. Modern world generation combines signals, curves, rules, and several passes.</p></div><div className="field-river">{[["Continentalness","ocean → coast → inland"],["Erosion","dramatic → comparatively flat"],["Peaks & valleys","valleys → slopes → peaks"],["Temperature","cold → temperate → hot"],["Humidity","dry → lush"]].map(([name,desc],i)=><div className="field" key={name}><span style={{"--offset":`${i*17}%`} as React.CSSProperties}/><b>{name}</b><small>{desc}</small></div>)}<div className="world-result"><span>terrain + biome</span><b>Snowy mountain</b><small>inland · rugged · peak · cold</small></div></div><div className="passes"><span>Base terrain</span><i/><span>Biomes</span><i/><span>Surface blocks</span><i/><span>Caves</span><i/><span>Structures & features</span></div></section>

    <section className="takeaway"><p className="kicker">Keep this idea</p><h2>Noise is not terrain.</h2><p>Noise is a reproducible spatial signal. Terrain is one interpretation. A shader can interpret the same kind of signal as color, displacement, normal direction, roughness, or opacity.</p><div className="formula">seed <span>+</span> coordinates <span>+</span> rules <b>= reproducible world</b></div></section>

    <section className="resources"><h2>Continue exploring</h2><div className="resource-links"><a href="https://learn.microsoft.com/en-us/minecraft/creator/documents/world-generation?view=minecraft-bedrock-stable" target="_blank" rel="noreferrer"><b>World Generation Overview</b><span>Microsoft’s official, high-level generation passes.</span></a><a href="https://www.minecraft.net/en-us/article/caves---cliffs-update--part-ii-dev-q-a" target="_blank" rel="noreferrer"><b>Caves & Cliffs developer Q&A</b><span>Multinoise, density, caves, aquifers, and development tools.</span></a><a href="https://dawnosaur.substack.com/p/how-minecraft-generates-worlds-you" target="_blank" rel="noreferrer"><b>How Minecraft generates worlds</b><span>An illustrated explanation of layers, splines, and biomes.</span></a><a href="https://github.com/Auburn/FastNoiseLite" target="_blank" rel="noreferrer"><b>FastNoiseLite</b><span>Explore noise types and their visual patterns.</span></a></div></section>
    <footer><span>Computer Graphics · Procedural Terrain</span><a href="#top">Return to the surface ↑</a></footer>
  </main>;
}
