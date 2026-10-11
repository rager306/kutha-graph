//! M056 S04 / Phase 126 — CE #28: Correct of a derived Behavior must not
//! inherit rule eligibility when the live replacement diverges from the
//! Behavior head proposition. Premise+pin liveness alone is not enough.

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
    let p = rt.intern("P-m056");
    let q = rt.intern("Q-m056");
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

/// Kill: Correct changes derived object away from the Behavior head while
/// premise+pin stay live — NormView must not present the laundered triple as
/// rule-eligible.
#[test]
fn correct_of_derived_object_change_does_not_inherit_eligibility() {
    let mut rt = Runtime::default();
    let (_claim_p, claim_q, q, rel, true_) = setup_derived_q(&mut rt);
    let laundered = rt.intern("laundered");

    rt.emit(Op::Correct {
        event_id: claim_q,
        object: laundered,
        valid_from: 2010,
        valid_to: Some(2030),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(
        rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: premise+pin still live — eligibility walk alone would pass"
    );
    assert!(
        rt.as_of(ValidTime::from(2017))
            .contains(&(q, rel, laundered)),
        "pin: as_of stays support-live for the Correct residual (ASOF-ELIG-01)"
    );
    assert!(
        !rt.as_of(ValidTime::from(2017)).contains(&(q, rel, true_)),
        "pin: original Behavior head object is no longer live"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Ok(triples) => {
            assert!(
                !triples.iter().any(|t| {
                    t.claim_id == claim_q && t.object == laundered && t.eligible
                }),
                "Correct of derived that changes object must not inherit rule eligibility; \
                 triples={triples:?}"
            );
            assert!(
                triples.iter().any(|t| t.object == true_ && t.eligible),
                "premise Assert remains Ok; triples={triples:?}"
            );
        }
        NormView::Abstain => panic!(
            "premise Assert should keep Ok; Abstain means over-filtering"
        ),
        other => panic!("expected Ok without laundered derived; got {other:?}"),
    }
}

/// Companion: CorrectInterval patch object on derived must not launder at the
/// patched VT cut.
#[test]
fn correct_interval_of_derived_object_change_does_not_inherit_eligibility() {
    let mut rt = Runtime::default();
    let (_claim_p, claim_q, q, rel, _true_) = setup_derived_q(&mut rt);
    let laundered = rt.intern("laundered-ci");

    rt.emit(Op::CorrectInterval {
        event_id: claim_q,
        object: laundered,
        patch_from: 2015,
        patch_to: Some(2025),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(
        rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: premise+pin still live"
    );
    assert!(
        rt.as_of(ValidTime::from(2017))
            .contains(&(q, rel, laundered)),
        "pin: patched residual live at 2017"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Ok(triples) => {
            assert!(
                !triples.iter().any(|t| {
                    t.claim_id == claim_q && t.object == laundered && t.eligible
                }),
                "CorrectInterval object patch on derived must not inherit eligibility; \
                 triples={triples:?}"
            );
        }
        NormView::Abstain => panic!(
            "premise Assert should keep Ok; Abstain means over-filtering"
        ),
        other => panic!("expected Ok without laundered derived; got {other:?}"),
    }
}
