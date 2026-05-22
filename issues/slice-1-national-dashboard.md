---
id: slice-1
title: National dashboard + advance the clock
status: done
type: AFK
lead: godot-ui
blocked-by: []
harness: godot
verify: res://verify_slice1.gd
---

## What to build
A main scene that instantiates `PolitikoEngine`, shows the national readout (tick, treasury,
approval, population from `get_root`), and an **Advance Month** button that calls `advance_tick()`
and refreshes the readout. The minimum *usable* artifact: a running nation.

## Acceptance criteria
- [x] Launching the project shows tick-0 values pulled from the engine (not hardcoded).
- [x] Clicking "Advance Month" advances the tick; treasury and approval visibly change.
- [x] Population shown is the aggregated root value (`get_root().population`).
- [x] Engine support already exists — no Rust change expected.

## Verification
`verify_slice1.gd` loads the real `Main.tscn`, reads the status-bar labels at tick 0, emits the
Advance button press, and asserts: population not placeholder, tick advanced, engine tick == 1,
treasury label changed, approval (engine float) changed, approval shown to 1 decimal. Exit 0.

## Notes
Top-status-bar layout (HITL layout decision, resolved 2026-05-22). Approval display widened to one
decimal so its sub-1%/month movement is visible. No Rust change — only GDScript display.
