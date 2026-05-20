//! Roll child values up the `parent` tree into their ancestors. This is the
//! single place national (and, later, regional) totals are computed — never by
//! counting or special-casing a level.

use std::collections::HashMap;

use crate::types::AdminUnit;

/// Index of the root unit (the one with `parent == None`). Panics if there is
/// no root, which would be a data-authoring bug.
pub fn root_index(units: &[AdminUnit]) -> usize {
    units
        .iter()
        .position(|u| u.parent.is_none())
        .expect("admin tree must have exactly one root (parent == None)")
}

/// Borrow the national root unit.
pub fn root(units: &[AdminUnit]) -> &AdminUnit {
    &units[root_index(units)]
}

/// Post-order subtree sum for one node, written into `out` keyed by unit id.
/// Returns `(population, revenue, education×pop, satisfaction×pop)` so parents
/// can pop-weight the averaged fields.
fn fold_subtree(
    id: u32,
    children: &HashMap<u32, Vec<u32>>,
    idx_of: &HashMap<u32, usize>,
    units: &[AdminUnit],
    out: &mut HashMap<u32, (f64, f64, f64, f64)>,
) -> (f64, f64, f64, f64) {
    let kids = children.get(&id);
    let agg = match kids {
        // Leaf: its aggregate is simply its own production.
        None => {
            let u = &units[idx_of[&id]];
            let pop = u.population as f64;
            (
                pop,
                u.revenue_last_tick,
                u.education_level as f64 * pop,
                u.satisfaction as f64 * pop,
            )
        }
        // Internal node: sum of children's subtrees.
        Some(kids) => {
            let mut acc = (0.0, 0.0, 0.0, 0.0);
            for &c in kids {
                let a = fold_subtree(c, children, idx_of, units, out);
                acc.0 += a.0;
                acc.1 += a.1;
                acc.2 += a.2;
                acc.3 += a.3;
            }
            acc
        }
    };
    out.insert(id, agg);
    agg
}

/// Recompute every aggregate (non-leaf) unit from its subtree:
/// - `population`, `revenue_last_tick` are summed,
/// - `education_level`, `satisfaction` are population-weighted averages.
///
/// Leaf units are left untouched (their values are their own production).
/// Works for any tree depth, so it scales unchanged from 17 regions to ~1,600
/// municipalities.
pub fn aggregate_up(units: &mut [AdminUnit]) {
    let idx_of: HashMap<u32, usize> = units.iter().enumerate().map(|(i, u)| (u.id, i)).collect();
    let mut children: HashMap<u32, Vec<u32>> = HashMap::new();
    for u in units.iter() {
        if let Some(p) = u.parent {
            children.entry(p).or_default().push(u.id);
        }
    }

    let root_id = units[root_index(units)].id;
    let mut out: HashMap<u32, (f64, f64, f64, f64)> = HashMap::new();
    fold_subtree(root_id, &children, &idx_of, units, &mut out);

    // Write results back into aggregate units only; leaves keep their own values.
    for u in units.iter_mut() {
        if !children.contains_key(&u.id) {
            continue; // leaf
        }
        let (pop, rev, edu_w, sat_w) = out[&u.id];
        u.population = pop.round() as u32;
        u.revenue_last_tick = rev;
        u.education_level = if pop > 0.0 { (edu_w / pop) as f32 } else { 0.0 };
        u.satisfaction = if pop > 0.0 { (sat_w / pop) as f32 } else { 0.0 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AdminLevel, InfrastructureState};

    fn leaf(id: u32, parent: u32, pop: u32, edu: f32, rev: f64, sat: f32) -> AdminUnit {
        AdminUnit {
            id,
            parent: Some(parent),
            name: format!("leaf{id}"),
            level: AdminLevel::Region,
            avg_elevation: 0.0,
            terrain_ruggedness: 0.0,
            coastal: false,
            population: pop,
            education_level: edu,
            industries: vec![],
            infrastructure: InfrastructureState::default(),
            satisfaction: sat,
            revenue_last_tick: rev,
            poverty: 0.0,
            employment_rate: 0.0,
            daily_wage: 0.0,
            is_aggregate: false,
        }
    }

    fn root_unit() -> AdminUnit {
        AdminUnit {
            id: 0,
            parent: None,
            name: "PH".into(),
            level: AdminLevel::National,
            avg_elevation: 0.0,
            terrain_ruggedness: 0.0,
            coastal: false,
            population: 0,
            education_level: 0.0,
            industries: vec![],
            infrastructure: InfrastructureState::default(),
            satisfaction: 0.0,
            revenue_last_tick: 0.0,
            poverty: 0.0,
            employment_rate: 0.0,
            daily_wage: 0.0,
            is_aggregate: true,
        }
    }

    #[test]
    fn root_sums_population_and_revenue() {
        let mut units = vec![
            root_unit(),
            leaf(1, 0, 100, 0.5, 10.0, 0.4),
            leaf(2, 0, 300, 0.9, 30.0, 0.8),
        ];
        aggregate_up(&mut units);
        let r = root(&units);
        assert_eq!(r.population, 400);
        assert_eq!(r.revenue_last_tick, 40.0);
    }

    #[test]
    fn averages_are_population_weighted() {
        // 100 people @ edu 0.5 and 300 @ 0.9 → weighted edu = (50 + 270)/400 = 0.8
        let mut units = vec![
            root_unit(),
            leaf(1, 0, 100, 0.5, 0.0, 0.2),
            leaf(2, 0, 300, 0.9, 0.0, 1.0),
        ];
        aggregate_up(&mut units);
        let r = root(&units);
        assert!((r.education_level - 0.8).abs() < 1e-5, "edu = {}", r.education_level);
        // satisfaction = (0.2*100 + 1.0*300)/400 = 0.8
        assert!((r.satisfaction - 0.8).abs() < 1e-5, "sat = {}", r.satisfaction);
    }

    #[test]
    fn three_level_tree_rolls_through_intermediate() {
        // root(0) -> region(1) -> municipalities(10,11). Region is an aggregate.
        let mut units = vec![
            root_unit(),
            AdminUnit { is_aggregate: true, ..leaf(1, 0, 0, 0.0, 0.0, 0.0) },
            leaf(10, 1, 200, 0.6, 5.0, 0.5),
            leaf(11, 1, 200, 0.4, 7.0, 0.9),
        ];
        aggregate_up(&mut units);
        let region = units.iter().find(|u| u.id == 1).unwrap();
        assert_eq!(region.population, 400);
        assert_eq!(region.revenue_last_tick, 12.0);
        assert_eq!(root(&units).population, 400, "root mirrors single region subtree");
    }
}
