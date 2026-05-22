# Tech Spec: Connectivity

- **Maps to:** new `rust/sim/src/systems/connectivity.rs` (+ shared `rust/sim/src/network.rs`) — **Tick slot:** 2 — **Status:** Spec'd-v1

Conventions: [`00-conventions.md`](00-conventions.md). The scaling algorithm (mandatory at
barangay LOD) is its own doc: [`connectivity-hierarchy.md`](connectivity-hierarchy.md).

## 1. Responsibility

Compute each operational unit's `market_access` from the tiered road/rail network using
**bottleneck (weakest-link) path capacity with mild distance decay**.

| Reads | Writes |
|-------|--------|
| `population`, node `transport_level`, edge `road_tier`/`rail_level`, topology (distance), `Sim::adjacency` | `AdminUnit.market_access` |

## 2. Data Fields

**New on `Sim` (`engine.rs`):**
```rust
pub struct Edge {
    pub a: u32,
    pub b: u32,
    pub road_tier: u8,   // 0 none, 1 rough, 2 farm-to-market, 3 highway
    pub rail_level: u8,  // 0 none, 1+ freight rail (separate high-capacity overlay)
    pub length_km: f32,  // for distance decay
}
pub struct Sim { /* ... */ pub adjacency: Vec<Edge> }
```
**Node infra (`types.rs`, `InfrastructureState`):** keep `transport_level: u32` as the *internal*
capacity of a unit (how well goods move *through* it). Gateways (`port_level`, `airport_level`)
are in the Infrastructure spec.

**New derived on `AdminUnit`:** `pub market_access: f32` (0..~1; default 0, recomputed each tick).

**Seed data (`data.rs`):** the barangay adjacency + tiers come from the data pipeline
([`../data-pipeline.md`](../data-pipeline.md)), not hand-authoring.

## 3. Tuning Constants (`constants.rs::Tuning`)

| Const | Meaning |
|-------|---------|
| `road_capacity: [f32; 4]` | Capacity per road tier (none/rough/FMR/highway), e.g. `[0.05, 0.3, 0.6, 1.0]` |
| `rail_capacity_per_level` | Capacity contributed by each rail level (high; the "game-changer") |
| `node_capacity_per_transport` | How a unit's internal `transport_level` maps to a pass-through capacity |
| `distance_decay_per_km` | Mild attenuation per km of path length |
| `market_access_weight` | Strength of the revenue multiplier (consumed in industries) |
| `min_segment_capacity` | Floor so a connected segment never reads as fully zero |

All capacities are tunable starting points; the bottleneck/decay shape is the contract.

## 4. Formulas

```
# Per-segment capacity (an edge, combined with the pass-through of the units it joins)
edge_capacity(e)   = max(road_capacity[e.road_tier], e.rail_level × rail_capacity_per_level)
node_capacity(u)   = u.transport_level × node_capacity_per_transport          # internal pass-through
segment_capacity   = min(edge_capacity(e), node_capacity(intermediate units))  # weakest link includes nodes

# Best path i→j = the path MAXIMIZING its bottleneck (widest-path), then distance-decayed:
path_capacity(i, j) = ( max over paths of  min(segment_capacity along path) )
                      × exp(−distance_decay_per_km × path_length_km(i, j))

# Market access = capacity-weighted reachable market, normalized
market_access(i) = ( Σ_j  population[j] × path_capacity(i, j) ) / national_population   # ~[0,1]
```

**Bottleneck semantics:** `min` along the path means one weak upstream segment (road *or* the
pass-through of an intermediate unit) caps access for everything routed behind it — the design's
core rule. Widest-path (max-min) is computed with a modified Dijkstra (max-heap on bottleneck);
distance decay is carried alongside the bottleneck on the chosen path.

Consumed downstream: industries multiply revenue by `(1 + market_access × market_access_weight)`
and gate **specialization** (Industries spec); politics may add `w_connectivity × min(1,
market_access)`.

## 5. Function Signatures

```rust
// network.rs — shared with trade; builds + holds the (hierarchical) access structure
pub struct Network { /* hierarchical bottleneck structure — see connectivity-hierarchy.md */ }
pub fn build_network(units: &[AdminUnit], adjacency: &[Edge], t: &Tuning) -> Network
pub fn path_capacity(net: &Network, from: u32, to: u32) -> f32

// connectivity.rs
pub fn run(units: &mut [AdminUnit], net: &Network, t: &Tuning)
```

## 6. Tick Placement

Slot 2 — after population (needs fresh population as market mass), before industries (produces the
`market_access` they read). Independent of `global_demand` (slot 3).

## 7. Reads / Writes Contract

Writes only `market_access` on leaves. The `Network` (bottleneck structure) is rebuilt only when
infrastructure/adjacency changes (build actions), not every tick; each tick recombines it with
current population. No backward edges.

## 8. Aggregation Impact

`market_access` is added to the `aggregate_up` fold as a **population-weighted average** (single
rollup path — see [`00-conventions.md`](00-conventions.md)).

## 9. Structural-Rule Compliance

The graph connects unit ids regardless of `AdminLevel`; bottleneck logic never counts or
special-cases a level. Distance/decay are topology-derived (rule #2). **This level-agnosticism is
load-bearing at barangay scale** — the same code runs on 17 regions or 42k barangays.

## 10. Performance / Scaling

Barangay LOD (~42k units) makes naive all-pairs widest-path infeasible (an N² access matrix is
~1.8B entries). The mandatory **hierarchical** approach — connectivity within a parent, reduced to
a gateway node, composed up the tree — is specified in
[`connectivity-hierarchy.md`](connectivity-hierarchy.md), including the per-tick cost budget and
incremental-recompute strategy. For coarse LODs (regions) a flat widest-path is fine; the same API
serves both.

## 11. GDExtension Surface

Unit dict gains `market_access`. New `#[func] get_edges() -> Array<VarDictionary>`
(`{a, b, road_tier, rail_level, capacity, is_bottleneck}`) for the map's network overlay and
bottleneck highlighting.

## 12. Test Plan

- Bottleneck: on a 3-node chain A–B–C, lowering the A–B tier caps C's access regardless of B–C
  (the weakest-link invariant).
- Distance decay: equal-bottleneck paths favor the nearer market.
- Rail beats the best road tier (a rail segment raises bottleneck/capacity above highway).
- `market_access ∈ [0,1]`; pop-weighted rollup via `aggregate_up`.
- Hierarchical result matches a flat widest-path computation on a small multi-level fixture (the
  hierarchy spec owns the scale tests).
