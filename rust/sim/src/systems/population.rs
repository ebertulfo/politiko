//! System 1 — Population. The foundation everyone else acts on.
//!
//! `GrowthRate = base_growth - education×edu_growth_damp + poverty×poverty_growth_boost`
//!
//! Educated populations grow slower; poorer ones grow faster. Uses `poverty`
//! computed by the industries system on the *previous* tick (population moves
//! first by design), so on a fresh sim it reads the primed derived values.

use crate::constants::Tuning;
use crate::types::AdminUnit;

/// Growth rate for one unit given its current education and poverty.
pub fn growth_rate(education_level: f32, poverty: f32, t: &Tuning) -> f32 {
    t.base_growth - education_level * t.edu_growth_damp + poverty * t.poverty_growth_boost
}

pub fn run(units: &mut [AdminUnit], t: &Tuning) {
    for u in units.iter_mut() {
        if !u.is_operational() {
            continue; // aggregates get population from aggregate_up, not growth
        }
        let rate = growth_rate(u.education_level, u.poverty, t);
        let next = (u.population as f64 * (1.0 + rate as f64)).round();
        u.population = next.max(0.0) as u32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn education_lowers_growth_poverty_raises_it() {
        let t = Tuning::default();
        let educated = growth_rate(0.9, 0.0, &t);
        let poor = growth_rate(0.1, 0.8, &t);
        assert!(poor > educated, "poor uneducated region should grow faster");
    }

    #[test]
    fn matches_brief_formula() {
        let t = Tuning::default();
        // 0.015 - 0.5*0.01 + 0.3*0.008 = 0.015 - 0.005 + 0.0024 = 0.0124
        let r = growth_rate(0.5, 0.3, &t);
        assert!((r - 0.0124).abs() < 1e-6, "rate = {r}");
    }
}
