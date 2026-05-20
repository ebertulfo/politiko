//! System 6 — Politics / Approval. The pressure, and the win/lose check.
//!
//! Per-unit satisfaction is a weighted blend of low poverty, employment,
//! education, and infrastructure service. National approval is the
//! population-weighted average (via `aggregate_up`). An election fires whenever
//! `tick % term_length == 0`; the player keeps governing if approval-derived
//! vote share exceeds 50%.

use crate::aggregate::{aggregate_up, root};
use crate::constants::Tuning;
use crate::types::{AdminUnit, ElectionResult, Nation};

/// Satisfaction (0..1) for one unit's current derived state.
pub fn satisfaction(u: &AdminUnit, t: &Tuning) -> f32 {
    let infra_score = (u.infrastructure.transport_level as f32 / t.target_transport).min(1.0);
    let s = t.w_poverty * (1.0 - u.poverty)
        + t.w_employment * u.employment_rate
        + t.w_education * u.education_level
        + t.w_infrastructure * infra_score;
    s.clamp(0.0, 1.0)
}

/// Recompute per-unit satisfaction and roll it up into `nation.approval`.
/// Shared by [`run`] and the engine's priming step; does **not** check elections.
pub fn compute_satisfaction(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning) {
    for u in units.iter_mut() {
        if u.is_operational() {
            u.satisfaction = satisfaction(u, t);
        }
    }
    aggregate_up(units); // pop-weighted average of satisfaction lands on the root
    nation.approval = root(units).satisfaction;
}

/// Approval-derived national vote share (0..1). v1 maps it 1:1; kept as a seam
/// for incumbency curves later.
pub fn voteshare(approval: f32) -> f32 {
    approval.clamp(0.0, 1.0)
}

/// Run the politics system. Returns `Some(result)` on an election tick.
pub fn run(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning) -> Option<ElectionResult> {
    compute_satisfaction(units, nation, t);

    if nation.tick > 0 && nation.tick % nation.term_length == 0 {
        let share = voteshare(nation.approval);
        Some(ElectionResult {
            tick: nation.tick,
            approval: nation.approval,
            voteshare: share,
            won: share > 0.5,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    #[test]
    fn approval_is_population_weighted_average_of_satisfaction() {
        let t = Tuning::default();
        let mut units = data::philippines_regions();
        // prime derived values so satisfaction isn't all zeros
        crate::systems::industries::run(&mut units, &t);
        let mut nation = Nation::default();
        compute_satisfaction(&mut units, &mut nation, &t);
        assert!((0.0..=1.0).contains(&nation.approval));
        assert!((nation.approval - root(&units).satisfaction).abs() < 1e-6);
    }

    #[test]
    fn election_only_fires_on_term_boundaries() {
        let t = Tuning::default();
        let mut units = data::philippines_regions();
        crate::systems::industries::run(&mut units, &t);

        let mut nation = Nation::default();
        nation.tick = 71;
        assert!(run(&mut units, &mut nation, &t).is_none());

        nation.tick = 72;
        let r = run(&mut units, &mut nation, &t).expect("election at tick 72");
        assert_eq!(r.tick, 72);
        assert_eq!(r.won, r.voteshare > 0.5);
    }
}
