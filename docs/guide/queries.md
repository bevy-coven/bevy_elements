# Gameplay queries

What a game asks the running simulation: the weather, the ground under a point, the water, the snow, the light.

```rust
use bevy::prelude::*;
use bevy_elements::*;

fn seek_shelter(env: Environment) {
    let weather = env.weather();
    if weather.precipitation_mm_h > 2.0 || (!env.is_daylight() && env.temperature_c() < 0.0) {
        // ...
    }
    let gust_at_head = env.wind_at(Vec3::new(10.0, 1.8, -4.0));
    let _ = (gust_at_head, env.snapshot_age());
}
```

`wind_at` answers from the GPU field as of the last readback (4 Hz by default, `ElementsReadback`) and falls back to the prevailing wind outside it or before the first readback; `snapshot_age` says how old the answer is. The queries work in an app with no window (a headless server rendering only to an image): there each frame waits for its own GPU work, since nothing presents to pace it, so a readback still lands one frame after it is requested.

`terrain_height(Vec2)` is the ground's bilinear at a world XZ (`None` off the terrain); `terrain_drawn_height(Vec2)` is the surface the full-detail patches actually draw, interpolated over the mesh's triangles (split along `(1, 0)-(0, 1)`), which equals the bilinear at every cell centre and differs inside a quad by at most a quarter of its twist. Seat a figure on a crease (a pad's edge, a ramp) with the second; `TerrainGrid::drawn_height` is the same on a layer you hold.

`terrain_normal(Vec2)` is the unit upward normal of the ground at a world XZ and `terrain_slope_deg(Vec2)` its slope in degrees, both from the gradient the trail kernel and `sinkage_at` use (central differences over a cell either side of the last readback, bilinear between cells): align a prop or a footstep, or test a slope, without re-deriving the step from `terrain_height`. `None` off the terrain and before the first readback.

`water_depth(Vec2)` is the water above the ground at a world XZ: `None` until the first readback or off the terrain, `Some(0.0)` on dry ground. `water_velocity(Vec2)` is the depth-weighted flow velocity (half resolution). Both are bilinear samples of the last readback.

`snow_depth(Vec2)` is the snow on the ground in metres and `wetness(Vec2)` how wet it is (0 dry, 1 a full 2 mm film); like the water queries they are `None` before the first readback or off the terrain.

`ground_look_at(Vec2)` answers what the terrain's ground cover is at a point in the shader's own terms (`GroundLook`): `wet` (the wetness map), `puddle` (the mask of the simulated water depth), `snow` (its cover) and `drawn_wet`, `min(max(wet, puddle), 1.4)`, the wetness the surface model wears on bare ground. A decal faded with `drawn_wet` agrees with the ground; grass and the rut-floor puddle of a trodden trail are not in it. `None` off the cover maps, without ground cover and before the first readback.

`sinkage_at(Vec2, half_extents, load_n)` is how deep, m, a contact of those half extents (m) under that load (N) would press into the terrain's ground there now: the trail kernel's own Bekker–Wong sinkage on the last readback (snow capped by CRREL's compaction limit, sand, soil to mud by wetness, 0 on rock). A game that draws a print only where it is at least 1 cm deep asks this and keeps no copy of the physics. `None` off the map, without ground cover and before the first readback; a `TrailGround` a game supplies is not read.

`trail_depth(Vec2)` is how far the ground is pressed below its undisturbed surface by `Imprint`s, m: the deepest point within 10 cm at High, `Some(0.0)` on untouched ground or a rim, `None` outside the trail window or before the first readback. It works on the crate's terrain and on a game's own ground (`TrailGround`).

`cloud_shadow_at(Vec3)` is the direct transmittance of the clouds toward the sun (the moon at night) at a world position, 0 in a thick cloud's shadow to 1 in the open, from the cloud shadow map sampled every 31.25 m over 8 km around the camera (1 outside it); `illuminance_at(Vec3)` is the horizontal illuminance there in lux, the clear-sky sun and moon light times `0.25 + 0.75 · T`. Only the clouds shadow them, not terrain or meshes. `cloud_shadow_at` is `None` before the first readback; `illuminance_at` then uses the cover-only estimate.

**A game lights its own night.** The crate meters its exposure on the scene's *illuminance* and
not on the frame's content, so a game's own light does not move the camera's exposure (Patry
2021's "illuminance-based exposure system… takes albedo out of the equation altogether"). What
a game needs is the other side of that: what power its light must have to read. The crate
answers it, in the camera's own terms:

```rust
use bevy::prelude::*;
use bevy_elements::*;

fn torch(env: Environment, mut lights: Query<&mut SpotLight>) {
    for mut light in &mut lights {
        // The scene-linear value the ground should read at: 0.18 is 18 % grey, the middle of
        // the picture, and the crate's metering holds it there at every hour.
        let luminance = env.luminance_for_display(0.18);
        // The lumens that put a surface of that albedo at that luminance, `distance` metres
        // away. The same number works for a `PointLight` and a `SpotLight`.
        light.intensity = env.light_intensity(luminance, 2.0, 0.14);
        // The night's own colour, so the light and the sky are one night.
        let c = env.night_light_colour();
        light.color = Color::linear_rgb(c.x, c.y, c.z);
    }
}
```

`exposure()` is the camera's exposure factor (`2^(−EV₁₀₀) / 1.2`, the same number Bevy's
`Exposure::exposure()` returns), and `display_value(luminance)` is the scene-linear value a
luminance has under it — the tonemapper's input, since Bevy applies the exposure inside the
PBR shader. `luminance_for_display(display)` is the inverse, `light_intensity(luminance,
distance, albedo)` gives a light's power in **lumens** (Bevy's `PointLight::intensity` and
`SpotLight::intensity` are luminous power, not candela), and `light_illuminance(intensity,
distance)` is the falloff on its own. The same numbers are in the `light` module and in the
`ElementsExposure` resource, for a system that does not hold an `Environment`.

