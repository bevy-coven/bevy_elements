# The bevy_elements guide

The README shows what the crate does and how to start. These pages are the reference: every domain in depth, its settings, its research note and its measured cost.

- [Putting it in a scene](scene.md) — the frame order, the camera the crate follows, a focus for the camera-relative systems, orthographic and isometric cameras, and world units.
- [Sky, weather and light](sky-weather.md) — the plugin table, the clock and ephemeris, the climate and its scheduler, the GPU wind field, fog, clouds with their reflections and rainbows, precipitation, lightning.
- [Terrain and ground](terrain.md) — the heightfield, its materials and grading, ground textures, sculpting, saving and restoring, snow and wetness cover, scattered rocks, dust transport, paths, trails.
- [Rivers, lakes and waves](water.md) — shallow-water hydrology and erosion, the water material and a still sea, and the wave field bodies leave behind.
- [Surfaces, materials and voxel snow](surfaces.md) — `WeatheredMaterial` for any mesh and its WESL import, `SurfaceCover`, and snow on `bevy_isomesh` voxel volumes.
- [Gameplay queries](queries.md) — everything a game asks the running simulation, and how to light a night scene from the crate's own metering.
- [Performance and platforms](performance.md) — measured GPU cost per pass and quality level, what the crate deliberately does not do, and where it runs.
- [Examples](examples.md) — all 23 examples, the environment variables they take and the keys they answer.
- [Research](research.md) — the paper note behind each system, with every deviation cited.

Two more places hold the parts that are pictures rather than prose:

- [`../gallery.md`](../gallery.md) — the before-and-after captures of every look the crate ships.
- [`../media/README.md`](../media/README.md) — every image and GIF with the command that regenerates it.

## Plays well with the family

- **`bevy_flora`** 0.46 (feature `flora`): add `ElementsFloraPlugin` next to `FloraPlugin`. It writes `FloraWind`, `FloraSeason` and `FloraWetness`; trees and their sway stay `bevy_flora`'s. A game on a git revision of `bevy_flora` patches it (`[patch.crates-io]`) so both crates use one copy.
- **`isomesh` / `bevy_isomesh`** (feature `isomesh`): add `IsomeshPlugin` next to `ElementsPlugins` and put `VoxelGround` on a `VoxelVolume` for snow on voxel ground. Meshing, chunks and the volume stay `bevy_isomesh`'s.
- **`bevy_carnage`, `bevy_light_grid`** (Bevy 0.19): no dependency either way. Glue in the game copies plain data across, e.g. `SurfaceWeathering::wetness` into a blood-drying rate, or `Environment::sun()` into a light grid's sky term.

See [Surfaces, materials and voxel snow](surfaces.md) for the voxel path and [`../research/`](../research) for the note of each integration.
