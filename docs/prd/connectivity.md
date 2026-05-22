# PRD: Connectivity

- **Status:** Spec'd-v1 — **Layer:** B — **Tick slot:** 2

## 1. Player Fantasy

You're not governing a list of regions — you're building the circulatory system of a country, one
road at a time, down to the barangay. A barangay with good roads but stranded behind a
poorly-connected neighbor stays poor anyway, because trade has to *flow through* that weak link to
reach it. Upgrade the chokepoint and a whole chain of communities downstream comes alive at once.
Lay a freight rail spine and you change the economic geography of the nation. This is the spine of
the whole game: where you build, and in what order, decides which places thrive.

## 2. Core Mechanic

Places are linked into a **network**, and trade **flows along paths**. The defining rule is the
**weakest link**: a path is only as good as its worst segment. A pristine highway is worthless if
it dead-ends into a rough mountain track upstream — the bottleneck throttles everything behind it.
Distance matters too, but mildly: nearer markets are a little better than far ones on equal roads.

Infrastructure comes in tiers, each a step-change in how much trade a segment can carry:

- **Rough road** → **Farm-to-market road** → **Highway** — the roads ladder.
- **Freight rail** — a separate, high-capacity overlay between hubs. The game-changer: where rail
  exists, bulk goods move at a scale roads can't match.

Both the *places* (a barangay's own internal infrastructure) and the *links between them* have a
tier, and the bottleneck path considers both. Out of this comes each place's **market access** —
how much of the country's economy it can actually reach and trade with.

## 3. Player Decisions

- **Find the chokepoint.** The highest-leverage build is rarely the poorest place — it's the weak
  segment that's strangling everything behind it. Reading the network for bottlenecks is the core
  skill.
- **Roads vs. rail.** Incremental road upgrades everywhere, or a few transformative (expensive)
  rail spines between major hubs?
- **Connect or develop?** Spend to connect a stranded community so spillover reaches it, vs. spend
  on its local industry/education (which underperforms while it's choked off).
- **Sequence.** Because of bottlenecks, *order* matters: upgrading downstream before the upstream
  chokepoint wastes money.

## 4. Feedback & Legibility

- A **market-access map mode**: the connected core glows, stranded areas stay dark.
- The network drawn over the map with **tier-coded links** (rough/FMR/highway/rail) and the
  **bottlenecks highlighted** — the player must be able to *see* the weak link that's throttling a
  region.
- Inspecting a place shows *why* its access is low: "your roads are fine — the bottleneck is two
  hops upstream."

## 5. Progression / Arc

Early: a few naturally-connected lowland clusters; vast stranded periphery. Mid: the player hunts
and clears bottlenecks, watching chains of communities wake up, and lays the first rail spine.
Late: a nation knit together by tiered roads and rail, where the remaining isolated places are
deliberate triage.

## 6. Failure Modes

Upgrading the wrong segment (not the bottleneck) — money spent, nothing flows. Developing a
stranded community's industry before connecting it. Ignoring the network so the periphery
stagnates, poverty rises, and approval erodes from the edges in.

## 7. Interactions

- **Reads:** node infrastructure (`transport_level`), link tiers (`road_tier`, `rail_level`),
  topology (distance/decay), population (market mass), the adjacency graph.
- **Feeds:** industries (market-access revenue multiplier + enables specialization), trade
  (exports/imports must *flow* to gateways through this network), politics (optional connectedness
  satisfaction).
- See [`../system-interaction-map.md`](../system-interaction-map.md). Algorithm/scaling in
  [`../techspec/connectivity-hierarchy.md`](../techspec/connectivity-hierarchy.md).

## 8. Tuning Intent

The bottleneck rule must be *legible and punishing enough* that hunting chokepoints feels like the
core strategic act — but distance decay should stay mild so it sharpens, not dominates. Rail must
feel like a genuine tier jump over the best road, justifying its cost. Market access must visibly
pay (connecting a region lifts it) without making local development (education/industry) pointless
— connectivity *amplifies* the local loop, never replaces it.

## 9. Out of Scope (v1)

Player-drawn free-form routes (the graph is data-authored from real geography). Congestion that
varies within a tick. Energy flowing along the same network (Layer D reuses this graph). Dynamic
re-routing / maintenance failures.
