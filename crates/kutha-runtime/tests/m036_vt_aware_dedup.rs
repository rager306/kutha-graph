//! M036 S01 / Phase 96 — Horn duplicate suppression must be VT-aware
//! (CE review P1 #11). Disjoint VT windows of the same triple must each
//! mint a derived inverse; overlapping windows still suppress.

use kutha_common::Op;
use kutha_runtime::Runtime;

fn live_known_by(rt: &Runtime, s: kutha_common::TermId, o: kutha_common::TermId, vt: u64) -> bool {
    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.subject == s && f.relation == kb && f.object() == o && f.is_live_at(u64::MAX, vt))
}

#[test]
fn disjoint_vt_windows_mint_separate_derived_inverses() {
    let mut rt = Runtime::default();
    let alice = rt.intern("Alice");
    let bob = rt.intern("Bob");
    let knows = rt.intern("knows");

    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2010,
        valid_to: Some(2015),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    assert!(
        live_known_by(&rt, bob, alice, 2012),
        "first window mints inverse"
    );

    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2020,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    assert!(
        live_known_by(&rt, bob, alice, 2012),
        "early window inverse stays live"
    );
    assert!(
        live_known_by(&rt, bob, alice, 2021),
        "disjoint later window must mint its own derived inverse"
    );
    assert!(
        !live_known_by(&rt, bob, alice, 2017),
        "gap between windows has no inverse"
    );
}

#[test]
fn overlapping_vt_windows_still_suppress_duplicate_derived() {
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
    rt.emit(Op::Assert {
        subject: alice,
        relation: knows,
        object: bob,
        valid_from: 2015,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();

    let kb = rt.dictionary().id("knownBy").expect("knownBy");
    let live_derived = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| {
            f.subject == bob
                && f.relation == kb
                && f.object() == alice
                && f.invalidated_at.is_none()
        })
        .count();
    assert_eq!(
        live_derived, 1,
        "overlapping VT must still suppress a second live derived inverse"
    );
    assert!(
        live_known_by(&rt, bob, alice, 2017),
        "single overlapping inverse covers the cut"
    );
}
