# Tech Spec: Infrastructure

- **Maps to:** `rust/sim/src/systems/infrastructure.rs` + build actions in `rust/sim/src/engine.rs` — **Tick slot:** 7 — **Status:** Built (basic transport) + Spec'd-v1 (tiers, rail, gateways)

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Own the topology-coupled **build cost** model and the player **build actions** for all
infrastructure: node internal transport, link road tiers, freight rail, and gateways. The per-tick
`run` is an intentional no-op slot in v1 (reserved for decay); upkeep is charged in economy.

| Reads | Writes |
|-------|--------|
| topology (`terrain_ruggedness`, `coastal`); treasury for affordability | (build actions) node `transport_level`, edge `road_tier`/`rail_level`, `port_level`/`airport_level`, `Nation.treasury` |

## 2. Data Fields

**Node (`InfrastructureState` in `types.rs`):**
```rust
pub struct InfrastructureState {
    pub transport_level: u32,   // internal pass-through (Layer A efficiency + connectivity node capacity)
    pub port_level: u32,        // gateway; requires coastal
    pub airport_level: u32,     // gateway; anywhere
}
```
**Link (`Edge` on `Sim::adjacency`, see Connectivity spec):** `road_tier: u8` (0..3),
`rail_level: u8`. Both authored, raised only by build actions.

## 3. Tuning Constants (`constants.rs::Tuning`)

| Const | Default / Meaning |
|-------|-------------------|
| `k_rugged` | 2.5 — ruggedness multiplier on build cost |
| `coastal_discount` | 0.8 — coastal cost factor |
| `base_transport_cost` | 5.0e9 — base PHP per internal transport level (existing) |
| `base_road_tier_cost: [f64; 4]` | base PHP to raise a link to each road tier |
| `base_rail_cost` | base PHP per rail level (high — the game-changer) |
| `base_port_cost`, `base_airport_cost` | base PHP per gateway level (major investment) |

## 4. Formulas

```
# Node / gateway builds use the unit's topology:
build_cost(unit, kind) = base_cost(kind) × (1 + terrain_ruggedness × k_rugged)
                                          × (coastal ? coastal_discount : 1)

# Link builds (road tier / rail) use the EDGE's topology (avg/max of endpoints + length):
edge_build_cost(edge, kind) = base_cost(kind) × (1 + edge_ruggedness × k_rugged)
                                              × (edge_is_coastal ? coastal_discount : 1)
                                              × length_factor(edge.length_km)
```
`edge_ruggedness`/`edge_is_coastal` derived from the edge's endpoint units (data pipeline supplies
the edge's own topology where available).

## 5. Function Signatures

```rust
// systems/infrastructure.rs
pub enum BuildKind { Transport, Port, Airport }     // node builds
pub enum LinkBuildKind { RoadTier, Rail }           // edge builds
pub fn build_cost(u: &AdminUnit, kind: BuildKind, t: &Tuning) -> f64
pub fn edge_build_cost(e: &Edge, units: &[AdminUnit], kind: LinkBuildKind, t: &Tuning) -> f64
pub fn run(_units: &mut [AdminUnit], _t: &Tuning) {}  // no-op slot

// engine.rs (Sim methods, out-of-tick)
pub fn build_node(&mut self, unit_id: u32, kind: BuildKind) -> Result<f64, BuildError>
pub fn build_link(&mut self, edge: (u32,u32), kind: LinkBuildKind) -> Result<f64, BuildError>
```
`BuildError` gains `RequiresCoastal` (Port on non-coastal) and `UnknownEdge`. The existing
`build_transport` becomes `build_node(id, BuildKind::Transport)` (or is kept as a thin alias for
back-compat with current tests).

## 6. Tick Placement

`run` at slot 7 (after economy charges upkeep). Builds are out-of-tick player actions; on success
they also `network::mark_dirty` the affected subtree so connectivity rebuilds lazily.

## 7. Reads / Writes Contract

Build actions check affordability (and coastal for ports), decrement treasury, raise the level,
and mark the connectivity `Network` dirty. Effects appear next tick via systems that read the
levels (connectivity, industries, trade, economy upkeep).

## 8. Aggregation Impact

None — infrastructure levels are authored leaf/edge fields, not aggregated.

## 9. Structural-Rule Compliance

All costs topology-driven (rule #2). Builds reject aggregates / unknown ids / non-coastal ports.
Edges connect ids regardless of level. Level-agnostic.

## 10. Extension Seams

`run` is the reserved slot for **per-tick decay** (Layer D). `InfrastructureState` is where
`flood_control`/`energy` (Layer D) get added. `Edge` can gain capacity/congestion attributes. The
cost model generalizes to any future build kind.

## 11. GDExtension Surface

`build_node(id, kind)`, `build_link([a,b], kind)`, `node_build_cost(id, kind)`,
`edge_build_cost([a,b], kind)`, each returning `{ok, cost, reason}` or a cost. Unit dict gains
`port_level`, `airport_level`; edge dict (`get_edges`) carries `road_tier`, `rail_level`.

## 12. Test Plan

- Topology cost: rugged inland > flat coastal (existing transport test generalizes to all kinds).
- Coastal discount applies; exact-constant checks per kind.
- `build_node`/`build_link`: pays cost, raises the right field, rejects when broke / on aggregates /
  unknown id-or-edge / Port-on-non-coastal (`RequiresCoastal`).
- A successful build marks the connectivity network dirty (observable via recomputed `market_access`).
