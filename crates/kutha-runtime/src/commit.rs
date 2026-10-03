//! COMMIT-01 option A: `Op::QuantumOutcome` is the User-quantum commit marker.
//! On open, a typed uncommitted suffix is not visible.

use kutha_common::{Event, Op, TermDictionary};

/// Whether an op commits by itself or needs a trailing `QuantumOutcome`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitClass {
    SelfCommit,
    NeedsOutcome,
}

/// Exhaustive match (no `_`). Product `Assert` is `NeedsOutcome`; reserved `kutha:*`
/// dictionary asserts are reclassified in [`committed_prefix`] because `TermId` is
/// an intern integer and `op_commit_class(&Op)` cannot see names.
pub fn op_commit_class(op: &Op) -> CommitClass {
    match op {
        Op::Assert { .. }
        | Op::Retract { .. }
        | Op::Correct { .. }
        | Op::CorrectInterval { .. }
        | Op::Behavior { .. } => CommitClass::NeedsOutcome,
        Op::Define { .. }
        | Op::QuantumOutcome { .. }
        | Op::JustificationCite { .. }
        | Op::StoreBlob { .. }
        | Op::TtAnchor { .. } => CommitClass::SelfCommit,
    }
}

fn class_with_dict(op: &Op, dict: &TermDictionary) -> CommitClass {
    if let Op::Assert { relation, .. } = op {
        if dict
            .lookup(*relation)
            .is_some_and(|n| n.starts_with("kutha:"))
        {
            return CommitClass::SelfCommit;
        }
    }
    op_commit_class(op)
}

fn dict_from_defines(events: &[Event]) -> TermDictionary {
    let mut dict = TermDictionary::default();
    for e in events {
        if let Op::Define { name } = &e.op {
            let _ = dict.intern(name);
        }
    }
    dict
}

/// Keep through the last `QuantumOutcome`, then keep SelfCommit ops until the
/// first NeedsOutcome op (dropped with the rest of the suffix). No outcome in
/// the log: keep a leading SelfCommit prefix (genesis / dictionary).
pub fn committed_prefix(events: Vec<Event>) -> Vec<Event> {
    let dict = dict_from_defines(&events);
    let start = events
        .iter()
        .rposition(|e| matches!(e.op, Op::QuantumOutcome { .. }))
        .map(|i| i + 1)
        .unwrap_or(0);
    let mut end = events.len();
    for (i, e) in events.iter().enumerate().skip(start) {
        if class_with_dict(&e.op, &dict) == CommitClass::NeedsOutcome {
            end = i;
            break;
        }
    }
    let mut out = events;
    out.truncate(end);
    out
}
