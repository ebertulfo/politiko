---
name: to-issues
description: Break a plan, spec, or PRD into independently-grabbable slices on the local issues/ board using tracer-bullet vertical slices. Use when the user wants to convert a plan/PRD into implementation tickets or break work into issues. Stage 3 of the AFK workflow (see docs/WORKFLOW.md).
---

# To Issues

**Stage 3 of the workflow** (`grill-me` → `to-prd` → `to-issues` → `/afk`). Break a plan/PRD into
independently-grabbable slices using **vertical slices (tracer bullets)**, and write them to the
local board.

There is no GitHub remote — the "issue tracker" is the `issues/` directory (one markdown file per
slice with status/blocking frontmatter). See `issues/README.md` for the exact file format.

## Process

### 1. Gather context
Work from the conversation context (usually the PRD just written by `/to-prd`). If the user passes a
reference, read it: a PRD under `docs/prd/`, a slice file under `issues/`, or a section of a doc.

### 2. Explore the codebase (optional)
If you haven't already, explore to understand the current state. Slice titles and descriptions use
the project's domain vocabulary (AdminUnit, leaf/aggregate, tick order) and respect the settled
decisions in `docs/REVIEW-TRACKER.md`.

### 3. Draft vertical slices
Break the plan into **tracer-bullet** slices. Each is a thin path through ALL layers end-to-end
(`politiko_sim` → `politiko_gdext` binding → Godot scene/GDScript → verifier), NOT a horizontal
slice of one layer.

<vertical-slice-rules>
- Each slice delivers a narrow but COMPLETE path through every layer it touches (logic, binding, UI, test)
- A completed slice is demoable or verifiable on its own
- Prefer many thin slices over few thick ones
</vertical-slice-rules>

Mark each slice **AFK** (an agent can implement + verify it alone) or **HITL** (needs a human
decision — a layout call, an architectural fork, a design review). Prefer AFK; reserve HITL for real
decisions. Assign a `lead` subagent (`sim-engine` for logic, `godot-ui` for presentation,
`game-balance` for tuning) and a `harness` (`cargo` for sim, `godot` for UI, `none` for HITL).

### 4. Quiz the user
Present the breakdown as a numbered list. For each slice show: **Title**, **Type** (HITL/AFK),
**Lead**, **Blocked by**, **Harness**, and the user stories it covers. Ask:
- Does the granularity feel right (too coarse / too fine)?
- Are the dependency relationships correct?
- Should any slices be merged or split?
- Are HITL vs AFK assignments right?

Iterate until the user approves.

### 5. Write the slices to the board
For each approved slice, create `issues/<id>-<kebab-title>.md` using the format in
`issues/README.md` (frontmatter + What to build / Acceptance criteria / Verification / Notes). Write
them in dependency order so `blocked-by` references real ids. New slices start `status: todo`.

Describe end-to-end behavior, not layer-by-layer implementation. Avoid file paths/snippets — they go
stale. Exception: a prototype-derived snippet that encodes a decision more precisely than prose
(state machine, schema, field shape) — inline only the decision-rich part and note it came from a
prototype.

Do NOT modify slices already marked `done`.

### Exit
When the board is written, the AFK slices are ready for **Stage 4**: recommend running `/afk` to
execute them autonomously.
