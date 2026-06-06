# Concept: Statesman (Game 1 — PRIORITY)

- **Status:** In development. Builds directly on the existing engine + 3D map.
- **Protagonist:** the **nation**. **Main UI: the map.**

## One-line pitch

You are the enduring leadership of a Philippines-inspired archipelago. Steer an uneven country —
dense capital, rich coast, poor rugged interior, scattered islands — toward prosperity, and survive
the people's verdict.

## Why this is the priority

We already have the hard half built: a tested economic-geography engine that models the country down
to the barangay, where **topology drives outcomes**, plus a working 3D Philippines map. Game 1 is the
game this foundation was built for — it's ~6 thin UI slices from a playable sandbox (see
[`../vertical-slices.md`](../vertical-slices.md)). We build **on top of** what exists rather than
starting fresh.

## Form

**Map-first.** The 3D Philippines is the primary readout and the primary interaction surface — regions
colored by a chosen metric (satisfaction, revenue, poverty, market access), terrain visibly readable.
Around it: a national dashboard (date, treasury, approval, levers, speed) and a region panel. This is
the whole-nation, top-down view — *the country is the subject.*

## Core loop

The loop already specified in [`../prd/00-overview.md`](../prd/00-overview.md):

```
people work → industries earn → you tax → you spend on roads & schools
  → infrastructure + education raise output and lower poverty
    → prosperity → satisfaction → approval → the election every 72 months
```

Deepened by the infrastructure spine: **fix your roads → clear the chokepoints → open to the world**
(Local → Regional connectivity → Global trade/gateways).

## Player decisions (levers)

National policy and spatial allocation: **tax rate**, **education investment**, **what to build and
where** (transport tiers, rail, ports/airports — each with a topology-driven cost), **debt**, and (Layer
C) **trade/tariff/industrial-policy** levers as the late-game economic-strategy payoff.

## What it reuses

**Almost everything.** The six built systems (population, industries, economy, infrastructure,
education, politics) + the spec'd Connectivity (B) and Trade/Global (C) layers. The engine is its core.
All existing settled decisions in [`../REVIEW-TRACKER.md`](../REVIEW-TRACKER.md) apply to this game:
open-ended sandbox, **no game-over**, elections every 72 months as a soft score hit, direct tax→approval
penalty, non-fatal debt.

## The open design problem (what to grill next)

Game 1's known weakness, from the brainstorm: it is a **competent economic sim that still needs
stakes** — a reason to care beyond a high score. The original hook was *"how politics runs a country."*
The direction to explore (distinct from Game 2's personal-corruption tycoon) is a **political layer that
makes *governing* hard**: interest groups / factions with competing demands, legitimacy and coalitions,
a political system you must work *through* rather than a set of sliders you simply own. That layer — not
personal enrichment — is what could give the map game its missing tension. **This is the next thing to
design.**

## World, Story & Setting (grill-me — in progress, 2026-06-06)

The narrative frame for Statesman. **Decided this session via `/grill-me`. Not yet PRD'd.**

**Premise.** A PAP/Singapore-style founding myth: at a generational rock-bottom, Filipinos do the
thing they've never done — set the old factions aside and hand *one* meritocratic movement a real
mandate to take the country *back to the top of the world*.

### Settled decisions

1. **Story's role: flavor wrapper.** Setting prose + a small founding screen — **no new sim
   mechanics.** Colors the existing economic sim; adds no faction/legitimacy systems. (The
   "governing-is-hard / factions" gap below stays **open and separate** — this grill deliberately
   does not solve it, only avoids contradicting a future version of it.)
2. **Player = the Party.** You *are* the enduring movement that was handed the mandate. Presidents
   are its rotating faces; you persist across all of them. Matches the PRD's "leaders come and go,
   you are always in charge."
3. **Reality level: real geography, fictional politics.** The map *is* the Philippines (real
   regions, the map we already render). The party, leaders, and founding event are **fictional** —
   no real parties or living figures.
4. **The fall (what "the top" means): the squandered head-start.** Grounded in real history — in
   the 1950s–60s the Philippines was one of Asia's richest, *ahead of* South Korea, Taiwan, and
   Singapore, then got lapped over 70 years. The catch-up is against neighbors who were once our
   juniors. (Resonant, real, non-partisan.)
5. **Founding moment: repudiation of the dynasties.** The reckoning throws out the dynastic /
   *trapo* / *padrino* old order; the Movement is the meritocratic clean break the people unite
   behind. **Ties directly to Trapo (Game 2):** Game 1's founding party is the establishment that
   overthrew the very class Game 2 plays.
6. **Tone: earnest, with a shadow.** Primarily a sincere catch-up / nation-building story — with a
   quiet question underneath: *a movement that overthrew the dynasties could calcify into the next
   one.* "Will you become the thing you replaced?" A theme, not a mechanic. Ties back to Trapo.
7. **Player authors the Party.** At a small **founding screen** (the one bit of Godot the wrapper
   touches) the player types the **Party name + a short slogan/creed**, seeded with rotating
   fictional suggestions they can accept or overwrite. Name → shown on the dashboard; slogan →
   loading/intro. *Suggestion seeds (our flavor carriers):*
   - **Kilusang Sulong** (the Forward Movement) — *"Walang atrasan"* — no going back; merit over
     name, nation over clan, the future over the favor.
   - **Partido Tanglaw** (the Lightbringers) — knowledge and merit as the lamp out of seventy dark
     years; govern by competence, not by clan.
   - **Kilusang Haraya** (the Vision) — dare to imagine the islands at the summit once more, then
     build it region by region.
8. **The era: unspecified near-future, "Year One."** The in-game clock starts at the founding as
   Year One of the new government; never names a real year, so it never dates. The 70-year decline
   is backstory; the founding is now.
9. **The yardstick: real tigers, as backdrop prose.** Name the real neighbors (Korea, Taiwan,
   Singapore…) in founding/loading flavor as the ones who lapped us — *"they were behind us in
   1960."* Pure narrative backdrop — **no tracked national-rank stat** (that would be a mechanic).

**Free reframe (no mechanic change).** The existing **72-month election becomes the mandate,
re-tested** — *"do the people still trust the Movement, or do they drift back toward the old men?"*
The soft-loss beat now carries the whole theme. Consistent with the settled **open-ended / no
game-over** rule: *"back to the top"* is the journey and the score's framing, **not** a win
condition.

### Open threads (resume the grill here)

- How leaders surface — generate fictional president names/faces at each election? (PRD already says
  the face changes.)
- The **diaspora / "come home"** angle as optional flavor — the decline produced the OFW diaspora;
  the Movement's promise to bring them home.
- Exactly where flavor surfaces beyond the founding screen (loading-screen lore, intro text,
  dashboard party name/slogan).
- Doc home — keep this in `statesman.md` vs. split into a dedicated worldbuilding doc once it grows.
- Next stage when ready: `/to-prd` to capture, then `/to-issues`.

## Relationship to Trapo (Game 2)

Statesman is the **national altitude** that Trapo's family spends a whole run climbing toward. Same
domain, opposite center of gravity (nation-as-protagonist vs. family-as-protagonist). See
[`trapo.md`](trapo.md).
</content>
