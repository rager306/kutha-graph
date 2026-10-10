//! M037 S01 / Phase 97 — norm_view eligibility keys off claim lineage
//! (CE review P1 #13). Correct/CorrectInterval of a Behavior-derived claim
//! keep claim_id but mint under Correct event ids; minting-event checks skip
//! derivation_eligible_at and launder ineligible derived facts into Ok.

use kutha_common::{Op, TransactionTime, ValidTime, WallTime, rule_definition_hash};
use kutha_runtime::{NormView, Runtime};

fn setup_derived_q(
    rt: &mut Runtime,
) -> (
    kutha_common::EventId,
    kutha_common::EventId,
    kutha_common::TermId,
    kutha_common::TermId,
    kutha_common::TermId,
) {
    let p = rt.intern("P-m037");
    let q = rt.intern("Q-m037");
    let rel = rt.intern("relatedTo");
    let true_ = rt.intern("true");
    rt.emit_rule("derive_pq", 2010, None).unwrap();

    let first = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let claim_p = first.receipt.event_ids[0];

    let derived = rt
        .emit(Op::Behavior {
            name: "derive_pq".into(),
            caused_by: claim_p,
            rule_version: rule_definition_hash("derive_pq"),
            subject: q,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
        })
        .unwrap();
    let claim_q = derived.receipt.event_ids[0];
    (claim_p, claim_q, q, rel, true_)
}

/// Kill: after premise retract, Correct of derived (known at wall) must not launder into Ok.
#[test]
fn correct_of_ineligible_derived_still_abstains_in_norm_view() {
    let mut rt = Runtime::default();
    let (claim_p, claim_q, q, rel, true_) = setup_derived_q(&mut rt);

    rt.emit(Op::Retract { event_id: claim_p }).unwrap();

    // Correct before knowledge cut so the Correct residual is visible at wall.
    rt.emit(Op::Correct {
        event_id: claim_q,
        object: true_,
        valid_from: 2010,
        valid_to: Some(2030),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    let live = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_q && f.invalidated_at.is_none())
        .expect("corrected derived residual live");
    assert_ne!(
        live.event_id, claim_q,
        "pin: Correct minting id differs from Behavior claim_id"
    );
    assert!(
        !rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: Q ineligible after premise retract"
    );
    assert!(
        rt.as_of(ValidTime::from(2017)).contains(&(q, rel, true_)),
        "pin: as_of stays support-live (ASOF-ELIG-01)"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Abstain => {}
        NormView::Ok(triples) => {
            let laundered = triples.iter().any(|t| t.claim_id == claim_q);
            panic!(
                "Correct of ineligible derived must not launder into Ok (claim-lineage gate); \
                 got Ok with claim_q present={laundered}, triples={triples:?}"
            );
        }
        other => panic!(
            "expected Abstain for ineligible corrected derived; got {other:?}"
        ),
    }
}

/// Kill companion: CorrectInterval of ineligible derived must Abstain at knowledge cut.
#[test]
fn correct_interval_of_ineligible_derived_still_abstains_in_norm_view() {
    let mut rt = Runtime::default();
    let (claim_p, claim_q, q, rel, true_) = setup_derived_q(&mut rt);

    rt.emit(Op::Retract { event_id: claim_p }).unwrap();
    rt.emit(Op::CorrectInterval {
        event_id: claim_q,
        object: true_,
        patch_from: 2015,
        patch_to: Some(2025),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(
        !rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: still ineligible after CorrectInterval"
    );
    assert!(
        rt.as_of(ValidTime::from(2017)).contains(&(q, rel, true_)),
        "pin: as_of still lists derived"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Abstain => {}
        NormView::Ok(triples) => {
            let laundered = triples.iter().any(|t| t.claim_id == claim_q);
            panic!(
                "CorrectInterval of ineligible derived must not launder into Ok; \
                 claim_q present={laundered}, triples={triples:?}"
            );
        }
        other => panic!("expected Abstain; got {other:?}"),
    }
}

/// Regression: Correct of still-eligible derived remains Ok.
#[test]
fn correct_of_eligible_derived_still_ok_in_norm_view() {
    let mut rt = Runtime::default();
    let (_claim_p, claim_q, q, rel, true_) = setup_derived_q(&mut rt);

    rt.emit(Op::Correct {
        event_id: claim_q,
        object: true_,
        valid_from: 2010,
        valid_to: Some(2030),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(
        rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: still eligible after Correct with live premise"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Ok(triples) => {
            assert!(
                triples.iter().any(|t| {
                    t.claim_id == claim_q
                        && t.subject == q
                        && t.relation == rel
                        && t.object == true_
                        && t.eligible
                }),
                "eligible corrected derived must appear in Ok, got {triples:?}"
            );
        }
        other => panic!("expected Ok for eligible corrected derived, got {other:?}"),
    }
}
