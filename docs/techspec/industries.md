# Tech Spec: Industries

- **Maps to:** `rust/sim/src/systems/industries.rs` — **Tick slot:** 4 — **Status:** Built (Layer A); B/C extensions Spec'd-v1

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Compute each operational unit's output/revenue, employment, wages, and resulting poverty.

| Reads | Writes |
|-------|--------|
| `population`, `education_level`, `industries[].level`, `infrastructure.transport_level` | `revenue_last_tick`, `employment_rate`, `poverty`, `daily_wage` |
| **v1 additions:** `market_access`, `export_price_mult` | (same) |

## 2. Data Fields

Reads authored population/education/industry levels/transport. Writes derived
`revenue_last_tick` (f64, PHP/mo), `employment_rate` (f32, 0–1), `poverty` (f32, 0–1),
`daily_wage` (f64, PHP/day). Leaf units only.

## 3. Tuning Constants

| Const | Default | Meaning |
|-------|---------|---------|
| `working_age_share` | 0.6 | Labor-force fraction of population |
| `labor_per_level` | 180,000 | Workers demanded per industry level |
| `labor_share` | 0.45 | Fraction of revenue paid as wages |
| `cost_of_living` | 9,000 | Monthly per-capita living cost (PHP) |
| `energy_available` | 1.0 | Energy multiplier (constant in v1) |
| `transport_efficiency_bonus` | 0.04 | Efficiency gain per transport level |
| `base_output(kind)` | Ag 1.6e9 / Mining 3.2e9 / Tourism 2.2e9 / Mfg 4.2e9 / Svc 5.5e9 | PHP/mo/level at full efficiency |
| `required_education(kind)` | Ag 0.20 / Mining 0.30 / Tourism 0.40 / Mfg 0.60 / Svc 0.70 | Education for full efficiency |
| **v1 add** `market_access_weight` | TBD | Strength of market-access revenue multiplier |

## 4. Formulas

```
workforce      = population × working_age_share
labor_demand   = Σ level × labor_per_level
labor_factor   = min(1, workforce / labor_demand)
employed       = min(labor_demand, workforce)
employment_rate= min(1, employed / workforce)

efficiency(ind) = min(1, education_level / required_education(kind))
                  × energy_available
                  × (1 + transport_level × transport_efficiency_bonus)
                  × labor_factor                       # efficiency itself stays uncapped only via transport
output(ind)     = level × base_output(kind) × efficiency(ind)
revenue_last_tick = Σ output(ind)

daily_wage = (revenue_last_tick × labor_share / employed) / 30
poverty    = clamp(1 − daily_wage / (cost_of_living / 30), 0, 1)
```

**v1 extension (keep separate from the capped `min(1, edu/req)` term so existing efficiency
tests hold):** apply a revenue-side market-access multiplier
`revenue_last_tick ×= (1 + market_access × market_access_weight)`. Industries here produce
*gross domestic* output. The **trade** system (slot 5) then adds the external uplift —
`export_price_mult` on the tradable share, the import-input bonus to Manufacturing, and the
**specialization** uplift (which reads this system's per-industry outputs to find the dominant
industry). None of that is applied here, so revenue is never double-counted.

## 5. Function Signatures

```rust
pub fn industry_efficiency(ind: &IndustryInstance, education_level: f32,
                           transport_level: u32, labor_factor: f64, t: &Tuning) -> f64
pub fn run(units: &mut [AdminUnit], t: &Tuning)
```
v1 extension: `run` gains read access to `market_access`/`export_price_mult` (already on the unit
by slot 4).

## 6. Tick Placement

After population (needs fresh population for labor) and after connectivity/global_demand (needs
`market_access`/`export_price_mult`). Before economy (produces the revenue it aggregates).

## 7. Reads / Writes Contract

Writes `poverty`/`daily_wage` that population reads next tick (lag). `market_access` must be
written by connectivity (slot 2) and `export_price_mult` by global_demand (slot 3) earlier the
same tick.

## 8. Aggregation Impact

`revenue_last_tick` is summed by `aggregate_up` (existing). The v1 trade fields it interacts with
are rolled up per the conventions extension (handled in the trade spec).

## 9. Structural-Rule Compliance

Leaf-only, no level branching. Topology enters indirectly via transport (built on topology cost)
and, in v1, via market_access (edge friction). Compliant.

## 10. Extension Seams

`energy_available` is the per-unit hook for the Layer-D energy system (currently the constant
`Tuning::energy_available`). The market-access/export multipliers are the B/C hooks, added as
separate multipliers to preserve the capped-efficiency contract.

## 11. GDExtension Surface

`get_unit` dict already exposes `revenue_last_tick`, `poverty`, `employment_rate`, `daily_wage`,
`industries[]`. No new `#[func]`; v1 adds derived keys (market_access etc.) via the unit dict.

## 12. Test Plan

- `industry_efficiency`: rises with education up to the requirement, then caps (existing
  `education_raises_efficiency_up_to_the_requirement`).
- Transport raises revenue (existing `transport_increases_revenue`).
- Poverty/employment bounded 0–1 across all units (existing `poverty_is_bounded_and_falls_with_income`).
- v1: market_access multiplier increases revenue monotonically; export pricing handled in trade tests.
