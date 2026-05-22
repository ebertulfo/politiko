# Issue board — local kanban

This repo has **no Git remote**, so the "issue tracker" is this directory: one markdown
file per vertical slice, with frontmatter the autonomous loop reads and updates. This is
the local-mode kanban from Matt Pocock's AI-coding workflow (see `docs/WORKFLOW.md`).

Each slice is a **tracer bullet** — a thin path through every layer (Rust `politiko_sim` →
`politiko_gdext` binding → Godot scene/GDScript → verifier), not a horizontal layer.

## File format

```
---
id: slice-2                    # stable, kebab-case; referenced by blocked-by
title: Region table
status: todo                   # todo | in-progress | blocked | review | done
type: AFK                      # AFK (agent-only) | HITL (needs a human decision)
lead: godot-ui                 # subagent that implements: sim-engine | godot-ui | game-balance
blocked-by: [slice-1]          # ids that must be `done` first; [] if none
harness: godot                 # the green gate: cargo | godot | none
verify: res://verify_slice2.gd # exact command/path the gate runs (see below)
---

## What to build
End-to-end behavior of this slice (not layer-by-layer). No file paths/snippets — they go stale.

## Acceptance criteria
- [ ] ...

## Verification
How the harness proves it green (the `verify` command + what it asserts).

## Notes
```

## The green gate (`harness`)

A slice is `done` only when its harness is **green** AND `spec-reviewer` signs off — never on
an agent's say-so. The loop re-runs the gate itself.

- `harness: cargo` → `cargo test -p politiko_sim` (red-green-refactor; assert invariants, not magnitudes).
- `harness: godot` → a committed headless verifier (`godot --headless --path . --script <verify>`)
  that asserts the acceptance criteria, like `verify_slice1.gd`. Exit code 0 = green.
- `harness: none` → HITL slices only; no automated gate, resolved by a human decision.

## Lifecycle

`todo` → (`/afk` picks it, deps met, type AFK) → `in-progress` → harness green + review → `done`.
A red gate the implementing agent can't fix → `blocked` (loop hard-stops). HITL slices are never
auto-started; the loop surfaces them and stops.

## Status snapshot

Run `/afk` (autonomous execution) — it reports the board state each pass. Slice 1 is `done`.
