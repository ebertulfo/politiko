//! End-to-end checks on the full tick loop and the closed gameplay loop.

use politiko_sim::Sim;

#[test]
fn election_fires_exactly_at_tick_72() {
    let mut sim = Sim::new_philippines();
    let mut elections = 0;
    for _ in 0..72 {
        let out = sim.advance_tick();
        if out.election.is_some() {
            elections += 1;
            assert_eq!(out.tick, 72, "election should only fire on the term boundary");
        }
    }
    assert_eq!(elections, 1, "exactly one election in a full term");
    assert_eq!(sim.nation.tick, 72);
}

#[test]
fn a_full_term_keeps_state_in_valid_ranges() {
    let mut sim = Sim::new_philippines();
    for _ in 0..72 {
        sim.advance_tick();
        assert!((0.0..=1.0).contains(&sim.nation.approval), "approval out of range");
        for u in sim.units.iter().filter(|u| u.is_operational()) {
            assert!((0.0..=1.0).contains(&u.satisfaction), "{} satisfaction", u.name);
            assert!((0.0..=1.0).contains(&u.poverty), "{} poverty", u.name);
            assert!((0.0..=1.0).contains(&u.education_level), "{} education", u.name);
            assert!(u.population > 0, "{} depopulated", u.name);
        }
    }
}

#[test]
fn national_population_equals_sum_of_regions_each_tick() {
    let mut sim = Sim::new_philippines();
    for _ in 0..24 {
        sim.advance_tick();
        let sum: u64 = sim
            .units
            .iter()
            .filter(|u| u.is_operational())
            .map(|u| u.population as u64)
            .sum();
        assert_eq!(sum, sim.root().population as u64, "rollup must equal the sum");
    }
}

#[test]
fn the_closed_loop_pays_off_investment() {
    // Invest heavily in education + transport; over a term, approval should end
    // up at least as high as a neglectful run. This exercises the whole loop:
    // education → industry efficiency → wages → poverty → satisfaction → approval.
    let mut invest = Sim::new_philippines();
    invest.nation.education_investment = 0.95;
    invest.nation.treasury = 1.0e13;
    // Build transport everywhere we can afford it, once.
    let ids: Vec<u32> = invest
        .units
        .iter()
        .filter(|u| u.is_operational())
        .map(|u| u.id)
        .collect();
    for id in ids {
        let _ = invest.build_transport(id);
    }

    let mut neglect = Sim::new_philippines();
    neglect.nation.education_investment = 0.10;

    for _ in 0..72 {
        invest.advance_tick();
        neglect.advance_tick();
    }

    assert!(
        invest.nation.approval > neglect.nation.approval,
        "investment ({}) should beat neglect ({})",
        invest.nation.approval,
        neglect.nation.approval
    );
}

#[test]
fn deep_debt_eventually_triggers_bankruptcy() {
    let mut sim = Sim::new_philippines();
    sim.nation.treasury = -sim.tuning.debt_ceiling * 0.95;
    sim.nation.tax_rate = 0.0; // collect nothing; interest compounds the debt
    let mut bankrupt = false;
    for _ in 0..36 {
        if sim.advance_tick().bankrupt {
            bankrupt = true;
            break;
        }
    }
    assert!(bankrupt, "unserviced debt should breach the ceiling within 3 years");
}
