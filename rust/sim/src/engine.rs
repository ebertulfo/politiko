//! The simulation owner. Holds all state ([`Sim`]) and drives the tick.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::aggregate::root;
use crate::constants::Tuning;
use crate::systems;
use crate::types::{AdminUnit, ElectionResult, Nation};

/// What happened during one [`Sim::advance_tick`].
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TickOutcome {
    pub tick: u32,
    /// `Some` only on an election tick.
    pub election: Option<ElectionResult>,
    /// Mirrors [`Nation::bankrupt`] after this tick.
    pub bankrupt: bool,
}

/// Why a build action failed.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BuildError {
    UnknownUnit,
    /// The unit is an aggregate (e.g. the national root); you build on leaves.
    NotOperational,
    InsufficientFunds { cost: f64, treasury: f64 },
}

/// The whole game state. The single state of truth.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sim {
    pub units: Vec<AdminUnit>,
    pub nation: Nation,
    pub tuning: Tuning,
}

impl Sim {
    /// A fresh game seeded with the 17 Philippine regions, primed so the first
    /// tick reads sane derived values.
    pub fn new_philippines() -> Self {
        let mut sim = Self {
            units: crate::data::philippines_regions(),
            nation: Nation::default(),
            tuning: Tuning::default(),
        };
        sim.mark_aggregates();
        sim.prime();
        sim
    }

    /// Flag every unit that has children as an aggregate. Run once at startup
    /// (and after any structural change to the tree). This is how the engine
    /// stays level-agnostic: behavior follows tree shape, not [`crate::AdminLevel`].
    fn mark_aggregates(&mut self) {
        let parents: HashSet<u32> = self.units.iter().filter_map(|u| u.parent).collect();
        for u in &mut self.units {
            u.is_aggregate = parents.contains(&u.id);
        }
    }

    /// Populate derived fields (revenue, wages, poverty, employment, satisfaction,
    /// approval) without advancing the clock, so tick 1 doesn't read zeros.
    fn prime(&mut self) {
        systems::industries::run(&mut self.units, &self.tuning);
        systems::politics::compute_satisfaction(&mut self.units, &mut self.nation, &self.tuning);
    }

    /// Advance one month, running the six systems in dependency order.
    pub fn advance_tick(&mut self) -> TickOutcome {
        self.nation.tick += 1;

        systems::population::run(&mut self.units, &self.tuning);
        systems::industries::run(&mut self.units, &self.tuning);
        systems::economy::run(&mut self.units, &mut self.nation, &self.tuning);
        systems::infrastructure::run(&mut self.units, &self.tuning);
        systems::education::run(&mut self.units, &self.nation, &self.tuning);
        let election = systems::politics::run(&mut self.units, &mut self.nation, &self.tuning);

        TickOutcome {
            tick: self.nation.tick,
            election,
            bankrupt: self.nation.bankrupt,
        }
    }

    /// Cost to raise a unit's transport level by one, given current topology.
    pub fn transport_cost(&self, unit_id: u32) -> Option<f64> {
        self.unit(unit_id)
            .map(|u| systems::infrastructure::build_cost(u, &self.tuning))
    }

    /// Player action: build one level of transport in a unit, paying from the
    /// treasury. Returns the cost paid on success.
    pub fn build_transport(&mut self, unit_id: u32) -> Result<f64, BuildError> {
        let idx = self
            .units
            .iter()
            .position(|u| u.id == unit_id)
            .ok_or(BuildError::UnknownUnit)?;
        if self.units[idx].is_aggregate {
            return Err(BuildError::NotOperational);
        }
        let cost = systems::infrastructure::build_cost(&self.units[idx], &self.tuning);
        if self.nation.treasury < cost {
            return Err(BuildError::InsufficientFunds {
                cost,
                treasury: self.nation.treasury,
            });
        }
        self.nation.treasury -= cost;
        self.units[idx].infrastructure.transport_level += 1;
        Ok(cost)
    }

    /// Borrow a unit by id.
    pub fn unit(&self, id: u32) -> Option<&AdminUnit> {
        self.units.iter().find(|u| u.id == id)
    }

    /// The national root unit (aggregated totals live here).
    pub fn root(&self) -> &AdminUnit {
        root(&self.units)
    }

    // --- Serialization (Phase 0.1 Step 6 will write these to `user://`) ---

    /// Serialize the entire game state to JSON.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Restore game state from JSON produced by [`Sim::to_json`].
    pub fn from_json(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl Default for Sim {
    fn default() -> Self {
        Self::new_philippines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_marked_aggregate_regions_are_not() {
        let sim = Sim::new_philippines();
        assert!(sim.root().is_aggregate);
        assert_eq!(sim.units.iter().filter(|u| u.is_operational()).count(), 17);
    }

    #[test]
    fn priming_fills_derived_values() {
        let sim = Sim::new_philippines();
        assert!(sim.root().population > 0, "aggregate population should be set");
        assert!(sim.root().revenue_last_tick > 0.0);
        assert!(sim.nation.approval > 0.0 && sim.nation.approval <= 1.0);
    }

    #[test]
    fn build_transport_costs_more_in_rugged_terrain() {
        let mut sim = Sim::new_philippines();
        sim.nation.treasury = 1.0e13; // plenty
        let car = 2; // CAR: rugged, inland
        let ncr = 1; // NCR: flat, coastal
        let car_cost = sim.transport_cost(car).unwrap();
        let ncr_cost = sim.transport_cost(ncr).unwrap();
        assert!(car_cost > ncr_cost);

        let before = sim.unit(car).unwrap().infrastructure.transport_level;
        let paid = sim.build_transport(car).unwrap();
        assert_eq!(paid, car_cost);
        assert_eq!(sim.unit(car).unwrap().infrastructure.transport_level, before + 1);
    }

    #[test]
    fn build_transport_rejects_when_broke_and_on_aggregates() {
        let mut sim = Sim::new_philippines();
        sim.nation.treasury = 0.0;
        assert!(matches!(
            sim.build_transport(1),
            Err(BuildError::InsufficientFunds { .. })
        ));
        assert_eq!(sim.build_transport(0), Err(BuildError::NotOperational));
        assert_eq!(sim.build_transport(999), Err(BuildError::UnknownUnit));
    }

    #[test]
    fn save_load_roundtrips() {
        let mut sim = Sim::new_philippines();
        sim.advance_tick();
        sim.advance_tick();
        let json = sim.to_json().unwrap();
        let restored = Sim::from_json(&json).unwrap();
        assert_eq!(restored.nation, sim.nation);
        assert_eq!(restored.units, sim.units);
    }
}
