---
name: tdd
description: Test-driven development with the red-green-refactor loop, one test at a time (vertical slices, never all-tests-then-all-code). Use when implementing a slice or fixing a bug test-first. This is the harness the AFK loop runs each slice through. Knows the dual gate — cargo for sim, headless .gd verifier for Godot/UI.
---

# Test-Driven Development — the harness

TDD is the **safety mechanism** of the AFK workflow (`docs/WORKFLOW.md`). One test → one
implementation → repeat. It is what makes unattended execution trustworthy: the gate is green or it
isn't.

## Philosophy

Tests verify **behavior through public interfaces**, not implementation details. Code can change
entirely; tests shouldn't. A good test reads like a specification of *what* the system does. In this
engine that means asserting **relationships and invariants** — monotonicity, ranges, conservation
(e.g. "higher tax ⇒ ≤ approval", "root population == sum of leaves") — **never brittle exact
magnitudes**, because `Tuning` constants get retuned. Prior art: `rust/sim/tests/integration.rs`,
per-module `#[cfg(test)]`, and the headless `verify_*.gd` acceptance scripts.

## Anti-pattern: horizontal slices

**DO NOT write all tests first, then all implementation.** Bulk-written tests test *imagined*
behavior and the *shape* of things, pass when behavior breaks, and outrun your understanding.

```
WRONG (horizontal):  RED: test1..test5   then  GREEN: impl1..impl5
RIGHT (vertical):    RED→GREEN: test1→impl1, test2→impl2, ...   (each cycle informed by the last)
```

## The two harnesses (pick by layer)

- **`cargo` — simulation logic** (`rust/sim`). Red-green-refactor with `cargo test -p politiko_sim`.
  This is the home of true TDD here. Run from `rust/`, PowerShell tool, with the PATH prefix
  `$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"`. Hand sim work to the **sim-engine** agent.
- **`godot` — presentation** (scenes/GDScript/binding). UI can't be unit-TDD'd the same way, so the
  gate is a committed **headless verifier** `verify_<slice>.gd` (pattern: `verify_slice1.gd`) that
  loads the real scene, drives it, and asserts the slice's acceptance criteria, exiting non-zero on
  failure. Write the verifier *first* (it's the RED), then build the scene to make it pass. Run:
  `godot --headless --path . --script res://verify_<slice>.gd`. Hand UI work to **godot-ui**.
  Verifier gotcha: a failed `assert` halts the script *before* `quit()` and hangs the run — use
  explicit pass/fail prints and always `quit(failures)` (see `verify_slice1.gd`).

## Workflow

1. **Plan.** Confirm the public interface and which behaviors matter most (you can't test
   everything — prioritize critical paths). Use the domain glossary for test/interface names. Respect
   the engine invariants (flat-list/tree-shape, topology-is-a-mechanic, single rollup path, Tuning
   discipline — see the `sim-engine` agent). List behaviors to test, not implementation steps.
2. **Tracer bullet.** Write ONE test for the first behavior → it fails (RED) → minimal code to pass
   (GREEN). This proves the path end-to-end.
3. **Incremental loop.** For each remaining behavior: RED (next test fails) → GREEN (minimal code).
   One test at a time; only enough code to pass it; don't anticipate future tests.
4. **Refactor.** Only while GREEN — never refactor while RED. Extract duplication, deepen modules,
   re-run the harness after each step.

## Per-cycle checklist
- [ ] Test describes behavior, not implementation
- [ ] Test uses the public interface only (would survive an internal refactor)
- [ ] Code is minimal for this test; no speculative features
- [ ] Harness re-run and reported honestly (green/red, with output)