The power a light needs scales *inversely* with the exposure: the same lamp wants 62 253 lm at
noon and 2.8 lm at midnight, for the same picture. A game that hard-codes a lamp's lumens is
wrong by five orders of magnitude between the two; a game that asks the crate is never wrong.
See `docs/research/game-lights.md`.

**Saving the weather and the snowpack.** The weather's save state is `WorldClock`, `Climate`, `GeoLocation` and `WeatherDirector` (the scheduler's chain position, a forced preset and the blend in progress); `Weather` is recomputed from them each frame. The ground's is `SurfaceWeathering` (the snowpack and the wetness), and the record of recent days is `ClimateHistory`. With the `serde` feature all six serialize; insert the saved copies before the next `ElementsSystems::Weather` run and the weather, the snow and the record continue exactly as saved, where a fresh director would restart the chain at `Clear`, a fresh `SurfaceWeathering` at bare, dry ground and a fresh record with no days. A snowpack or a record loaded onto a clock more than an hour from the one it was saved at neither melts, grows nor records across the gap. The ground-cover maps on a terrain are not saved: they are re-seeded from the loaded snowpack, times each cell's slope and shelter exposure, so roofs keep the ground under them bare; drifts re-form as it snows. The other GPU layers (water, terrain) are not part of it. `tests/save.rs` shows the round trip; `weathered_town` writes and loads a save with `ELEMENTS_SAVE` and `ELEMENTS_LOAD`.

**A forecast.** `WeatherDirector::forecast(&climate, &location, &clock, hours)` returns the preset the scheduler will choose for each of the next `hours` game hours, by running the same seeded chain forward without changing anything: the weather then comes as forecast (with the same climate and location), whatever is forced meanwhile, since the chain keeps stepping underneath a force. A morning board can say that a storm comes this afternoon. `SurfaceWeathering` compares with `==` (its private snowpack state included), so a save that holds it can derive `PartialEq`.
