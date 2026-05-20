//! System 3 — Economy / Treasury. The resource constraint.
//!
//! Rolls all unit revenue up to the national root, then:
//! `treasury += national_revenue × tax_rate - national_expense`.
//! Debt is allowed; negative treasury accrues `debt_interest_rate` per month.
//! Falling below `debt_ceiling` flags bankruptcy (IMF default game-over).

use crate::aggregate::{aggregate_up, root};
use crate::constants::Tuning;
use crate::types::{AdminUnit, Nation};

/// Total monthly maintenance + upkeep across all operational units.
pub fn national_expense(units: &[AdminUnit], t: &Tuning) -> f64 {
    units
        .iter()
        .filter(|u| u.is_operational())
        .map(|u| {
            u.total_industry_level() as f64 * t.maintenance_per_industry_level
                + u.infrastructure.transport_level as f64 * t.upkeep_per_transport_level
        })
        .sum()
}

pub fn run(units: &mut [AdminUnit], nation: &mut Nation, t: &Tuning) {
    // Roll revenue (and population) up the tree — national totals never count.
    aggregate_up(units);

    let national_revenue = root(units).revenue_last_tick;
    let expense = national_expense(units, t);

    nation.treasury += national_revenue * nation.tax_rate as f64 - expense;

    // Debt interest on whatever is below zero.
    if nation.treasury < 0.0 {
        let interest = -nation.treasury * t.debt_interest_rate;
        nation.treasury -= interest;
    }

    nation.bankrupt = nation.treasury < -t.debt_ceiling;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;
    use crate::systems::industries;

    #[test]
    fn debt_accrues_interest() {
        let mut units = data::philippines_regions();
        let mut nation = Nation::default();
        nation.treasury = -1.0e12; // ₱1T in debt
        nation.tax_rate = 0.0; // isolate the interest term (no revenue collected)
        let t = Tuning::default();
        industries::run(&mut units, &t);
        run(&mut units, &mut nation, &t);
        // -1e12 - expenses, then -5% interest on the (more) negative balance.
        assert!(nation.treasury < -1.0e12, "debt should grow: {}", nation.treasury);
    }

    #[test]
    fn bankrupt_flips_past_the_ceiling() {
        let mut units = data::philippines_regions();
        let mut nation = Nation::default();
        let t = Tuning::default();
        nation.treasury = -t.debt_ceiling - 1.0;
        industries::run(&mut units, &t);
        run(&mut units, &mut nation, &t);
        assert!(nation.bankrupt);
    }

    #[test]
    fn revenue_collected_scales_with_tax_rate() {
        let t = Tuning::default();
        let base = data::philippines_regions();

        let mut low = base.clone();
        let mut low_nation = Nation { treasury: 0.0, tax_rate: 0.05, ..Nation::default() };
        industries::run(&mut low, &t);
        run(&mut low, &mut low_nation, &t);

        let mut high = base.clone();
        let mut high_nation = Nation { treasury: 0.0, tax_rate: 0.25, ..Nation::default() };
        industries::run(&mut high, &t);
        run(&mut high, &mut high_nation, &t);

        assert!(high_nation.treasury > low_nation.treasury, "higher tax collects more");
    }
}
