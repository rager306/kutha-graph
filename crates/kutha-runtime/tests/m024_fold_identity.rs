//! M024 S01 / Phase 79 — fold identity: live-gated Correct/Retract;
//! Correct on multi-residual event_id; CorrectInterval multi-residual fail-closed.

use kutha_common::{EventId, Op};
use kutha_runtime::{Runtime, RuntimeError};

fn statute_ci_three_residuals(rt: &mut Runtime) -> (EventId, EventId) {
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let limit_80 = rt.intern("80-kmh");

    let prem = rt
        .emit(Op::Assert {
            subject: article,
            relation: in_force,
            object: limit_50,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    rt.emit(Op::CorrectInterval {
        event_id: prem,
        object: limit_80,
        patch_from: 2015,
        patch_to: Some(2020),
    })
    .unwrap();

    let ci = rt
        .log()
        .iter()
        .filter_map(|e| match &e.op {
            Op::CorrectInterval { .. } => Some(e.id),
            _ => None,
        })
        .last()
        .expect("CI event");

    let live = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.event_id == ci && f.invalidated_at.is_none())
        .count();
    assert_eq!(live, 3, "fixture must mint three live residuals");
    (prem, ci)
}

/// G-A05: Retract of a dead id (original Assert after CI) must fail closed.
#[test]
fn retract_dead_event_id_fails_closed() {
    let mut rt = Runtime::default();
    let (prem, _ci) = statute_ci_three_residuals(&mut rt);
    let err = rt
        .emit(Op::Retract { event_id: prem })
        .expect_err("dead Assert id must not silently no-op");
    assert!(
        matches!(err, RuntimeError::UnknownFact { event_id } if event_id == prem),
        "expected UnknownFact({prem}), got {err:?}"
    );
}

/// G-A05: Correct of a dead id must fail closed.
#[test]
fn correct_dead_event_id_fails_closed() {
    let mut rt = Runtime::default();
    let (prem, _ci) = statute_ci_three_residuals(&mut rt);
    let obj = rt.intern("90-kmh");
    let err = rt
        .emit(Op::Correct {
            event_id: prem,
            object: obj,
            valid_from: 2010,
            valid_to: None,
        })
        .expect_err("dead Assert id Correct must fail closed");
    assert!(
        matches!(err, RuntimeError::UnknownFact { event_id } if event_id == prem),
        "expected UnknownFact({prem}), got {err:?}"
    );
}

/// G-A02: Correct(ci_id) with three live residuals invalidates all and mints one replacement.
#[test]
fn correct_multi_residual_event_id_collapses_to_one() {
    let mut rt = Runtime::default();
    let (_prem, ci) = statute_ci_three_residuals(&mut rt);
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_90 = rt.intern("90-kmh");

    rt.emit(Op::Correct {
        event_id: ci,
        object: limit_90,
        valid_from: 2010,
        valid_to: None,
    })
    .expect("Correct on CI id with three live residuals must succeed");

    let live_ci = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.event_id == ci && f.invalidated_at.is_none())
        .count();
    assert_eq!(live_ci, 0, "CI residuals must be invalidated");

    let live_new: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.invalidated_at.is_none() && f.object() == limit_90)
        .collect();
    assert_eq!(
        live_new.len(),
        1,
        "exactly one live replacement for multi-residual Correct"
    );
    assert_eq!(live_new[0].valid_from, 2010);
    assert_eq!(live_new[0].valid_to, None);

    let triple = (article, in_force, limit_90);
    assert!(rt.as_of(2012).contains(&triple));
    assert!(rt.as_of(2017).contains(&triple));
    assert!(rt.as_of(2021).contains(&triple));
}

/// G-A02: CorrectInterval targeting an id with >1 live residual fails closed
/// even when the patch intersects the first-live residual (would otherwise succeed).
#[test]
fn correct_interval_multi_residual_fails_closed() {
    let mut rt = Runtime::default();
    let (_prem, ci) = statute_ci_three_residuals(&mut rt);
    let obj = rt.intern("100-kmh");
    let err = rt
        .emit(Op::CorrectInterval {
            event_id: ci,
            object: obj,
            patch_from: 2011,
            patch_to: Some(2013),
        })
        .expect_err("CorrectInterval on multi-residual id must fail closed");
    assert!(
        matches!(err, RuntimeError::IntervalPatchRejected { event_id } if event_id == ci),
        "expected IntervalPatchRejected({ci}), got {err:?}"
    );
}
