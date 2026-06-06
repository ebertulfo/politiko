# Concept: Corruption (Statesman core mechanic — IN DESIGN)

- **Status:** Brainstorm in progress (2026-06-06). **Not yet settled, not PRD'd.** This is the
  candidate for Statesman's missing **stakes layer** — the political tension the doc
  [`statesman.md`](statesman.md) flags as "the next thing to design." Two deep-research rounds
  done (mechanics of corruption; political economy of patronage/survival). The election-stakes
  knot now has a **proposed resolution** (see "The deferred bill" below).
- **Belongs to:** Statesman (Game 1). Builds on the existing engine; adds one new per-unit state
  field + effects, no new crate.
- **Theme it carries:** the Statesman shadow — *"a movement that overthrew the dynasties could
  calcify into the next one. Will you become the thing you replaced?"* Corruption is that theme
  made mechanical.

## One-line pitch

Corruption is the **instrument that lets you stay in power and get things done faster — by
spending the country's long-term health and your own founding ideals.** It is political debt: you
borrow speed and survival now and repay it with compounding rot. Lean on it and you become the
dynasty you replaced.

## Why this exists

Statesman's known weakness: it's *a competent economic sim that still needs stakes* — a reason to
care beyond a high score. Corruption is the answer because it is the one mechanic that is
**simultaneously** (a) a tempting tool the player *wants* to use, (b) a slow-burning cost, and
(c) the literal embodiment of the founding story's shadow. It turns "govern the country" into a
moral/strategic tension instead of an optimization.

## The core idea: two loops to staying in power

Statesman already has an **honest loop**:

```
HONEST:  build → prosper → satisfaction → approval → survive the 72-month mandate
```

Corruption opens a **shadow loop** that reaches the same goal — staying in power — *without* going
through prosperity:

```
SHADOW:  skim / grease → war chest + patronage → buy approval, loyalty & compliance directly
                              → survive the mandate, decoupled from whether the country improved
```

