# PRD: Economy & Treasury

- **Status:** Built — **Layer:** A — **Tick slot:** 6

## 1. Player Fantasy

The hard budget line. Everything you want to do costs money, and the money comes from taxing an
economy you're simultaneously trying to grow. This is the system that says *no* — the constraint
that makes every other decision a real choice.

## 2. Core Mechanic

Every month, all regional revenue rolls up to a national total. You collect a slice of it as
**tax**. Against that you pay **expenses** — maintaining industries and infrastructure. The
difference moves your **treasury**. You may go into **debt** (negative treasury), but debt
accrues interest every month, and if it grows past a ceiling you **default** and lose.

## 3. Player Decisions

- **The tax rate** — the central tension. Tax high to fund aggressive building and education,
  or tax low to leave money compounding in the productive economy. (In v1 tax has no direct
  approval penalty, so the cost of high tax is purely economic; a future unrest system makes it
  political too.)
- **Deficit spending** — borrow to invest now and grow your way out, vs. live within your means.
  Debt is a tool and a trap.

## 4. Feedback & Legibility

- Treasury and its monthly delta are front-and-center on the dashboard.
- The player must see the components: revenue collected, expenses paid, interest charged.
- A clear danger state as debt approaches the default ceiling.

## 5. Progression / Arc

Early: a starting treasury and tight margins; every build is a real tradeoff. Mid: a growing
economy widens your fiscal room. Late: a connected, exporting economy can fund big projects —
or a mismanaged one is buried in compounding debt.

## 6. Failure Modes

**Default (game over)** from runaway debt. Softer failures: taxing so high you choke growth, or
so low you can never afford the infrastructure/education that would grow the base. Letting
maintenance costs from sprawling infrastructure outrun the revenue it generates.

## 7. Interactions

- **Reads:** aggregated regional revenue (incl. trade in v1), the tax lever, all upkeep.
- **Writes:** treasury, bankruptcy flag. Funds every build action.
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

The default tax rate (~14%, realistic for the Philippines) and starting treasury should make the
opening *tight but survivable*. Debt interest must be punishing enough that deficits are a
deliberate gamble, not a free lunch. Maintenance costs should make over-building a real risk, so
infrastructure isn't a pure "always build more" decision.

## 9. Out of Scope (v1)

Bond markets / variable interest, IMF programs with conditions, tax-evasion/corruption skim
(Layer D). Tax is a single national rate, not per-region or per-industry.
