# PRD: Population

- **Status:** Built — **Layer:** A — **Tick slot:** 1

## 1. Player Fantasy

The country is alive and changing under you. People aren't a fixed number you optimize around —
they grow, and *how* they grow reflects how well you're governing. A prosperous, educated region
stabilizes; a poor one balloons with people you then have to feed and employ.

## 2. Core Mechanic

Each region's population grows (or shrinks) every month at a rate driven by two of your
outcomes: **education slows growth** (educated populations have fewer children) and **poverty
speeds it up** (the classic poverty–fertility link). Population is the base everything else
multiplies against — more people means more potential workers, but also more mouths and higher
expectations.

## 3. Player Decisions

Population is not directly controllable — it's a *consequence*. The player influences it
indirectly through the two levers that feed it: education investment (slows growth, long-term)
and anything that reduces poverty (industries, jobs, wages). The decision it forces: do you
invest to bend the demographic curve, or ride a fast-growing poor population for cheap labor
and accept the instability?

## 4. Feedback & Legibility

- Region panel shows current population and its trend (growing/shrinking).
- The player should be able to see *why* — a poor region visibly growing fast, an educated one
  leveling off.
- Because population reacts to **last month's** economy, the UI should show trend lines, not
  just this month's snapshot, so the lag is legible rather than confusing.

## 5. Progression / Arc

Early game: poor regions grow fast, creating pressure. Mid game: as education and jobs land,
growth moderates and the workforce quality rises. Late game: a well-run country has stable,
educated, productive population centers.

## 6. Failure Modes

Neglect leads to runaway growth in poor regions — more people, same few jobs, rising poverty,
falling satisfaction. Population growth without matching industry/jobs is a downward spiral the
player can fall into by ignoring the poor regions.

## 7. Interactions

- **Reads:** education and poverty (the latter from last tick's industries).
- **Feeds:** industries (labor supply), economy (via revenue), politics (more people = more
  weight in approval).
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

Growth should be slow enough to feel demographic, not exponential-blowup. The education and
poverty terms should be strong enough that the player can *see* their governance bending the
curve over a term, but not so strong that one bad year wrecks a region.

## 9. Out of Scope (v1)

Internal migration between regions and overseas workers/remittances (Layer D). In v1 a region's
population only changes through births/deaths-style growth, never by moving.
