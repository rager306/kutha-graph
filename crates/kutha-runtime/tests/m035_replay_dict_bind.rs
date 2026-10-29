//! M035 S01 / Phase 95 — GraphFold replay must bind reserved kutha:* for
//! product queries (CE review P1 #10). Bare `replay` leaves dictionary_rel_flags
//! empty so as_of includes genesis allows; bound replay matches live.

use kutha_common::{Op, ValidTime};
use kutha_runtime::{GraphFold, Runtime};

#[test]
fn bare_replay_as_of_includes_dictionary_facts() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let live = rt.as_of(ValidTime::from(2017));
    let bare = GraphFold::replay(rt.log().as_slice()).as_of(ValidTime::from(2017));
    assert!(
        bare.len() > live.len(),
        "dictionary-blind replay must leak kutha:* into product as_of (live={}, bare={})",
        live.len(),
        bare.len()
    );
    assert!(
        live.contains(&(alice, knows, bob)),
        "product triple present on live"
    );
}

#[test]
fn replay_with_dict_as_of_matches_live() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let live = rt.as_of(ValidTime::from(2017));
    let rebuilt = GraphFold::replay_with_dict(rt.log().as_slice(), rt.dictionary());
    assert_eq!(
        rebuilt.as_of(ValidTime::from(2017)),
        live,
        "bound replay product as_of must match live Runtime"
    );
}

#[test]
fn replay_check_returns_bound_fold() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let rebuilt = rt.replay_check().expect("fingerprint still matches");
    assert_eq!(
        rebuilt.as_of(ValidTime::from(2017)),
        rt.as_of(ValidTime::from(2017)),
        "replay_check fold must be query-ready (reserved bound)"
    );
}
