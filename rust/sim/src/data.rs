//! Seed data: the national root plus the 17 administrative regions of the
//! Philippines, with hand-seeded topology, population, education and industries.
//!
//! These are plausible starting values for a game, not census figures. Topology
//! (elevation/ruggedness/coastal) will be SRTM-derived later; everything here is
//! data, so swapping in ~1,600 municipalities is a data change, not a code one.
//!
//! Note: this is the canonical 17-region set (pre-2024, before the Negros Island
//! Region). Adding NIR later is one more entry in the vector.

use crate::types::{AdminLevel, AdminUnit, IndustryInstance, IndustryType, InfrastructureState};

use IndustryType::*;

/// Build a leaf region. Derived fields start at zero and are filled by the
/// engine's priming pass before the first tick.
#[allow(clippy::too_many_arguments)]
fn region(
    id: u32,
    name: &str,
    avg_elevation: f32,
    terrain_ruggedness: f32,
    coastal: bool,
    population: u32,
    education_level: f32,
    transport_level: u32,
    industries: &[(IndustryType, u32)],
) -> AdminUnit {
    AdminUnit {
        id,
        parent: Some(0),
        name: name.to_string(),
        level: AdminLevel::Region,
        avg_elevation,
        terrain_ruggedness,
        coastal,
        population,
        education_level,
        industries: industries
            .iter()
            .map(|&(kind, level)| IndustryInstance { kind, level })
            .collect(),
        infrastructure: InfrastructureState { transport_level },
        satisfaction: 0.0,
        revenue_last_tick: 0.0,
        poverty: 0.0,
        employment_rate: 0.0,
        daily_wage: 0.0,
        is_aggregate: false,
    }
}

/// The national root unit (id 0). Its values are aggregates of its children.
fn national_root() -> AdminUnit {
    AdminUnit {
        id: 0,
        parent: None,
        name: "Republic of the Philippines".to_string(),
        level: AdminLevel::National,
        avg_elevation: 0.0,
        terrain_ruggedness: 0.0,
        coastal: true,
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

/// National root + the 17 regions. Index 0 is always the root.
pub fn philippines_regions() -> Vec<AdminUnit> {
    vec![
        national_root(),
        // id, name, elev(m), rugged, coastal, pop, edu, transport, industries
        region(1, "NCR", 10.0, 0.10, true, 13_500_000, 0.85, 5, &[(Services, 6), (Manufacturing, 3)]),
        region(2, "CAR", 1200.0, 0.85, false, 1_800_000, 0.70, 2, &[(Agriculture, 2), (Mining, 2), (Tourism, 2)]),
        region(3, "Region I – Ilocos", 120.0, 0.40, true, 5_300_000, 0.68, 3, &[(Agriculture, 3), (Tourism, 2)]),
        region(4, "Region II – Cagayan Valley", 250.0, 0.50, true, 3_700_000, 0.62, 2, &[(Agriculture, 4), (Mining, 1)]),
        region(5, "Region III – Central Luzon", 60.0, 0.15, true, 12_400_000, 0.72, 4, &[(Agriculture, 4), (Manufacturing, 3), (Services, 2)]),
        region(6, "Region IV-A – CALABARZON", 150.0, 0.35, true, 16_200_000, 0.74, 4, &[(Manufacturing, 5), (Services, 3), (Agriculture, 2)]),
        region(7, "Region IV-B – MIMAROPA", 200.0, 0.50, true, 3_200_000, 0.58, 1, &[(Agriculture, 2), (Tourism, 3), (Mining, 2)]),
        region(8, "Region V – Bicol", 180.0, 0.55, true, 6_100_000, 0.60, 2, &[(Agriculture, 3), (Tourism, 2)]),
        region(9, "Region VI – Western Visayas", 90.0, 0.30, true, 7_900_000, 0.66, 3, &[(Agriculture, 4), (Tourism, 2), (Services, 2)]),
        region(10, "Region VII – Central Visayas", 220.0, 0.45, true, 8_100_000, 0.70, 3, &[(Services, 3), (Tourism, 3), (Manufacturing, 2)]),
        region(11, "Region VIII – Eastern Visayas", 160.0, 0.50, true, 4_500_000, 0.56, 1, &[(Agriculture, 3), (Mining, 1)]),
        region(12, "Region IX – Zamboanga Peninsula", 140.0, 0.45, true, 3_900_000, 0.55, 1, &[(Agriculture, 3), (Manufacturing, 1)]),
        region(13, "Region X – Northern Mindanao", 350.0, 0.55, true, 5_000_000, 0.63, 2, &[(Agriculture, 4), (Manufacturing, 2)]),
        region(14, "Region XI – Davao", 300.0, 0.50, true, 5_200_000, 0.66, 3, &[(Agriculture, 4), (Services, 2), (Manufacturing, 1)]),
        region(15, "Region XII – SOCCSKSARGEN", 280.0, 0.45, true, 4_900_000, 0.58, 2, &[(Agriculture, 4), (Mining, 1)]),
        region(16, "Region XIII – Caraga", 240.0, 0.60, true, 2_800_000, 0.54, 1, &[(Mining, 3), (Agriculture, 2)]),
        region(17, "BARMM", 120.0, 0.50, true, 4_900_000, 0.45, 1, &[(Agriculture, 3)]),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exactly_one_root_and_seventeen_regions() {
        let units = philippines_regions();
        assert_eq!(units.iter().filter(|u| u.parent.is_none()).count(), 1);
        assert_eq!(units.iter().filter(|u| u.level == AdminLevel::Region).count(), 17);
    }

    #[test]
    fn ids_are_unique_and_parents_resolve() {
        let units = philippines_regions();
        let ids: std::collections::HashSet<u32> = units.iter().map(|u| u.id).collect();
        assert_eq!(ids.len(), units.len(), "duplicate ids");
        for u in &units {
            if let Some(p) = u.parent {
                assert!(ids.contains(&p), "{} has dangling parent {p}", u.name);
            }
        }
    }
}
