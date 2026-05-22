---
name: afk
description: Autonomous phased execution — the away-from-keyboard loop. Repeatedly picks the next unblocked AFK slice from the issues/ board, dispatches it to the right subagent under the TDD harness, verifies the green gate itself, gets a spec-reviewer sign-off, marks it done, and continues. Use when the user wants to run the board down unattended, says "go AFK", "execute the slices", or "/afk". Stage 4 of the workflow (see docs/WORKFLOW.md).
---

# AFK — Autonomous Phased Execution

**Stage 4** (`grill-me` → `to-prd` → `to-issues` → **`/afk`**). The board (`issues/*.md`) is already
planned and aligned; humans are out of the loop. You are the **orchestrator**: you do not implement
slices yourself — you dispatch them to subagents, run the green gate yourself, and drive the board.

## Preconditions (check once, up front)
- `issues/` exists with at least one `status: todo`, `type: AFK` slice. If not, report and stop.
- Permission mode won't prompt mid-run. Subagent edits/commands need to run unattended — if the
  session will prompt on every edit, tell the user to enable a non-prompting mode (e.g. accept-edits)
  before this is truly AFK, then continue.

## The loop

Repeat until a stop condition fires:

1. **Scan the board.** Parse every `issues/*.md` frontmatter (`id`, `status`, `type`, `lead`,
   `blocked-by`, `harness`, `verify`).
2. **Find ready work.** A slice is *ready* iff `status: todo`, `type: AFK`, and every `blocked-by`
   id is `done`. Pick the ready slice in dependency/`id` order. Resolve ties deterministically.
   - If the only ready or remaining work is `type: HITL` → **STOP**, surface it, ask the user to
     resolve the human decision (then they can re-run `/afk`).
   - If nothing is ready and todo slices remain → **STOP**, report the blocking chain.
   - If no todo slices remain → **STOP**, report the board is drained. 🎉
3. **Claim it.** Set the slice's `status: in-progress` (edit its frontmatter).
4. **Dispatch to the lead subagent** via the Agent tool (`subagent_type`: the slice's `lead` —
   `sim-engine` / `godot-ui`; use `game-balance` only for tuning slices). Give the agent: the full
   slice body, an instruction to work **test-first under the `tdd` skill** (write the `verify`
   target first as RED, then minimal impl to GREEN, then refactor), the exact `verify` command, and
   the order to commit nothing. Tell it to report the harness result honestly.
5. **Run the gate YOURSELF.** Never trust the agent's claim — re-run the slice's `verify`:
   - `harness: cargo` → `$env:Path="$env:USERPROFILE\.cargo\bin;$env:Path"; cargo test -p politiko_sim`
     (PowerShell tool, from `rust/`).
   - `harness: godot` → `<godot> --headless --path . --script <verify>` (the console exe in
     `docs/WORKFLOW.md`); for a binding change, `cargo build -p politiko_gdext` + an editor scan first.
   - **GREEN** (exit 0 / tests pass) → go to 6. **RED** → give the agent ONE more attempt with the
     failure output; still red → set `status: blocked`, append the failure to the slice's Notes,
     and **STOP** (a red gate is a hard stop — do not thrash or fudge the test to pass).
6. **Review.** Dispatch `spec-reviewer` to (a) confirm the change matches the slice + settled
   decisions and (b) propagate any behavior/decision change into `docs/` and `REVIEW-TRACKER.md`.
   For non-trivial code, optionally run the `code-review` skill. Block-and-fix on a blocking finding.
7. **Mark done.** Tick the acceptance-criteria boxes, set `status: done`, and note what shipped +
   how it was verified.
8. **Report one line** for the slice (`✅ slice-N: <title> — gate green, reviewed`) and loop to 1.

## Stop conditions (any one ends the run)
- Board drained, a blocked dependency chain, or only HITL work remains.
- A red gate the lead agent couldn't fix in one retry (slice → `blocked`).
- A blocking review finding that can't be auto-resolved.

## Rules
- **The gate is the truth.** Never edit a test/verifier solely to make it pass; never mark a slice
  done on the agent's word without re-running the gate.
- **One slice in flight at a time** in this single-machine loop (slices without shared deps *could*
  be parallelized with background agents later — keep it sequential for now to keep the board sane).
- **Honesty over completion.** A red gate, a skipped step, or a blocked slice is reported plainly —
  never a "done" that wasn't verified.
- **Resumable.** Each pass starts by re-reading the board, so the loop survives interruption: just
  re-run `/afk` (or `/loop /afk` to have it re-checked on an interval).

## End-of-run summary
Report: slices completed (with gate results), anything left `blocked`/`todo` and why, and any HITL
decision now waiting on the user.
