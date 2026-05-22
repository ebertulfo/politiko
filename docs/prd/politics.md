# PRD: Politics & Approval

- **Status:** Built — **Layer:** A — **Tick slot:** 9

## 1. Player Fantasy

The verdict. Everything you do is ultimately judged by the people, and every six years they
decide whether you keep the job. Approval is the scoreboard and the pressure — the reason you
can't just optimize the economy in a vacuum and ignore where people actually live.

## 2. Core Mechanic

Each region has a **satisfaction** level blended from how its people are doing: low **poverty**,
high **employment**, rising **education**, and adequate **infrastructure** (plus connectedness in
v1). National **approval** is the population-weighted average of regional satisfaction — so
keeping the big population centers happy matters more, but neglected regions still drag you down.

Every **72 months** an **election** fires. Approval converts to a vote share; above 50% and you
keep governing, otherwise it's game over.

## 3. Player Decisions

Politics isn't a lever you pull directly — it's the lens that makes the *spatial distribution* of
your other decisions matter. It forces questions like: do I pour resources into the populous
capital region (most approval weight) or shore up a poor region that's dragging the average and
could become a future crisis?

## 4. Feedback & Legibility

- National approval is a permanent dashboard readout, ideally with a trend.
- The map colored by satisfaction shows *where* you're winning and losing.
- A countdown to the next election creates rising tension; the game auto-pauses on election.
- The election screen reports the vote share and the win/lose outcome.

## 5. Progression / Arc

Early: approval reflects the starting state; you're triaging. Mid: your investments start moving
the needle, region by region. Late term: the scramble before the election — where can you still
move satisfaction in the time left? (Reinforcing that late investment in slow systems like
education won't save you — you had to start early.)

## 6. Failure Modes

**Election loss (game over)** at term end with vote share ≤ 50%. Getting there by: ignoring poor
regions, letting poverty rise, under-building infrastructure, or chasing economic numbers while
satisfaction quietly erodes.

## 7. Interactions

- **Reads:** poverty, employment, education, transport (and market access in v1) — i.e. the
  outputs of every other system.
- **Writes:** satisfaction (per region), national approval, the election result.
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

The satisfaction weights encode what the game thinks "good governance" is — currently poverty
matters most, then jobs, then education and infrastructure. These weights are the heart of the
game's values and the primary difficulty dial. The approval→vote-share mapping is a deliberate
seam for future incumbency curves; v1 keeps it simple (1:1) so the player can reason about it.

## 9. Out of Scope (v1)

Regional/sectoral political blocs, scandals, coups, snap elections, approval penalties for taxes
(Layer D — unrest/corruption). v1 has exactly one election, at term end, decided by average
satisfaction.
