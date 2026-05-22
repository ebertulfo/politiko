# Politiko

Single-player political sim. **Godot 4.6 renders & does IO; Rust owns ALL state and logic.**
Godot reads and draws — it never owns game data.

## Layout
- `rust/sim` (`politiko_sim`) — pure logic, **zero Godot dependency**, the state of truth.
- `rust/gdext` (`politiko_gdext`) — cdylib binding (godot-rust 0.5). Exposes class `PolitikoEngine`.
  Marshalling only — **no game logic here**.
- Godot project at repo root (`Main.tscn`, `*.gd`).
- `docs/` — design source of truth (PRDs + tech specs + `REVIEW-TRACKER.md`). Read the relevant
  spec before changing a system; don't re-derive from memory.
- `issues/` — local kanban board the AFK workflow drives.

## Build / test / run
Toolchain is **not on PATH** and needs the **PowerShell tool, not Bash** — prefix cargo with
`$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"` (Bash errors on `$env:`).
- Test the engine (fast, no Godot): `cargo test -p politiko_sim` (from `rust/`).
- Build the extension: `cargo build -p politiko_gdext` → `rust/target/debug/politiko_gdext.dll`.
- After a binding rebuild, rescan once so Godot re-registers it:
  `godot --headless --editor --path . --quit`.
- Headless verifier (the UI green gate): `godot --headless --path . --script res://verify_<slice>.gd`.
  Godot exe path + machine specifics: `docs/WORKFLOW.md`.

## Engine invariants (load-bearing — never violate)
- **Flat list, tree-shape logic.** All units in one `Vec<AdminUnit>`; role derives from `parent`
  (`is_aggregate`), never from hardcoded counts or branching on level. This is what scales 17 regions
  → ~42k barangays with a data swap and no logic change.
- **Single rollup path.** `aggregate.rs::aggregate_up` is the ONLY way national/regional totals are made.
- **Topology is a mechanic.** `terrain_ruggedness`/`coastal` affect outcomes, not just visuals.
- **Tuning discipline.** Every magic number lives in `constants.rs::Tuning`; never inline literals.
- **Derived vs authored.** `satisfaction`/`revenue_last_tick`/`poverty`/`employment_rate`/`daily_wage`
  are recomputed each tick — never hand-authored.

## Conventions
- Tests assert **relationships/invariants** (monotonicity, ranges, conservation), NOT exact
  magnitudes — constants get tuned.
- Keep logic in `politiko_sim`; keep Godot a thin presentation layer. Match surrounding code's idiom.

## Workflow
We build via a 4-stage pipeline: `/grill-me` → `/to-prd` → `/to-issues` → `/afk` (TDD as the harness).
See `docs/WORKFLOW.md`. Subagents: `sim-engine`, `godot-ui`, `game-balance`, `spec-reviewer`.

## Settled decisions (don't re-litigate — see docs/REVIEW-TRACKER.md)
Open-ended sandbox (no game-over); elections every 72 months (soft loss); tax carries a direct
approval penalty; debt is non-fatal (`bankrupt` = austerity flag, never terminal).
