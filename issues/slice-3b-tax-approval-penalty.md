---
id: slice-3b
title: Tax → approval penalty (sim)
status: todo
type: AFK
lead: sim-engine
blocked-by: []
harness: cargo
verify: cargo test -p politiko_sim
---

## What to build
Implement the **settled Rev.2 decision**: tax carries a *direct approval penalty* in `politics.rs`
(in addition to suppressing reinvestment), governed by a new `Tuning` constant. Without it, the
slice-3a lever has no interesting downside. Ripples to `prd/politics.md` + `techspec/politics.md`.

## Acceptance criteria
- [ ] A test asserts higher `tax_rate` lowers `satisfaction`/approval, all else equal (monotonic).
- [ ] The new constant lives in `Tuning`; no inlined literal.
- [ ] `cargo test -p politiko_sim` passes.

## Verification
`harness: cargo` → `cargo test -p politiko_sim`. New test asserts monotonicity (two runs identical
except `tax_rate`; higher tax ⇒ ≤ approval). Assert relationships, not magnitudes.

## Notes
Settled decision — do not re-litigate (see `docs/REVIEW-TRACKER.md`). spec-reviewer propagates the
new constant + behavior into `prd/politics.md` and `techspec/politics.md` on sign-off.
