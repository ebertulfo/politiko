//! System 4 — Infrastructure. Topology-coupled. Gates development, lowers friction.
//!
//! Per-tick upkeep is charged by the economy system. The defining mechanic lives
//! in [`build_cost`]: raising `transport_level` is cheap on flat coastal lowland
//! and expensive in rugged inland terrain. The `run` step is reserved for future
//! per-tick infrastructure dynamics (decay, etc.) and is intentionally inert in
//! v1 so the slot exists without inventing mechanics.

use crate::constants::Tuning;
use crate::types::AdminUnit;

/// PHP cost to raise a unit's `transport_level` by one.
///
/// `BuildCost = base × (1 + ruggedness × k_rugged) × (coastal ? coastal_discount : 1)`
pub fn build_cost(u: &AdminUnit, t: &Tuning) -> f64 {
    let terrain = 1.0 + u.terrain_ruggedness as f64 * t.k_rugged;
    let coast = if u.coastal { t.coastal_discount } else { 1.0 };
    t.base_transport_cost * terrain * coast
}

/// No per-tick infrastructure dynamics in v1. Kept as an explicit no-op so the
/// dependency-ordered tick reads uniformly and later phases have a home.
pub fn run(_units: &mut [AdminUnit], _t: &Tuning) {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AdminLevel, AdminUnit, InfrastructureState};

    fn unit(ruggedness: f32, coastal: bool) -> AdminUnit {
        AdminUnit {
            id: 1,
            parent: Some(0),
            name: "t".into(),
            level: AdminLevel::Region,
            avg_elevation: 0.0,
            terrain_ruggedness: ruggedness,
            coastal,
            population: 0,
            education_level: 0.0,
            industries: vec![],
            infrastructure: InfrastructureState::default(),
            satisfaction: 0.0,
            revenue_last_tick: 0.0,
            poverty: 0.0,
            employment_rate: 0.0,
            daily_wage: 0.0,
            is_aggregate: false,
        }
    }

    #[test]
    fn rugged_inland_costs_more_than_flat_coastal() {
        let t = Tuning::default();
        let cheap = build_cost(&unit(0.1, true), &t); // flat, coastal
        let dear = build_cost(&unit(0.9, false), &t); // rugged, inland
        assert!(dear > cheap, "topology must drive cost: {dear} vs {cheap}");
    }

    #[test]
    fn coastal_discount_applies() {
        let t = Tuning::default();
        let inland = build_cost(&unit(0.5, false), &t);
        let coastal = build_cost(&unit(0.5, true), &t);
        assert!((coastal / inland - t.coastal_discount).abs() < 1e-9);
    }

    #[test]
    fn matches_brief_constants() {
        // base 5e9, rugged 0.5, coastal → 5e9 * (1 + 0.5*2.5) * 0.8 = 5e9 * 2.25 * 0.8 = 9e9
        let t = Tuning::default();
        let c = build_cost(&unit(0.5, true), &t);
        assert!((c - 9.0e9).abs() < 1.0, "cost = {c}");
    }
}
