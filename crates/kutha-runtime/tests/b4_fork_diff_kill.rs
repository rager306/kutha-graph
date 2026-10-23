//! Independent verifier kill-tests for M156 / phase 230 `diff_folds`.
//! Expectations come from ADR-061 (fact-set FoldDiff) and the public API:
//! domain is `facts()` including dead rows; identity is `(event_id, valid_from,
//! valid_to)` plus rank among equal keys by a total order over every other
//! field; `seq` never keys, ranks, or marks changed. Not executor tests as SoT.

use kutha_common::{Event, EventId, Op, SupportPolarity};
use kutha_runtime::{
    diff_folds, store, Fact, FactChange, FoldDiff, GraphFold, Runtime, RuntimeError,
};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

mod common;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn tail_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(8));
    LimitGuard
}

fn persist_dir(label: &str) -> common::PersistScratch {
    common::PersistScratch::new(&format!("b4-kill-{label}"))
}

fn kill_case_count() -> u64 {
    match std::env::var("KUTHA_KILL_CASES") {
        Err(std::env::VarError::NotPresent) => 16,
        Ok(s) => {
            let n: u64 = s.parse().unwrap_or_else(|_| {
                panic!("KUTHA_KILL_CASES must be a positive integer, got {s:?}")
            });
            if n == 0 {
                panic!("KUTHA_KILL_CASES must be a positive integer, got {s:?}");
            }
            n
        }
        Err(e) => panic!("KUTHA_KILL_CASES must be a positive integer, got {e}"),
    }
}

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn bounded(&mut self, n: u64) -> u64 {
        if n == 0 {
            0
        } else {
            self.next() % n
        }
    }
}

fn shuffle<T>(items: &mut [T], rng: &mut Rng) {
    let n = items.len();
    if n < 2 {
        return;
    }
    for i in (1..n).rev() {
        let j = rng.bounded((i + 1) as u64) as usize;
        items.swap(i, j);
    }
}

fn same_except_seq(a: &Fact, b: &Fact) -> bool {
    a.subject == b.subject
        && a.relation == b.relation
        && a.object() == b.object()
        && a.valid_from == b.valid_from
        && a.valid_to == b.valid_to
        && a.ingested_at == b.ingested_at
        && a.invalidated_at == b.invalidated_at
        && a.claim_id == b.claim_id
        && a.event_id == b.event_id
        && a.delivery_key == b.delivery_key
        && a.polarity == b.polarity
}

fn pol_rank(p: Option<SupportPolarity>) -> u8 {
    match p {
        None => 0,
        Some(SupportPolarity::Positive) => 1,
        Some(SupportPolarity::Negative) => 2,
    }
}

/// Independent row fingerprint: every Fact field except `seq`.
fn row_fp(f: &Fact) -> String {
    format!(
        "{:?}|{}|{}|{}|{}|{:?}|{}|{:?}|{:?}|{:?}|{}",
        f.event_id,
        f.subject,
        f.relation,
        f.object(),
        f.valid_from,
        f.valid_to,
        f.ingested_at,
        f.invalidated_at,
        f.claim_id,
        f.delivery_key,
        pol_rank(f.polarity)
    )
}

fn bag(rows: &[Fact]) -> BTreeMap<String, usize> {
    let mut m = BTreeMap::new();
    for f in rows {
        *m.entry(row_fp(f)).or_insert(0) += 1;
    }
    m
}

/// Apply the edit script to `a`'s rows without reading `b`. Matching ignores `seq`.
fn apply_diff(rows_a: &[Fact], diff: &FoldDiff) -> Vec<Fact> {
    let mut bag_rows = rows_a.to_vec();
    for rem in &diff.removed {
        let i = bag_rows
            .iter()
            .position(|f| same_except_seq(f, rem))
            .expect("removed row not present in a (modulo seq)");
        bag_rows.remove(i);
    }
    for ch in &diff.changed {
        let i = bag_rows
            .iter()
            .position(|f| same_except_seq(f, &ch.before))
            .expect("changed.before not present in a (modulo seq)");
        bag_rows[i] = ch.after.clone();
    }
    bag_rows.extend(diff.added.iter().cloned());
    bag_rows
}

fn assert_empty(diff: &FoldDiff, ctx: &str) {
    assert!(
        diff.added.is_empty() && diff.removed.is_empty() && diff.changed.is_empty(),
        "{ctx}: expected empty FoldDiff, got added={} removed={} changed={}",
        diff.added.len(),
        diff.removed.len(),
        diff.changed.len()
    );
}

