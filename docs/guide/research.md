# Research

Every system was studied in the paper library first; each note names its sources, its equations and every deviation.

Every system was studied in the home-still paper library first. The note for each one, in [`../research/`](../research), names the papers, the equations adopted, every deviation from the plan with its source, and the numbers the tests assert.

- [Bevy 0.20 render APIs](../research/bevy-0.20-render.md) — the verified paths and signatures this crate builds on.
- [What the home-still corpus holds](../research/corpus.md) — the reading list per domain.
- [Celestial](../research/celestial.md) — Meeus, Reda & Andreas (SPA), Krisciunas & Schaefer, Jensen et al., Ballesteros, USNO Circular 171, Larson et al. (tone reproduction).
- [Twilight and the night sky](../research/twilight.md) — Hulburt 1953, Haber et al. 2005, Hillaire 2020, Patry 2021: the sun lighting the sky below the horizon, the night glow behind the atmosphere, the glow-capped exposure.
- [Climate and weather](../research/climate-weather.md) — Richardson 1981, ISO 2533, Jennings et al. 2018.
- [Wind](../research/wind.md) — Stam 1999, Harris 2004, Bridson et al. 2007, Kim et al. 2008, WMO.
- [Lightning](../research/lightning.md) — Reed & Wyvill 1994, Kim & Lin 2004, Poelman et al. 2021, Rakov, Tatarchuk 2006.
- [Fog](../research/fog.md) — Wronski 2014, Hillaire 2015, Wenzel 2006, Real-Time Rendering.
- [Terrain](../research/terrain.md) — Strugar 2009 (CDLOD), Andersson 2007, Musgrave et al. 1989, Galin et al. 2019, Mei et al. 2007.
- [Paths and roads](../research/paths.md) — Pouyat et al. 2020 (the trail impacts: bared soil, compaction, vegetation loss), Andersson 2007 (mask-based terrain compositing), Green 2007 via RTR4 §15.4.3 (coverage distance fields), Sumner et al. 1999, Tatarchuk 2006.
- [Terrain level of detail under an orthographic camera](../research/orthographic-lod.md) — Strugar 2009, Asirvatham & Hoppe 2005 (clipmaps centred on the viewer), Luebke et al. 2003 and Lindstrom et al. 1996 (screen-space error), the Unreal and Unity terrain docs (a vertex on every height sample).
- [Precipitation](../research/precipitation.md) — Marshall–Palmer drops, Garg & Nayar, Langer et al., Wang & Wade, Kanazawa & Uchida, Rasmussen et al., Tatarchuk.
- [Rainbows](../research/rainbow.md) — Sadeghi et al. 2012 (primary and secondary bows, Airy and Lorenz–Mie), Airy 1838 and van de Hulst 1957, Marshall & Palmer 1948, Wyman et al. 2013.
- [Clouds](../research/clouds.md) — Schneider 2015 and 2017 (Horizon Zero Dawn, Nubis), Hillaire 2016 and 2020, Högfeldt 2016, Patry 2021, Real-Time Rendering, Kasten & Czeplak 1980.
- [Wetness, puddles and weathered materials](../research/wetness.md) — Lagarde 2013 (wet surfaces, rain), Jensen et al. 1999, Tatarchuk 2006, FAO-56; with [snow cover](../research/snow-cover.md) for the mesh curve.
- [Hydrology](../research/hydrology.md) — Mei et al. 2007, Jákó 2011, Chentanez & Müller 2010, Kass & Miller 1990, Bates et al. 2010 / Cozzolino et al. 2020 (local-inertia pipes), FAO-56.
- [Dust and aeolian transport](../research/dust.md) — Paris et al. 2019 (Desertscape simulation), Kok et al. 2012 (the physics of wind-blown sand and dust), Bagnold, Fryberger & Dean, Hersen et al.
- [Erosion](../research/erosion.md) — Mei et al. 2007, Jákó 2011, Musgrave et al. 1989, Krištof et al. 2009, Jain et al. 2024, Galin et al. 2019.
- [Water rendering](../research/water-rendering.md) — Vlachos 2010 (Portal 2 water flow), Chentanez & Müller 2010, Mei et al. 2007, Jákó & Tóth 2011, Pope & Fry 1997, Real-Time Rendering.
- [Voxel snow](../research/voxel-snow.md) — the column-top terrain `VoxelGround` feeds the snow maps, the snow slab of `SnowyField` over isomesh's signed distance fields, per-column sample-and-hold swaps and the chunks they dirty; Festenberg & Gumhold 2011, Lorensen & Cline 1987, Tereshin et al. 2021.
- [A focus and orthographic cameras](../research/view-anchor.md) — Marschner & Shirley (orthographic rays), Real-Time Rendering, Puig-Centelles et al. 2009 and 2011 (the rain container around what the observer sees), Losasso & Hoppe 2004, Tatarchuk 2006.
- [A game keeps its own look and lights](../research/integration.md) — Patry 2021 (art direction over a physical sky), Schneider & Vos 2015, Tatarchuk 2006; the camera's dressing flags, `CelestialSettings`, the public globals buffer.
- [The snowpack in a save, and a snow season](../research/snow-season.md) — Jennings et al. 2018 (the rain–snow threshold varies from −0.4 to 2.4 °C), Patry 2021; `SnowSeason`, `Weather::snow_possible`, the restored pack and the seeded cover maps.
- [A record of recent days](../research/climate-history.md) — the ETCCDI's daily indices (frost, icing and wet days; Zhang et al. 2011), Thurner et al. 2017, Richardson 1981 (the daily weather record); `ClimateHistory`, `Environment::recent_days` and `recent_climate`, issue #31.
- [Sky light and reflections under cloud](../research/sky-light.md) — Hillaire 2016 (clouds in every view, the environment map included), Lagarde & de Rousiers 2014 (the distant light probe, refreshed with the weather), Unreal's real-time sky light capture, Bauer 2019 (RDR2), Karis 2013 (split sum), Kittler & Darula 2014 (the graded overcast); the clouds' sky probe.
- [A game's look for rain and snow](../research/precipitation-look.md) — Tatarchuk 2006 (artist-set rain direction and speed, random particle size), Lagarde 2012 (Remember Me: rain stronger than physics where it would vanish), Wang & Wade 2004, Garg & Nayar 2007, Tiira et al. 2016 (snowflake size distributions); `PrecipitationLook`, and the scheduler's forecast.
- [Trails on a game's own ground](../research/trails-game-ground.md) — Barré-Brisebois 2014 (Batman: Arkham Origins: "we don't have terrain", snow surfaces a game marks), Tran 2018 (a world-space trail map projected onto the ground material), Hanák 2021 (RotTR's sliding window, TLOU2's parallax prints on any mesh), Tatarchuk 2006; `TrailGround`, `with_trails()`, `trail_depth`.
