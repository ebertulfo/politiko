# PRD: Game Overview

- **Status:** Built (core) + Spec'd-v1 (Layers B/C)

## 1. Player Fantasy

You are the permanent guiding hand of a Philippines-inspired archipelago — not a single
president, but the enduring statecraft behind the nation. Leaders come and go (an election
every six years can hand the presidency to a new face), but **you are always in charge**.
You don't micromanage citizens — you set national policy and decide *where* the country's
limited money goes. The fantasy is **stewardship of a real, uneven country**: a dense
capital, rich coastal lowlands, poor rugged interiors, scattered islands. Every region is
different because its *geography* is different, and your job is to grow the nation over the
long haul while keeping the public on side.

## 2. The Core Loop

```
people work in industries
  → industries earn revenue
    → you tax revenue into the treasury
      → you spend the treasury on infrastructure & education
        → infrastructure + education raise industry output and lower poverty
          → prosperity raises satisfaction
            → satisfaction is approval
              → approval is your standing: it swings the election every six years and scores your rule
```

This loop must be legible and satisfying on its own. Everything else deepens it.

## 3. The Infrastructure Spine

The thing that makes the *map* matter (not just a list of places) is that infrastructure reaches
across three scales — and the world is modeled all the way down to the **barangay**:

- **Local (A):** A road raises a place's *own* industry efficiency.
- **Regional (B):** Trade flows along a network, and a path is only as good as its **weakest link**.
  A well-developed barangay stranded behind a choked neighbor stays poor; clear the upstream
  chokepoint and a whole chain comes alive. Roads come in tiers (rough → farm-to-market → highway),
  and a **freight-rail** spine is a game-changer for bulk goods. Geography now drives *outcomes*,
  not just build cost.
- **Global (C):** Build a **port** (coastal only) or an **airport** and you plug a place — and the
  hinterland connected to it — into the world: **exports** of your tradable surplus at world prices,
  and **imported inputs** that supercharge manufacturing and let regions **specialize**. The world
  isn't flat: a few scripted booms and busts give trade income weather. Coastal-vs-landlocked,
  connected-vs-stranded, becomes a deep strategic axis. (International tourism and foreign
  investment come in a later phase; Tourism is a domestic industry for now.)

So the progression arc is **fix your roads → clear the chokepoints → open to the world.**

## 4. Player Decisions (the levers)

- **Tax rate** — collect more now vs. leave money in the productive economy. A double brake:
  high tax both starves industry of reinvestment **and directly costs approval**. National lever.
- **Education investment** — a slow bet; pays off over many months by raising the ceiling on
  every industry. National lever.
- **What to build, and where** — transport, ports, airports, each with a topology-driven cost.
  This is the core spatial decision: cheap flat-coastal wins vs. expensive but unlocking
  rugged/interior builds vs. a port that opens exports for a whole hinterland.
- **Debt** — you may run a negative treasury (5%/mo interest). A debt spiral forces austerity
  and bleeds approval, but in v1 it is **self-punishing, not fatal** — there is no game-over.

## 5. Feedback & Legibility

- The **map** is the primary readout: regions colored by a chosen metric (satisfaction,
  revenue, poverty, market access), with terrain visibly readable (mountains vs lowlands).
- A **dashboard**: date, treasury, approval, tax/education levers, speed controls.
- A **region panel**: its industries, education, infrastructure, and its derived numbers.
- The player must learn two lags: **education is slow**, and **population reacts to last
  month's economy**. Good UI surfaces trends, not just instantaneous values.

## 6. Setbacks (no hard loss in v1)

v1 is **open-ended** — a sandbox with no game-over. Instead of failure states there are
setbacks that make the job harder and dent your score:

- **Election defeat** — at a six-year election, approval-derived vote share ≤ 50%. A hit to
  your reputation/score and to momentum, but you **keep governing** (the leader's face changes;
  your levers don't).
- **Debt spiral** — runaway interest and forced austerity strangle your budget.
- **Stagnation** — bad play, not a loss: taxing the economy flat, neglecting education so
  industries can't grow, or pouring money into rugged regions connected to nothing.

## 7. Time

Real-time with pause; speeds 1×/2×/3×. 1 tick = 1 month. **Open-ended — no fixed end.** An
election every 72 months is a recurring beat (auto-pause on election).

## 8. Tuning Intent (top level)

- The **local loop must read as fun before B/C effects dominate.** The global layer is
  deliberately exogenous and bounded in v1 so export income enriches the loop rather than
  replacing it — schools and roads should still be the spine of a winning strategy.
- Topology should make some regions *feel* cheap and others *feel* like frontier projects.
- Winning elections should be achievable but not automatic with competent play.

## 9. Out of Scope (v1)

Energy grid, world-market price cycles/shocks, disasters, internal migration / OFW
remittances, public unrest / coup, corruption / personal wealth. See [`../roadmap.md`](../roadmap.md).
