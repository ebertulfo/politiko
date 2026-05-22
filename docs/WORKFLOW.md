# The AFK Workflow

How we build Politiko: Matt Pocock's AI-coding pipeline, adapted to this Rust+Godot repo. Four
stages, splitting into a **human-in-the-loop front half** (you align the design) and an
**away-from-keyboard back half** (agents execute it). Each stage has a skill.

```
  grill-me  ──►  to-prd  ──►  to-issues  ──►  /afk
  (Stage 1)     (Stage 2)    (Stage 3)      (Stage 4)
  ╰──── HITL: you + Claude ────╯            ╰── AFK: agents alone ──╯
```

The whole bet: **front-load alignment so execution is safe to delegate.** Most of the value is in
Stages 1–3; if those are right, Stage 4 is a loop you can walk away from. TDD is the harness that
keeps the loop honest.

## Stage 1 — `/grill-me` (HITL · alignment)
Claude interviews you relentlessly, one question at a time, down every branch of the design tree,
each with a recommended answer. Cannot be delegated — it extracts what's only in your head. Answer
by dictation if it's faster. Questions answerable from `docs/` or the code are looked up, not asked.
Settled decisions in `docs/REVIEW-TRACKER.md` are never reopened. **Output:** shared understanding.

## Stage 2 — `/to-prd` (HITL · capture)
Claude synthesizes the conversation into a PRD at `docs/prd/<feature>.md` (no interviewing — the
grill already happened). Sketches the **deep modules** (small testable interface, deep
implementation — usually a new `systems/*.rs` + the fields it touches + its tick-order slot) and
confirms which get tests. The PRD is a *destination doc*: structured, not re-read. **Output:** a PRD.

## Stage 3 — `/to-issues` (HITL→AFK · plan)
Claude breaks the PRD into **tracer-bullet vertical slices** — each a thin path through every layer
it touches (sim → binding → Godot → verifier), demoable on its own. You approve the granularity,
dependencies, and HITL/AFK split. Slices are written to the **local board** `issues/*.md` (this repo
has no remote — the directory *is* the tracker; see `issues/README.md`). **Output:** a ready board.

## Stage 4 — `/afk` (AFK · execute)
Claude becomes an orchestrator and runs the board down unattended:

> scan board → pick next unblocked AFK slice → dispatch to its `lead` subagent **test-first under
> the `tdd` skill** → **re-run the green gate itself** → `spec-reviewer` sign-off + doc ripple →
> mark `done` → repeat.

Hard stops: a red gate the agent can't fix in one retry (slice → `blocked`), only HITL work left, or
a blocked dependency chain. The loop is **resumable** — re-run `/afk`, or `/loop /afk` to re-check on
an interval.

## The TDD harness (`/tdd`)
The safety mechanism. One test → one impl → repeat (never all-tests-then-all-code). Assert
**relationships/invariants**, not exact magnitudes (constants get tuned). Dual gate by layer:
- **`cargo`** — sim logic: `cargo test -p politiko_sim` (red-green-refactor). Lead: `sim-engine`.
- **`godot`** — presentation: a committed headless verifier `verify_<slice>.gd` (pattern:
  `verify_slice1.gd`) that drives the real scene and asserts the acceptance criteria, exit 0 = green.
  Lead: `godot-ui`. *Gotcha:* a failed `assert` hangs the headless run — print pass/fail and always
  `quit(failures)`.

A slice is `done` only when its gate is green **and** `spec-reviewer` signs off — never on an
agent's say-so.

## Cast (subagents)
- **sim-engine** — Rust `politiko_sim` logic + the binding surface. TDD home.
- **godot-ui** — scenes / GDScript / `PolitikoEngine` binding. Presentation layer.
- **game-balance** — `Tuning` constants + feedback-loop analysis (read-only elsewhere).
- **spec-reviewer** — reconciles `docs/` with tested code; maintains `REVIEW-TRACKER.md`. Read-only on code.

## Machine notes
- Cargo/Godot are **not on PATH** — run via the **PowerShell tool** with
  `$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"` (the Bash tool errors on `$env:`).
- Godot console exe: `C:\Users\edria\Downloads\Godot_v4.6.2-stable_win64.exe\Godot_v4.6.2-stable_win64_console.exe`
- The extension registers only after an editor scan writes `.godot/extension_list.cfg`
  (`godot --headless --editor --path . --quit` once after a binding rebuild).
- **For a truly unattended `/afk` run, use a non-prompting permission mode** (e.g. accept-edits), or
  every subagent edit/command will prompt. To cut the prompts, add the gate commands
  (`cargo test`/`cargo build`, the godot headless verifier) to `.claude/settings.local.json`'s allow
  list — run `/fewer-permission-prompts`, which scans your transcripts and proposes the entries.

## Status
Stage 4 board is seeded from the original `docs/vertical-slices.md` plan. **Slice 1 is `done`**;
slices 2, 3a, 3b, 5, 6 are ready (3b unblocked; the rest need slice-1, which is done), slice 4 waits
on slice 2. Run `/afk` to execute.