fn assert_mirror(ab: &FoldDiff, ba: &FoldDiff, ctx: &str) {
    assert_eq!(
        bag(&ab.added),
        bag(&ba.removed),
        "{ctx}: added vs reverse removed"
    );
    assert_eq!(
        bag(&ab.removed),
        bag(&ba.added),
        "{ctx}: removed vs reverse added"
    );
    let mut ab_ch: BTreeMap<String, usize> = BTreeMap::new();
    for c in &ab.changed {
        *ab_ch
            .entry(format!("{}=>{}", row_fp(&c.before), row_fp(&c.after)))
            .or_insert(0) += 1;
    }
    let mut ba_ch: BTreeMap<String, usize> = BTreeMap::new();
    for c in &ba.changed {
        *ba_ch
            .entry(format!("{}=>{}", row_fp(&c.after), row_fp(&c.before)))
            .or_insert(0) += 1;
    }
    assert_eq!(ab_ch, ba_ch, "{ctx}: changed not swapped");
}

fn assert_totality(a: &GraphFold, b: &GraphFold, ctx: &str) {
    let aa = diff_folds(a, a);
    assert_empty(&aa, &format!("{ctx} diff(a,a)"));
    assert_eq!(
        a.diff(a),
        aa,
        "{ctx}: GraphFold::diff wrapper vs diff_folds on a,a"
    );
    let ab = diff_folds(a, b);
    let ba = diff_folds(b, a);
    assert_eq!(a.diff(b), ab, "{ctx}: GraphFold::diff wrapper");
    assert_mirror(&ab, &ba, ctx);
    let got = apply_diff(a.facts(), &ab);
    assert_eq!(
        bag(&got),
        bag(b.facts()),
        "{ctx}: apply_diff(a, diff_folds(a,b)) must equal b as a multiset modulo seq"
    );
}

fn fold_from_values(facts: Vec<Value>) -> GraphFold {
    serde_json::from_value(json!({
        "facts": facts,
        "next_seq": 10_000u64,
    }))
    .expect("GraphFold deserialize from facts JSON")
}

fn facts_json(fold: &GraphFold) -> Vec<Value> {
    serde_json::to_value(fold)
        .expect("serialize GraphFold")
        .get("facts")
        .and_then(Value::as_array)
        .cloned()
        .expect("facts array")
}

fn permute_facts(fold: &GraphFold, rng: &mut Rng) -> GraphFold {
    let mut facts = facts_json(fold);
    shuffle(&mut facts, rng);
    fold_from_values(facts)
}

fn bump_seq(fold: &GraphFold, delta: u64) -> GraphFold {
    let mut facts = facts_json(fold);
    for (i, f) in facts.iter_mut().enumerate() {
        let seq = f.get("seq").and_then(Value::as_u64).unwrap_or(0);
        f["seq"] = json!(seq.saturating_add(delta).saturating_add(i as u64 * 17));
    }
    fold_from_values(facts)
}

fn fact_value(
    seq: u64,
    subject: u32,
    relation: u32,
    object: u32,
    valid_from: u64,
    valid_to: Option<u64>,
    ingested_at: u64,
    invalidated_at: Option<u64>,
    claim: u128,
    event: u128,
    delivery_key: Option<&str>,
    polarity: Option<&str>,
) -> Value {
    json!({
        "seq": seq,
        "subject": subject,
        "relation": relation,
        "object": object,
        "valid_from": valid_from,
        "valid_to": valid_to,
        "ingested_at": ingested_at,
        "invalidated_at": invalidated_at,
        "claim_id": EventId::from_u128(claim).to_string(),
        "event_id": EventId::from_u128(event).to_string(),
        "delivery_key": delivery_key,
        "polarity": polarity,
    })
}

fn polarity_token(rng: &mut Rng) -> Option<&'static str> {
    match rng.bounded(3) {
        0 => None,
        1 => Some("positive"),
        _ => Some("negative"),
    }
}

