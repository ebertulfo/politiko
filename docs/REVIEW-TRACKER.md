# Spec Review Tracker

A living, multi-session tracker for reviewing and iterating on the design docs. The point: the
specs were drafted fast (some with your input on the *mechanics*, some not on the *prose*), so this
records what's been reviewed, what hasn't, and what's still open — so we can pick up in any session
without re-deriving state.

**How to use it:** when we review a doc, update its **Status** and clear/append its **Open
questions**. Log settled decisions in the Decisions Log so we never re-litigate them. Keep statuses
to the tokens below.

## Status legend

| Token | Meaning |
|-------|---------|
| `DRAFT` | Written without your review — needs a read-through and/or a design brainstorm. |
| `DESIGN-AGREED` | Mechanics decided with you (e.g. the Layer-B/C brainstorm); the prose reflects it but you haven't read the doc text yet. |
| `MATCHES-CODE` | Reverse-documented from tested Rust — accurate by construction. Low risk; a skim is enough. |
| `REVIEWED` | You've read the doc and it's good. |
| `LOCKED` | Final and agreed; ready to implement. Change only with intent. |

## Suggested review order (the queue)

1. **`prd/00-overview.md`** — confirm the top-level fantasy/loop reads right; everything hangs off it.
2. **`system-interaction-map.md`** + **`techspec/00-conventions.md`** — the contracts every spec leans on.
3. **`techspec/connectivity-hierarchy.md`** — highest-risk, fully unreviewed; the 42k-scale algorithm.
4. **`data-pipeline.md`** — large unreviewed effort + a licensing decision (OSM/ODbL).
5. **Connectivity / Trade / Infrastructure / Industries** prose (`DESIGN-AGREED` → `REVIEWED`).
6. Core specs (`MATCHES-CODE`) — quick skims.
7. **`roadmap.md`** — lowest priority (Layer D).

## Tracker

| Doc | Layer | Status | Open questions / next action |
|-----|-------|--------|------------------------------|
| `prd/00-overview.md` | — | REVIEWED | 2026-05-21: reframed to open-ended Vic3 model. Build-LOD question deferred to interaction-map. |
| `system-interaction-map.md` | — | REVIEWED | 2026-05-22: verified vs. code. Reframed `bankrupt` as non-fatal austerity flag; tagged unbuilt [B]/[C] rows. |
| `techspec/00-conventions.md` | — | REVIEWED | 2026-05-22: structural rules + `aggregate_up` fold confirmed accurate vs. code. Noted lib.rs tick-order + System-N numbering drift. |
| `techspec/connectivity-hierarchy.md` | B | DRAFT | 2026-05-22 reviewed (not locked — prototype-gated). Algorithm shape sound; §6 approximation honest. Open: (1) recombine must take **max over P's portals toward S**, not one portal; (2) external subtree treated as **point mass at its border** — omits portal→interior leg, a 2nd uncalled-out approximation; (3) per-tick is **O(N·depth·b)** not strictly O(N) — name a concrete ms budget; (4) **prerequisite: no `Edge`/adjacency/`network.rs` exists in code at all** — the Edge/adjacency data model (per connectivity.md) must land first; (5) build a `prototype` (LOGIC harness) validating hierarchical vs flat widest-path + benchmarking 42k recombination before LOCK. |
| `data-pipeline.md` | — | DRAFT | Economy-synthesis weighting model; **OSM ODbL vs DPWH** licensing call; output format (JSON/bincode); 42k zonal-stats feasibility. |
| `prd/connectivity.md` | B | DESIGN-AGREED | Read prose. Does "hunt the chokepoint" come through? |
| `techspec/connectivity.md` | B | DESIGN-AGREED | Confirm capacity formula (node+edge `min`), normalization, `market_access_weight`. Tier capacity values TBD. |
| `prd/trade-global.md` | C | DESIGN-AGREED | Read prose. Specialization risk/reward framing. |
| `techspec/trade-global.md` | C | DESIGN-AGREED | Specialization formula; scripted-event kinds/magnitudes; export premium vs domestic baseline. |
| `prd/infrastructure.md` | A/B/C | DESIGN-AGREED | Read prose (tiers/rail/gateways). |
| `techspec/infrastructure.md` | A/B/C | DESIGN-AGREED | Cost balance across tiers/rail/gateways; keep `build_transport` alias? |
| `prd/industries.md` | A | DESIGN-AGREED | Read prose (market-access + specialization additions). |
| `techspec/industries.md` | A | DESIGN-AGREED | Confirm market-access multiplier placement (separate from capped efficiency). |
| `prd/population.md` | A | MATCHES-CODE | Skim. |
| `techspec/population.md` | A | MATCHES-CODE | Skim. |
| `prd/economy.md` | A | MATCHES-CODE | Skim. Tax has no approval penalty in v1 — intended? |
| `techspec/economy.md` | A | MATCHES-CODE | Skim. Gateway upkeep added here. |
| `prd/education.md` | A | MATCHES-CODE | Skim. |
| `techspec/education.md` | A | MATCHES-CODE | Skim. |
| `prd/politics.md` | A | MATCHES-CODE | Skim. `w_connectivity` rebalance if added. |
| `techspec/politics.md` | A | MATCHES-CODE | Skim. |
| `roadmap.md` | D | DRAFT | Low priority; sanity-check the deferred set + hook points. |
| `README.md` | — | DRAFT | Index/scope correct after Rev.2. |

