# PRD: Education

- **Status:** Built — **Layer:** A — **Tick slot:** 8

## 1. Player Fantasy

The long game. Education is the investment that doesn't pay off this month, or next — but the
President who funds it builds a country that can run modern industries and lift itself out of
poverty. It's the lever that separates short-term vote-buying from real nation-building.

## 2. Core Mechanic

You set a single national **education investment** level. Each region's education slowly
converges toward it over many months. Education is the **ceiling on industry efficiency** — most
visibly for high-skill industries (Services, Manufacturing) that are dead weight in an
uneducated region. It also **slows population growth**, stabilizing poor regions over time.

## 3. Player Decisions

- **How high to set investment** — and to live with the fact that the payoff is delayed. Funding
  education is a bet that you'll still be in office (or that the country will) when it matures.
- The implicit tradeoff against everything else competing for the treasury *now*.

## 4. Feedback & Legibility

- Region panels show education level and that it's trending toward your investment setting.
- The **slowness must be legible** — the player should understand they're moving a level that
  takes ~years to close the gap, so they don't expect instant results and rage-quit the lever.
- The downstream effect (industries unlocking) should be visibly attributable to education.

## 5. Progression / Arc

Early: education is low; only low-skill industries work; the temptation is to ignore it. Mid: a
President who invested early sees Manufacturing/Services become viable across more regions. Late:
an educated workforce is the foundation of a high-output, world-connected economy.

## 6. Failure Modes

Neglecting education entirely — the country is locked out of high-value industries and poor
regions keep growing fast. Over-correcting late (raising investment in year 5 when there's no
time for it to mature before the election).

## 7. Interactions

- **Reads:** the national education investment lever.
- **Feeds:** industries (efficiency ceiling), population (slows growth), politics (a satisfaction
  component). Runs *late* in the tick because nothing downstream depends on it the same tick.
- See [`../system-interaction-map.md`](../system-interaction-map.md).

## 8. Tuning Intent

Convergence speed (`edu_gain`) is the signature knob: deliberately slow (~years to close the
gap) so education *feels* like a long-term commitment, not a quick fix. This slowness is the
whole design point — it creates the tension between short-term and long-term governance.

## 9. Out of Scope (v1)

Per-region education spending, school infrastructure as a build, brain-drain via emigration
(Layer D). Education is a single national lever in v1.