fn random_fact(rng: &mut Rng, seq: u64, collide: Option<(u128, u64, Option<u64>)>) -> Value {
    let (event, vf, vt) = collide.unwrap_or_else(|| {
        let event = rng.bounded(32) as u128 + 1;
        let vf = 1990 + rng.bounded(40);
        let vt = match rng.bounded(4) {
            0 => None,
            1 => Some(u64::MAX),
            n => Some(vf + 1 + n),
        };
        (event, vf, vt)
    });
    let dk = if rng.bounded(2) == 0 {
        None
    } else {
        Some("dk")
    };
    let dk = dk.map(|s| format!("{s}-{}", rng.bounded(5)));
    fact_value(
        seq,
        rng.bounded(6) as u32,
        rng.bounded(4) as u32 + 1,
        rng.bounded(8) as u32,
        vf,
        vt,
        rng.bounded(50),
        if rng.bounded(4) == 0 {
            Some(rng.bounded(50))
        } else {
            None
        },
        rng.bounded(16) as u128 + 1,
        event,
        dk.as_deref(),
        polarity_token(rng),
    )
}

fn random_fold(rng: &mut Rng, n: usize) -> GraphFold {
    let mut facts = Vec::with_capacity(n);
    let mut last_key: Option<(u128, u64, Option<u64>)> = None;
    for i in 0..n {
        let collide = if last_key.is_some() && rng.bounded(3) == 0 {
            last_key
        } else {
            None
        };
        let v = random_fact(rng, i as u64, collide);
        last_key = v.get("event_id").and_then(Value::as_str).and_then(|s| {
            EventId::parse_str(s).ok().map(|id| {
                let vf = v.get("valid_from").and_then(Value::as_u64).unwrap_or(0);
                let vt = if v.get("valid_to").map(Value::is_null).unwrap_or(true) {
                    None
                } else {
                    v.get("valid_to").and_then(Value::as_u64)
                };
                (id.as_u128(), vf, vt)
            })
        });
        facts.push(v);
    }
    fold_from_values(facts)
}

fn mutate_fold(a: &GraphFold, rng: &mut Rng) -> GraphFold {
    let mut facts = facts_json(a);
    if !facts.is_empty() && rng.bounded(2) == 0 {
        let i = rng.bounded(facts.len() as u64) as usize;
        facts.remove(i);
    }
    if !facts.is_empty() && rng.bounded(2) == 0 {
        let i = rng.bounded(facts.len() as u64) as usize;
        facts[i]["polarity"] = match rng.bounded(3) {
            0 => json!(null),
            1 => json!("positive"),
            _ => json!("negative"),
        };
        if rng.bounded(2) == 0 {
            facts[i]["object"] = json!(rng.bounded(9));
        }
        if rng.bounded(3) == 0 {
            facts[i]["invalidated_at"] = if rng.bounded(2) == 0 {
                json!(null)
            } else {
                json!(rng.bounded(80))
            };
        }
        if rng.bounded(3) == 0 {
            facts[i]["delivery_key"] = if rng.bounded(2) == 0 {
                json!(null)
            } else {
                json!(format!("mut-{}", rng.bounded(7)))
            };
        }
    }
    if rng.bounded(2) == 0 {
        let extra_seq = 8_000 + rng.bounded(100);
        facts.push(random_fact(rng, extra_seq, None));
    }
    if rng.bounded(4) == 0 && !facts.is_empty() {
        let clone = facts[rng.bounded(facts.len() as u64) as usize].clone();
        facts.push(clone);
    }
    shuffle(&mut facts, rng);
    for (i, f) in facts.iter_mut().enumerate() {
        f["seq"] = json!(rng.bounded(50_000) + i as u64);
    }
    fold_from_values(facts)
}

fn assert_ev(
    id: u128,
    subject: u32,
    relation: u32,
    object: u32,
    valid_from: u64,
    valid_to: Option<u64>,
    ingested_at: u64,
    delivery_key: Option<&str>,
    polarity: Option<SupportPolarity>,
) -> Event {
    let mut e = Event::new(
        Op::Assert {
            subject,
            relation,
            object,
            valid_from,
            valid_to,
            claim: None,
            delivery_key: delivery_key.map(str::to_string),
            polarity,
        },
        ingested_at,
    );
    e.id = EventId::from_u128(id);
    e
}

fn retract_ev(id: u128, target: u128, ingested_at: u64) -> Event {
    let mut e = Event::new(
        Op::Retract {
            event_id: EventId::from_u128(target),
        },
        ingested_at,
    );
    e.id = EventId::from_u128(id);
    e
}

fn copy_store(src: &Path, dst: &Path) {
    fs::create_dir_all(dst).unwrap();
    for ent in fs::read_dir(src).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let from = ent.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_store(&from, &to);
        } else {
            fs::copy(&from, &to).unwrap();
        }
    }
}

