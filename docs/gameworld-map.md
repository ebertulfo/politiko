# Gameworld Map — Spike Findings & Open Questions

- **Status:** Spike done (2026-06-06), **not** yet PRD'd or sliced. This doc captures what the
  throwaway prototype at `spike/gameworld/` validated so we can plan the real subsystem
  deliberately (grill-me → to-prd → to-issues), instead of growing the spike ad hoc.

## What the spike proved

A 3D, topographic, clickable map of the Philippines wired to the live engine is **feasible and
not a major lift**, and the right technical shape is now known:

1. **Rendering = custom displaced heightmap mesh + draped relief texture.** A ~1M-vert `ArrayMesh`
   built from a real DEM, with a baked natural-colour + hillshade texture (unshaded), reads as a
   proper physical-relief map. **Terrain3D is the wrong tool** (editable/streaming middleware,
   ~65 km scale cap) and **Province Map Builder is 2D-only** — neither fits. Confirmed Godot 4.6
   renders 1M+ verts fine on the dev GPU.
2. **Interaction = province-ID overlay (the Paradox technique).** A rasterized province-ID map
   (R channel = region id), sampled in a custom shader, draws borders and highlights selection;
   left-click ray-picks → region id → `PolitikoEngine.get_unit(id)`. Works against the real binding.
3. **Data sources & alignment.** Elevation from AWS Open Data terrain tiles (no-auth, prototype)
   aligns pixel-exact with **geoBoundaries PHL ADM1** (17 regions, gbOpen CC BY) via a shared
   Mercator projection. All 17 boundary regions map 1:1 onto the engine's region ids.
4. **Architecture matches the engine (and Victoria 3).** The map is **visual + a clickable
   overlay**; terrain affects gameplay through per-unit stats (`avg_elevation`/`ruggedness`/
   `coastal`) seeded **offline** by zonal stats — never runtime terrain analysis. The map *reads*
   engine state; it does not own it.

## Reusable vs throwaway

- **Keep as reference (concepts, not code):** the rendering recipe, the province-ID shader idea,
  the geoBoundaries→engine-id mapping, the Mercator alignment math, the data-source choices.
- **Throwaway (do not ship):** all `spike/gameworld/*.gd` — mesh built procedurally in `_ready`,
  shader embedded as a string, click-pick against the flat y=0 plane, no LOD, hardcoded tuning.
  The Python builders are prototype-grade (single-mesh PNG, no reprojection, AWS tiles not SRTM).

## Open questions for planning (the decision tree to resolve)

1. **Priority / sequencing.** `vertical-slices.md` reaches a *playable sandbox* in ~6 thin UI
   slices (dashboard, region table, levers, build, save, election) — **none need the 3D map**.
   The map is a separate, heavier visual track. Do we (a) finish the sandbox-via-tables path
   first and add the map after, or (b) make the map the primary UI now? **This is the first
   decision.**
2. **LOD & scale.** v1 engine ships region-level but is designed to scale to ~42k barangays. The
   spike's single-mesh + single province-PNG **will not** scale to barangay LOD. What is the map's
   v1 target — region-only? — and what's the path to finer LOD (tiling/streaming, vector borders)?
3. **Data pipeline ownership.** When do we build the *real* offline pipeline (SRTM not AWS tiles,
   reprojection, zonal stats seeding `data.rs`, province map as a versioned build artifact) vs
   keep quick scripts? Ties into `data-pipeline.md`.
4. **Interaction scope for v1.** Beyond select+inspect: map modes (political / satisfaction /
   poverty heat-maps), hover tooltips, build actions on the map, the connectivity/road overlay —
   which are in v1?
5. **Integration point.** Does the map replace/augment `Main.tscn` (the slice-1 dashboard)? How do
   selection and the existing region table coexist?
6. **Shipping concerns.** SRTM (public-domain) for the shipped DEM; OSM road licensing (ODbL trap,
   see `data-pipeline.md §6`); committed vs generated assets; performance budget.

## How to slice it (later)

The map is a big subsystem; once priority + LOD are settled it breaks into tracer-bullet slices,
e.g.: static relief mesh from a committed heightmap → province-ID overlay + borders →
click-to-select wired to a real region panel → map modes → seed terrain from zonal stats. Each is
a thin path through data → Godot → verifier, demoable on its own.

See `spike/gameworld/README.md` for how the prototype works, and project memory
`politiko-map-data-sources` for the data-source decisions.
