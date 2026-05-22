---
id: slice-2
title: Region table
status: todo
type: AFK
lead: godot-ui
blocked-by: [slice-1]
harness: godot
verify: res://verify_slice2.gd
---

## What to build
A table/`ItemList` of the operational (leaf) regions — name, population, `revenue_last_tick`,
`satisfaction` — filtered to `is_aggregate == false`, refreshed on each tick. Lives in the
content panel below the Slice 1 status bar.

## Acceptance criteria
- [ ] Exactly the operational regions are listed (aggregate root excluded).
- [ ] Values refresh after each Advance Month.
- [ ] Data comes from `get_all_units()`; count matches `operational_unit_count()`.

## Verification
`verify_slice2.gd` loads the main scene, asserts the row count equals
`engine.operational_unit_count()` and excludes the aggregate root, advances a tick, and asserts at
least one displayed value changed. Exit 0.

## Notes
Engine support already exists (`get_all_units`, `operational_unit_count`) — no Rust change expected.
