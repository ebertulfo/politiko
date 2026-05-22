# PRD: Industries

- **Status:** Built — **Layer:** A (extended by B/C) — **Tick slot:** 4

## 1. Player Fantasy

This is where the country actually *makes money* and where your investments cash out. Watching
a region's factories and farms become more productive because you educated its workforce and
built its roads is the central satisfaction of the game.

## 2. Core Mechanic

Each region hosts industries of five kinds — **Agriculture, Manufacturing, Services, Tourism,
Mining** — each at some development level. Every month they produce revenue, but only as
efficiently as their conditions allow. Efficiency rises with:

- **Education** — each industry needs a minimum education level to run at full tilt (Services
  and Manufacturing demand far more than Agriculture or Mining).
- **Transport** — local infrastructure boosts output.
- **Labor** — there must be enough working-age people to staff the industries.

Industries also set **wages**, which determine **poverty** — the bridge from the economy back
to the population and satisfaction.

In v1 this is extended by the spine: **market access** (Layer B) widens the market an
industry sells into (a revenue multiplier), and Trade (Layer C) lets a well-connected,
import-supplied region **specialize** — drop self-sufficiency and lean into its strongest
industry — and sell tradable surplus abroad. (Those export/import/specialization effects are
settled in the Trade system; Industries produces the gross domestic output they build on.)

## 3. Player Decisions

- Which industries to grow where (you upgrade industry levels — a spend).
- Whether to fix the *enablers* first: an uneducated region's Services industry is dead weight
  until you raise its schooling; an industry with no transport underperforms.
- Reading comparative advantage: rugged interior regions lean Agriculture/Mining (low education
  needs); coastal/urban regions can support Manufacturing/Services/Tourism.

## 4. Feedback & Legibility

- Region panel lists each industry, its level, and an upgrade action with cost.
- The player should see *why* an industry underperforms — "needs education", "needs labor",
  "needs transport", "no market access" — not just a low number.
- Revenue-by-region is a primary map coloring mode.

## 5. Progression / Arc

Early: subsistence Agriculture and Mining in poor regions; a productive urban core. Mid: as
education rises, Manufacturing and Services become viable in more regions. Late: connected,
educated regions with export-oriented industry plugged into the world economy.

## 6. Failure Modes

Over-investing in high-education industries before the workforce can run them (wasted spend).
Growing industry faster than the labor force can staff it (under-utilized capital). Ignoring
the enabler systems so industries stagnate and wages — and therefore satisfaction — never rise.

## 7. Interactions

- **Reads:** population (labor), education (efficiency ceiling), transport (efficiency bonus),
  and in v1 market access + export price multiplier.
- **Writes:** revenue, employment, wages → poverty (the input population reads next tick).
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

The education requirement per industry is the key knob: it should make education investment
*feel* like it unlocks new economic tiers. Wages should be high enough in productive regions to
visibly lift people out of poverty, and low enough in stagnant ones that neglect hurts.

## 9. Out of Scope (v1)

Energy as a real constraint (assumed full in v1). Endogenous world-market cycles (v1 world
prices are exogenous, moved only by scripted beats). International tourism arrivals and FDI
(Tourism stays a *domestic* industry in v1). Per-good supply chains beyond the import-input bonus.
