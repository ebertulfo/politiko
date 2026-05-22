# Roadmap — Layer D (Deferred Systems)

Systems beyond v1. Each will slot in as a new `rust/sim/src/systems/*.rs` file plus new
`AdminUnit`/`Nation`/`World` fields, **without modifying the existing systems** — exactly the
extensibility the engine was built for (`systems/mod.rs`: "they will slot in as additional
systems and additional unit fields without touching these six"). One paragraph each on what
they do and where they hook in. No full specs yet.

## Energy grid

Replaces the constant `Tuning::energy_available = 1.0` with a **per-unit** `energy_available`
derived by a new energy system. Power plants become a buildable infrastructure type; the grid
reuses the Connectivity adjacency model (energy can flow along edges). **Hook:**
`industries::industry_efficiency` already multiplies by `energy_available` — make it per-unit
and produce it in a system that runs before industries, alongside connectivity. Brownouts in
under-supplied regions cap industrial output.

## World-market cycles

Promotes the v1 *exogenous, scripted-beats* `World` struct into a fully **endogenous, cyclical**
system: commodity supercycles, price shocks, demand crashes, supply gluts that respond to global
supply & demand. **Hook:** it *is* the `global_demand` system, upgraded from a fixed authored event
timeline to real dynamics. Zero new plumbing — only `World`'s update rule changes. (This is
precisely why v1 keeps the world layer exogenous: the seam is pre-built so this becomes a
content/tuning change, not a refactor.)

## International tourism & FDI

The two trade channels cut from v1 (which ships exports + imported inputs only). **Tourism
arrivals:** international visitors flow in through airports/ports, scaled by global tourism demand
and a region's attractiveness (scenery, `coastal`, heritage), driving the existing Tourism
industry beyond its domestic base. **FDI:** foreign capital flows to gateway-equipped, educated,
well-connected regions, accelerating industry growth. **Hook:** both are additional channels in the
`trade` system + new derived fields (`tourism_arrivals`, `fdi`) added to the `aggregate_up` fold —
no change to exports/imports. Tourism arrivals reuse the gateway + connectivity reach already built
for exports.

## Disasters

A stochastic event system (typhoons, earthquakes, volcanic activity) that damages
`transport_level`/gateways and spikes `poverty` in topology-exposed units — coastal +
low-`avg_elevation` units take storm surge; high-`terrain_ruggedness` units take landslides.
**Hook:** writes to existing fields between population and industries; reuses topology already
on `AdminUnit`. Pairs with a future flood-control infrastructure type and disaster-relief
spending that competes with development for the treasury.

## Migration / OFW & remittances

Two coupled flows. **Internal migration:** people drift from poor, low-`market_access` units
to high-opportunity ones, reshaping `population` over time. **OFW (overseas) & remittances:** a
derived `ofw_share` of the workforce works abroad and sends money home, an external inflow to
household income / `treasury` — historically enormous for the Philippines. **Hook:** runs after
population, before industries; consumes `market_access` and `daily_wage` differentials;
remittances feed the same wage→poverty path industries already computes; the Connectivity graph
carries internal-migration friction.

## Unrest / stability / coup

A pressure system reading sustained low `satisfaction` / high `poverty` per unit, accumulating
a derived `unrest` stock that can trigger regional instability, force snap elections, or end
the game in a coup — a loss condition distinct from bankruptcy and election defeat. **Hook:**
runs after politics; reads its outputs; adds a second/third loss condition beside `bankrupt`
and election loss.

## Corruption / "the Broker"

A hidden-economy lever that skims a fraction of tax collection and build spending into the
player's personal wealth, trading short-term `political_capital` (the currently-inert `Nation`
field) for long-term `satisfaction` erosion and `unrest` risk — plus an assassination/scandal
tail risk. **Hook:** activates the dormant `political_capital` field; intercepts the economy
system's tax collection and the build actions in `engine.rs`. The morally-grey core of the
late game.
