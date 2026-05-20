//! Tuning knobs. Every magic number in the simulation lives here so designers
//! can rebalance without touching logic. All values are starting points from the
//! Phase 0.1 brief and are meant to be tuned.

use serde::{Deserialize, Serialize};

use crate::types::IndustryType;

/// All simulation tuning parameters. One instance lives on [`crate::Sim`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tuning {
    // --- Topology coupling (infrastructure build cost) ---
    /// How sharply ruggedness raises build cost. `BuildCost = Base × (1 + rugged × k)`.
    pub k_rugged: f64,
    /// Multiplier applied when a unit is coastal (cheaper to develop). 0.8 = 20% off.
    pub coastal_discount: f64,
    /// Base PHP cost to raise `transport_level` by one, before topology.
    pub base_transport_cost: f64,

    // --- Population (matches brief formula) ---
    /// `GrowthRate = base_growth - education×edu_growth_damp + poverty×poverty_growth_boost`.
    pub base_growth: f32,
    pub edu_growth_damp: f32,
    pub poverty_growth_boost: f32,

    // --- Labor & wages ---
    /// Fraction of population that is working-age / in the labor force.
    pub working_age_share: f64,
    /// Workers demanded per unit of industry level.
    pub labor_per_level: f64,
    /// Fraction of industry output paid out as wages (drives poverty).
    pub labor_share: f64,
    /// Monthly baseline cost of living per capita, PHP.
    pub cost_of_living: f64,

    // --- Industry output ---
    /// Energy availability multiplier. v1 has no energy grid → constant 1.0.
    pub energy_available: f64,
    /// Per-level efficiency bonus from each point of `transport_level`.
    pub transport_efficiency_bonus: f64,

    // --- Economy / treasury ---
    /// Monthly PHP maintenance per point of industry level.
    pub maintenance_per_industry_level: f64,
    /// Monthly PHP upkeep per point of `transport_level`.
    pub upkeep_per_transport_level: f64,
    /// Monthly interest charged on negative treasury (debt). 0.05 = 5%/mo.
    pub debt_interest_rate: f64,
    /// Debt magnitude at which the nation is bankrupt (IMF default game-over).
    pub debt_ceiling: f64,

    // --- Education ---
    /// Per-tick fraction of the gap to the investment target that is closed. Slow.
    pub edu_gain: f32,

    // --- Politics / satisfaction ---
    pub w_poverty: f32,
    pub w_employment: f32,
    pub w_education: f32,
    pub w_infrastructure: f32,
    /// `transport_level` considered "fully served" for the satisfaction infra score.
    pub target_transport: f32,
    /// Months per presidential term. Election fires when `tick % term_length == 0`.
    pub term_length: u32,
}

impl Default for Tuning {
    fn default() -> Self {
        Self {
            k_rugged: 2.5,
            coastal_discount: 0.8,
            base_transport_cost: 5.0e9,

            base_growth: 0.015,
            edu_growth_damp: 0.01,
            poverty_growth_boost: 0.008,

            working_age_share: 0.6,
            labor_per_level: 180_000.0,
            labor_share: 0.45,
            cost_of_living: 9_000.0,

            energy_available: 1.0,
            transport_efficiency_bonus: 0.04,

            maintenance_per_industry_level: 0.3e9,
            upkeep_per_transport_level: 0.15e9,
            debt_interest_rate: 0.05,
            debt_ceiling: 5.0e12,

            edu_gain: 0.02,

            w_poverty: 0.35,
            w_employment: 0.25,
            w_education: 0.20,
            w_infrastructure: 0.20,
            target_transport: 6.0,
            term_length: 72,
        }
    }
}

impl Tuning {
    /// Full-efficiency monthly output (PHP) per point of level for an industry.
    pub fn base_output(&self, kind: IndustryType) -> f64 {
        match kind {
            IndustryType::Agriculture => 1.6e9,
            IndustryType::Manufacturing => 4.2e9,
            IndustryType::Services => 5.5e9,
            IndustryType::Tourism => 2.2e9,
            IndustryType::Mining => 3.2e9,
        }
    }

    /// Education level (0..1) at which an industry reaches full labor efficiency.
    /// Below this, efficiency scales linearly with `education_level / required`.
    pub fn required_education(&self, kind: IndustryType) -> f64 {
        match kind {
            IndustryType::Agriculture => 0.20,
            IndustryType::Mining => 0.30,
            IndustryType::Tourism => 0.40,
            IndustryType::Manufacturing => 0.60,
            IndustryType::Services => 0.70,
        }
    }
}
