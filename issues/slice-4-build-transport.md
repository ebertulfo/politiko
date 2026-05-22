---
id: slice-4
title: Build transport in a region
status: todo
type: AFK
lead: godot-ui
blocked-by: [slice-2]
harness: godot
verify: res://verify_slice4.gd
---

## What to build
Select a region from the table; show its `transport_cost`; a **Build** button calls
`build_transport(id)`. On success, treasury drops and `transport_level` rises; on failure, show the
returned `reason` (e.g. `InsufficientFunds`, `NotOperational`).

## Acceptance criteria
- [ ] Building in rugged inland CAR costs more than flat coastal NCR (topology mechanic is visible).
- [ ] A failed build surfaces the `reason` string; treasury is unchanged on failure.
- [ ] Engine support already exists — no Rust change expected.

## Verification
`verify_slice4.gd` asserts `transport_cost(CAR) > transport_cost(NCR)`; performs a successful build
and asserts treasury dropped by the cost and `transport_level` rose by 1; forces a failure and
asserts treasury unchanged and a non-empty `reason`. Exit 0.

## Notes
Needs the region selection from slice-2. Engine support already exists.
