# System Interaction Map

The master view of how Politiko's systems depend on each other. Read this before any
individual system doc. Conventions (leaf vs aggregate, the one-tick lag, the rollup contract) are
in [`techspec/00-conventions.md`](techspec/00-conventions.md).

## Tick order (v1)

1 tick = 1 month. `engine.rs::advance_tick` runs the systems in this exact order so each reads
fresh data:

```
1. population        foundation; reads LAST tick's poverty/wage
2. connectivity      market_access via hierarchical bottleneck network              [Layer B]
3. global_demand     advance scripted World; set export_price_mult                  [Layer C]
4. industries        employ population; produce gross output, wages, poverty;
                     apply market_access revenue multiplier
5. trade             exports + imports + specialization → external_revenue          [Layer C]
6. economy           aggregate trade-adjusted revenue; tax; expenses; debt
7. infrastructure    upkeep slot (per-tick dynamics reserved)
8. education         slow convergence toward the investment lever
9. politics          satisfaction → approval → election check
```

Then `advance_tick` returns a `TickOutcome { tick, election, bankrupt }`. `bankrupt` is a
**non-fatal austerity flag**, not a game-over: it goes true when treasury falls below
`-debt_ceiling`, driving forced austerity + an approval hit, but the player keeps every lever and
keeps governing (open-ended sandbox — see the Rev.2 decisions in `REVIEW-TRACKER.md`).

## Dependency diagram

```
        ┌─────────────────────────── one-tick lag ───────────────────────────┐
        │ (population reads industries' poverty/wage from the previous tick)  │
        ▼                                                                     │
  ┌────────────┐    ┌──────────────┐   ┌───────────────┐                      │
  │ population │    │ connectivity │   │ global_demand │                      │
  └─────┬──────┘    └──────┬───────┘   └──────┬────────┘                      │
        │ population       │ market_access    │ export_price_mult             │
        └────────┬─────────┴──────────────────┘  (+ scripted World)           │
                 ▼                                                            │
          ┌────────────┐  revenue_last_tick, employment_rate, poverty, wage   │
          │ industries │  (× market_access multiplier) ──────────────────────┘
          └─────┬──────┘
                │ gross domestic output + tradable output
                ▼
          ┌────────────┐  exports + imports + specialization → external_revenue
          │   trade    │  (folded into revenue_last_tick; gateway reach via the network)
          └─────┬──────┘
                ▼
          ┌────────────┐  treasury, bankrupt
          │  economy   │◄── tax_rate, infra + gateway upkeep
          └─────┬──────┘
                ▼
          ┌────────────┐  (per-tick infra dynamics reserved; upkeep charged in economy)
          │ infrastruct│
          └─────┬──────┘
                ▼
          ┌────────────┐  education_level (slow)
          │ education  │◄── education_investment
          └─────┬──────┘
                ▼
          ┌────────────┐  satisfaction → approval → election
          │  politics  │
          └────────────┘
```

Acyclic apart from the single documented backward edge (population ← previous tick's industry
output).

## Reads / writes per system

"Reads" are inputs from earlier this tick (or the lagged population edge); "Writes" are the fields
the system mutates. National-scope state is on `Nation` / `World`.

Rows tagged **[B]/[C]** (and the fields they read/write — `market_access`, `export_price_mult`,
`exports_value`, `imports_value`, `external_revenue`, `World`, `Network`, `Sim::adjacency`, gateway
levels) are **target v1 spec, not yet in code**. The built engine runs the six untagged rows.

| # | System | Reads | Writes |
|---|--------|-------|--------|
| 1 | population | `education_level`, `poverty` (prev tick) | `population` |
| 2 | connectivity **[B]** | `population`, node `transport_level`, edge `road_tier`/`rail_level`, topology, `Sim::adjacency` | `market_access` |
| 3 | global_demand **[C]** | `World`, `Nation.tick`, `industries[].kind` | `World` (scripted events), `export_price_mult` |
| 4 | industries | `population`, `education_level`, `industries[].level`, `transport_level`, `market_access` | `revenue_last_tick`, `employment_rate`, `poverty`, `daily_wage` |
| 5 | trade **[C]** | `revenue_last_tick`, `industries[]`, gateway levels, `Network`, `market_access`, `World.price`/`demand_index`, `export_price_mult` | `exports_value`, `imports_value`, `external_revenue`; folds into `revenue_last_tick` |
| 6 | economy | aggregated `revenue_last_tick`, `Nation.tax_rate`, industry levels + infra/gateway upkeep | `Nation.treasury`, `Nation.bankrupt` (non-fatal austerity flag) |
| 7 | infrastructure | — (v1: upkeep charged in economy) | — |
| 8 | education | `Nation.education_investment`, `education_level` | `education_level` |
| 9 | politics | `poverty`, `employment_rate`, `education_level`, `transport_level`, `market_access` (optional), `Nation.tick`/`term_length` | `satisfaction`, `Nation.approval`, election |

### Consistency check

Every field read is written by an earlier-slot system, is an authored input, or is the documented
one-tick population lag:

- `market_access` (read by industries #4, trade #5, politics #9) ← connectivity #2. ✔
- `export_price_mult` (read by trade #5) ← global_demand #3. ✔
- `revenue_last_tick` (read by trade #5, economy #6) ← industries #4, amended by trade #5. ✔
- `poverty`, `daily_wage` (read by population #1) ← industries #4 *previous* tick (lag). ✔
- Authored/external inputs (topology, levels, edge tiers, `tax_rate`, `education_investment`,
  `World`) exist before the tick. ✔

No dangling reads. (No `tourism_*`/`fdi` fields in v1 — those are Layer D.)

## Aggregation touch-points

Systems producing aggregate-visible fields extend the single `aggregate_up` fold (see conventions):
**industries/trade** sum `revenue_last_tick`, `exports_value`, `imports_value`, `external_revenue`;
**connectivity** pop-weights `market_access`. `economy` and `politics` consume the aggregate root.