This is **selectorate theory** (Bueno de Mesquita & Smith, *The Dictator's Handbook*): leaders
survive by rewarding the coalition that keeps them in power. A *broad* coalition must be paid in
**public goods** (real prosperity — the honest loop); a *small, bought* coalition is paid in
**private goods** (patronage — the shadow loop). The reform Movement starts needing the whole
country's faith, but it can quietly **shrink its coalition** — survive on fewer, well-fed
loyalists. The day it does, it has *become a dynasty by definition* (a dynasty just *is* a
small-coalition patronage machine). That coalition-shrink **is** the calcification.

### What corruption actually buys the *party* (the upside we must not forget)

The headline upside of corruption is **not** "a faster road." It is to the actor doing the
corrupting. Concretely, corruption buys:

1. **A war chest** — skimmed rents as off-books money the player redirects (patronage, favored
   regions, bridging a budget gap).
2. **Bought approval / loyalty** — patronage props up the numbers *even when the economy is flat*,
   so the player survives the mandate without delivering.
3. **Compliance** — captured regions and bought officials execute the agenda without resistance.

"Build faster, grease the wheels" is one *minor* use. The real temptation is: **corruption lets
you keep power when governing honestly isn't winning fast enough.** That is the reformer's
Faustian bargain and the tension the sim was missing.

### The formal spine: coalition breadth (W)

Selectorate theory (Bueno de Mesquita & Smith, *The Logic of Political Survival*; *The Dictator's
Handbook*) gives the two loops a single load-bearing variable: the size of the **winning coalition
W** — the base whose support you actually need to keep the mandate. (Empirically the *direction* is
robust; specific magnitudes are contested — treat W as a design device, not a calibrated quantity.)

- **Large W (broad base)** → you can only satisfy many people with **public goods** → survival is
  *coupled* to prosperity → **the honest loop.** Expensive; you can genuinely lose.
- **Small W (narrow base)** → you satisfy a few with **private goods** (patronage, rents) →
  survival is *decoupled* from prosperity → **the shadow loop.** "Kleptocracy flourishes at exactly
  this configuration."

The pivotal mechanic: **W is never a slider the player sets — it is *derived* from how the player
buys survival** (fits the engine's derived-vs-authored rule). Win on broad satisfaction/public
goods → W large. Survive by paying patrons/machines/local rents → **each act of patronage
implicitly shrinks W.** And shrinking W is *seductive*, because a small coalition is:

- **cheaper** to maintain (`cost of survival ∝ W` — "pay just enough, not a penny more"),
- **more coup-/defection-proof** (the *loyalty trap*: `loyalty ∝ 1 − W/S` — a big pool of
  interchangeables means your few loyalists dare not defect),
- **longer-tenured** (`tenure ∝ 1/W` — small-coalition rulers last far longer),
- **more graft headroom** (`1/W`).

So the country rots (public metrics fall) while the player becomes *more* secure and *longer*-lived.
**That is "becoming the dynasty" — not a scripted morality meter, but a low-W attractor the player
slides into because it is locally optimal every turn.** The theme emerges from the math.

### The "buy approval" step (machine-politics dials)

When the player taps patronage to prop up support, model it with the clientelism literature's
texture (Stokes; Nichter; Ravanilla) rather than a clean transfer:

- `delivered support = spend × conversion × (1 − slippage)` — **slippage** is secret-ballot
  defection (bought support evaporates); only **reciprocity** retains it.
- A per-unit **reciprocity/loyalty stock** that rises with repeated spending and lowers slippage —
  the self-reinforcing flywheel (the machine gets stickier; a fresh challenger faces full slippage).
- **Conversion scales inversely with income** — patronage is cheaper/more effective in poorer
  units (high marginal utility of cash), coupling it to existing `poverty`/`daily_wage`. As a region
  develops, the machine's grip naturally weakens (the modernization tendency, for free).
- A **payment floor** (tiny spends do nothing) + concave returns to block micro-spend exploits.

## The grease curve (corruption is not a debuff — it's leverageable)

Per **Yuen Yuen Ang** (*China's Gilded Age*): *theft* (skimming, ghost projects) drains, but
*access money* (elite exchange / greasing / patronage) can genuinely **produce output** while
accumulating systemic risk — "theft is a toxic drug, access money is steroids." So corruption's
effect is an **inverted-U with a cliff**, not a straight penalty:

```
            throughput / output           leakage / rot
   clean  │      ___                    │              ___________
          │    /     \  ← CLIFF         │            /
          │  /         \                │  ________/
          │/             \_____         │ /
          └───────────────────────      └───────────────────────
            0   grease    redline 1       0          redline   1
                 zone     (tipping pt)            (rises, then dominates)
```

- **Clean (low):** honest but *slow* — full bureaucratic friction; captured/rugged regions drag.
- **Grease zone (moderate):** the steroid band — cheaper cooperation, less friction, faster
  builds, a bump to output. **Net positive: throughput gain > leakage cost.** "Getting shit done."
- **Past the redline (high):** the effect *inverts* — leakage dominates, ghost projects, scandals,
  and the region falls into the **captured basin that is expensive to climb out of** (hysteresis).

The player's skill is **riding the productive band** — but see *The trap* below: the band is
unstable and wants to slide into rot.

## The trap (why this can't be exploited as a free lunch)

From the multiple-equilibria literature (Andvig & Moene; Stephenson; Caulkins et al.): corruption
is **frequency-dependent** — detection odds and moral cost both *fall* as corruption rises, so it
**compounds when high**. This gives **two stable basins (clean / captured) with an unstable tipping
threshold between them**, and **hysteresis** (cheap to enter, expensive to exit — model the exit
threshold higher than the entry threshold, or cleanup cost super-linear in entrenchment).

Consequences for the design:
- The **productive grease band is unstable** — a greased region's corruption creeps upward on its
  own (contagion + compounding). The redline drifts *toward* you.
- Holding the band therefore demands **continuous accountability investment** (the slow, costly
  brake). Leverage is a **treadmill, not a free lunch.**
- A one-tick crackdown can't save a captured region — only **sustained** pressure past the
  threshold flips it back (big-push *or* accumulated-incremental; both just "stay above the cleanup
  rate long enough to cross the threshold").

## Autonomous climate + government influence (not a thing you "operate")

Corruption is an **ambient regional property the player governs**, not a machine the player runs
directly:

- **Autonomous:** each region's corruption evolves every tick (Klitgaard pressure + contagion) —
  businesses grease, officials skim — without player action.
- **Influenced by policy toggles:** enforcement intensity, transparency/e-gov, red-tape,
  civil-service pay, deregulation — these shift the *whole curve* for everyone (climate-level
  accelerator/brake).
- **Tapped directly, occasionally, via patronage actions:** the active war-chest / grease button
  (deliberate leverage — the *utang na loob* favor).
- **The economy leverages it too**, via the same inverted-U: moderate corruption can *raise* a
  region's output (access money greases business), only dragging past the redline. So corruption is
  **not** a big direct industry debuff — neutral-to-helpful in the band, toxic past it.

## Klitgaard as the per-unit engine

`Corruption = Monopoly + Discretion − Accountability` (Klitgaard, *Controlling Corruption*, 1988).
A heuristic, not literal math — adapted as the per-tick **pressure** on each unit:

```
pressure = w_M·monopoly + w_D·discretion − w_A·accountability + w_C·contagion
corruption (a slow STOCK with memory) → evolves toward pressure, compounds above the threshold,
                                        decays only under sustained anti-corruption pressure
```

- **Monopoly / Discretion — endogenous (player-created):** concentrating power, rushing big
  builds in one region with no oversight, pressing patronage. *This is the "you did this" half.*
- **Accountability — exogenous (inherited per-region seed):** weak institutions you start with;
  slowly raisable by institution-building levers. *This is the "you inherited this" half.*
- **Contagion — the trap term:** pressure rises when the unit (and neighbors/parent) are already
  corrupt.

This resolves the "endogenous **or** exogenous?" question as **both**, cleanly split by term.

## Effects (where corruption touches the existing sim)

A per-region `leakage_rate` derived from `corruption` (objective anchors from research: floor
~20–28% of an infra budget vanishes, extreme capture ~85%, low-income public investment 30–53%
wasted). Every government peso spent in a region is multiplied by `(1 − leakage)` before it
delivers. Two **physically distinct graft signatures** (Olken):

- **Price-padding** → `effective_build_cost ×= 1/(1 − leakage)` → you build **less** per peso
  (grease = faster builds at *higher peso cost* — "grease needs grease").
- **Materials-skimming** → `build_quality` down → builds **decay faster** (hook straight into the
  engine's existing terrain-driven decay; corrupt low-quality builds fail faster in rugged/coastal
  regions).

Plus the **party-upside** flows (war chest / bought approval / compliance) wired into treasury,
the politics/approval system, and regional policy compliance.

## Anti-corruption: levers with cost, lag, decay, and backfire

Each lever costs budget/political capital **now**, pays off **later/partially**, and **decays** —
so fighting corruption *competes with building*. None is permanent (officials adapt and displace):

| Lever | Effect (evidence-anchored) | Cost | Catch |
|---|---|---|---|
| **Random audits** | ~8pp leakage cut + media spillover to neighbors (Olken; Brazil) | per-tick budget | must be **unpredictable**; decays if not sustained |
| **Digitize payments / e-gov** | big one-time cut (~40% relative on covered flows; India Smartcards) | high upfront + multi-tick rollout | benefit only at **full** rollout |
| **Civil-service pay / merit** | modest, **conditional** reduction | permanent wage bill | **weak alone** — needs audits running too |
| **Independent anti-graft body** | deepest long-run cut; raises the *thresholds* permanently (HK ICAC) | high sustained funding + political capital | **decades-long lag**; can be **weaponized** (legitimacy/backfire risk); fails without rule-of-law |

## Scandal as event (the visible reckoning)

When hidden corruption crosses a threshold or a random audit triggers, a **scandal** fires →
approval crash, asset freezes, official resignations, mandate pressure. Template: the real 2025
Philippine **flood-control scandal** (421 confirmed ghost projects of ~8,000 audited; a standard
**20–30% "SOP" kickback** on public works; contractors left 2–5% margin → substandard/ghost work).

## National scoreboard

A **CPI-style index**, rolled up via the engine's single `aggregate_up` path from per-unit
corruption/accountability state — the number the player watches trend. Authentic: the real
Philippines sits at **32/100 and falling** (Transparency International 2025, an all-time low). Framed
as slow national decay, **never a game-over** (consistent with the settled non-fatal / open-ended
rules).

## The deferred bill: what you lose when you lose the mandate (PROPOSED resolution of the knot)

The shadow loop only has teeth if losing power costs something — and the research gives a resolution
that is *self-balancing*: **the shadow loop builds its own punishment.** While in power, banked rents
and a narrowed coalition are *assets*; the moment the player loses the mandate, they lose immunity and
those assets flip into liabilities. Three deferred bills, all **non-fatal** (a brutal setback, never
game-over — honors the settled rules):

1. **The reckoning.** Accumulated corruption becomes prosecutable — asset/war-chest seizure, charges.
   *Grounded:* ~half of ousted autocrats face exile/jail/death; 78+ countries have prosecuted
   ex-leaders since 2000, charges *overwhelmingly* corruption. **Severity scales with `1/W`** — a
   narrow/personalist regime draws a harsh fate (~75% bad outcome), a broad/clean one exits safe
   (<20%). *How* you held power sets *what* you lose.
2. **The reversal.** The successor freezes and rolls back the player's levers and in-flight projects
   wholesale (real anchor: a new administration rescinding ~78 predecessor orders in one stroke). Your
   legacy decays; you re-enter from a weaker position. Tunable `reversal_fraction`.
3. **The bust-snap.** Rent-bought loyalty is *fair-weather* — it holds in a boom and **snaps in a
   crisis**. A small-W regime is durable while prosperity rises but **brittle** when the economy turns:
   a downturn (debt crisis / wage drop / growth stall) detonates dormant `grievance` into
   scandal/unrest, and the bought coalition defects *during* the crunch. The same corruption that was
   glue becomes the detonator. (Anchor: the corruption-stability brake vanishes in downturns; rent
   loyalty snaps in crises.)

Net: the cheap survival path compounds a hidden liability and a brittleness that detonates exactly when
the player can least afford it. The "soft loss" election finally *means something* without a campaign
minigame — you are still building a nation; the stakes just became real. **Corruption and
election-stakes were the same knot; W ties them together.**

## Settled-so-far (this brainstorm)

1. **Corruption is leverageable, not a pure debuff** — inverted-U grease curve (Ang's access money).
2. **Its real upside is to the *party*** (survival/war chest/compliance), not just the corrupted
   layer — the shadow loop vs. the honest loop (selectorate theory).
3. **It's an autonomous regional climate** the player governs via policy toggles + occasional
   direct patronage — not a system the player hand-operates.
4. **Per-municipality stock with memory** (not a per-tick derived value), on the generic
   `AdminUnit`, rolled up via `aggregate_up`. City/Municipality is the **decision layer** for v1.
5. **Self-reinforcing trap** (two basins, tipping point, hysteresis) → leverage is a treadmill.
6. **Non-fatal**, CPI-style scoreboard, scandals as events. Honors the open-ended/no-game-over rules.
7. **Grease = faster builds at higher peso cost.**
8. **Coalition breadth (W) is the formal spine** — derived (never set), shrunk by patronage; small W
   = cheaper/coup-proof/longer-tenured survival decoupled from prosperity = the dynasty attractor.
9. **The election-stakes knot has a proposed resolution: the deferred bill** (reckoning + reversal +
   bust-snap, severity ∝ 1/W). Self-balancing; the shadow loop builds its own punishment.

## Open questions (resume here)

- **THE KNOT — what do you lose when you lose the mandate? → PROPOSED (needs sign-off).** Resolved
  above as **the deferred bill** (reckoning + reversal + bust-snap, severity ∝ 1/W). Awaiting the
  user's confirmation that this is the direction. The earlier candidates (lose capability/runway;
  lose score only) are *subsumed* — reversal = lose runway, the reckoning/seizure = lose more.
- **Ambition: minimal spine vs. full model.** Minimal = corruption-per-region (done) + one derived
  **W** + patronage shrinks W + losing triggers a reckoning scaled by W. Full adds the grievance
  reservoir, broker slippage/reciprocity stocks, civil-service levers, the bust-snap as its own
  system. *Which do we build first?* (Guards the "nation-building, not a politics sim" line.)
- **What exactly does "grease" buy** beyond build speed — also industry output? compliance? Keep to
  1–2 for legibility. (Industry-output coupling flagged as raw — the economy should *leverage*
  corruption, not be dominated by it.)
- **The favor economy** — is patronage a finite resource pool (political capital you accrue/manage)
  or free-to-press and priced purely in the corruption it generates?
- **Ang's "access money" depth** — keep the morally-grey "some corruption greases growth" (richer,
  truer) vs. simpler clean-cut drag (more legible)? Currently leaning grey.
- **Barangay layer** — corruption fields live on the generic `AdminUnit` so a future barangay LOD
  (petty/front-line corruption, ~42k units) "just works," but it is **out of scope for v1**
  (perf + connectivity-hierarchy + data-pipeline costs; not a decision layer).
- **How much is "politics"?** Keep the lever framed as *building faster / surviving to keep
  building* (nation-building), not a campaign minigame (that's Trapo's domain).

## Relationship to the engine (load-bearing)

- New per-unit fields (`corruption`, `accountability`) on the **flat `Vec<AdminUnit>`** — role
  derives from the tree, no special-casing; rolls up via the single `aggregate_up` path.
- `corruption` is a **stateful stock with memory** (like `education_level`), *not* a per-tick
  derived value (like `satisfaction`) — required by the hysteresis/compounding dynamics. Worth an
  explicit engine note.
- The **leakage multiplier sits on flows that already exist** (gov spending, build cost/quality,
  industry output, approval) — a coefficient, not a new subsystem.
- All magnitudes live in `constants.rs::Tuning`; tests assert **relationships** (higher corruption
  → less built and/or faster decay; sustained audits monotonically lower leakage; the band is
  net-positive below threshold and net-negative above), never exact magnitudes.

## Relationship to Trapo (Game 2)

Same domain, mirror image. Statesman governs corruption as an **ambient climate** from the top
(non-fatal, score = the nation). Trapo *operates* corruption from the **inside** as a family
(personal, fatal — jail/extinction). Statesman's `corruption`/patronage climate is the world
Trapo's dynasty swims in. The mechanics are deliberately compatible.

## Research foundation (two deep-research rounds, 2026-06-06)

**Round one — mechanics of corruption.** Sources behind the above (with confidence flags in briefs):
- **Klitgaard**, *Controlling Corruption* (1988) — `C = M + D − A`.
- **Olken**, *Monitoring Corruption* (NBER w11753 / JPE 2007) — ~24–28% measured "missing
  expenditures"; price-padding vs. quantity-skimming; audits ~8pp.
- **Reinikka & Svensson** (World Bank WP 2709 / QJE 2004) — Uganda ~87% leakage; transparency
  collapses it.
- **IMF PIMA** — public-investment efficiency loss ~30% (up to ~53% low-income).
- **Andvig & Moene (1990); Stephenson (2020); Caulkins et al. (2013)** — frequency-dependence,
  multiple equilibria, tipping points, traps.
- **Persson, Rothstein & Teorell (2013)** — collective-action vs. principal-agent; "why
  anticorruption reforms fail."
- **Yuen Yuen Ang**, *China's Gilded Age* (2020) — the theft/exchange × elite/non-elite 2×2;
  access money as growth-greasing steroids.
- **Philippines context** — padrino/*utang na loob*; PDAF/Napoles (~₱10B); DPWH "SOP" 20–30%;
  2025 flood-control scandal (421 ghost projects); dynastic saturation ~75–85%; CPI 32/100 (TI 2025).

**Round two — political economy of patronage & survival** (hardens the shadow loop + the knot):
- **Bueno de Mesquita & Smith**, *The Logic of Political Survival* / *The Dictator's Handbook* —
  selectorate theory: W → public vs private goods; loyalty norm `W/S`; `tenure ∝ 1/W`;
  `kleptocracy ∝ 1/W`. *Direction robust; magnitudes contested (state capacity matters — ISQ 2015).*
- **Stokes; Nichter; Ravanilla et al.** — machine politics: brokers, vote- vs turnout-buying,
  secret-ballot slippage, reciprocity, targeting the poor; PH cost-per-vote ~₱250–500 (mayoral).
- **Grease-vs-sand (Leff/Huntington vs Myrdal); Gerschewski (legitimation/repression/co-optation);
  "Beyond Patronage"** — corruption as fair-weather glue: stabilizing in booms, snaps in crises;
  anti-corruption grievance drove protests in 55 countries since 2017.
- **Goemans/Archigos; post-tenure-fate datasets** — losing power: ~half of ousted autocrats face
  exile/jail/death; 78+ countries prosecuted ex-leaders since 2000 (overwhelmingly corruption);
  personalist rulers ~75% bad fate vs <20% for regular exits; successors reverse predecessors
  wholesale (~78 orders rescinded in one stroke).
- **Levitsky & Way (competitive authoritarianism); PRI, ZANU-PF, EDSA→dynasties (calcifiers) vs.
  Singapore PAP / HK ICAC / Botswana DCEC (stayed clean)** — calcification mechanism = funding
  survival through patronage + capturing the referees; clean path = a self-binding constraint
  (paid-for meritocracy, an independent watchdog you can't switch off) the incumbent submits to
  *while still powerful*. Nuance: it's the *threat of losing*, not term limits, that disciplines.
