//! M013 S06 / Phase 27: D-08 RED — unknown-relation admit is not shared by
//! justification/action. Uses current public emit / record_justification / record_action
//! (must compile). Never edit named m012_*/m012a_*/m011_*/ff5/ff6 expects.
//! Never edit `.kutha/STATE.md`.

use kutha_common::{EventId, Op};
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn unknown_relation_rejected_for_assert_justification_and_action() {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let ghost = rt.intern("notALegalRelation");
    let limit = rt.intern("50-kmh");

    let err = rt
        .emit(Op::Assert {
            subject: article,
            relation: ghost,
            object: limit,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "notALegalRelation"),
        "{err:?}"
    );

    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let outcomes_before = rt.outcome_records().len();

    let dummy = EventId::nil();
    let _ = rt.record_justification(dummy, Vec::new(), Vec::new(), "", u64::MAX, 2010);
    let _ = rt.record_action("missing", u64::MAX, 2010);

    assert!(
        rt.outcome_records().len() > outcomes_before,
        "justification/action must share Assert admit/receipt (outcome_records grow)"
    );
    assert!(
        !rt.fold()
            .facts()
            .iter()
            .any(|f| f.relation == ghost),
        "no new fact may carry interned notALegalRelation"
    );
}
