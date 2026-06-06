# Gameworld spike — Philippines 3D terrain from a real DEM

**Throwaway prototype** (2026-06-06). Proves the path: real elevation data → heightmap →
displaced 3D mesh in Godot 4.6 → **clickable region overlay wired to the live Rust engine**.
Not production code — to be formalized later (see the open questions at the bottom).

Controls: **left-click** = select a region · **right-drag** = orbit/tilt · **wheel** = zoom ·
the top bar (click it) advances the month.

## What it does
1. `build_heightmap.py` — pulls public-domain elevation from **AWS Open Data "Terrain Tiles"**
   (terrarium PNG tiles, *no API key*, zoom 9 ≈ 425 m/px) across the PH bbox, decodes to meters,
   stitches, crops, and writes:
   - `data/heightmap_rg.png` — 16-bit elevation split across R(hi)/G(lo) bytes (mesh source;
     survives Godot's 16→8-bit PNG downsample).
   - `data/relief.png` — full-res **natural-colour + hillshade** texture (the map look).
   - `data/hillshade.png` — small preview · `data/meta.json` — dims + height scaling.
2. `build_provinces.py` — downloads **geoBoundaries PHL ADM1** (17 regions, gbOpen CC BY) and
   rasterizes a **province-ID map** (`data/provinces.png`, R channel = engine region id) aligned
   to the heightmap via the same Mercator projection. `data/provinces_debug.png` = colour-per-
   region eyeball check. Names map 1:1 onto `data.rs` region ids (ARMM→BARMM, etc.).
3. `spike_main.gd` + `Spike3D.tscn` — builds a displaced mesh (~1M verts), drapes `relief.png`
   (unshaded, relief baked in) plus the province-ID map in a custom shader that draws region
   **borders** and **highlights** the selected region. Left-click ray-picks the map → region id →
   pulls that unit's live stats from **PolitikoEngine** into a side panel. Single continuous
   surface (sea flat at y=0, land rising); `cull_disabled`.

## Run it
```powershell
# 1. build the data (needs: pip install numpy Pillow)
python spike/gameworld/build_heightmap.py
python spike/gameworld/build_provinces.py

# 2a. open Spike3D.tscn in the editor and press F6 to fly around, OR
# 2b. headless gate — asserts the mesh built with real relief, exits 0:
&  "<godot>" --headless --path . --script res://spike/gameworld/verify_spike.gd
# 2c. save a screenshot to data/render3d.png (windowed, needs a GPU):
&  "<godot>" --path . --script res://spike/gameworld/shoot_spike.gd
```

## Knobs (`spike_main.gd` consts; some overridable via env for quick renders)
- `VERTICAL_EXAGGERATION` — mountain height. 1.5 ≈ flat, 2.5 (default), 5 = dramatic/spiky.
  Env: `POLITIKO_EXAG`.
- `HEIGHT_CURVE_K` — peak compression. 0 = linear; higher (8 default, ~15 max) pulls the tallest
  peaks down while keeping lowland/hill relief. Env: `POLITIKO_CURVE`.
- `TARGET_GRID` — mesh resolution cap (longest side, in cells); higher = smoother coasts, slower.
- `MAP_WIDTH` — world units across the bbox · `ZOOM` (`build_heightmap.py`) — DEM resolution.
- Env: `POLITIKO_PITCH` (start camera angle), `POLITIKO_SELECT` (auto-select a region id),
  `POLITIKO_SHOT` (screenshot filename). Relief palette: `build_heightmap.py::colour_ramp`.

## Known approximations (fix when formalizing)
- Web-Mercator distortion left uncorrected (islands stretch slightly N–S).
- Bathymetry clamped to sea level (flat ocean, no depth shading or animated water).
- Relief shading is baked into the texture (unshaded), so it ignores a movable sun.
- Click-pick intersects the flat **y=0 plane**, not the raised mesh — tiny parallax error near
  tall peaks at low camera angles (fine for picking 17 big regions).
- The whole 1M-vert mesh is one surface with no LOD/streaming — fine here, not at barangay LOD.

## What this does NOT do yet (the real subsystem)
- **Feeding mechanics** — per the data pipeline, terrain becomes gameplay via *offline zonal
  stats* per `AdminUnit` (`avg_elevation`/`terrain_ruggedness`/`coastal`), not runtime terrain
  analysis. The map currently *reads* engine stats; it doesn't yet *seed* them from the DEM.
  See `docs/data-pipeline.md` and `docs/techspec/connectivity-hierarchy.md`.
- **Map modes** (political / satisfaction / poverty heat-maps), hover tooltips, and the
  connectivity/road overlay are all natural next layers on the same province-ID shader.
