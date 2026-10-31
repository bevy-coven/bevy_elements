# Performance and platforms

Measured GPU cost per pass, the quality levels, and where the crate runs.

GPU time per frame on an RTX 3090 (Vulkan), the minimum over Bevy's diagnostic history so it is the full-clock cost (`examples/bench_elements.rs`; a lightly loaded GPU idles at 210–450 MHz and reads 5–10× slower).

| Quality | Wind grid | Jacobi iterations | Wind | Ground-fog density | Total |
|---|---|---|---|---|---|
| Low | 32 × 8 × 32 | 10 | 0.087 ms | 0.037 ms | 0.124 ms |
| Medium | 48 × 12 × 48 | 10 | 0.119 ms | 0.037 ms | 0.156 ms |
| High | 64 × 16 × 64 | 12 | 0.233 ms | 0.037 ms | 0.270 ms |
| Ultra | 96 × 24 × 96 | 40 | 1.704 ms | 0.037 ms | 1.741 ms |

The budget for wind + fog at High is 0.3 ms. Ultra keeps 40 Jacobi iterations and the plan's grid; it is for large budgets. The fog column predates precipitation: with the shaft term (a second two-octave fBm and a weather-map sample per texel) the density kernel reads 0.058 ms at High (the kernel does not depend on the quality; measured at High only).

Precipitation at High (150 000 particles), 3840 × 2160, the worst case of 25 mm/h (every rain particle active) with 14 m/s wind, minimum over the diagnostic history with the GPU at 1800 MHz:

| Blockers | `elements_precip_sim` | `elements_precip_draw` | Fog kernel increase | Total |
|---|---|---|---|---|
| none | 0.028 ms | 0.126 ms | 0.021 ms | 0.175 ms |
| 256 (the most uploaded) | 0.227 ms | 0.126 ms | 0.021 ms | 0.374 ms |

The budget is 0.4 ms. The simulation tests every uploaded blocker for every particle, so its cost is the blocker count times the particle count. `PrecipitationLook::readable()` costs nothing measurable: `elements_precip_draw` in a `Thunderstorm` read 0.123 and 0.115 ms against 0.122 and 0.126 ms at the defaults (two runs each, the GPU at 1695–1755 MHz).

Clouds at 1920 × 1080, PartlyCloudy (the scene the clouds note fixes for the bench), minimum over the diagnostic history with the GPU at 1800 MHz; the 128³ + 32³ noise volumes are generated once at startup (0.201 ms):

| Quality | March resolution | Primary steps | Weather map | Shadow map | March | Resolve | Composite | Total |
|---|---|---|---|---|---|---|---|---|
| Low | 1/4 | 32 | 0.085 ms | 0.081 ms | 0.040 ms | 0.067 ms | 0.035 ms | 0.307 ms |
| Medium | 1/4 | 48 | 0.086 ms | 0.066 ms | 0.041 ms | 0.067 ms | 0.034 ms | 0.293 ms |
| High | 1/2 | 64 | 0.085 ms | 0.069 ms | 0.095 ms | 0.068 ms | 0.035 ms | 0.351 ms |
| Ultra | 1/2 | 96 | 0.085 ms | 0.067 ms | 0.125 ms | 0.068 ms | 0.034 ms | 0.378 ms |

The budget is 1.0 ms at High. The weather map, shadow map, resolve and composite passes cost about 0.27 ms at every quality, so the qualities differ only in the march, and Low and Medium (both a quarter of the resolution) within the measurement's noise. The sky probe (`elements_clouds_sky_probe`, the clouds in the environment map) does not depend on the screen: 0.047 ms at Low, **0.062 ms** at High and 0.155 ms at Ultra for Bevy's default 128² cube (3840 × 2160, PartlyCloudy, the GPU at 1680–1770 MHz).

Terrain with 1024² cells over 2048 m (`hills(42)`; measured before High took Medium's 512² simulation size, below), 3840 × 2160, the camera 30 m above the ground looking 20° above the horizon (about half the frame is ground), minimum over the diagnostic history with the GPU at 1800–1950 MHz: the generator and first pack (`elements_terrain`: generate, pack, flow pack, leaf bounds and readback kernels, once at spawn, then again only when hydrology or erosion changes a layer) 0.34 ms; the terrain drawn: Bevy's `main_opaque_pass_3d` 0.088 ms (mean 0.146) against 0.016 ms (0.025) for the flat test scene, and the depth prepass 0.005 ms (0.002 ms for the flat scene). The selection picks at most 133 patches at High over 1500 random cameras, and the pool update writes nothing for a patch that kept its slot and bounds.

Ground textures (`TerrainTextures`), 3840 × 2160, the eye-level meadow (`hills`, `ELEMENTS_EYE=0,0,0 ELEMENTS_LOOK=12,-3,8 ELEMENTS_AGL=1.7`, ground filling the frame below the horizon), the opaque pass's mean with base and change interleaved in the same clock state (P5; at 4K the history's minimum reads zero): without a set 6.8–7.4 ms against the base's 7.1–7.3 (the textured path is compiled only into a set's pipeline), with the examples' soil and grass set 8.8–9.4 ms, **+26 %**. Anisotropic filtering, the set's size and the number of taps fetched did not move it; the shader's peak register count did (`docs/research/ground-textures.md`, Deviation 19).

