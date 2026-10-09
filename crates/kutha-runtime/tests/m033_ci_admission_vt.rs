//! M033 S01 / Phase 93 — admission stale_support must cover any CI residual
//! at row.vt (CE review P1 #8). fact_index_by_event_id alone returns one
//! representative (often the prefix); patch/suffix VT must still admit.

use kutha_common::{Op, TransactionTime, ValidTime, rule_definition_hash};
use kutha_runtime::{Runtime, RuntimeError};

/// Premise Assert for Behavior eligibility stays live; a separate CorrectInterval
/// event is what the justification cites (the multi-residual case under #8).
fn fixture() -> (Runtime, kutha_common::EventId, kutha_common::EventId) {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let carol = rt.intern("Carol");
    let p = rt.intern("P");
    let q = rt.intern("Q");
    let knows = rt.intern("knows");
    let related = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    let pin = rule_definition_hash("derive_pq");
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();

    let cause = rt
        .emit(Op::Assert {
            subject: p,
            relation: related,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap()
        .receipt
        .event_ids[0];

    let prem = rt
        .emit(Op::Assert {
            subject: alice,
            relation: knows,
            object: bob,
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
        object: carol,
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
        .expect("CorrectInterval event");

    let live = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.event_id == ci && f.invalidated_at.is_none())
        .count();
    assert_eq!(live, 3, "prefix/patch/suffix residuals under CI id");

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: cause,
            rule_version: pin,
            subject: q,
            relation: related,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    (rt, derived.receipt.event_ids[0], ci)
}

#[test]
fn admission_admits_when_cited_ci_residual_covers_patch_vt() {
    let (mut rt, claim_q, ci) = fixture();
    let pin = rule_definition_hash("derive_pq");
    let jid = rt
        .record_justification(
            claim_q,
            vec![ci],
            vec![ci],
            pin,
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("record_justification");
    assert!(
        rt.check_admission(&jid).is_ok(),
        "patch VT covered by a live CI residual must admit"
    );
}

#[test]
fn admission_still_denies_when_no_ci_residual_covers_vt() {
    let (mut rt, claim_q, ci) = fixture();
    rt.emit(Op::Retract { event_id: ci }).unwrap();
    let pin = rule_definition_hash("derive_pq");
    let jid = rt
        .record_justification(
            claim_q,
            vec![ci],
            vec![ci],
            pin,
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("record still writes denied cite");
    let err = rt.check_admission(&jid).unwrap_err();
    assert!(
        matches!(
            err,
            RuntimeError::AdmissionDenied {
                reason: "stale_support",
                ..
            }
        ),
        "dead CI must stay stale_support; got {err:?}"
    );
}
