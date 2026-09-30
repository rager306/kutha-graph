//! M012a S06 / HOT-01: fold-internal as_of and claim_supported_at skip non-overlapping facts.

use kutha_common::{EventId, Op, TermId};
use kutha_runtime::Runtime;

/// Brute-force live triples at `as_of(vt)` using `facts()` + `is_live_at` only (test oracle).
fn brute_force_as_of(rt: &Runtime, vt: u64) -> Vec<(TermId, TermId, TermId)> {
    let mut triples: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.is_live_at(u64::MAX, vt))
        .map(|f| (f.subject, f.relation, f.object()))
        .collect();
    triples.sort_unstable();
    triples
}

fn emit_related_to(
    rt: &mut Runtime,
    subject: TermId,
    rel: TermId,
    object: TermId,
    valid_from: u64,
) -> EventId {
    rt.emit(Op::Assert {
        subject,
        relation: rel,
        object,
        valid_from,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap()
    .receipt
    .event_ids[0]
}

fn tracer_runtime() -> (Runtime, EventId, TermId, TermId, TermId) {
    let mut rt = Runtime::default();
    let rel = rt.intern("relatedTo");
    for i in 0..128u32 {
        let s = rt.intern(&format!("decoy-s-{i}"));
        let o = rt.intern(&format!("decoy-o-{i}"));
        emit_related_to(&mut rt, s, rel, o, 3000);
    }
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let c = rt.intern("Carol");
    let d = rt.intern("Dana");
    let live_one = emit_related_to(&mut rt, a, rel, b, 2017);
    emit_related_to(&mut rt, a, rel, c, 2017);
    emit_related_to(&mut rt, b, rel, d, 2017);
    (rt, live_one, a, rel, b)
}

#[test]
fn as_of_and_claim_supported_at_skip_non_overlapping_facts() {
    let (rt, live_claim, a, rel, b) = tracer_runtime();
    let fold = rt.fold();
    let fact_len = fold.facts().len();
    assert!(
        fact_len > 8,
        "decoy-heavy log must have more facts than the slack bound"
    );

    let mut got = fold.as_of(2017);
    got.sort_unstable();
    let expected = brute_force_as_of(&rt, 2017);
    assert_eq!(
        got, expected,
        "as_of(2017) must match brute-force is_live_at(MAX, 2017)"
    );
    assert!(
        got.contains(&(a, rel, b)),
        "YearCe 2017 live triple must appear in as_of(2017)"
    );

    let overlapping = fold
        .facts()
        .iter()
        .filter(|f| f.valid_from <= 2017 && f.valid_to.is_none_or(|to| 2017 < to))
        .count() as u64;

    fold.reset_hot_examine_count();
    let _ = fold.as_of(2017);
    let as_of_examined = fold.hot_examine_count();
    assert!(
        as_of_examined < fact_len as u64,
        "as_of must not examine every fact (examined {as_of_examined}, facts {fact_len})"
    );
    assert!(
        as_of_examined <= overlapping + 8,
        "as_of examine {as_of_examined} exceeds overlapping {overlapping} plus slack 8"
    );

    assert!(
        fold.claim_supported_at(live_claim, u64::MAX, 2017),
        "minting Event.id of a 2017 Assert must be supported at as_of(2017)"
    );
    assert!(
        !fold.claim_supported_at(EventId::nil(), u64::MAX, 2017),
        "nil claim must not be supported"
    );

    let claim_slots = fold
        .facts()
        .iter()
        .filter(|f| f.claim_id == live_claim)
        .count() as u64;

    fold.reset_hot_examine_count();
    let supported = fold.claim_supported_at(live_claim, u64::MAX, 2017);
    let claim_examined = fold.hot_examine_count();
    assert!(supported);
    assert!(
        claim_examined <= claim_slots + 2,
        "claim_supported_at examine {claim_examined} exceeds claim slots {claim_slots} plus 2"
    );
    assert!(
        claim_examined < fact_len as u64,
        "claim_supported_at must not walk all facts (examined {claim_examined}, facts {fact_len})"
    );
}
