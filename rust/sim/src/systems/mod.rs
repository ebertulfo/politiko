//! The six base systems (v1 scope). Each is a pure function over the unit slice
//! plus national state and tuning. They run in dependency order from
//! [`crate::engine::Sim::advance_tick`]:
//!
//! 1. [`population`]     — foundation; moves first using last tick's wages.
//! 2. [`industries`]     — employ the updated population, produce revenue + wages.
//! 3. [`economy`]        — roll revenue up, apply taxes/expenses/debt interest.
//! 4. [`infrastructure`] — upkeep & build gating (topology-coupled cost).
//! 5. [`education`]       — slow convergence toward the investment lever.
//! 6. [`politics`]       — satisfaction → approval → election check.
//!
//! Disasters, world market, migration, unrest, corruption, and energy are
//! intentionally absent (scope gates). They will slot in as additional systems
//! and additional unit fields without touching these six.

pub mod economy;
pub mod education;
pub mod industries;
pub mod infrastructure;
pub mod politics;
pub mod population;
