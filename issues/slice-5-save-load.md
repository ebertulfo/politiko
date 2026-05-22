---
id: slice-5
title: Save / Load
status: todo
type: AFK
lead: godot-ui
blocked-by: [slice-1]
harness: godot
verify: res://verify_slice5.gd
---

## What to build
**Save** writes `to_json()` to `user://save.json`; **Load** reads it and calls `load_json()`.
GDScript handles the `user://` file IO; the engine handles (de)serialization.

## Acceptance criteria
- [ ] Advance → Save → advance again → Load restores the saved tick and state.
- [ ] `load_json` returning `false` (bad data) is handled without crashing.

## Verification
`verify_slice5.gd`: advance to tick T, save; advance further; load; assert `get_tick() == T` and a
sampled unit value matches the saved snapshot. Then `load_json("not json")` and assert it returns
false and the scene is still alive. Exit 0.

## Notes
Engine support already exists (`to_json`/`load_json`). Only `user://` file IO is new (GDScript).