fn first_checkpoint_index(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Op::Checkpoint on current log")
}

fn emit_keyed(
    rt: &mut Runtime,
    subject: &str,
    relation: &str,
    object: &str,
    valid_from: u64,
    valid_to: Option<u64>,
    key: &str,
) -> EventId {
    let subject = rt.intern(subject);
    let relation = rt.intern(relation);
    let object = rt.intern(object);
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from,
        valid_to,
        claim: None,
        delivery_key: Some(key.into()),
        polarity: None,
    })
    .unwrap()
    .receipt
    .event_ids[0]
}

fn pad(rt: &mut Runtime, n: usize, tag: &str) {
    for i in 0..n {
        let _ = rt.intern(&format!("{tag}-term-{i}"));
        emit_keyed(
            rt,
            "art-12-speed",
            "inForceAs",
            &format!("{tag}-obj-{i}"),
            2010,
            Some(2020),
            &format!("{tag}-dk-{i}"),
        );
    }
}

#[test]
fn kill_totality_mirror_aa_random_pairs() {
    let n = kill_case_count();
    for i in 0..n {
        let mut rng = Rng(0xC0FFEE ^ (i.wrapping_mul(0x9E37_79B9_7F4A_7C15)));
        let n_facts = 4 + rng.bounded(10) as usize;
        let a = random_fold(&mut rng, n_facts);
        let b = mutate_fold(&a, &mut rng);
        assert_totality(&a, &b, &format!("case {i}"));
        let shuffled = permute_facts(&a, &mut rng);
        assert_totality(&a, &shuffled, &format!("case {i} shuffled a vs a"));
        assert_empty(
            &diff_folds(&a, &shuffled),
            &format!("case {i}: shuffle of the same bag must be empty"),
        );
    }
}

#[test]
fn kill_equal_keys_full_duplicates_and_rank_not_insertion() {
    let id = 0xABCDu128;
    let mut events = Vec::new();
    let variants: [(u32, Option<SupportPolarity>, &str); 5] = [
        (1, None, "a"),
        (2, Some(SupportPolarity::Positive), "b"),
        (3, Some(SupportPolarity::Negative), "c"),
        (4, Some(SupportPolarity::Positive), "d"),
        (5, None, "e"),
    ];
    for (i, (obj, pol, dk)) in variants.iter().enumerate() {
        events.push(assert_ev(
            id,
            1,
            2,
            *obj,
            2010,
            None,
            10 + i as u64,
            Some(dk),
            *pol,
        ));
    }
    // Two full duplicates of variants[0], still the same identity key.
    events.push(assert_ev(id, 1, 2, 1, 2010, None, 10, Some("a"), None));
    events.push(assert_ev(id, 1, 2, 1, 2010, None, 10, Some("a"), None));
    let a = GraphFold::replay(&events);
    assert!(
        a.facts().len() >= 7,
        "replay must keep colliding rows, got {}",
        a.facts().len()
    );
    let mut rng = Rng(42);
    for i in 0..kill_case_count() {
        let mut shuffled_events = events.clone();
        shuffle(&mut shuffled_events, &mut rng);
        let b = GraphFold::replay(&shuffled_events);
        let b = bump_seq(&b, 1000 + i * 13);
        assert_empty(
            &diff_folds(&a, &b),
            &format!(
                "equal-key collisions + full duplicates must ignore insert order and seq (case {i})"
            ),
        );
        assert_totality(&a, &b, &format!("equal-key case {i}"));
    }
}

#[test]
fn kill_seq_only_empty_but_real_field_change_is_not() {
    let e = assert_ev(
        7,
        1,
        2,
        3,
        2010,
        Some(2020),
        1,
        Some("k"),
        Some(SupportPolarity::Positive),
    );
    let a = GraphFold::replay(std::slice::from_ref(&e));
    let b = bump_seq(&a, 99);
    assert_empty(
        &diff_folds(&a, &b),
        "rows that differ only in seq must produce an empty diff",
    );
    assert_ne!(
        a.facts()[0].seq,
        b.facts()[0].seq,
        "fixture must actually differ in seq"
    );

    let mut e2 = e.clone();
    if let Op::Assert { polarity, .. } = &mut e2.op {
        *polarity = Some(SupportPolarity::Negative);
    }
    let c = GraphFold::replay(std::slice::from_ref(&e2));
    let d = diff_folds(&a, &c);
    assert!(
        d.added.is_empty() && d.removed.is_empty() && d.changed.len() == 1,
        "polarity-only change must be changed, not hidden by seq: {d:?}"
    );
    assert!(
        same_except_seq(&d.changed[0].before, &a.facts()[0]),
        "changed.before must match a"
    );
    assert!(
        same_except_seq(&d.changed[0].after, &c.facts()[0]),
        "changed.after must match c"
    );
    assert_ne!(
        a.facts()[0].polarity,
        c.facts()[0].polarity,
        "fixture must differ in a real field"
    );
}

