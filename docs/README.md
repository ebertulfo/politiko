# Politiko — Game Systems Documentation

This folder is the design source of truth for Politiko's simulation systems. Every
gameplay system has two documents:

- a **PRD** (`prd/<system>.md`) — player-facing: the fantasy, the mechanic, the
  decisions it surfaces, how the player sees it working, and the designer's tuning intent.
- a **Tech Spec** (`techspec/<system>.md`) — implementation: data fields, formulas,
  function signatures, tick placement, reads/writes, and the test plan, all mapped to the
  real Rust files under `rust/sim/src/`.

Read the PRD to understand *why* a system exists and *what the player feels*; read the
Tech Spec to *build it*. They mirror each other one-to-one.

> **Iterating on these docs across sessions?** Use [`REVIEW-TRACKER.md`](REVIEW-TRACKER.md) — it
> tracks each spec's review status, open questions, and the settled-decisions log.

## Start here

1. [`prd/00-overview.md`](prd/00-overview.md) — the whole-game player fantasy and the core loop.
2. [`system-interaction-map.md`](system-interaction-map.md) — how every system reads from and
   writes to every other, and the order they run each tick. **This is the master diagram; read
   it before any individual system doc.**
3. [`techspec/00-conventions.md`](techspec/00-conventions.md) — the structural rules and
   contracts every Tech Spec assumes (flat-list/tree logic, the `Tuning` discipline, the
   `aggregate_up` rollup contract, the derived-field contract).
4. [`roadmap.md`](roadmap.md) — systems deferred to later phases (Layer D) and where they hook in.
5. [`data-pipeline.md`](data-pipeline.md) — how the ~42k-barangay seed dataset is sourced/synthesized.

## The infrastructure spine (the organizing idea)

Politiko's systems are layered by economic *reach*, and **infrastructure is the spine that
connects the layers**:

| Layer | Reach | What infrastructure does | Status |
|-------|-------|--------------------------|--------|
| **A** | Local | Transport raises a place's *own* industry efficiency | Built |
| **B** | Regional | Tiered roads + a freight-rail overlay form a network where trade is throttled by its **weakest link**; clearing chokepoints lifts whole chains (market access) | v1 |
| **C** | Global | Gateways (ports/airports) plug a place + its connected hinterland into an exogenous, **scripted** world economy via **exports + imported inputs** (the latter enabling specialization) | v1 |
| **D** | Systemic | Energy, world-market cycles, disasters, migration/OFW, unrest, corruption, tourism-arrivals & FDI | Later |

## Status legend

Each doc's header carries a status:

- **Built** — implemented and tested in `rust/sim/`.
- **Spec'd-v1** — designed here, scheduled for v1, not yet implemented.
- **Deferred** — Layer D; roadmap paragraph only, no full spec yet.

## v1 scope

v1 ships all eight systems: the six core (population, industries, economy, infrastructure,
education, politics) plus **Connectivity** (Layer B) and **Trade / Global Economy** (Layer C).

- **Playable LOD = barangay (~42,000 units).** The engine is level-agnostic, so the same code runs
  on regions or barangays; connectivity is computed **hierarchically** over the parent tree to be
  tractable at this scale (see [`techspec/connectivity-hierarchy.md`](techspec/connectivity-hierarchy.md)).
  The seed dataset is sourced/synthesized via the [`data-pipeline.md`](data-pipeline.md).
- **Global layer is exogenous** — world prices/demand move only on a few **scripted beats**, *not*
  the full world-market-cycle system (Layer D). v1 trade channels are **exports + imported inputs**
  only; international tourism arrivals and FDI are deferred.
- **Recommended build order within v1:** Connectivity first (the endogenous spatial loop), then
  Gateways/Trade.

## Document index

| System | Layer | PRD | Tech Spec |
|--------|-------|-----|-----------|
| Population | A | [prd](prd/population.md) | [spec](techspec/population.md) |
| Industries | A | [prd](prd/industries.md) | [spec](techspec/industries.md) |
| Economy & Treasury | A | [prd](prd/economy.md) | [spec](techspec/economy.md) |
| Infrastructure | A/B/C | [prd](prd/infrastructure.md) | [spec](techspec/infrastructure.md) |
| Education | A | [prd](prd/education.md) | [spec](techspec/education.md) |
| Politics & Approval | A | [prd](prd/politics.md) | [spec](techspec/politics.md) |
| Connectivity | B | [prd](prd/connectivity.md) | [spec](techspec/connectivity.md) |
| Trade & Global Economy | C | [prd](prd/trade-global.md) | [spec](techspec/trade-global.md) |

Supporting specs (no PRD): [`techspec/connectivity-hierarchy.md`](techspec/connectivity-hierarchy.md)
(the barangay-scale connectivity algorithm) and [`data-pipeline.md`](data-pipeline.md) (seed data).
