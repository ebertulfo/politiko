# Concept: Statesman — Game Vision & Pillars

- **Status:** Clarified 2026-06-07. The load-bearing vision for Statesman (Game 1) — what *kind* of
  game it is. Anchors everything downstream; design choices (including whether the
  [corruption](corruption.md) mechanic earns its place) are judged against this.
- **Relationship:** sharpens the "Form" / "Core loop" of [`statesman.md`](statesman.md) and the
  player fantasy in [`../prd/00-overview.md`](../prd/00-overview.md). Does not contradict them.

## The pitch in one breath

**Cities: Skylines, but at the scale of a nation** — with a **Victoria-3-style living economy**, and
**infrastructure as the load-bearing spine.** You are the enduring stewardship of an uneven
Philippines-inspired archipelago, and your job is to turn a scatter of isolated local economies into
*one integrated national economy* — not an imperial capital state where everything pools in the
capital while the provinces languish.

## The three touchstones (what we take, what we leave)

- **Cities: Skylines → the verb and the view.** You are a *builder/planner*, working **map-first** on
  the real 3D geography. You don't micromanage citizens; you place and fund infrastructure and shape
  *where* development happens. Scale jumps from a city to a **whole country**.
- **Victoria 3 → the economy.** A *living* economy underneath: production chains, goods, employment,
  supply/demand and prices, markets at **local → regional → national → global** layers. Pops work,
  industries earn, goods move, prices respond. We take the systemic depth.
- **…but with MORE emphasis on infrastructure than Vic3.** This is the deliberate divergence.
  Infrastructure is not a modifier — it is the **spine the whole economy hangs on.**

## The thesis: why infrastructure is the spine

Modern economies run on infrastructure. It is what makes:

- **money flow** (trade and markets reach across regions),
- **energy power the industries** (production is gated by power, not just labor/capital),
- **human resources be usable across the whole nation** (labor and goods can actually *get* to where
  they're productive), rather than stranded in pockets.

So the central design problem is **spatial: integrate an uneven country.** A rugged interior, scattered
islands, a dense capital, rich coasts — each a different local economy because its *geography* is
different (topology is already a mechanic in the engine). Infrastructure — transport tiers, rail,
ports/airports, energy, and the spec'd **Connectivity (Layer B)** + **Trade/Global (Layer C)** layers —
is the bridge that converts isolated local economies into a single national one.

## The central tension: integrate, don't concentrate

> The win is a **broad, connected, resilient** national economy.
> The failure is the **imperial capital state** — top-line growth that pools in the capital and a few
> strongholds while most of the country stays stranded, fragile, and poor.

Both can post "growth." They are different games. Steering toward integration (the hard, distributed,
deliberate path) over concentration (the easy, lopsided one) is the soul of Statesman. This is also the
yardstick any *stakes* mechanic must serve — see corruption below.

## The three pillars

1. **Build the nation** *(the Cities-Skylines verb)* — place and fund infrastructure across a real,
   uneven, topology-driven geography; decide *where* the country develops.
2. **Run a living economy** *(the Victoria-3 verb)* — production, employment, goods, prices, and
   markets respond to what you build, across local→regional→national→global layers.
3. **Integrate, don't concentrate** *(the thesis & tension)* — infrastructure + connectivity is how
   you make the whole uneven country function as one economy instead of an imperial capital state.

## What "success" means

Open-ended sandbox, **no game-over** (settled). Success is the economy — but **not merely GDP**: a
*broad, integrated, resilient* economy is the real prize, a *hollow, concentrated, fragile* one is the
hidden failure. Framed by the story as *"back to the top — wholly, and cleanly."* The journey is the
score's framing, not a win condition.

## Economic layers

**National → Regional → Local**, where **Local = City/Municipality** for v1 (the smallest decision
unit). Barangay (~42k units) is the eventual deepest zoom (texture/LOD), deferred behind the
connectivity-hierarchy and data-pipeline work. All units live in the engine's one flat
`Vec<AdminUnit>`; role derives from the tree; totals roll up through the single `aggregate_up` path.

## Design north stars (non-negotiables)

- **Spatial allocation under constraint is the fun.** Getting the right infrastructure to the right
  place so the uneven country works as a whole.
- **Topology is a mechanic**, not decoration (terrain/coastal drive cost and outcomes).
- **Legibility:** the map shows where it's working and where it isn't (regions colored by metric).
- **Reuse the engine:** flat list + tree-shape logic, single rollup, derived-vs-authored discipline,
  all tuning in `constants.rs::Tuning`. Godot renders; Rust owns state.

## How the existing design maps onto this

- **Central, not peripheral:** Connectivity (Layer B) and Trade/Global (Layer C) are the *spine* of
  the vision — the integration layers — so they rise in priority.
- **Already built (Layer A):** population, industries, economy, infrastructure, education, politics —
  the living-economy substrate the spine integrates.
- **The 6 UI slices** carry this toward a playable sandbox (map readout, region table, build, levers).

## Where corruption sits (PROVISIONAL)

[Corruption](corruption.md) is a **candidate stakes layer**, reframed to serve *this* vision: the
**gravity toward the imperial capital state** — it captures development into capturable centers,
degrades quality, and costs you *control over the shape of your economy* (a non-financial currency).
**It is provisional.** Pending the "corruption-distorts-allocation" research, it may be **shelved**;
the vision above stands fully on its own without it. The vision is the trunk; corruption is a branch
that must earn its place.

## Open questions

- The exact Vic3-depth of the economy we adopt vs. simplify (how many goods, how explicit the chains).
- How infrastructure's "spine" role is expressed mechanically beyond the existing market-access
  multiplier (energy as a hard gate? logistics capacity? freight vs. people?).
- Whether corruption survives its research gate, or the stakes come from somewhere else (or from the
  integrate-vs-concentrate tension alone).
