# Tech Spec: Economy & Treasury

- **Maps to:** `rust/sim/src/systems/economy.rs` — **Tick slot:** 6 — **Status:** Built

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Roll revenue up to the nation, collect tax, pay expenses, accrue debt interest, flag bankruptcy.

| Reads | Writes |
|-------|--------|
| aggregated root `revenue_last_tick`, `Nation.tax_rate`, all units' industry levels + infra/gateway levels (for expense) | `Nation.treasury`, `Nation.bankrupt` |

## 2. Data Fields

Reads the aggregate root (after `aggregate_up`) and per-leaf levels for expenses. Writes
`Nation.treasury` (f64, PHP, may be negative) and `Nation.bankrupt` (bool).

## 3. Tuning Constants

| Const | Default | Meaning |
|-------|---------|---------|
| `maintenance_per_industry_level` | 0.3e9 | Monthly upkeep per industry level (PHP) |
| `upkeep_per_transport_level` | 0.15e9 | Monthly upkeep per transport level (PHP) |
| `debt_interest_rate` | 0.05 | Monthly interest on negative treasury |
| `debt_ceiling` | 5.0e12 | Debt magnitude that triggers bankruptcy |
| **v1 add** `port_upkeep`, `airport_upkeep` | TBD | Monthly upkeep per gateway level |

## 4. Formulas

```
aggregate_up(units)                       # produces root.revenue_last_tick (trade-adjusted in v1)
national_revenue = root.revenue_last_tick
national_expense = Σ_leaf ( industry_level × maintenance_per_industry_level
                          + transport_level × upkeep_per_transport_level
                          + port_level × port_upkeep + airport_level × airport_upkeep )  # gateways v1
treasury += national_revenue × tax_rate − national_expense
if treasury < 0:  treasury −= (−treasury) × debt_interest_rate
bankrupt = treasury < −debt_ceiling
```

## 5. Function Signatures

```rust
pub fn national_expense(units: &[AdminUnit], t: &Tuning) -> f64
pub fn run(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning)
```

## 6. Tick Placement

After trade (so the revenue it aggregates already includes external trade income). Before
infrastructure/education/politics. It owns the (only) explicit `aggregate_up` call needed for
revenue plus the gateway-upkeep sum.

## 7. Reads / Writes Contract

Relies on industries (slot 4) + trade (slot 5) having finalized `revenue_last_tick` per leaf.
Writes national treasury/bankrupt consumed by the game-over check and the dashboard.

## 8. Aggregation Impact

Calls `aggregate_up`. In v1 the fold is extended (per conventions) so the root revenue includes
trade contributions; this system reads only the root total, so the extension is transparent here.

## 9. Structural-Rule Compliance

Expense iterates `is_operational()` leaves; revenue comes from the rollup, never a region count.
Compliant.

## 10. Extension Seams

Tax collection is the interception point for the Layer-D corruption skim. Debt logic is the seam
for a future bond-market / IMF system (variable rates, conditions).

## 11. GDExtension Surface

`get_treasury()`, `get_tax_rate()`, `set_tax_rate()` exist. `advance_tick()` returns `bankrupt`.
No new `#[func]` needed for v1 economy core.

## 12. Test Plan

- Debt accrues interest when negative (existing `debt_accrues_interest`).
- Bankruptcy flips past the ceiling (existing `bankrupt_flips_past_the_ceiling`).
- Higher tax collects more (existing `revenue_collected_scales_with_tax_rate`).
- Integration: deep debt + zero tax triggers bankruptcy within 3 years (existing
  `deep_debt_eventually_triggers_bankruptcy`).
