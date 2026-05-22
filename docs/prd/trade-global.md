# PRD: Trade & Global Economy

- **Status:** Spec'd-v1 — **Layer:** C — **Tick slot:** 3 (demand) + 5 (settlement)

## 1. Player Fantasy

Open the country to the world — on purpose, in the right place. Build a **port** in a coastal hub
with mining or manufacturing behind it and its surplus starts selling abroad at world prices. But
it only works if the goods can actually *reach* the gateway through your road/rail network — a port
with a choked hinterland is an expensive monument. The flip side is just as strategic: gateways
bring in **imported inputs** that supercharge manufacturing and let your best regions stop trying
to make everything and just be great at one thing. Coastal-vs-landlocked, connected-vs-stranded,
becomes the deepest axis of the late game.

## 2. Core Mechanic

Two trade channels, both routed through **gateways** (ports — coastal only; airports — anywhere)
and gated by the **connectivity network**:

- **Exports.** Tradable surplus (Agriculture, Manufacturing, Mining) that can flow to a gateway
  sells abroad at the prevailing **world price**, earning a premium over pure domestic sale.
- **Imports.** Gateways supply imported inputs that boost Manufacturing output — and, crucially,
  **enable specialization**: a well-connected, import-supplied region can stop being self-sufficient
  and concentrate on its highest-advantage industry.

The outside world is **exogenous but not flat**: world prices and demand sit in the background and
shift on a few **scripted beats** during a playthrough — a commodity boom, a bust, an import-cost
spike — giving trade income texture and the occasional scramble, without (yet) reacting to the
player.

## 3. Player Decisions

- **Where to place gateways** — only pays where there's tradable industry *and* a bottleneck-free
  route feeding it. Reading the hinterland is the skill.
- **Specialize or stay safe?** Lean a region into one export industry for big returns and import the
  rest (efficient, but exposed to a world-price swing), or keep it diversified and resilient.
- **Ride the beats** — invest ahead of a known commodity boom, or hedge against a bust.

## 4. Feedback & Legibility

- Per gateway region: **export value**, **import inflow**, and the trade income they generate, so
  the payoff of a port is concrete.
- Show the **hinterland**: which connected regions are reaching the world *through* this gateway
  (and where a bottleneck is strangling that flow).
- A world-conditions readout (current prices/demand) and a clear signal when a scripted beat hits.

## 5. Progression / Arc

Early: a closed domestic economy; gateways unaffordable. Mid: the first port turns a connected
coastal hub into an export engine and reshapes the optimal map; specialization begins. Late: a
trading nation with multiple gateways and specialized export regions, riding (and occasionally
weathering) the world's scripted beats.

## 6. Failure Modes

A gateway with no tradable industry or a choked hinterland (idle capacity, wasted spend).
Over-specializing a region into a single export, then getting caught by a bust beat. Letting
gateway upkeep outrun trade income.

## 7. Interactions

- **Reads:** gateway levels (Infrastructure), the connectivity network (hinterland reach to
  gateways), industry composition, the exogenous `World` (prices, demand, scripted events).
- **Writes:** export/import values that fold into region revenue → economy (treasury); enables the
  Industries specialization mechanic; jobs feed politics.
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

**The headline risk: keep the global layer from drowning the local loop.** Export and import
income must *amplify* the schools-roads-connectivity loop, not replace it — a winning strategy is
still built on domestic development, with trade as a powerful multiplier for regions that earn it
through connectivity. World prices should make exporting clearly better than domestic-only, but
bounded; scripted beats should create memorable moments, not coin-flip ruin. Specialization should
be a genuine risk/reward dial, not a free "always specialize."

## 9. Out of Scope (v1)

International **tourism arrivals** and **FDI** as channels (deferred to Layer D — Tourism stays a
domestic industry in v1). Endogenous world-market cycles/shocks reacting to global supply & demand
(Layer D — the `World` struct + scripted timeline is built to be promoted into that). Trade policy
(tariffs, agreements), trade balance as a political issue.