#[test]
fn kill_open_interval_is_not_u64_max_identity() {
    let open = assert_ev(9, 1, 2, 3, 2010, None, 1, Some("k"), None);
    let max_closed = assert_ev(9, 1, 2, 3, 2010, Some(u64::MAX), 1, Some("k"), None);
    let a = GraphFold::replay(std::slice::from_ref(&open));
    let b = GraphFold::replay(std::slice::from_ref(&max_closed));
    let d = diff_folds(&a, &b);
    assert!(
        d.changed.is_empty(),
        "None and Some(u64::MAX) are distinct valid_to values so identity must not collapse them into changed: {d:?}"
    );
    assert_eq!(d.removed.len(), 1, "open interval must be removed: {d:?}");
    assert_eq!(d.added.len(), 1, "max-closed interval must be added: {d:?}");
    assert_eq!(d.removed[0].valid_to, None);
    assert_eq!(d.added[0].valid_to, Some(u64::MAX));
    assert_totality(&a, &b, "open vs max-closed");
    let both_ab = GraphFold::replay(&[open.clone(), max_closed.clone()]);
    let both_ba = GraphFold::replay(&[max_closed, open]);
    assert_empty(
        &diff_folds(&both_ab, &both_ba),
        "both windows in one fold: distinct keys must not rank-tie on insert order",
    );
}

#[test]
fn kill_invalidated_at_none_vs_max_rank_is_total() {
    let live = fact_value(1, 1, 2, 3, 2010, None, 5, None, 1, 9, Some("k"), None);
    let dead_max = fact_value(
        2,
        1,
        2,
        3,
        2010,
        None,
        5,
        Some(u64::MAX),
        1,
        9,
        Some("k"),
        None,
    );
    let a = fold_from_values(vec![live.clone(), dead_max.clone()]);
    let b = fold_from_values(vec![dead_max, live]);
    assert_empty(
        &diff_folds(&a, &b),
        "None vs Some(u64::MAX) on invalidated_at must not tie-break by insertion order",
    );
    assert_totality(&a, &b, "invalidated_at None vs MAX");
}

#[test]
fn kill_dead_rows_are_in_the_domain() {
    let assert_id = 11u128;
    let live = GraphFold::replay(&[assert_ev(
        assert_id,
        1,
        2,
        3,
        1990,
        Some(2000),
        1,
        Some("expired"),
        None,
    )]);
    assert_eq!(live.facts().len(), 1);
    assert!(
        live.facts()[0].invalidated_at.is_none(),
        "expired-by-VT row stays in facts() with no invalidated_at"
    );
    let empty = GraphFold::default();
    let d_exp = diff_folds(&empty, &live);
    assert_eq!(d_exp.added.len(), 1, "expired row must still be added");
    assert!(d_exp.removed.is_empty() && d_exp.changed.is_empty());

    let retracted = GraphFold::replay(&[
        assert_ev(assert_id, 1, 2, 3, 2010, None, 1, Some("live"), None),
        retract_ev(12, assert_id, 2),
    ]);
    assert_eq!(retracted.facts().len(), 1);
    assert!(
        retracted.facts()[0].invalidated_at.is_some(),
        "Retract must leave a dead row in facts()"
    );
    let before = GraphFold::replay(&[assert_ev(
        assert_id,
        1,
        2,
        3,
        2010,
        None,
        1,
        Some("live"),
        None,
    )]);
    let d = diff_folds(&before, &retracted);
    assert!(
        d.added.is_empty() && d.removed.is_empty() && d.changed.len() == 1,
        "identity-stable Retract is changed, not add/remove (dead rows in domain): {d:?}"
    );
    assert!(d.changed[0].before.invalidated_at.is_none());
    assert!(d.changed[0].after.invalidated_at.is_some());
    assert_totality(&before, &retracted, "retract dead row");

    let mut rt = Runtime::default();
    let id = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "dead-rt",
    );
    let cut = rt.log().len();
    rt.emit(Op::Retract { event_id: id }).unwrap();
    let fork = rt.fork_at(cut).unwrap();
    let d_rt = diff_folds(fork.fold(), rt.fold());
    assert!(
        rt.fold()
            .facts()
            .iter()
            .any(|f| f.event_id == id && f.invalidated_at.is_some()),
        "runtime Retract must keep the dead row"
    );
    assert!(
        d_rt.changed.iter().any(|c| {
            c.before.event_id == id
                && c.before.invalidated_at.is_none()
                && c.after.event_id == id
                && c.after.invalidated_at.is_some()
        }),
        "runtime fork-then-retract must surface the dead row as changed: {d_rt:?}"
    );
    assert_totality(fork.fold(), rt.fold(), "runtime retract");
}

