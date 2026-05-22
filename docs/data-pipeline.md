# Data Pipeline — Building the ~42,000-Barangay Seed Dataset

- **Status:** Spec'd-v1 — **Owner track:** separate from engine code; a large one-time/offline effort.

v1 ships at **barangay** level of detail. This doc specifies how the seed dataset is sourced and,
where necessary, **synthesized** — because much of the economy is not published below the region.
The output is loaded by the engine (it is far too large to hand-author in `data.rs`).

> **Reality check.** This is a substantial GIS + data-engineering project in its own right. The
> engine work and this pipeline proceed in parallel; the engine is validated at coarse LOD first
> (the data is level-agnostic, so a region-level dataset runs the same code) while this pipeline
> brings the barangay dataset up.

## 1. What the engine needs

Per the data model (see [`techspec/00-conventions.md`](techspec/00-conventions.md) and the system
specs), the dataset must provide:

- **Unit tree:** ~42k barangays under municipalities/cities under provinces under regions under the
  national root — the `AdminUnit { id, parent, name, level }` hierarchy (PSGC).
- **Authored unit fields:** `population`, `education_level`, `industries[]`, topology
  (`avg_elevation`, `terrain_ruggedness`, `coastal`), and initial `infrastructure` levels.
- **Adjacency graph:** `Edge { a, b, road_tier, rail_level, length_km }` between neighboring units,
  plus tagged long-haul **corridors** used as shortcut portals by the hierarchical connectivity
  algorithm ([`techspec/connectivity-hierarchy.md`](techspec/connectivity-hierarchy.md)).

## 2. Sources (and what each can/can't give)

| Need | Source | Availability | License note |
|------|--------|--------------|--------------|
| Unit tree (PSGC codes/hierarchy) | PSA PSGC; `altcoder/philippines-psgc-shapefiles` | Full to barangay | Gov / open |
| Barangay boundaries | HDX COD-AB (ADM4), PSA, geoBoundaries | Full to barangay | geoBoundaries CC BY (commercial-safe); GADM **non-commercial** — avoid |
| Population | 2020 Census of Population (PSA) | **Available at barangay** | Gov data |
| Topology (elevation/ruggedness/coastal) | SRTM DEM (public domain) → zonal stats per barangay polygon | Derivable | **Public domain** (chosen DEM) |
| Education level | Regional/municipal literacy (FLEMMS, census) | Region/municipal only → **downscale** | Gov data |
| Industries / sector mix | Regional GRDP by industry (PSA OpenSTAT); municipal poverty | Region (sector); not barangay → **synthesize** | Gov data |
| Road tiers / rail | OSM road & rail network; or DPWH road data | Derivable | **OSM = ODbL (share-alike) — see §6** |

## 3. Stages

1. **Build the unit tree.** Parse PSGC into the `AdminUnit` hierarchy; assign ids; set `parent` and
   `level`. The national root is id 0. (Level is for display/authoring; logic stays level-agnostic.)
2. **Join population.** Attach 2020 census barangay population to each leaf.
3. **Topology via zonal stats.** For each barangay polygon, run zonal statistics over the SRTM DEM:
   `avg_elevation` = mean; `terrain_ruggedness` = normalized slope std-dev (run `gdaldem slope`
   first); `coastal` = polygon intersects the coastline. ~42k zonal computations — heavy but
   offline (rasterstats / QGIS / GDAL).
4. **Synthesize the economy** (the hard part — see §4): downscale regional GRDP-by-sector and
   regional/municipal education onto barangays.
5. **Build adjacency + tiers.** Derive neighbor links from shared boundaries (and/or the OSM road
   graph). Classify each link's `road_tier` from road class (OSM `highway=*` → rough/FMR/highway, or
   DPWH classification); set `rail_level` from rail lines; compute `length_km` from geometry. Tag
   trunk highways / rail spines as **corridors** (shortcut portals).
6. **Emit the data asset.** Serialize to a compact loadable format (see §5), versioned.

## 4. Synthesizing the economy (no barangay-level source exists)

GRDP by industry is published at the **region**; poverty/education at best at the **municipality**.
So barangay economy is *modeled*, not measured. Recommended approach:

- **Allocate regional sector output to barangays** by a weighted plausibility score using signals we
  *do* have per barangay: population (size), urban/rural class (PSGC + density), `coastal`,
  `avg_elevation`, `terrain_ruggedness`, land-cover if available. E.g. Mining weights toward
  rugged/mineralized barangays; Agriculture toward rural lowland; Services/Manufacturing toward
  urban/coastal; preserve the regional sector totals as a constraint (the barangay shares must sum
  back to the published regional figure).
- **Education**: start from the region/municipal literacy/attainment figure, apply within-area
  variation by urban/rural and income proxies, normalized to 0–1.
- **Initial infrastructure**: seed from road-network density / classification (a barangay on a
  highway starts higher than one on a rough track).

This produces a *plausible, internally consistent* starting world — explicitly an approximation to
be validated and tuned, not ground truth. Document the weighting model so it's reproducible and
tunable.

## 5. Output format & engine implication

At 42k units + their edges, the seed is **not** hand-written `data.rs`. Emit a versioned data asset
(e.g. compressed JSON or `bincode`) under the project, loaded at startup into `Sim`. Engine
implication: `data.rs` shifts from literal seed functions to a **loader** (and the existing
`philippines_regions()` becomes a small built-in fallback / test fixture). Keep a tiny coarse
dataset (regions) committed for fast tests; ship the full barangay asset separately.

## 6. Licensing (commercial game — be careful)

- **DEM = SRTM:** public domain. Clean.
- **PSGC / census / GRDP:** PSA government statistics; factual data, usable — verify PSA terms,
  attribute where asked.
- **Boundaries:** prefer **geoBoundaries (CC BY 4.0)** or HDX COD; **avoid GADM** (non-commercial).
- **OSM road/rail = ODbL.** This is the trap: a database *derived* from OSM can carry **share-alike**
  obligations on the derived database, plus attribution. For a commercial product, either (a) keep
  the OSM-derived network as a separable, ODbL-compliant database with attribution, or (b) source
  road classifications from **DPWH/government** data instead to avoid ODbL share-alike. Decide this
  before committing road data into the shipped asset.

## 7. Tooling

Python (`geopandas`, `rasterio`/`rasterstats`, `osmnx`/`pyrosm`, `pandas`), GDAL CLI, QGIS for
inspection. The pipeline is offline and re-runnable; outputs are versioned data assets, not runtime
code. See the terrain/economic sourcing research already captured in project memory.

## 8. Validation

- Barangay populations sum to municipal/regional/national census totals.
- Synthesized sector output sums back to published regional GRDP.
- Topology sanity (coastal barangays near the coastline; elevation matches known terrain).
- The adjacency graph is connected within each island group; corridors present where expected.
- Spot-check market-access outputs against intuition (known isolated areas read as low access).
