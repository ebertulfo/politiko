//! GDExtension binding: the thin layer Godot talks to. It owns a [`Sim`] and
//! exposes read-only views (as `VarDictionary`/`Array`) plus the player actions
//! and the tick. **No game logic lives here** — it only marshals data across the
//! boundary. All decisions happen in `politiko_sim`.

use godot::builtin::VarDictionary;
use godot::prelude::*;
use politiko_sim::types::{AdminLevel, AdminUnit, IndustryType};
use politiko_sim::Sim;

struct PolitikoExtension;

#[gdextension]
unsafe impl ExtensionLibrary for PolitikoExtension {}

/// Godot-facing handle to the simulation. Instantiate one in GDScript:
/// `var engine = PolitikoEngine.new()`.
#[derive(GodotClass)]
#[class(base=RefCounted)]
pub struct PolitikoEngine {
    sim: Sim,
    #[allow(dead_code)]
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for PolitikoEngine {
    fn init(base: Base<RefCounted>) -> Self {
        Self {
            sim: Sim::new_philippines(),
            base,
        }
    }
}

#[godot_api]
impl PolitikoEngine {
    /// Smoke-test the binding from GDScript: `print(engine.version())`.
    #[func]
    fn version(&self) -> GString {
        GString::from("Politiko sim engine 0.1")
    }

    // --- The clock ---

    /// Advance one month. Returns `{ tick, bankrupt, has_election, won, approval, voteshare }`.
    #[func]
    fn advance_tick(&mut self) -> VarDictionary {
        let out = self.sim.advance_tick();
        let mut d = vdict! {
            "tick" => out.tick,
            "bankrupt" => out.bankrupt,
            "has_election" => out.election.is_some(),
        };
        if let Some(e) = out.election {
            d.set("won", e.won);
            d.set("approval", e.approval);
            d.set("voteshare", e.voteshare);
        }
        d
    }

    // --- National readouts ---

    #[func]
    fn get_tick(&self) -> i64 {
        self.sim.nation.tick as i64
    }
    #[func]
    fn get_treasury(&self) -> f64 {
        self.sim.nation.treasury
    }
    #[func]
    fn get_approval(&self) -> f64 {
        self.sim.nation.approval as f64
    }
    #[func]
    fn get_political_capital(&self) -> f64 {
        self.sim.nation.political_capital as f64
    }
    #[func]
    fn get_tax_rate(&self) -> f64 {
        self.sim.nation.tax_rate as f64
    }
    #[func]
    fn get_term_length(&self) -> i64 {
        self.sim.nation.term_length as i64
    }

    // --- National policy levers (player actions) ---

    #[func]
    fn set_tax_rate(&mut self, rate: f64) {
        self.sim.nation.tax_rate = rate.clamp(0.0, 1.0) as f32;
    }
    #[func]
    fn set_education_investment(&mut self, level: f64) {
        self.sim.nation.education_investment = level.clamp(0.0, 1.0) as f32;
    }

    // --- Unit views ---

    /// Number of operational (leaf) units the player governs.
    #[func]
    fn operational_unit_count(&self) -> i64 {
        self.sim.units.iter().filter(|u| u.is_operational()).count() as i64
    }

    /// All units (root + leaves) as an array of dictionaries.
    #[func]
    fn get_all_units(&self) -> Array<VarDictionary> {
        let mut arr = Array::new();
        for u in &self.sim.units {
            arr.push(&unit_to_dict(u));
        }
        arr
    }

    /// One unit by id, or an empty dictionary if unknown.
    #[func]
    fn get_unit(&self, id: i64) -> VarDictionary {
        match self.sim.unit(id as u32) {
            Some(u) => unit_to_dict(u),
            None => VarDictionary::new(),
        }
    }

    /// The national root with aggregated totals.
    #[func]
    fn get_root(&self) -> VarDictionary {
        unit_to_dict(self.sim.root())
    }

    // --- Infrastructure action ---

    /// Cost to raise transport by one level in a unit (topology-coupled). -1 if unknown.
    #[func]
    fn transport_cost(&self, id: i64) -> f64 {
        self.sim.transport_cost(id as u32).unwrap_or(-1.0)
    }

    /// Build one transport level. Returns `{ ok, cost, reason }`.
    #[func]
    fn build_transport(&mut self, id: i64) -> VarDictionary {
        match self.sim.build_transport(id as u32) {
            Ok(cost) => vdict! { "ok" => true, "cost" => cost, "reason" => &GString::new() },
            Err(e) => vdict! {
                "ok" => false,
                "cost" => 0.0,
                "reason" => &GString::from(format!("{e:?}").as_str()),
            },
        }
    }

    // --- Saves (Step 6 wires these to user://) ---

    #[func]
    fn to_json(&self) -> GString {
        GString::from(self.sim.to_json().unwrap_or_default().as_str())
    }

    /// Replace state from a JSON string. Returns true on success.
    #[func]
    fn load_json(&mut self, json: GString) -> bool {
        match Sim::from_json(&json.to_string()) {
            Ok(s) => {
                self.sim = s;
                true
            }
            Err(_) => false,
        }
    }
}

fn level_name(level: AdminLevel) -> &'static str {
    match level {
        AdminLevel::National => "National",
        AdminLevel::Region => "Region",
        AdminLevel::Municipality => "Municipality",
    }
}

fn industry_name(kind: IndustryType) -> &'static str {
    match kind {
        IndustryType::Agriculture => "Agriculture",
        IndustryType::Manufacturing => "Manufacturing",
        IndustryType::Services => "Services",
        IndustryType::Tourism => "Tourism",
        IndustryType::Mining => "Mining",
    }
}

/// Marshal a unit into a Godot Dictionary for display. Read-only snapshot.
fn unit_to_dict(u: &AdminUnit) -> VarDictionary {
    let mut industries = Array::<VarDictionary>::new();
    for ind in &u.industries {
        industries.push(&vdict! {
            "kind" => &GString::from(industry_name(ind.kind)),
            "level" => ind.level,
        });
    }

    vdict! {
        "id" => u.id,
        "parent" => u.parent.map(|p| p as i64).unwrap_or(-1),
        "name" => &GString::from(u.name.as_str()),
        "level" => &GString::from(level_name(u.level)),
        "avg_elevation" => u.avg_elevation,
        "terrain_ruggedness" => u.terrain_ruggedness,
        "coastal" => u.coastal,
        "population" => u.population,
        "education_level" => u.education_level,
        "transport_level" => u.infrastructure.transport_level,
        "industries" => &industries,
        "satisfaction" => u.satisfaction,
        "revenue_last_tick" => u.revenue_last_tick,
        "poverty" => u.poverty,
        "employment_rate" => u.employment_rate,
        "daily_wage" => u.daily_wage,
        "is_aggregate" => u.is_aggregate,
    }
}
