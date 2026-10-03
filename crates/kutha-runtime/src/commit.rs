//! COMMIT-01 option A: `Op::QuantumOutcome` is the User-quantum commit marker.
//! On open, a typed uncommitted suffix is not visible.

use kutha_common::{Event, Op};

/// Whether an op commits by itself or needs a trailing `QuantumOutcome`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitClass {
    SelfCommit,
    NeedsOutcome,
}

/// Exhaustive match (no `_`). RED stub: every variant is `SelfCommit` so the
/// product suffix stays visible (old open behaviour).
pub fn op_commit_class(op: &Op) -> CommitClass {
    match op {
        Op::Assert { .. }
        | Op::Retract { .. }
        | Op::Correct { .. }
        | Op::CorrectInterval { .. }
        | Op::Behavior { .. }
        | Op::Define { .. }
        | Op::QuantumOutcome { .. }
        | Op::JustificationCite { .. }
        | Op::StoreBlob { .. }
        | Op::TtAnchor { .. } => CommitClass::SelfCommit,
    }
}

/// RED stub: identity so new-behaviour oracles fail until GREEN trims.
pub fn committed_prefix(events: Vec<Event>) -> Vec<Event> {
    events
}
