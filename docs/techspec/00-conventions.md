# Tech Spec Conventions

Shared contracts every system Tech Spec assumes. Stated once here so individual specs can
reference them instead of restating. These mirror the structural invariants documented in
`rust/sim/src/lib.rs` (Rules #1–#2, the rollup contract, the one-tick lag). Note that `lib.rs`'s
*tick-order* docstring lists the **built six-system** order, whereas the tick order below is the
**v1 nine-system target** (the four extra slots land with Layers B/C); reconcile `lib.rs` when they do.

## Structural rule #1 — flat list, tree-shape logic

All units live in one flat `Vec<AdminUnit>` (`Sim::units`). **No code may hardcode "17",
count regions, or branch on `AdminLevel` to decide behavior.** Hierarchy is expressed only
through `AdminUnit::parent` (`Option<u32>`, `None` for the national root).

- A unit's role is derived from *tree shape*, not its level. `engine.rs::mark_aggregates`
  sets `AdminUnit::is_aggregate = true` for any unit that is some other unit's parent.
- Per-unit systems operate on **operational (leaf)** units only — `u.is_operational()` is
  `!u.is_aggregate`. Aggregates get their values from `aggregate_up`, never from system logic.
- This is what lets the engine scale across LODs with a data swap and no logic change. Every
  new system must honor it.

**Barangay-scale corollary.** v1's playable LOD is **barangay (~42,000 units)**. Level-agnosticism
is therefore not a nicety — it's load-bearing. Anything that scales superlinearly in unit count
(notably connectivity's all-pairs bottleneck pathfinding) must be expressed **hierarchically over
the parent tree**, computing within a parent and composing up — see
[`connectivity-hierarchy.md`](connectivity-hierarchy.md). Per-tick work must stay ~O(N); heavy
structure is rebuilt only on player builds, not every tick. Large seed data is loaded from a data
asset, not hand-authored in `data.rs` — see [`../data-pipeline.md`](../data-pipeline.md).

## Structural rule #2 — topology is a mechanic

`terrain_ruggedness` (0–1) and `coastal` (bool) must affect *outcomes and feasibility*, not
just visuals. The canonical example is `infrastructure::build_cost`. New systems that touch
the map (Connectivity edge friction, Port coastal-gating) extend this, never bypass it.

## The `Tuning` discipline

Every magic number lives in `constants.rs::Tuning` (or its per-industry methods). Systems
read constants from the `&Tuning` they are passed; they never inline literals. A Tech Spec's
"Tuning Constants" section is the authoritative list of additions a system needs. Defaults
are starting points for designers to tune, not balance guarantees.

## The derived-field contract

`AdminUnit` fields are split into **authored** (inputs: topology, population, education,
industries, infrastructure levels) and **derived** (recomputed every tick by systems, never
hand-authored): `satisfaction`, `revenue_last_tick`, `poverty`, `employment_rate`,
`daily_wage`, and the Layer-B/C additions (`market_access`, `export_price_mult`,
`exports_value`, `imports_value`, `external_revenue`). (Tourism-arrivals and FDI are Layer D.)

Rules for derived fields:
- They default to `0.0` and are fully recomputed each tick (no accumulation across ticks
  unless explicitly a stock).
- A system may read another system's derived output **only if that system runs earlier in
  the tick**, or the dependency is a documented one-tick lag (see below).
- New games are *primed* (`engine.rs::prime`) so the first tick reads sane derived values
  rather than zeros.

## The one-tick lag (the only feedback edge)

`population` runs first and reads `poverty`/`daily_wage` produced by `industries` on the
**previous** tick. This backward edge is intentional and is the only cycle in the system
graph; everything else is a strict forward dependency within a tick. New systems must not
introduce additional backward edges without documenting them here.

## The `aggregate_up` rollup contract (single path)

`aggregate.rs::aggregate_up` is the **only** way national/regional totals are produced.
It post-order folds each subtree and writes results into aggregate units:

- **Summed** up the tree: `population`, `revenue_last_tick`.
- **Population-weighted averaged**: `education_level`, `satisfaction`.
- Leaf-only metrics (`poverty`, `employment_rate`, `daily_wage`) are left zero on aggregates.

**Extension rule:** when a new system produces a field that must be visible at the
aggregate/national level (e.g. national `exports_value`, `external_revenue`, regional
`market_access`), it is added to the `aggregate_up` fold — summed or pop-weighted as
appropriate — **not** computed via a separate leaf-counting pass. There is exactly one
rollup path. The v1 additions to the fold:

| Field | Rollup |
|-------|--------|
| `exports_value` | sum |
| `imports_value` | sum |
| `external_revenue` | sum |
| `market_access` | population-weighted average |

`aggregate_up` is called multiple times per tick (inside `economy`, inside `politics`, and
at prime). It is idempotent given fixed leaf inputs, so extra calls are safe.

## Tick order (authoritative)

See [`../system-interaction-map.md`](../system-interaction-map.md). v1 order:
`population → connectivity → global_demand → industries → trade → economy → infrastructure
→ education → politics`. A system's "Tick slot" header field refers to this **nine-system v1**
numbering. Heads-up: the Rust per-module docstrings number by the *current six-system* order
(e.g. `economy.rs` = "System 3", `politics.rs` = "System 6"), so cite systems by **name**, not by
number, when a spec and the code might disagree.

## Player actions (out-of-tick)

Build actions (`build_transport`, future `build_gateway`) are methods on `Sim`, invoked
between ticks by the player, not part of `advance_tick`. They mutate authored infrastructure
levels and the treasury immediately, and return a `Result<_, BuildError>`.

## Test-plan style

Tech Specs specify tests in the style of `rust/sim/tests/integration.rs` and the per-module
`#[cfg(test)]` blocks: assert on **relationships and invariants** (monotonicity, ranges,
conservation), not brittle exact magnitudes, because constants are expected to be tuned.
