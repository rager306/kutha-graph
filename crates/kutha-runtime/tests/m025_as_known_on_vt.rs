//! M025 S01 / Phase 81 — G-A08 / G-C03: `as_known_on` is TT-only by design.
//! A fact not yet in force on VT still appears at a knowledge cut (TR-02 trap pin).

use kutha_common::{Op, ValidTime};
use kutha_runtime::Runtime;

/// Future VT (valid_from after the wall cut): `as_of` empty; `as_known_on` contains.
#[test]
fn as_known_on_includes_not_yet_in_force_vt_fact() {
    let mut rt = Runtime::default();
    let art = rt.intern("art-m025-future");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("limit-m025");

    // Ingest first so the knowledge cut's TT includes the fact; VT still starts 2020.
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

    let triple = (art, rel, obj);
    assert!(
        !rt.as_of(ValidTime::from(2017)).contains(&triple),
        "as_of(2017) must not see valid_from=2020"
    );
    let known = rt
        .as_known_on(2017)
        .expect("TtAnchor 2017 resolves the knowledge cut");
    assert!(
        known.contains(&triple),
        "as_known_on is TT-only by design: ingested fact visible despite future VT"
    );
}

/// Closed past VT survives knowledge cut (existing KNOWN-02; pin kept for M025 close).
#[test]
fn as_known_on_keeps_closed_vt_that_as_of_sees_in_window() {
    let mut rt = Runtime::default();
    let art = rt.intern("art-m025-closed");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("50-kmh-m025");

    rt.emit(Op::Assert {
        subject: art,
        relation: rel,
        object: obj,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();

    let triple = (art, rel, obj);
    assert!(rt.as_of(ValidTime::from(2015)).contains(&triple));
    let known = rt.as_known_on(2015).expect("anchor");
    assert!(
        known.contains(&triple),
        "closed VT interval stays visible on TT-only known-on"
    );
}
