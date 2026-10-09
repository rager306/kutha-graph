//! M026 S02 / Phase 86 — matrix honesty pins (tautology / honor-system relabel).
//! Does not change product fold behavior.

mod common;

use common::sem::assert_seed_budget;

/// Characterization: seed floor is env policy (`KUTHA_SEEDS`), not domain semantics.
#[test]
fn seed_budget_is_env_floor_not_domain() {
    assert_seed_budget();
}
