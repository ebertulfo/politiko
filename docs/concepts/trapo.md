# Concept: Trapo (Game 2 — PARKED)

- **Status:** Captured, **not in development.** Parked behind Statesman (Game 1).
- **Protagonist:** the **family** (a dynasty, across generations). **Main UI: menus.**
- *Working title — Tagalog for a corrupt traditional politician, and literally "rag": a rags-to-riches pun.*

## One-line pitch

From a barangay hardware store to the power behind the president. Build a corrupt-contractor dynasty,
climb from the gutter to the national stage, and don't get caught. **Fuck around and find out.**

## Form

**Menu-first** — think *Football Chairman*: clean menus, character cards, decision cards, numbers. The
map (if present) is a stylized board / list of locations, **not** a living 3D economic simulation. Low
art and rendering burden — the form is *proven solo-shippable*, which is much of why it's appealing.

## Core loop

```
pick a profession + a starting barangay
  → run a thin business (hire competent people + simple supply/demand)
    → win & inflate GOVERNMENT CONTRACTS (the corruption vector)
      → skim the difference; fund / bribe / install politicians
        → gain influence; breadth unlocks height
          → climb: barangay → city → province → region → national
            → manage Heat & survival → exit: die out, flee, or retire rich
```

**Breadth unlocks height:** you can't bribe your way straight to the top — you must dominate enough of
a tier to reach the office above it. (Soft-gated: reckless overreach is *allowed*, just brutal.)

The loop is **fractal** — the same "win the contract, skim it, fund the official who awards the next
one" plays at every altitude (fix the basketball court → the national highway program), with the
watchdog scaling up alongside you (a nosy councilor → the Ombudsman → a Senate blue-ribbon probe).

## Starting profession (lean flavour, for now)

Your profession = *what you vend to the state* and *where you can start*. A small set, primarily
**flavour + a starting-position constraint** (farmer → rural barangays; software dev → anywhere).
Playstyle divergence comes **for free from the terrain** (a rugged rural start simply plays differently),
not from bespoke per-profession systems. Deep per-profession divergence is **parked**.

## The pawns (politicians as characters)

Stat block: **competence, integrity (inverse = corruptibility), loyalty (to you), ambition, charisma,
agenda** (reformist / populist / machine-trapo / technocrat). The central tension:

> A competent, high-integrity reformer makes the country thrive — but **won't be bought and may
> prosecute you.** A corrupt, pliable incompetent is **cheap to own** — but wrecks the regions he
> touches. Every appointment trades **control vs. quality.**

## You — the dynasty

The persistent unit is the **family**, not a person. Members (blood) can hold office, marry, inherit;
**operatives** (hired crew — fixer, lawyer, enforcer, accountant) execute plays. The patriarch is mortal;
the family endures.

**Levers:** *electoral* (bankroll, vote-buy, secure endorsements) · *coercive* (bribe, blackmail,
scandal, the dark tail) · *patronage* (pork to a region, contracts to your own firm, place family) ·
*social* (marriage alliances, media).

**Scores in tension:** **Fortune** (dynasty wealth) vs. **Legacy** (national prosperity you enabled),
plus **Dominion** (offices controlled) and **Heat/Infamy** (exposure).

## The governing law of failure

> **Game-over is personal — and "personal" means the *family*. Politics is just weather.**

- Head **assassinated** with an heir → **succession**, not game-over (a dramatic beat; some personal
  loyalty evaporates).
- Family **extinct** (no heir) or head **jailed** → game-over.
- **Flee** → escape (run ends). **Retire / cash out** → the *win* (bank the score; bloodline withdraws at
  its peak).
- Coups, lost elections, toppled pawns → **weather**: setbacks that wreck invested influence but don't
  end the run — *unless* they expose you personally (then they route into the knife or the cuffs).

So half the game is **managing your own survival** with the same levers (own the prosecutor, ally the
general, marry a dangerous rival close).

## Roguelike frame

A run ends on die / flee / retire. Meta-progression to design: bloodline reputation, an offshore war
chest (esp. via flee), unlocked professions, infamy carryover. **Signature idea:** past/fallen families
become **NPC dynasties in future runs** — the country is populated by the spiders you used to be.

## What it reuses

Domain knowledge + the corruption mechanics + the character-stat concept. **Not** the heavy spatial
engine — this is largely a fresh build with a different (menu/character-driven) architecture.

## Relationship to Statesman (Game 1)

[`statesman.md`](statesman.md) **is Trapo's endgame.** When the family controls the president, you are
steering the nation — the deep economic strategy (tariffs, industrial policy) becomes the *prize at the
top of the climb.* Whichever game we park is the other's seed or canopy, not wasted.

## Parked open questions

- Professions: confirm flavour-only for v1 vs. eventual deep divergence.
- Survival/security: first-class system or lighter background risk?
- Jail = family-over, or survive-in-disgrace?
- Succession: smooth (heir takes the chair) vs. contested (a scramble; rivals smell blood)?
- Meta-progression model + the NPC-dynasty idea.
</content>
