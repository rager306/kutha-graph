//! M025 S03 / Phase 83 — legal answer envelope `norm_view(vt, wall)`.
//!
//! Combines VT force with knowledge cut; live ∩ eligible is the facade's job
//! (R-22 / G-B01). Typed non-success: Conflict / Abstain / Unknown / MissingAnchor
//! (G-B04 / G-C10 / TR-06). Does not change `as_of` or ASOF-ELIG-01.

use kutha_common::{Op, SupportPolarity, TransactionTime, ValidTime, WallTime, rule_definition_hash};
use kutha_runtime::{NormView, Runtime};

/// No TtAnchor at or before wall → MissingAnchor (not Option::None).
#[test]
fn norm_view_missing_anchor_when_no_tt_anchor() {
    let mut rt = Runtime::default();
    let art = rt.intern("art-nv-miss");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("limit-nv");
    rt.emit(Op::Assert {
        subject: art,
        relation: rel,
        object: obj,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert_eq!(
        rt.norm_view(ValidTime::from(2015), WallTime::from(2015)),
        NormView::MissingAnchor,
        "without TtAnchor, envelope must be MissingAnchor"
    );
}

/// Retracted premise: as_of still lists derived; norm_view must not treat it as in force.
#[test]
fn norm_view_abstains_when_derived_ineligible_after_premise_retract() {
    let mut rt = Runtime::default();
    let p = rt.intern("P-nv");
    let q = rt.intern("Q-nv");
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
    let event_a = claim_p;

    let second = rt
        .emit(Op::Assert {
            subject: p,
            relation: rel,
            object: true_,
            valid_from: 2010,
            valid_to: None,
            claim: Some(claim_p),
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    assert!(!second.receipt.event_ids.is_empty());
    let event_b = rt
        .fold()
        .facts()
        .iter()
        .find(|f| f.claim_id == claim_p && f.event_id != event_a)
        .unwrap()
        .event_id;

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

    rt.emit(Op::Retract { event_id: event_a }).unwrap();
    rt.emit(Op::Retract { event_id: event_b }).unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(
        !rt.derivation_eligible_at(claim_q, TransactionTime::MAX, ValidTime::from(2017)),
        "pin: Q ineligible after last premise support withdrawn"
    );
    assert!(
        rt.as_of(ValidTime::from(2017)).contains(&(q, rel, true_)),
        "pin: as_of stays support-live (ASOF-ELIG-01 unchanged)"
    );

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Abstain => {}
        other => panic!("expected Abstain for ineligible-only picture, got {other:?}"),
    }
}

/// Happy path: asserted fact in force on VT and known on wall.
#[test]
fn norm_view_ok_lists_assert_in_force_at_knowledge_cut() {
    let mut rt = Runtime::default();
    let art = rt.intern("art-nv-ok");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("50-kmh-nv");

    let out = rt
        .emit(Op::Assert {
            subject: art,
            relation: rel,
            object: obj,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
    let claim = out.receipt.event_ids[0];
    rt.emit_tt_anchor(2015).unwrap();

    match rt.norm_view(ValidTime::from(2015), WallTime::from(2015)) {
        NormView::Ok(triples) => {
            assert!(
                triples.iter().any(|t| {
                    t.subject == art
                        && t.relation == rel
                        && t.object == obj
                        && t.claim_id == claim
                        && t.eligible
                }),
                "expected eligible assert triple, got {triples:?}"
            );
        }
        other => panic!("expected Ok, got {other:?}"),
    }
}

/// Future VT relative to force cut: not in force → Unknown (anchor present).
#[test]
fn norm_view_unknown_when_fact_not_yet_in_force() {
    let mut rt = Runtime::default();
    let art = rt.intern("art-nv-future");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("limit-future-nv");

    rt.emit(Op::Assert {
        subject: art,
        relation: rel,
        object: obj,
        valid_from: 2020,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    assert!(rt.as_of(ValidTime::from(2017)).is_empty());
    assert!(
        rt.as_known_on(WallTime::from(2017))
            .expect("anchor")
            .contains(&(art, rel, obj)),
        "knowledge cut still sees future-VT fact (TT-only)"
    );
    assert_eq!(
        rt.norm_view(ValidTime::from(2017), WallTime::from(2017)),
        NormView::Unknown,
        "force∩knowledge empty → Unknown"
    );
}

/// Both polarities live on a claim at the cut → Conflict.
#[test]
fn norm_view_conflict_when_both_polarities_live() {
    let mut rt = Runtime::default();
    let s = rt.intern("S-nv");
    let rel = rt.intern("relatedTo");
    let o = rt.intern("O-nv");

    let first = rt
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("pol-pos-nv".into()),
            polarity: Some(SupportPolarity::Positive),
        })
        .unwrap();
    let claim = first.receipt.event_ids[0];
    rt.emit(Op::Assert {
        subject: s,
        relation: rel,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: Some(claim),
        delivery_key: Some("pol-neg-nv".into()),
        polarity: Some(SupportPolarity::Negative),
    })
    .unwrap();
    rt.emit_tt_anchor(2017).unwrap();

    match rt.norm_view(ValidTime::from(2017), WallTime::from(2017)) {
        NormView::Conflict {
            claim: c,
            report,
        } => {
            assert_eq!(c, claim);
            assert!(!report.positive_supports.is_empty());
            assert!(!report.negative_supports.is_empty());
        }
        other => panic!("expected Conflict, got {other:?}"),
    }
}
