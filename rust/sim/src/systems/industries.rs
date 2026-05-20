//! System 2 — Industries. Employ the population and produce output.
//!
//! Per industry: `output = level × base_output × efficiency`, where
//! `efficiency = min(1, education/required) × energy × transport_bonus × labor_factor`.
//!
//! Also derives wages → poverty for the population system, and employment_rate
//! for the politics system. This is the heart of the closed loop: education and
//! transport raise efficiency, which raises output, which raises wages, which
//! lowers poverty and feeds satisfaction.

use crate::constants::Tuning;
use crate::types::{AdminUnit, IndustryInstance};

/// Efficiency of a single industry (0..). Capped contributions keep it bounded
/// except for the transport bonus, which is intentionally an accelerator.
pub fn industry_efficiency(
    ind: &IndustryInstance,
    education_level: f32,
    transport_level: u32,
    labor_factor: f64,
    t: &Tuning,
) -> f64 {
    let edu_eff = (education_level as f64 / t.required_education(ind.kind)).min(1.0);
    let transport_bonus = 1.0 + transport_level as f64 * t.transport_efficiency_bonus;
    edu_eff * t.energy_available * transport_bonus * labor_factor
}

pub fn run(units: &mut [AdminUnit], t: &Tuning) {
    for u in units.iter_mut() {
        if !u.is_operational() {
            continue;
        }

        // --- Labor market ---
        let workforce = u.population as f64 * t.working_age_share;
        let labor_demand: f64 = u
            .industries
            .iter()
            .map(|i| i.level as f64 * t.labor_per_level)
            .sum();
        // Capital is under-utilized when there aren't enough workers to staff it.
        let labor_factor = if labor_demand > 0.0 {
            (workforce / labor_demand).min(1.0)
        } else {
            1.0
        };
        let employed = labor_demand.min(workforce);
        u.employment_rate = if workforce > 0.0 {
            (employed / workforce).min(1.0) as f32
        } else {
            0.0
        };

        // --- Output / revenue ---
        let transport = u.infrastructure.transport_level;
        let revenue: f64 = u
            .industries
            .iter()
            .map(|ind| {
                let eff = industry_efficiency(ind, u.education_level, transport, labor_factor, t);
                ind.level as f64 * t.base_output(ind.kind) * eff
            })
            .sum();
        u.revenue_last_tick = revenue;

        // --- Wages → poverty ---
        let total_wages = revenue * t.labor_share;
        let daily_wage = if employed > 0.0 {
            (total_wages / employed) / 30.0
        } else {
            0.0
        };
        u.daily_wage = daily_wage;
        let daily_col = t.cost_of_living / 30.0;
        u.poverty = (1.0 - daily_wage / daily_col).clamp(0.0, 1.0) as f32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::types::IndustryType;

    fn ind(kind: IndustryType, level: u32) -> IndustryInstance {
        IndustryInstance { kind, level }
    }

    #[test]
    fn education_raises_efficiency_up_to_the_requirement() {
        let t = Tuning::default();
        let i = ind(IndustryType::Services, 1); // required 0.7
        let low = industry_efficiency(&i, 0.35, 0, 1.0, &t); // half the requirement
        let met = industry_efficiency(&i, 0.85, 0, 1.0, &t); // above requirement → capped
        let over = industry_efficiency(&i, 1.0, 0, 1.0, &t); // well above → same cap
        assert!(low < met, "below requirement is less efficient");
        assert!((met - over).abs() < 1e-9, "efficiency caps once the requirement is met");
        assert!((met - 1.0).abs() < 1e-9, "cap is 1.0 at full transport/labor");
    }

    #[test]
    fn transport_increases_revenue() {
        let mut a = data::philippines_regions();
        let mut b = a.clone();
        // pick a leaf with industries
        let idx = a.iter().position(|u| u.is_operational() && !u.industries.is_empty()).unwrap();
        b[idx].infrastructure.transport_level += 5;
        run(&mut a, &Tuning::default());
        run(&mut b, &Tuning::default());
        assert!(
            b[idx].revenue_last_tick > a[idx].revenue_last_tick,
            "more transport → more revenue"
        );
    }

    #[test]
    fn poverty_is_bounded_and_falls_with_income() {
        let mut units = data::philippines_regions();
        run(&mut units, &Tuning::default());
        for u in units.iter().filter(|u| u.is_operational()) {
            assert!((0.0..=1.0).contains(&u.poverty), "{} poverty out of range", u.name);
            assert!((0.0..=1.0).contains(&u.employment_rate));
        }
    }
}
