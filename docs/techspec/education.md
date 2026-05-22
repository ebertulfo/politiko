# Tech Spec: Education

- **Maps to:** `rust/sim/src/systems/education.rs` — **Tick slot:** 8 — **Status:** Built

Conventions: [`00-conventions.md`](00-conventions.md).

## 1. Responsibility

Slowly converge each operational unit's `education_level` toward the national investment lever.

| Reads | Writes |
|-------|--------|
| `Nation.education_investment`, `AdminUnit.education_level` | `AdminUnit.education_level` |

## 2. Data Fields

Reads `Nation.education_investment` (f32, 0–1, player lever) and current `education_level`.
Writes authored `education_level` (f32, 0–1). Leaf units only.

## 3. Tuning Constants

| Const | Default | Meaning |
|-------|---------|---------|
| `edu_gain` | 0.02 | Fraction of the gap to the target closed per tick (slow) |

## 4. Formulas

```
delta = (education_investment − education_level) × edu_gain
education_level = clamp(education_level + delta, 0, 1)
```
Geometric convergence: ~`1/edu_gain` ≈ 50 ticks (~4 years) to substantially close the gap.

## 5. Function Signatures

```rust
pub fn run(units: &mut [AdminUnit], nation: &Nation, t: &Tuning)
```

## 6. Tick Placement

Runs **late** (slot 8): nothing downstream this tick depends on the new education value, so it
is safe to lag by one tick. Its effects are felt next tick by industries and politics.

## 7. Reads / Writes Contract

Pure convergence; reads only the national lever and its own prior value. No cross-system inputs.

## 8. Aggregation Impact

`education_level` is population-weighted averaged up the tree by `aggregate_up` (existing).

## 9. Structural-Rule Compliance

Leaf-only, single national lever, no level branching. Level-agnostic.

## 10. Extension Seams

Per-region education spending or school infrastructure (Layer D) would add inputs here; the
convergence model is the floor those would build on.

## 11. GDExtension Surface

`set_education_investment(level)` exists; `education_level` exposed via the unit dict. No new
`#[func]` needed.

## 12. Test Plan

- Moves toward the target but by no more than `edu_gain × gap` per tick (existing
  `education_moves_toward_investment_but_slowly`).
- Converges to the target over many ticks (existing `converges_to_target_over_many_ticks`).
