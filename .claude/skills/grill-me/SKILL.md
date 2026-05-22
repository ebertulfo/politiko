---
name: grill-me
description: Interview the user relentlessly about a plan or design until reaching shared understanding, resolving each branch of the decision tree one question at a time. Use when the user wants to stress-test a plan, get grilled on a design, start a new feature, or says "grill me". Stage 1 of the AFK workflow (see docs/WORKFLOW.md).
---

# Grill Me

**Stage 1 of the workflow** (`grill-me` → `to-prd` → `to-issues` → `/afk`; see `docs/WORKFLOW.md`).
This is the human-in-the-loop step — it cannot be delegated to an agent, because its whole purpose
is to extract what is only in the user's head. Alignment built here is what makes Stages 2–4 safe to
run away-from-keyboard.

Interview the user relentlessly about every aspect of the plan until you reach a shared
understanding. Walk down each branch of the design tree, resolving dependencies between decisions
one by one. Expect this to take many questions (Pocock's grills run 40–80+). Do not stop early
because the design "seems clear enough" — the unasked question is where the rework hides.

## Rules

- **Ask one question at a time.** Wait for the answer before the next. Each answer reshapes the tree.
- **Always offer your recommended answer**, with a one-line rationale, so the user can just confirm.
  Use `AskUserQuestion` when the choice is a clean fork; plain prose when it's open-ended.
- **If a question can be answered by reading the codebase or `docs/`, do that instead of asking.**
  This repo's design source of truth is the `docs/` tree (PRD + tech specs + `REVIEW-TRACKER.md`).
- **Never re-litigate a settled decision.** Decisions in `docs/REVIEW-TRACKER.md`'s Decisions Log
  (open-ended/no-game-over, 72-month soft elections, tax→approval penalty, non-fatal debt, barangay
  LOD, hierarchical connectivity) are locked — confirm you're respecting them, don't reopen them.
- **Use the project's vocabulary** (AdminUnit, leaf/aggregate, tick order, market_access, tracer
  bullet) so the resulting PRD lands in the codebase's language.
- The user may **dictate** answers — keep your questions short and answerable in one breath.

## What to cover

Drive toward resolving, at minimum: the problem from the player's perspective; which system(s) and
which layer(s) (sim / binding / Godot) are touched; new `AdminUnit`/`Nation`/`World` fields and where
in the tick order they're produced/read; the deep modules with testable interfaces; what is in vs.
out of scope; and what "done" looks like as a verifiable behavior.

## Exit

When the decision tree is resolved and the user signals alignment, say so and recommend running
`/to-prd` to capture it. Do not write the PRD here — that's Stage 2.
