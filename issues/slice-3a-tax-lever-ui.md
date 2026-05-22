---
id: slice-3a
title: Tax lever (UI)
status: todo
type: AFK
lead: godot-ui
blocked-by: [slice-1]
harness: godot
verify: res://verify_slice3a.gd
---

## What to build
A slider (0–1) bound to `get_tax_rate`/`set_tax_rate`, with a label showing the current rate.
Advancing reflects the treasury response.

## Acceptance criteria
- [ ] Moving the slider calls `set_tax_rate`; the readout matches.
- [ ] Higher tax collects more treasury per tick (visible after advancing).

## Verification
`verify_slice3a.gd` sets the slider to a low rate, advances N ticks, records treasury delta; resets,
sets a high rate, advances N ticks, and asserts the high-rate treasury delta is strictly greater.
Also asserts the rate label matches `engine.get_tax_rate()`. Exit 0.

## Notes
Engine support already exists. Pairs with slice-3b (the approval downside) but does not depend on it.
