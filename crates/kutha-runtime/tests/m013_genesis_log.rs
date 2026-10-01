//! M013 S04 / Phase 25: genesis-log RED oracles (YAML HashSet fallback).
//! Constructors still load YAML; 25-02 wires genesis. Do not intern extra YAML names.

use kutha_common::Op;
use kutha_runtime::{Runtime, RuntimeError};

#[test]
fn retract_seeded_related_to_fails_closed_after() {
    let mut rt = Runtime::default();
    let cut = rt.log().len();

    let retract_id = rt.log().iter().find_map(|e| match &e.op {
        Op::AllowRelation { name, .. } if name == "relatedTo" => Some(e.id),
        _ => None,
    });
    if let Some(id) = retract_id {
        rt.emit(Op::Retract { event_id: id }).unwrap();
    }

    let relation = rt.intern("relatedTo");
    let subject = rt.intern("alice-seed");
    let object = rt.intern("bob-seed");
    let n = rt.log().len();

    let err = rt
        .emit(Op::Assert {
            subject,
            relation,
            object,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap_err();
    assert!(
        matches!(err, RuntimeError::UnknownRelation { ref name } if name == "relatedTo"),
        "{err:?}"
    );
    assert_eq!(n, rt.log().len(), "fail-closed: log must not grow");

    let mut prefix = rt.fork_at(cut);
    let rel = prefix.intern("relatedTo");
    let s = prefix.intern("alice-seed");
    let o = prefix.intern("bob-seed");
    prefix
        .emit(Op::Assert {
            subject: s,
            relation: rel,
            object: o,
            valid_from: 2017,
            valid_to: None,
            claim: None,
            delivery_key: None,
            polarity: None,
        })
        .unwrap();
}
