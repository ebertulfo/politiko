# Concept: Corruption (Statesman core mechanic — IN DESIGN)

- **Status:** Brainstorm in progress (2026-06-06). **Not yet settled, not PRD'd.** This is the
  candidate for Statesman's missing **stakes layer** — the political tension the doc
  [`statesman.md`](statesman.md) flags as "the next thing to design."
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

## Open questions (resume here)

- **THE KNOT — what do you lose when you lose the mandate?** The shadow loop only has value if
  staying in power *costs* something to lose. *Left deliberately unresolved.* Candidates aired:
  (a) **lose capability/runway** — a new administration freezes levers / reverses policy / imposes
  an austerity term (you keep playing, governing gets harder); (b) **lose score/legacy only** — the
  mandate is the scoreboard, little changes turn-to-turn. **Corruption and election-stakes are the
  same design knot — neither is finishable without the other.**
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

## Research foundation (round-one deep-research, 2026-06-06)

Mechanism sources behind the above (verified, with confidence flags in the research briefs):
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
- **Bueno de Mesquita & Smith**, *The Dictator's Handbook* — selectorate theory (basis for the
  shadow loop; to be hardened by a planned round-two dive on the political economy of patronage).
- **Philippines context** — padrino/*utang na loob*; PDAF/Napoles (~₱10B); DPWH "SOP" 20–30%;
  2025 flood-control scandal (421 ghost projects); dynastic saturation ~75–85%; CPI 32/100 (TI 2025).

> **Note:** the *party-upside / shadow loop* (selectorate theory, machine politics, vote-buying,
> rents-as-glue) has **not** yet had a dedicated deep-research pass — it was reasoned from theory.
> A round-two research dive on the political economy of patronage is the planned way to harden it.