## Decisions Log (settled — do not re-litigate)

- **2026-05-20 — v1 system set:** all 8 systems (6 core built + Connectivity + Trade/Global).
- **2026-05-20 — Playable LOD:** barangay (~42,000 units); engine level-agnostic; connectivity computed hierarchically over the parent tree.
- **2026-05-20 — Connectivity model:** bottleneck (weakest-link) path capacity + mild distance decay.
- **2026-05-20 — Infrastructure:** roads tier ladder (rough→FMR→highway) + separate freight-rail overlay; infra on **both** nodes (`transport_level`) and links (`road_tier`/`rail_level`).
- **2026-05-20 — Market-access payoff:** revenue multiplier **and** specialization.
- **2026-05-20 — Global world:** exogenous **scripted beats** (authored event timeline), not drift, not endogenous cycles (those are Layer D).
- **2026-05-20 — Trade channels:** **exports + imported inputs only**; tourism-arrivals and FDI deferred to Layer D (Tourism stays a domestic industry).
- **2026-05-20 — Build order within v1:** Connectivity (incl. hierarchy algorithm) → Gateways/Trade.
- **2026-05-21 — Game shape:** **open-ended sandbox, no game-over** (Victoria 3 model). Player is the *permanent* steward; the leader/president changes at elections but the player is always in charge.
- **2026-05-21 — Elections:** recurring beat every 72 months. Defeat (approval-derived vote share ≤ 50%) is a **soft score/reputation hit** — player keeps all levers and keeps governing.
- **2026-05-21 — Tax brake:** tax now carries a **direct approval penalty** (in addition to suppressing reinvestment). Ripples to `techspec/politics.md` + `prd/politics.md`.
- **2026-05-21 — Debt:** negative treasury allowed (5%/mo interest); spiral forces austerity + bleeds approval but is **not fatal** — no IMF/bankruptcy game-over in v1. Ripples to `techspec/economy.md`.
- **2026-05-22 — `bankrupt` field:** kept as a bool flag (`treasury < -debt_ceiling`) but **reframed as a non-fatal austerity / debt-spiral signal**, never game-over. `TickOutcome.bankrupt` stays as a UI signal. Code comments still say "IMF default game-over" — fix during the economy.md review (see open questions).

## Open cross-cutting questions (not tied to one doc)

- [ ] **All `Tuning` constants are TBD.** A tuning/balancing pass is its own workstream once systems run.
- [ ] **Licensing for commercial release:** OSM road data is ODbL (share-alike) — use it as a separable DB, or source road classes from DPWH instead? (See `data-pipeline.md` §6.)
- [ ] **Seed hierarchy:** 17 vs 18 regions (Negros Island Region) — moot at barangay LOD but the PSGC tree must decide. The 2024 census adds another wrinkle.
- [ ] **`data.rs` becomes a loader** (42k units can't be hand-authored) — confirm the data-asset format + a small committed coarse fixture for tests.
- [ ] **Renderer:** confirm `project.godot` is actually on Forward+ (a stale read showed Mobile).
- [ ] **Prototype gate:** the hierarchical connectivity algorithm should be prototyped + benchmarked before the rest of Connectivity is locked.
- [ ] **Build LOD (from overview review):** does the player build at region/province level (barangay = substrate) or drill down and build anywhere? Resolve during `system-interaction-map.md` review, then reconcile §1/§4 of the overview.
- [ ] **Open-ended reframe ripples:** propagate the no-game-over + tax-approval-penalty + non-fatal-debt decisions into `prd/politics.md`, `techspec/politics.md`, `techspec/economy.md`, and `roadmap.md` when those come up in the queue.
- [ ] **Stale "game-over" code comments (do during economy.md review):** `types.rs:115` (`bankrupt` = "IMF default") and `economy.rs:6` ("IMF default game-over") still imply a terminal state — reword to the non-fatal austerity framing. *Code edit, not a doc edit.*
- [ ] **`lib.rs` tick-order docstring** documents the built six-system order; update it to the v1 nine-system order when Layers B/C land (flagged in `00-conventions.md`).
- [ ] **Tick-slot numbering:** specs number by the nine-system v1 order; Rust per-module docstrings number by the current six. Cite systems by name, not number, until they converge.

## Progress

- Reviewed/Locked: 3 / 23
- Design-agreed (prose unread): 8 / 23
- Draft (needs review): 4 / 23
- Matches-code (skim): 8 / 23
