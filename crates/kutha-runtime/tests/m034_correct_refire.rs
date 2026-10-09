//! M034 S01 / Phase 94 — Correct follow-ons must re-fire Horn when the
//! replacement keeps the same triple (CE review P1 #9). Mirror CorrectInterval:
//! except-set + fires-then-retracts so LIFO retracts before re-fires.

use kutha_common::Op;
use kutha_runtime::Runtime;

fn live_triple(
    rt: &Runtime,
    s: kutha_common::TermId,
    r: kutha_common::TermId,
    o: kutha_common::TermId,
    vt: u64,
) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == r && f.object() == o && f.is_live_at(u64::MAX, vt))
}

fn live_known_by(rt: &Runtime, s: kutha_common::TermId, o: kutha_common::TermId) -> bool {
    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    live_triple(rt, s, kb, o, 2017)
}

#[test]
fn correct_same_triple_keeps_derived_inverse_live() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
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
    assert!(
        live_known_by(&rt, bob, alice),
        "inverse_knows mints Bob knownBy Alice"
    );

    // VT-only Correct: same (alice, knows, bob) triple — derived must re-fire.
    rt.emit(Op::Correct {
        event_id: prem,
        object: bob,
        valid_from: 2010,
        valid_to: Some(2030),
    })
    .unwrap();

    assert!(
        live_triple(&rt, alice, knows, bob, 2017),
        "corrected premise still live inside new VT"
    );
    assert!(
        live_known_by(&rt, bob, alice),
        "same-triple Correct must keep derived inverse live (except-set + fire order)"
    );
}

#[test]
fn correct_object_change_re_fires_new_inverse() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let carol = rt.intern("Carol");
    let knows = rt.intern("knows");
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
    assert!(live_known_by(&rt, bob, alice));

    rt.emit(Op::Correct {
        event_id: prem,
        object: carol,
        valid_from: 2010,
        valid_to: None,
    })
    .unwrap();

    assert!(
        !live_known_by(&rt, bob, alice),
        "stale Bob inverse must retract"
    );
    assert!(
        live_known_by(&rt, carol, alice),
        "Carol inverse must re-fire after Correct of premise"
    );
}
