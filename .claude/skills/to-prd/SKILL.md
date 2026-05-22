---
name: to-prd
description: Synthesize the current conversation into a Product Requirements Document under docs/prd/. Use after a grill-me alignment session, or when the user wants to capture the current design as a PRD. Do NOT interview — synthesize what is already known. Stage 2 of the AFK workflow (see docs/WORKFLOW.md).
---

# To PRD

**Stage 2 of the workflow** (`grill-me` → `to-prd` → `to-issues` → `/afk`). Take the current
conversation context and codebase understanding and produce a PRD. **Do NOT interview the user** —
synthesize what you already know (the grilling already happened in Stage 1).

The PRD is a *destination document*: it captures the agreed design in a structured form. It is not
meant to be read meticulously afterward — alignment was built in the grill, not by re-reading this.

## Process

1. **Explore** the repo and `docs/` to ground the PRD in the current state. Use the project's domain
   vocabulary throughout (AdminUnit, leaf/aggregate, tick order, the six built systems + v1 nine-system
   target). Respect the settled decisions in `docs/REVIEW-TRACKER.md`.

2. **Sketch the deep modules** you'll build or modify — a deep module encapsulates a lot behind a
   small, testable interface that rarely changes. For this engine that usually means a new
   `rust/sim/src/systems/*.rs` (pure logic) plus the `AdminUnit`/`Nation` fields it reads/writes, and
   the slot it takes in the tick order. **Check with the user** that these modules match their
   expectations, and **which modules they want tests written for** (the TDD targets).

3. **Write the PRD** to `docs/prd/<feature>.md` using the template below. (PRDs live in `docs/prd/`;
   this repo has no issue tracker, so the file *is* the published artifact — Stage 3 `/to-issues`
   reads it.) If it extends an existing system's PRD, update that file instead of creating a new one.

<prd-template>
# <Feature> — PRD

## Problem Statement
The problem the player faces, from the player's perspective.

## Solution
The solution, from the player's perspective.

## User Stories
A LONG, numbered list, each: `As a <actor>, I want <feature>, so that <benefit>`. Extensive — cover
all aspects.

## Implementation Decisions
Modules built/modified and their interfaces; new fields and their tick-order placement; technical
clarifications; architectural decisions; schema/serialization changes. NO file paths or code
snippets — they go stale. Exception: a prototype-derived snippet that encodes a decision more
precisely than prose (a formula, field shape, state machine) — inline only the decision-rich part.

## Testing Decisions
What makes a good test here (assert observable behavior / invariants — monotonicity, ranges,
conservation — NOT exact magnitudes, since `Tuning` constants get tuned). Which modules get tests.
Prior art (`rust/sim/tests/integration.rs`, per-module `#[cfg(test)]`, headless `verify_*.gd`).

## Out of Scope
What this PRD deliberately does not cover (e.g. Layer-D systems in `docs/roadmap.md`).

## Further Notes
Anything else.
</prd-template>

## Exit
When the PRD is written, recommend running `/to-issues` to break it into tracer-bullet slices on the
local board (Stage 3).
