//! # Politiko simulation engine
//!
//! This crate is the **state of truth** for the game. It is pure logic: it has
//! zero dependency on Godot or any rendering API (enforced at the crate level —
//! `godot` is not in this crate's dependency tree). Godot reads this state and
//! draws it; it never owns game data.
//!
//! ## The two non-negotiable structural rules
//!
//! 1. **Never hardcode "17" or special-case "region".** Everything iterates over
//!    a flat [`Vec`] of [`AdminUnit`]s. National totals are produced by rolling
//!    children up through [`AdminUnit::parent`] via [`aggregate::aggregate_up`],
//!    never by counting. The same code runs on ~1,600 municipalities later with
//!    only a data swap.
//! 2. **Topology is a mechanic.** Infrastructure cost reads
//!    `terrain_ruggedness` and `coastal` (see [`systems::infrastructure::build_cost`]).
//!
//! ## Tick order
//!
//! [`Sim::advance_tick`] runs the six base systems in dependency order so each
//! reads fresh data:
//! population → industries → economy → infrastructure → education → politics.

pub mod aggregate;
pub mod constants;
pub mod data;
pub mod engine;
pub mod systems;
pub mod types;

pub use constants::Tuning;
pub use engine::{BuildError, Sim, TickOutcome};
pub use types::{
    AdminLevel, AdminUnit, ElectionResult, IndustryInstance, IndustryType, InfrastructureState,
    Nation,
};
