# Tech Spec: Trade & Global Economy

- **Maps to:** new `rust/sim/src/systems/global_demand.rs` (slot 3) + `rust/sim/src/systems/trade.rs` (slot 5); shared `network.rs` — **Status:** Spec'd-v1

Conventions: [`00-conventions.md`](00-conventions.md). Depends on Connectivity (`market_access`,
the `Network` for gateway reach) and typed gateways (Infrastructure spec). v1 channels = **exports
+ imports only** (tourism-arrivals and FDI are Layer D).

## 1. Responsibility

Two halves around industries:
- **`global_demand` (slot 3):** advance the exogenous `World` along its **scripted-event timeline**;
  project each unit's `export_price_mult`.
- **`trade` (slot 5):** turn produced tradable output + gateway reach into `exports_value` and
  `imports_value`, fold the net external uplift into `external_revenue` → `revenue_last_tick`, and
  apply the **specialization** effect.

| System | Reads | Writes |
|--------|-------|--------|
| global_demand | `World`, `Nation.tick`, `industries[].kind` | `World` (event-driven), `export_price_mult` |
| trade | `revenue_last_tick`, `industries[]`, gateway levels, `Network`, `market_access`, `World.*` | `exports_value`, `imports_value`, `external_revenue`; `revenue_last_tick += external_revenue` |

## 2. Data Fields

**New on `Sim` (`engine.rs`):**
```rust
pub struct WorldPrices { pub agriculture: f32, pub manufacturing: f32, pub mining: f32 } // export premia
pub struct ScriptedEvent {
    pub at_tick: u32,
    pub kind: WorldEventKind,   // CommodityBoom{good}, CommodityBust{good}, ImportCostSpike, ...
    pub magnitude: f32,
    pub duration: u32,
}
pub struct World {
    pub demand_index: f32,           // global appetite, baseline ~1.0
    pub price: WorldPrices,          // current per-tradable export premium
    pub events: Vec<ScriptedEvent>,  // authored timeline (data-seeded)
}
pub struct Sim { /* ... */ pub world: World }
```
**New derived on `AdminUnit` (`types.rs`):** `export_price_mult: f32`, `exports_value: f64`,
`imports_value: f64`, `external_revenue: f64` (all default 0, recomputed each tick).
**Removed vs the earlier draft:** no `tourism_arrivals`, no `tourism_potential`, no `fdi` (Layer D).

## 3. Tuning Constants (`constants.rs::Tuning`)

| Const | Meaning |
|-------|---------|
| `export_tradable_share` | Fraction of tradable output that seeks export |
| `gateway_throughput_per_level` | Export/import capacity per gateway level |
| `import_input_bonus` | Manufacturing revenue uplift per unit of imported input |
| `specialization_strength` | How strongly high access+imports boosts the dominant industry |
| `specialization_access_threshold` | Market-access level at which specialization unlocks |
| `world_baseline_prices`, `world_event_*` | Baseline tradable premia + scripted-event shaping |

## 4. Formulas

```
# --- global_demand (slot 3) ---
apply active ScriptedEvents at Nation.tick → adjust World.price[good] / demand_index (clamped)
export_price_mult(i) = Σ_tradable( output_share(kind, i) × World.price[kind] ) × World.demand_index

# --- trade (slot 5) ---
# Gateway reach: capacity available to i via the bottlenecked network to any gateway
gateway_reach(i) = Σ_j path_capacity(i, j) × (port_level[j] + airport_level[j]) × gateway_throughput_per_level

tradable_output(i) = Σ output(Ag, Mfg, Mining at i)
exportable(i)      = min( tradable_output(i) × export_tradable_share , gateway_reach(i) )
exports_value(i)   = exportable(i)                                  # gross exported (readout)
export_uplift(i)   = exportable(i) × export_price_mult(i)           # premium over domestic sale

imports_value(i)   = min( mfg_output(i) × import_input_bonus , gateway_reach(i)-derived cap )

# Specialization: high access + import access lets i over-weight its strongest industry
if market_access(i) ≥ specialization_access_threshold and gateway_reach(i) > 0:
    specialization_uplift(i) = dominant_industry_output(i) × specialization_strength
                               × f(market_access(i))                # bounded
else specialization_uplift(i) = 0

external_revenue(i)   = export_uplift(i) + imports_value(i) + specialization_uplift(i)
revenue_last_tick(i) += external_revenue(i)
```

Domestic output is already counted by industries (slot 4); trade adds only the **external uplift**
(export premium, import bonus, specialization), so revenue is never double-counted.

## 5. Function Signatures

```rust
// global_demand.rs
pub fn run(units: &mut [AdminUnit], world: &mut World, nation: &Nation, t: &Tuning)
// trade.rs
pub fn run(units: &mut [AdminUnit], world: &World, net: &Network, t: &Tuning)
```

## 6. Tick Placement

`global_demand` slot 3 (sets prices/`export_price_mult` before industries). `trade` slot 5 (after
industries produce output, before economy aggregates). It mutates `revenue_last_tick` in place.

## 7. Reads / Writes Contract

`trade` requires industries (slot 4) outputs, connectivity (slot 2) `Network`/`market_access`, and
global_demand (slot 3) prices. Specialization reads `market_access` from connectivity.

## 8. Aggregation Impact

Extends the single `aggregate_up` fold (per [`00-conventions.md`](00-conventions.md)) to **sum**
`exports_value`, `imports_value`, `external_revenue` up the tree. (Tourism/FDI are NOT in the fold
in v1.)

## 9. Structural-Rule Compliance

Gateway reach uses the level-agnostic `Network`; ports gated by `coastal` at build time. `World` is
national-scope external state on `Sim`. No level counting.

## 10. Extension Seams

`World` + `events` are built to be **promoted** to the Layer-D world-market system (replace scripted
events with endogenous supply/demand cycles — nothing else changes). Tourism-arrivals and FDI slot
in as additional channels + derived fields without touching exports/imports. Tariffs would intercept
`export_uplift`/`imports_value`.

## 11. GDExtension Surface

Unit dict gains `exports_value`, `imports_value`, `external_revenue`, `export_price_mult`. New
`#[func] get_world() -> VarDictionary` (`{demand_index, prices, active_events}`). Gateway build
actions live in the Infrastructure spec.

## 12. Test Plan

- No gateways anywhere → `exports_value`/`imports_value`/`external_revenue` = 0 and
  `revenue_last_tick` equals the pure-domestic value (no double count).
- A port in a connected coastal tradable region raises its `external_revenue` and the treasury; a
  **choked hinterland** (bottleneck) reduces `gateway_reach` and thus exports — proves the
  Connectivity dependency.
- A landlocked region exports only when its network path to a gateway is bottleneck-free enough.
- Specialization unlocks only above the access threshold with import access; bounded uplift.
- A scripted `CommodityBoom`/`Bust` moves `export_price_mult` for the affected good for its duration.
