# Tech Spec: Politics & Approval

- **Maps to:** `rust/sim/src/systems/politics.rs` — **Tick slot:** 9 — **Status:** Built

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Compute per-unit satisfaction, roll it up to national approval, and fire elections on term
boundaries.

| Reads | Writes |
|-------|--------|
| `poverty`, `employment_rate`, `education_level`, `infrastructure.transport_level`, (`market_access` optional v1), `Nation.tick`/`term_length` | `AdminUnit.satisfaction`, `Nation.approval`, returns `Option<ElectionResult>` |

## 2. Data Fields

Reads derived `poverty`/`employment_rate`, authored `education_level`/`transport_level` (and
optional v1 `market_access`). Writes derived `satisfaction` (f32, 0–1) and `Nation.approval`
(f32, 0–1). Returns `ElectionResult { tick, approval, voteshare, won }` on election ticks.

## 3. Tuning Constants

| Const | Default | Meaning |
|-------|---------|---------|
| `w_poverty` | 0.35 | Satisfaction weight: low poverty |
| `w_employment` | 0.25 | Satisfaction weight: employment |
| `w_education` | 0.20 | Satisfaction weight: education |
| `w_infrastructure` | 0.20 | Satisfaction weight: infrastructure service |
| `target_transport` | 6.0 | Transport level counted as "fully served" |
| `term_length` | 72 | Months per term (also on `Nation`) |
| **v1 add (optional)** `w_connectivity` | TBD | Satisfaction weight for market access; rebalance the others to sum 1 |

## 4. Formulas

```
infra_score  = min(1, transport_level / target_transport)
satisfaction = clamp( w_poverty×(1−poverty) + w_employment×employment_rate
                    + w_education×education_level + w_infrastructure×infra_score
                    [+ w_connectivity×min(1, market_access)] , 0, 1)

aggregate_up(units)                  # pop-weighted average of satisfaction → root
approval  = root.satisfaction
voteshare = clamp(approval, 0, 1)    # 1:1 in v1; seam for incumbency curves
election fires when tick > 0 && tick % term_length == 0;  won = voteshare > 0.5
```
Weights must sum to 1.0; if `w_connectivity` is added, reduce the others accordingly.

## 5. Function Signatures

```rust
pub fn satisfaction(u: &AdminUnit, t: &Tuning) -> f32
pub fn compute_satisfaction(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning)
pub fn voteshare(approval: f32) -> f32
pub fn run(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning) -> Option<ElectionResult>
```

## 6. Tick Placement

Runs **last** — it reads the outputs of every other system. `compute_satisfaction` is also
called at `prime()` (no election) so a fresh sim has a valid approval before tick 1.

## 7. Reads / Writes Contract

All inputs are produced earlier this tick (poverty/employment by industries slot 4, education
by education slot 8, market_access by connectivity slot 2). No lag.

## 8. Aggregation Impact

Calls `aggregate_up` to pop-weight satisfaction into the root (existing). Reads root only.

## 9. Structural-Rule Compliance

Per-leaf satisfaction; approval via rollup, never a region count. Election keyed on
`tick % term_length`, never a hardcoded 72 in logic (uses `Nation.term_length`). Compliant.

## 10. Extension Seams

`voteshare()` is the explicit seam for incumbency/turnout curves. The satisfaction blend is the
hook for Layer-D unrest (sustained low satisfaction → unrest stock) and tax-approval penalties.
`Nation.political_capital` is reserved for the corruption system.

## 11. GDExtension Surface

`get_approval()` exists; `advance_tick()` returns `has_election`/`won`/`approval`/`voteshare`;
`satisfaction` exposed via the unit dict. No new `#[func]` for v1 core.

## 12. Test Plan

- Approval equals the pop-weighted average of satisfaction (existing
  `approval_is_population_weighted_average_of_satisfaction`).
- Election only fires on term boundaries; `won` matches `voteshare > 0.5` (existing
  `election_only_fires_on_term_boundaries`).
- Integration: exactly one election in 72 ticks; investment beats neglect on approval (existing
  `election_fires_exactly_at_tick_72`, `the_closed_loop_pays_off_investment`).