#[test]
fn kill_fail_closed_cut_never_reaches_diff() {
    let _g = tiny_limits();
    let dir = persist_dir("fail-closed");
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 8, "fc");
    rt.emit_tt_anchor(2015).unwrap();
    store::persist(&rt, &dir).unwrap();
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let opened = store::open(&dir).unwrap();
    let c = first_checkpoint_index(opened.log().as_slice());
    let len = opened.log().len();
    for n in 1..=c {
        match opened.fork_at(n) {
            Err(RuntimeError::ForkCutUnavailable { cut, log_len }) => {
                assert_eq!(cut, n);
                assert_eq!(log_len, len);
            }
            Ok(_) => panic!("fork_at({n}) Ok — diff must not be reachable on cut in 1..={c}"),
            Err(e) => panic!("fork_at({n}) wrong error {e:?}"),
        }
    }
}

#[test]
fn kill_checkpointed_fork_matches_plain_twin() {
    let _g = tail_limits();
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    pad(&mut rt, 10, "twin");
    rt.emit_tt_anchor(2015).unwrap();
    pad(&mut rt, 4, "tail");
    let ckp = persist_dir("twin-ckp");
    let twin = persist_dir("twin-plain");
    store::persist(&rt, &ckp).unwrap();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let ckpt = store::open(&ckp).unwrap();
    let plain = store::open(&twin).unwrap();
    let c = first_checkpoint_index(ckpt.log().as_slice());
    let len = ckpt.log().len();
    assert!(
        len > c + 2,
        "need a tail with an interior cut, c={c} len={len}"
    );
    let tail0 = ckpt.log().as_slice()[c + 1].id;
    let plain_tail_start = plain
        .log()
        .as_slice()
        .iter()
        .position(|e| e.id == tail0)
        .expect("plain log must contain the checkpointed tail head");
    let k = (len - (c + 1)) / 2;
    assert!(k >= 1, "interior tail offset");
    let n_ckpt = c + 1 + k;
    let n_plain = plain_tail_start + k;
    let ckpt_fork = ckpt.fork_at(n_ckpt).expect("legal tail cut");
    let plain_fork = plain.fork_at(n_plain).expect("matching plain cut");
    let d_ckpt = diff_folds(ckpt_fork.fold(), ckpt.fold());
    let d_plain = diff_folds(plain_fork.fold(), plain.fold());
    assert_eq!(
        bag(&d_ckpt.added),
        bag(&d_plain.added),
        "checkpointed fork vs head added != plain twin"
    );
    assert_eq!(
        bag(&d_ckpt.removed),
        bag(&d_plain.removed),
        "checkpointed fork vs head removed != plain twin"
    );
    let ch = |xs: &[FactChange]| {
        let mut m = BTreeMap::new();
        for c in xs {
            *m.entry(format!("{}=>{}", row_fp(&c.before), row_fp(&c.after)))
                .or_insert(0) += 1;
        }
        m
    };
    assert_eq!(
        ch(&d_ckpt.changed),
        ch(&d_plain.changed),
        "checkpointed fork vs head changed != plain twin"
    );
    assert_totality(ckpt_fork.fold(), ckpt.fold(), "checkpointed fork vs head");
    assert_totality(plain_fork.fold(), plain.fold(), "plain fork vs head");
    let d0 = diff_folds(ckpt_fork.fold(), ckpt_fork.fold());
    assert_empty(&d0, "diff(fork,fork)");
}
