# PRD: Infrastructure

- **Status:** Built (basic transport) + Spec'd-v1 (tiers, rail, gateways) — **Layer:** A/B/C — **Tick slot:** 7 (+ out-of-tick build actions)

## 1. Player Fantasy

This is your hands on the country — the main thing you physically *build*, from a barangay's first
real road to a national freight-rail spine to a deep-water port. It's where geography fights back:
a highway across a flat coastal plain is cheap; the same road over a mountain is a fortune.
Infrastructure is the spine that turns scattered communities into a connected, outward-facing
nation, and *what you build, where, and in what order* is the central craft of the game.

## 2. Core Mechanic

Infrastructure exists at two places and across tiers:

- **Internal (the unit):** a place's own `transport_level` — how well goods move *through* it. A
  choked barangay is a bottleneck for everything routed past it.
- **Links (between units):** each connection has a **road tier** — rough → farm-to-market →
  highway — and, separately, a **freight-rail level**. Rail is a high-capacity overlay between
  hubs: the game-changer for bulk goods.
- **Gateways:** **Ports** (coastal only) and **Airports** (anywhere) plug a place — and its
  connected hinterland — into the world economy (handled by Trade).

Every build's **cost scales with topology**: `base × (1 + ruggedness × k) × (coastal discount)`.
Flat coastal lowlands are cheap; rugged interiors are expensive. This single coupling is why the
topographic map exists, and why some links are routine and others are megaprojects.

## 3. Player Decisions

- **What to build, where:** internal capacity, a road-tier upgrade on a specific link, a rail spine
  between hubs, or a gateway. Each interacts with the bottleneck network differently.
- **Clear the chokepoint:** because connectivity is weakest-link, the best build is often the one
  bad segment throttling a whole chain — not the poorest place.
- **Roads vs. rail vs. gateways:** cheap incremental coverage, a transformative rail trunk, or a
  port that opens the world — all competing for the same treasury.
- **Mind the upkeep:** every level adds permanent monthly cost; you can over-build.

## 4. Feedback & Legibility

- Build costs shown per target *before* committing, so topology is felt in the price.
- The network rendered with **tier-coded links** and **bottlenecks highlighted**, so the player can
  see where a build would actually help.
- Existing internal/link/gateway levels and their upkeep visible.

## 5. Progression / Arc

Early: cheap internal roads and farm-to-market links in productive flat areas; hunt the first
chokepoints. Mid: highways and the first rail spine reshape flows; connect stranded clusters. Late:
ports/airports open the country to the world — the big, expensive, transformative builds.

## 6. Failure Modes

Upgrading a segment that isn't the bottleneck (no flow improvement). Building in rugged areas that
connect to nothing. Over-building until upkeep outruns revenue. A gateway with a choked hinterland.

## 7. Interactions

- **Build cost reads:** topology (`terrain_ruggedness`, `coastal`).
- **Internal transport feeds:** industries (efficiency) and connectivity (pass-through capacity).
- **Link tiers + rail feed:** connectivity (segment capacity / bottlenecks).
- **Gateways feed:** Trade (exports/imports).
- **Upkeep feeds:** economy (expense).
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

The ruggedness multiplier and coastal discount must make some builds *feel* routine and others
*feel* like frontier megaprojects. Each road tier should be a clear step up; rail should clearly
out-class the best road for bulk, justifying its cost. Gateways should be major, deliberate
investments. Upkeep should keep "always build more" from being correct.

## 9. Out of Scope (v1)

Per-tick infrastructure decay (an inert slot is reserved). Flood-control and energy infrastructure
types (Layer D). Player-drawn free-form routes (the graph comes from real geography; the player
upgrades existing links, doesn't draw new ones).
