//! Core data model. One flat list of [`AdminUnit`]s describes the whole nation
//! at any granularity. A region is a unit whose `parent` is the national root;
//! a municipality (future) is a unit whose `parent` is a region. Same struct
//! everywhere — no level is special-cased in logic.

use serde::{Deserialize, Serialize};

/// Where a unit sits in the administrative hierarchy. Used for display and data
/// authoring only — **logic must not branch on this to decide behavior**; it
/// branches on tree shape (`parent` / has-children) instead, so new levels slot
/// in without code changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AdminLevel {
    National,
    Region,
    Municipality, // future
    // Barangay,  // far future
}

/// The kinds of industry a unit can host.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IndustryType {
    Agriculture,
    Manufacturing,
    Services,
    Tourism,
    Mining,
}

/// One industry present in a unit, at some development `level`.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndustryInstance {
    pub kind: IndustryType,
    pub level: u32,
}

/// Infrastructure state for a unit. Only transport exists in v1; flood control,
/// energy, etc. are added as fields in later phases without touching the six
/// base systems.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InfrastructureState {
    /// Raises industry efficiency and lowers friction. Player-built.
    pub transport_level: u32,
}

/// A single administrative unit. Authored fields are the inputs; the `// derived`
/// block is recomputed every tick and must never be hand-authored as truth.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AdminUnit {
    pub id: u32,
    /// `None` for the national root; otherwise the id of the parent unit.
    pub parent: Option<u32>,
    pub name: String,
    pub level: AdminLevel,

    // --- Topology (drives infrastructure cost + feasibility) ---
    pub avg_elevation: f32,      // meters, hand-seeded now / SRTM later
    pub terrain_ruggedness: f32, // 0.0–1.0
    pub coastal: bool,

    // --- Population ---
    pub population: u32,
    pub education_level: f32, // 0.0–1.0

    // --- Economy ---
    pub industries: Vec<IndustryInstance>,
    pub infrastructure: InfrastructureState,

    // --- Derived per-tick (recomputed by the systems, not authored) ---
    pub satisfaction: f32,      // 0.0–1.0
    pub revenue_last_tick: f64, // PHP/month
    pub poverty: f32,           // 0.0–1.0
    pub employment_rate: f32,   // 0.0–1.0 of the workforce
    pub daily_wage: f64,        // PHP/day, per employed worker

    /// `true` when this unit has children, i.e. its values are an aggregate of
    /// its subtree rather than its own production. Set by the engine from the
    /// tree shape; the national root and (later) regions-with-municipalities are
    /// aggregates. The per-unit systems skip aggregates and operate on leaves.
    #[serde(default)]
    pub is_aggregate: bool,
}

impl AdminUnit {
    /// Leaf units that actually produce/grow. Aggregates (root, future parents)
    /// get their values from [`crate::aggregate::aggregate_up`] instead.
    pub fn is_operational(&self) -> bool {
        !self.is_aggregate
    }

    /// Total industry level summed across all industries in this unit.
    pub fn total_industry_level(&self) -> u32 {
        self.industries.iter().map(|i| i.level).sum()
    }
}

/// National-scope state that is not a property of any single unit. The state of
/// truth for treasury, approval, the clock, and national policy levers.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nation {
    /// Months elapsed. 1 tick = 1 month.
    pub tick: u32,
    /// PHP. May go negative (debt is allowed; interest accrues).
    pub treasury: f64,
    /// Fraction of national revenue collected as tax, 0..1.
    pub tax_rate: f32,
    /// Pop-weighted national approval, 0..1. Recomputed by the politics system.
    pub approval: f32,
    /// Education policy lever, 0..1 — the equilibrium `education_level` converges to.
    pub education_investment: f32,
    /// Reserved for later political-action mechanics (UI phase). Inert in v1.
    pub political_capital: f32,
    /// Months per term. Election fires when `tick % term_length == 0`.
    pub term_length: u32,
    /// Set true when treasury falls below the debt ceiling (IMF default).
    pub bankrupt: bool,
}

impl Default for Nation {
    fn default() -> Self {
        Self {
            tick: 0,
            treasury: 500.0e9, // ₱500B starting treasury
            tax_rate: 0.14,
            approval: 0.0,
            education_investment: 0.6,
            political_capital: 0.0,
            term_length: 72,
            bankrupt: false,
        }
    }
}

/// Result of an election, returned from the tick on which it fires.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElectionResult {
    pub tick: u32,
    pub approval: f32,
    /// Approval-derived vote share, 0..1.
    pub voteshare: f32,
    /// `true` if `voteshare > 0.5` — the player keeps governing.
    pub won: bool,
}
