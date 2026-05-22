# Tech Spec: Hierarchical Connectivity (scaling the bottleneck model to barangay LOD)

- **Maps to:** `rust/sim/src/network.rs` — **Supports:** [`connectivity.md`](connectivity.md) — **Status:** Spec'd-v1

Conventions: [`00-conventions.md`](00-conventions.md). This is the load-bearing algorithm for v1's
barangay LOD (~42,000 units). It is the hardest piece; expect a prototype + benchmark before the
rest of Connectivity is finalized.

## 1. The problem

`market_access(i) = Σ_j population[j] × path_capacity(i, j)` with widest-path (max-min bottleneck)
capacity. Flat, this is all-pairs widest-path over ~42k nodes — infeasible (an N² access matrix
alone is ~1.8 billion `f32` ≈ 7 GB; all-pairs compute is far worse). We exploit the **existing
`parent` tree** (nation → region → province → municipality → barangay) to make it near-linear.

## 2. Key idea — decompose by the tree, route through portals

A real trade route between two barangays in different municipalities physically crosses municipal
borders at specific points. We mirror that:

- An **inter-parent edge** (an `Edge` whose endpoints have different parents) is a **portal** of
  both subtrees.
- A path `i → j` across the hierarchy is **composed of legs**:
  `i → (portal of i's subtree) → … coarse hops between subtrees … → (portal of j's subtree) → j`.
- Its **bottleneck = the min over all legs**; its **distance = the sum over legs**. This is exactly
  the per-segment `min` rule from [`connectivity.md`](connectivity.md), applied across levels.

## 3. The algorithm

Bottom-up build, then population recombination.

**(a) Within-parent solve (every internal node of the tree).** For each parent `P` with children
`C`, build the subgraph of `C` plus the intra-`P` edges. Compute, with widest-path:
- `internal_access(c)` = capacity-weighted reachable mass of the *other children of P* (the local
  market a child can reach without leaving `P`).
- `portal_cap(c, k)` = the bottleneck capacity (and distance) from child `c` to each **portal** `k`
  of `P` (the border crossings into siblings of `P` one level up).

**(b) Reduce each subtree to a super-node.** `P` becomes a coarse node carrying: its total
`population` (already summed by `aggregate_up`), and its **portal profile** (`portal_cap` from its
internal mass to each of its own portals). This is what the next level up sees.

**(c) Coarse solve up the tree.** Repeat (a)/(b) at each higher level, where the "children" are the
super-nodes and the edges are the inter-parent portals. Each coarse graph is small (bounded by the
branching factor — a province has tens of municipalities, etc.).

**(d) Compose leaf market access.** For leaf `i` in subtree `P`:
```
market_access(i) = internal_access(i)                                   # same-parent market
                 + Σ over external subtrees S of
                     min( portal_cap(i → P's portal toward S),          # leg out of P
                          coarse_path_cap(P → S) )                       # hops between subtrees
                   × population[S] × distance_decay(...)
```
External markets are aggregated **at the coarsest level where they diverge from `i`** — so a leaf's
access to a far province is computed once against that province's *total* population, never
barangay-by-barangay. This is what removes the N² blowup.

## 4. Complexity & per-tick budget

- **Build cost** (on infrastructure/adjacency change): `Σ_levels Σ_parents O(b² )` where `b` =
  children per parent. With modest branching (tens), total work is ~`O(N · b)` — near-linear in the
  42k units, done **only when the player builds** (not every tick).
- **Per-tick cost:** populations change via growth, but the bottleneck structure does not. Each tick
  we recombine cached capacities with current (aggregated) populations: `O(N)` over leaves plus the
  cheap coarse sums (`aggregate_up` already produces subtree populations). This is the only
  per-tick connectivity cost and is well within budget at 42k.
- **Memory:** portal profiles are `O(N · portals_per_node)`, not `O(N²)`. Bounded and small.

## 5. Incremental recompute

A build action changes one segment. Only its parent's within-parent solve and the coarse solves on
the **path from that parent to the root** need recomputation — `O(depth × b²)`. Mark dirty subtrees
on build; rebuild lazily before the next tick. Full rebuilds only on load.

## 6. Accepted approximation (and the mitigation)

Hierarchical routing is exact **iff** the optimal path's border crossings are the modeled portals.
Error appears only when the true widest path weaves across hierarchy boundaries in a way the portal
set doesn't capture (e.g., a major national corridor that bypasses local routing). Mitigation:
allow **explicit shortcut portals** — promote real long-haul links (a trunk highway, a rail spine)
to higher-level edges so they're first-class in the coarse graph. The data pipeline
([`../data-pipeline.md`](../data-pipeline.md)) tags such corridors. Remaining error is bounded and
acceptable for a game; validated against flat widest-path on medium fixtures.

## 7. Data structures (`network.rs`)

```rust
pub struct Network {
    // Per internal node: cached within-parent access + portal profiles + coarse graph.
    // Keyed by unit id; dirty-flagged subtrees for incremental rebuild.
}
pub fn build_network(units: &[AdminUnit], adjacency: &[Edge], t: &Tuning) -> Network
pub fn mark_dirty(net: &mut Network, changed_unit: u32)               // called by build actions
pub fn rebuild_dirty(net: &mut Network, units: &[AdminUnit], adjacency: &[Edge], t: &Tuning)
pub fn recombine_market_access(net: &Network, units: &mut [AdminUnit], t: &Tuning) // per-tick, O(N)
pub fn path_capacity(net: &Network, from: u32, to: u32) -> f32        // on-demand (UI, trade reach)
```
`connectivity::run` calls `rebuild_dirty` (if needed) then `recombine_market_access`.

## 8. Structural-rule compliance

Pure tree-shape logic: it uses `parent`/`is_aggregate`, never `AdminLevel` or a region count. It
therefore runs unchanged at any LOD — the same code that's correct on 17 regions is the code that
scales to 42k barangays.

## 9. Test plan

- **Equivalence:** on small + medium multi-level fixtures, hierarchical `market_access` matches a
  brute-force flat widest-path within the documented approximation tolerance.
- **Bottleneck across levels:** a weak portal between two provinces caps cross-province access for
  all barangays behind it.
- **Incremental == full:** `mark_dirty`+`rebuild_dirty` after a build yields the same result as a
  full rebuild.
- **Scale/perf:** synthetic 42k-unit tree meets a stated per-tick budget (e.g. recombination well
  under a millisecond-class target) and a bounded build time; benchmark gates the design.
