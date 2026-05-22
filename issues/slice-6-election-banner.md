---
id: slice-6
title: Election banner
status: todo
type: AFK
lead: godot-ui
blocked-by: [slice-1]
harness: godot
verify: res://verify_slice6.gd
---

## What to build
On Advance, inspect the `advance_tick()` result; if `has_election`, show a non-modal banner with
`voteshare` and won/lost. Per the settled decision, an election is a **soft** beat — the game
continues regardless of outcome.

## Acceptance criteria
- [ ] At tick 72 (default `term_length`) a banner appears with the vote share.
- [ ] The game keeps running and all levers stay usable whether `won` is true or false.

## Verification
`verify_slice6.gd` advances to `get_term_length()` ticks, asserts the banner became visible and its
text contains the vote share, then asserts the Advance button is still enabled and a further advance
still works. Exit 0.

## Notes
Engine support already exists (`advance_tick()` returns `has_election`/`won`/`voteshare`). Soft loss
— never terminal (see `docs/REVIEW-TRACKER.md`).
