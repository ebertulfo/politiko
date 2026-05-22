# Tech Spec: Population

- **Maps to:** `rust/sim/src/systems/population.rs` — **Tick slot:** 1 — **Status:** Built

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Grow each operational unit's population by an education-/poverty-driven monthly rate.

| Reads | Writes |
|-------|--------|
| `AdminUnit.education_level`, `AdminUnit.poverty` (from previous tick) | `AdminUnit.population` |

## 2. Data Fields

- Reads authored `education_level` (f32, 0–1) and derived `poverty` (f32, 0–1, written by
  industries last tick — the documented one-tick lag).
- Writes authored `population` (u32). Operational (leaf) units only; aggregates are skipped
  (`is_operational()`).

## 3. Tuning Constants

| Const | Default | Meaning |
|-------|---------|---------|
| `base_growth` | 0.015 | Base monthly growth rate |
| `edu_growth_damp` | 0.01 | Education's damping coefficient |
| `poverty_growth_boost` | 0.008 | Poverty's boosting coefficient |

## 4. Formulas

```
GrowthRate = base_growth − education_level × edu_growth_damp + poverty × poverty_growth_boost
population = round(population × (1 + GrowthRate)), floored at 0
```

## 5. Function Signatures

```rust
pub fn growth_rate(education_level: f32, poverty: f32, t: &Tuning) -> f32
pub fn run(units: &mut [AdminUnit], t: &Tuning)
```

## 6. Tick Placement

Runs **first** so it reads the prior tick's economic outcome (poverty/wages). This is the only
backward dependency in the system graph and is intentional (see conventions: one-tick lag).

## 7. Reads / Writes Contract

`poverty` is consumed as a *lagged* input; it is recomputed later this tick by industries. On a
fresh sim, `prime()` populates `poverty` first so tick 1 is not degenerate.

## 8. Aggregation Impact

None directly. `population` is summed up the tree by `aggregate_up` (existing behavior).

## 9. Structural-Rule Compliance

Iterates all units, acts only on `is_operational()` leaves; no level branching. Level-agnostic.

## 10. Extension Seams

Migration/OFW (Layer D) inserts after this system to move population between units; this system's
growth model is unaffected.

## 11. GDExtension Surface

Population surfaced via `get_unit`/`get_root` dict key `population`. No new `#[func]` needed.

## 12. Test Plan

- `growth_rate`: poverty/uneducated grows faster than educated/rich (existing
  `education_lowers_growth_poverty_raises_it`).
- Exact-formula check at known inputs (existing `matches_brief_formula`).
- Integration: population stays > 0 for all units across a full term (existing in
  `tests/integration.rs::a_full_term_keeps_state_in_valid_ranges`).
