//! M012a S06 / HOT-01: fold-internal as_of and claim_supported_at skip non-overlapping facts.

use kutha_common::{EventId, Op, TermId};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::path::PathBuf;

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

fn brute_force_claim_supported(rt: &Runtime, claim: EventId, tt: u64, vt: u64) -> bool {
    rt.fold()
        .facts()
        .iter()
        .any(|f| f.claim_id == claim && f.is_live_at(tt, vt))
}

fn uuid_like() -> String {
    format!(
        "{:x}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    )
}

fn temp_dir(label: &str) -> PathBuf {
    std::env::temp_dir().join(format!("kutha-m012a-s06-{label}-{}", uuid_like()))
}

#[test]
fn hot_as_of_matches_brute_force_after_retract() {
    let mut rt = Runtime::default();
    let a = rt.intern("Alice");
    let b = rt.intern("Bob");
    let rel = rt.intern("relatedTo");
    let event_id = emit_related_to(&mut rt, a, rel, b, 2017);
    let triple = (a, rel, b);

    let mut live = rt.fold().as_of(2017);
    live.sort_unstable();
    assert!(live.contains(&triple));
    assert!(rt.fold().claim_supported_at(event_id, u64::MAX, 2017));
    assert_eq!(live, brute_force_as_of(&rt, 2017));
    assert_eq!(
        rt.fold().claim_supported_at(event_id, u64::MAX, 2017),
        brute_force_claim_supported(&rt, event_id, u64::MAX, 2017)
    );

    rt.emit(Op::Retract { event_id }).unwrap();

    let mut after = rt.fold().as_of(2017);
    after.sort_unstable();
    assert!(
        !after.contains(&triple),
        "retracted 2017 triple must leave as_of(2017)"
    );
    assert!(!rt.fold().claim_supported_at(event_id, u64::MAX, 2017));
    assert_eq!(after, brute_force_as_of(&rt, 2017));
    assert_eq!(
        rt.fold().claim_supported_at(event_id, u64::MAX, 2017),
        brute_force_claim_supported(&rt, event_id, u64::MAX, 2017)
    );
}

#[test]
fn snapshot_fold_rebuilds_hot_maps_from_facts() {
    let (rt, _live_claim, a, rel, b) = tracer_runtime();
    let before = {
        let mut triples = rt.fold().as_of(2017);
        triples.sort_unstable();
        triples
    };
    let fp = rt.fold().fingerprint();
    assert!(before.contains(&(a, rel, b)));

    let dir = temp_dir("snap-rebuild");
    store::persist(&rt, &dir).unwrap();

    let snap: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join("snapshot.json")).unwrap()).unwrap();
    let fold = snap.get("fold").expect("snapshot fold object");
    assert!(fold.get("facts").is_some(), "fold JSON must include facts");
    assert!(
        fold.get("next_seq").is_some(),
        "fold JSON must include next_seq"
    );
    assert!(
        fold.get("vt_by_from").is_none() && fold.get("claim_facts").is_none(),
        "hot maps must not be required snapshot keys"
    );

    let opened = store::open(&dir).expect("persist then open");
    let mut opened_as_of = opened.fold().as_of(2017);
    opened_as_of.sort_unstable();
    assert_eq!(opened_as_of, before);
    assert_eq!(opened.fold().fingerprint(), fp);
    opened.replay_check().unwrap();

    let fact_len = opened.fold().facts().len();
    opened.fold().reset_hot_examine_count();
    let _ = opened.fold().as_of(2017);
    let examined = opened.fold().hot_examine_count();
    assert!(
        examined < fact_len as u64,
        "opened as_of must still skip decoys (examined {examined}, facts {fact_len})"
    );

    let _ = fs::remove_dir_all(&dir);
}
