//! System 5 — Education. Slow-moving workforce quality; gates industries.
//!
//! Each unit's `education_level` converges toward the national
//! `education_investment` lever by a fraction `edu_gain` per tick. Deliberately
//! slow, so investment decisions pay off over many months — and it is safe to
//! run late in the tick because nothing downstream this tick depends on it.

use crate::constants::Tuning;
use crate::types::{AdminUnit, Nation};

pub fn run(units: &mut [AdminUnit], nation: &Nation, t: &Tuning) {
    let target = nation.education_investment;
    for u in units.iter_mut() {
        if !u.is_operational() {
            continue;
        }
        let delta = (target - u.education_level) * t.edu_gain;
        u.education_level = (u.education_level + delta).clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    #[test]
    fn education_moves_toward_investment_but_slowly() {
        let t = Tuning::default();
        let mut units = data::philippines_regions();
        let nation = Nation { education_investment: 0.95, ..Nation::default() };

        let before: Vec<f32> = units.iter().map(|u| u.education_level).collect();
        run(&mut units, &nation, &t);

        for (i, u) in units.iter().enumerate() {
            if !u.is_operational() {
                continue;
            }
            // Moved toward the target...
            assert!(u.education_level >= before[i], "{} should not regress", u.name);
            // ...but by no more than edu_gain of the gap (slow).
            let max_step = (0.95 - before[i]) * t.edu_gain + 1e-6;
            assert!(u.education_level - before[i] <= max_step, "{} moved too fast", u.name);
        }
    }

    #[test]
    fn converges_to_target_over_many_ticks() {
        let t = Tuning::default();
        let mut units = data::philippines_regions();
        let nation = Nation { education_investment: 0.8, ..Nation::default() };
        for _ in 0..500 {
            run(&mut units, &nation, &t);
        }
        for u in units.iter().filter(|u| u.is_operational()) {
            assert!((u.education_level - 0.8).abs() < 0.01, "{} = {}", u.name, u.education_level);
        }
    }
}