Wet ground (the porous sheen, the film albedo, the canopy, the rock's cavity occlusion), the same view and the mountain foot (`mountains`, `ELEMENTS_LOOK=60,0,60`), 3840 × 2160, the opaque pass over the early prepass of the same run (the ratio cancels the clock state; base and change interleaved): dry ground 55.2–55.6 against the base's 55.5–57.4, **unchanged**; soaked under `Rain` (`ELEMENTS_GROW_HOURS=3`) the meadow 57.2–58.7 against 54.7–55.3, **+5 %**, and the mountain 56.4–57.1 against 55.6–55.8, **+2 %**: the wet path's arithmetic, paid only where the ground is wet (`docs/research/wet-ground.md`, Cost).

Hydrology and erosion (`elements_hydrology`: every substep's two kernels, the thermal kernels on every 4th, and the readback kernels on a readback tick), minimum over the diagnostic history after 240 warm-up frames, a 2 km procedural terrain with a 50 m³/s `WaterSource`, the quality's full substeps per frame; the GPU held 1800–1950 MHz (P2) with the clocks logged in the same shell:

| Quality | Grid | Substeps per frame | `elements_hydrology` | Terrain pack passes (`elements_terrain`) |
|---|---|---|---|---|
| Low | 256² | 2 | 0.033 ms | 0.022 ms |
| Medium | 512² | 4 | 0.255 ms | 0.042 ms |
| High | 512² | 4 | 0.258 ms | 0.043 ms |
| Ultra | 2048² | 16 | 14.75 ms | 0.478 ms |

The plan's budget for hydrology and erosion at High is 0.6 ms for 1024² at 8 substeps, which measures **1.86 ms** (0.23 ms per substep, 80 % of the card's memory bandwidth, as `docs/research/hydrology.md` predicted), so High takes Medium's values (512², 4 substeps; `ElementsQuality::terrain_max_resolution` and `hydrology_substeps`) and Ultra keeps the research resolution and 16 substeps (14.75 ms; it is for large budgets). A 600 s bake at High is 7500 frames: 125 s at 60 fps.

Water (`WaterPlugin`), `ELEMENTS_TERRAIN=lake ELEMENTS_QUALITY=ultra` (a 256 m plain with a 100 m round basin filled by a 200 m³/s source, the camera 45 m up so the lake covers about a third of the frame), minimum over the diagnostic history after the lake is full, the GPU in P2 at 1800–1950 MHz with the clocks logged in the same shell: Bevy's `main_transparent_pass_3d`, which holds both water passes, costs **0.394 ms** at 1920 × 1080 (mean 0.409; 0.002 ms with `ELEMENTS_WATER=off`) and 1.196 ms at 3840 × 2160; the plan has no water budget and the note's informational target (0.3 ms for a half-screen view at 1080p) is missed by about 1.3× at an estimated 43 % coverage (`docs/research/water-rendering.md`, Validation 14). The `water_normals` kernel runs once at spawn (10 dispatches, 512² down to 1 texel).
Ground cover (`elements_snow`: source, drift flux and apply and two diffusion passes per step; `elements_wetness`: the wetness update, the render pack and, on a readback tick, the readback kernel; both at most 20 times a second), a Blizzard so the drift runs, minimum over the diagnostic history, 3840 × 2160, the GPU at 1800–1950 MHz with the clocks logged in the same shell:

| Quality | Grid | `elements_snow` per step | `elements_wetness` per step | Per frame at the 20 Hz cap (60 fps) |
|---|---|---|---|---|
| High | 512² | 0.084 ms | 0.042 ms | 0.042 ms |
| Ultra | 2048² | 1.366 ms | 0.514 ms | 0.63 ms |

The per-frame target (≤ 0.3 ms at High) holds up to 1024² (0.47 ms a step, 0.16 ms a frame). `elements_shelter`, the mask that keeps snow and rain from the ground under `PrecipitationBlocker`s, runs only when the blockers or the wind change: 0.025 ms at 512² with no blocker, 1.67 ms at 2048². See [`snow-cover.md`](../research/snow-cover.md) and [`wetness.md`](../research/wetness.md) for the model and the validation numbers.
Aeolian transport (`elements_aeolian_*`, at 3840 × 2160 with the GPU at 1800–1950 MHz, minimum over the diagnostic history, `ELEMENTS_TERRAIN=dunes` in a forced `DustStorm`): at High's 512² grid one step costs **0.236 ms** (capacity 0.023, flux scan 0.113, bed move 0.013, two avalanche passes 0.087), 0.079 ms per 60 Hz frame at the 20 steps a second cap; at Ultra's 2048² 1.997 ms per step, 0.666 ms per frame. The plan has no budget for dust; the note's target was ≤ 0.3 ms per frame at 1024², which interpolates to about 0.19 ms.

Trails (`elements_trails`: stamp, merge, block occupancy and relax over the window, every frame; `bench_elements` with `ELEMENTS_TERRAIN=hills ELEMENTS_IMPRINTS=16`, 1920 × 1080, the GPU at 1500–1935 MHz, minimum over the diagnostic history): **0.119 ms** at High's 1024² × 5 cm window with 16 wheels pressing, against the note's 0.15 ms; on a game's own ground (`ELEMENTS_TERRAIN=game`, no map lookups) 0.112–0.116 ms. The readback for `trail_depth` (`elements_trails_readback`) costs 0.025 ms on its 4 Hz ticks. The parallax in the terrain material skips any pixel whose ray meets no trail (the published block occupancy): `main_opaque_pass_3d` read 0.054–0.058 ms with `ELEMENTS_TRAILS=off` and 0.057–0.058 ms on, the trails a sliver of the frame; with the camera 6 m above the 16 wheels' rings, ruts filling much of the lower half of the frame on a game's plane in `WeatheredMaterial::with_trails()`, 0.413 ms against 0.226–0.236 ms off (P2, 1755–1800 MHz). See [`trails.md`](../research/trails.md) and [`trails-game-ground.md`](../research/trails-game-ground.md).

Waves (`elements_waves_force`: the bodies' head and the per-texel forcing, depth and current; `elements_waves_step`: one 1/60 s substep of all three cascades; `elements_waves_publish`: slopes and foam into the field; `bench_elements` with `ELEMENTS_TERRAIN=lake ELEMENTS_CONTACTS=8`, the camera over the filling lake, 1920 × 1080, the GPU at 1740–1800 MHz, minimum over the diagnostic history): High (3 × 512²) 0.037 + 0.217 + 0.033 = **0.29 ms** a frame with currents everywhere (each tile texel then fetched upstream, four loads), 0.23 ms on still water; Medium the same; Low (3 × 256²) 0.15 ms. Dry texels cost only their depth, so a window over land is nearly free. The water's shading of the field is within the run-to-run spread of `main_transparent_pass_3d`. This is above the note's 0.15 ms: a third cascade was the price of holding Airy's dispersion over 15 cm to 10 m waves (`waves.md` Deviation 1, 10).

Voxel snow is CPU work on the main world (the column-top scans and the chunk remesh), not a GPU pass, so it reports through the ignored `voxel_cost` release test rather than `bench_elements` (`docs/research/voxel-snow.md`): the initial scan of 16 384 columns (the `voxel_snow` example's 128 × 128) is 11.7 ms once at spawn; a user edit's column rescan 0.125 ms and one crest chunk's Marching Cubes 0.428 ms, both under the frame budget the note gives them (2 ms); the two depth-swap frames of a 60 game-hour run cost no more than the frame budget (2.9 ms of the 33 ms target); the median frame over 3 600 frames is 2.86 ms.

## Platforms

Native (Vulkan, Metal, DX12) and wasm WebGPU. WebGL2 is unsupported: the simulation needs compute. Simulation state lives in storage buffers and published fields in write-only `rgba16float` storage textures: the wind kernels share one layout of seven storage buffers and one storage texture, the fog kernel one storage texture, inside WebGPU's default limits of 8 and 4 per stage (devices limited to the `downlevel` 4 storage buffers are not supported; the hydrology substep kernels bind exactly 8). The cloud kernels bind at most one storage texture and two storage buffers each, the precipitation kernel four storage buffers; the precipitation draw reads its particles in the vertex stage, which needs `DownlevelFlags::VERTEX_STORAGE` (every Vulkan, Metal, DX12 and WebGPU device has it). GPU timestamps for the bench exist on Vulkan and DX12 only.

## What it deliberately does not do

- Trees, foliage, leaf colour and snow on branches: `bevy_flora` owns them.
- Isosurface and voxel meshing: `isomesh` owns it.
- Oceans and FFT waves.
- Bit-deterministic GPU simulation. Astronomy, the weather scheduler and lightning are deterministic and seeded; GPU results are not.
- A Purkinje shift or other low-light colour change: exposure keeps night dark, but colour handling is left to the tonemapper.
- Twigs, fallen branches, leaf litter and other organic debris on the ground: they are foliage, and `bevy_flora` owns it. This crate scatters the rock.
