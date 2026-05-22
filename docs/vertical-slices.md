# Vertical Slices — the path to "usable"

A build plan in the **tracer-bullet** style (Matt Pocock's `to-issues` skill): each slice is a thin
path through *every* layer — `politiko_sim` (Rust) → `politiko_gdext` (binding) → Godot scene/GDScript
→ verifiable — rather than a horizontal layer built in isolation. Prefer many thin slices over few
thick ones; each completed slice is demoable on its own.

## The key insight (2026-05-22)

We have been building **horizontally**: six engine systems + 30 tests, and a binding that already
exposes the whole core loop — but **nothing on screen**. The gdext surface (`rust/gdext/src/lib.rs`)
already marshals: `advance_tick`, `get_tick`/`get_treasury`/`get_approval`/`get_tax_rate`/
`get_term_length`, `set_tax_rate`/`set_education_investment`, `operational_unit_count`/
`get_all_units`/`get_unit`/`get_root`, `transport_cost`/`build_transport`, and `to_json`/`load_json`.

**Consequence:** the slices below are almost entirely **Godot UI work**. The engine rarely needs to
change. "Playable sandbox" is ~6 thin slices away, not a major lift. This concretizes roadmap Steps
4–6 (map UI, dashboard/loop, save UI).

`HITL` = needs a human decision (design/layout). `AFK` = an agent can implement + verify alone.
Lead agent: **godot-ui** unless noted; sim work goes to **sim-engine**.

---

## Slice 1 — National dashboard + advance the clock
**Type:** HITL (first-ever layout decision), then AFK · **Blocked by:** None — start immediately.

### What to build
A main scene that instantiates `PolitikoEngine`, shows the national readout (tick, treasury,
approval, population from `get_root`), and an **Advance Month** button that calls `advance_tick()`
and refreshes the readout. This is the minimum *usable* artifact: a running nation.

### Acceptance criteria
- [ ] Launching the project shows tick-0 values pulled from the engine (not hardcoded).
- [ ] Clicking "Advance Month" advances the tick; treasury and approval visibly change.
- [ ] Population shown is the aggregated root value (`get_root().population`).
- [ ] Engine support already exists — no Rust change expected.

---

## Slice 2 — Region table
**Type:** AFK · **Blocked by:** Slice 1.

### What to build
A table/`ItemList` of the 17 operational (leaf) regions — name, population, `revenue_last_tick`,
`satisfaction` — filtered to `is_aggregate == false`, refreshed on each tick.

### Acceptance criteria
- [ ] Exactly the operational regions are listed (aggregate root excluded).
- [ ] Values refresh after each Advance Month.
- [ ] Data comes from `get_all_units()`; count matches `operational_unit_count()`.

---

## Slice 3a — Tax lever (UI)
**Type:** AFK · **Blocked by:** Slice 1.

### What to build
A slider (0–1) bound to `get_tax_rate`/`set_tax_rate`, with a label showing the current rate.
Advancing reflects the treasury response.

### Acceptance criteria
- [ ] Moving the slider calls `set_tax_rate`; the readout matches.
- [ ] Higher tax collects more treasury per tick (visible after advancing).

## Slice 3b — Tax → approval penalty (sim)
**Type:** AFK · **Lead:** sim-engine · **Blocked by:** None (decision already settled).

### What to build
Implement the **settled Rev.2 decision**: tax carries a *direct approval penalty* in `politics.rs`
(in addition to suppressing reinvestment), with a new `Tuning` constant. Without this, Slice 3a's
lever has no interesting downside. Ripples to `prd/politics.md` + `techspec/politics.md`.

### Acceptance criteria
- [ ] A test asserts higher `tax_rate` lowers `satisfaction`/approval, all else equal (monotonic).
- [ ] New constant lives in `Tuning`; no inlined literal.
- [ ] `cargo test -p politiko_sim` passes.

---

## Slice 4 — Build transport in a region
**Type:** AFK · **Blocked by:** Slice 2 (needs region selection).

### What to build
Select a region from the table; show its `transport_cost`; a **Build** button calls
`build_transport(id)`. On success, treasury drops and `transport_level` rises; on failure, show the
returned `reason` (e.g. `InsufficientFunds`, `NotOperational`).

### Acceptance criteria
- [ ] Building in rugged inland CAR costs more than flat coastal NCR (topology mechanic is visible).
- [ ] A failed build surfaces the `reason` string; treasury is unchanged on failure.
- [ ] Engine support already exists — no Rust change expected.

---

## Slice 5 — Save / Load
**Type:** AFK · **Blocked by:** Slice 1.

### What to build
**Save** writes `to_json()` to `user://save.json`; **Load** reads it and calls `load_json()`. GDScript
handles the `user://` file IO; the engine handles (de)serialization.

### Acceptance criteria
- [ ] Advance → Save → advance again → Load restores the saved tick and state.
- [ ] `load_json` returning `false` (bad data) is handled without crashing.

---

## Slice 6 — Election banner
**Type:** AFK · **Blocked by:** Slice 1.

### What to build
On Advance, inspect the `advance_tick()` result; if `has_election`, show a non-modal banner with
`voteshare` and won/lost. Per the open-ended decision, an election is a **soft** beat — the game
continues regardless of outcome.

### Acceptance criteria
- [ ] At tick 72 (default `term_length`) a banner appears with the vote share.
- [ ] The game keeps running and all levers stay usable whether `won` is true or false.

---

## Notes
- **Publishing:** `to-issues` normally publishes these to a GitHub issue tracker; this repo has **no
  remote**, so this doc is the artifact. Set up a remote later to auto-publish AFK slices as issues.
- **Validation via prototype:** for risky design questions (e.g. the Layer-B connectivity algorithm),
  reach for a throwaway harness rather than guessing — that is the next big item (`connectivity-hierarchy.md`).
- **Suggested order:** 1 → (2, 3a, 5, 6 in any order) → 4. Slice 3b can run in parallel on the sim side.
