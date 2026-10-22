//! Independent kill tests for M154 sealed-prefix checkpoint (phase 228).
//! Public API only. Each case is capable of failing a lazy implementation.

use kutha_common::{
    rule_definition_hash, Event, EventId, Op, RuleAst, RuleAtom, RuleTerm, TransactionTime,
    ValidTime, WallTime,
};
use kutha_runtime::{store, GraphFold, Runtime, RuntimeError};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    // One event per segment so the sealed prefix includes graph rows.
    // Defines pack first; max_events=3 can seal intern rows only.
    store::set_test_segment_limits(Some(4096), Some(1));
    LimitGuard
}

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("b1-kill-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn horn_related() -> RuleAst {
    let v = |i| RuleTerm::var(i).unwrap();
    let atom = |rel: &str, s, o| RuleAtom::new(rel, s, o).unwrap();
    RuleAst::horn(
        atom("relatedTo", v(0), v(2)),
        vec![atom("relatedTo", v(0), v(1)), atom("relatedTo", v(1), v(2))],
    )
    .unwrap()
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

struct Seed {
    rt: Runtime,
    post_retract: EventId,
    post_correct: EventId,
    jid: String,
}

fn seed_runtime() -> Seed {
    let mut rt = Runtime::default();
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_rule_ast(&horn_related(), 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();

    rt.emit_tt_anchor(2015).unwrap();

    let statute = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );

    let retract_in_prefix = emit_keyed(
        &mut rt,
        "art-retract-prefix",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "retract-in-prefix",
    );
    rt.emit(Op::Retract {
        event_id: retract_in_prefix,
    })
    .unwrap();

    let correct_in_prefix = emit_keyed(
        &mut rt,
        "art-correct-prefix",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "correct-in-prefix",
    );
    let obj_80 = rt.intern("80-kmh");
    rt.emit(Op::Correct {
        event_id: correct_in_prefix,
        object: obj_80,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .unwrap();

    let interval_in_prefix = emit_keyed(
        &mut rt,
        "art-interval-prefix",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "interval-in-prefix",
    );
    let obj_50 = rt.intern("50-kmh");
    rt.emit(Op::CorrectInterval {
        event_id: interval_in_prefix,
        object: obj_50,
        patch_from: 2012,
        patch_to: Some(2018),
    })
    .unwrap();

    let post_retract = emit_keyed(
        &mut rt,
        "art-post-retract",
        "inForceAs",
        "50-kmh",
        2010,
        None,
        "post-retract-target",
    );
    let post_correct = emit_keyed(
        &mut rt,
        "art-post-correct",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "post-correct-target",
    );

    let a = rt.intern("A");
    let b = rt.intern("B");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: a,
        relation: rel,
        object: b,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: Some("horn-ab".into()),
        polarity: None,
    })
    .unwrap();

    let jid = rt
        .record_justification(
            statute,
            vec![statute],
            vec![statute],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("record_justification");
    rt.record_action(&jid, TransactionTime::MAX, ValidTime::from(2017))
        .expect("record_action");

    for i in 0..12 {
        let _ = rt.intern(&format!("pad-term-{i}"));
        emit_keyed(
            &mut rt,
            "art-12-speed",
            "inForceAs",
            &format!("pad-obj-{i}"),
            2010,
            Some(2020),
            &format!("pad-{i}"),
        );
    }

    rt.emit_tt_anchor(2021).unwrap();

    Seed {
        rt,
        post_retract,
        post_correct,
        jid,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Pit {
    fingerprint: [u8; 32],
    dead: usize,
    facts: usize,
    dict: Vec<String>,
    as_of: Vec<(u64, Vec<(u32, u32, u32)>)>,
    known_on: Vec<(u64, Option<Vec<(u32, u32, u32)>>)>,
    dict_as_of: Vec<(u32, u32, u32)>,
    fact_rows: Vec<(u64, EventId, Option<u64>, u32, u32, u32)>,
    policy: Option<String>,
    admitted: Option<bool>,
}

fn capture(rt: &Runtime, jid: &str) -> Pit {
    let vts = [2009u64, 2012, 2015, 2017, 2019, 2020, 2025];
    let walls = [2000u64, 2015, 2018, 2021, 2030];
    Pit {
        fingerprint: rt.fold().fingerprint(),
        dead: rt
            .fold()
            .facts()
            .iter()
            .filter(|f| f.invalidated_at.is_some())
            .count(),
        facts: rt.fold().facts().len(),
        dict: rt.dictionary().strings().to_vec(),
        as_of: vts
            .iter()
            .map(|&vt| (vt, rt.as_of(ValidTime::from(vt))))
            .collect(),
        known_on: walls
            .iter()
            .map(|&w| (w, rt.as_known_on(WallTime::from(w))))
            .collect(),
        dict_as_of: rt.fold().dictionary_as_of(ValidTime::from(2017)),
        fact_rows: rt
            .fold()
            .facts()
            .iter()
            .map(|f| {
                (
                    f.seq,
                    f.event_id,
                    f.invalidated_at,
                    f.subject,
                    f.relation,
                    f.object(),
                )
            })
            .collect(),
        policy: rt
            .fold()
            .live_policy_pin_at(TransactionTime::MAX, ValidTime::from(2017)),
        admitted: rt.admission_status_at(jid, TransactionTime::MAX, ValidTime::from(2017)),
    }
}

fn assert_pit_eq(got: &Pit, want: &Pit, msg: &str) {
    assert_eq!(got.fingerprint, want.fingerprint, "{msg}: fingerprint");
    assert_eq!(got.dead, want.dead, "{msg}: dead fact count");
    assert_eq!(got.facts, want.facts, "{msg}: fact count");
    assert_eq!(got.dict, want.dict, "{msg}: dictionary strings");
    assert_eq!(got.as_of, want.as_of, "{msg}: as_of");
    assert_eq!(got.known_on, want.known_on, "{msg}: as_known_on");
    assert_eq!(got.dict_as_of, want.dict_as_of, "{msg}: dictionary_as_of");
    assert_eq!(got.fact_rows, want.fact_rows, "{msg}: fact rows");
    assert_eq!(got.policy, want.policy, "{msg}: live policy pin");
    assert_eq!(got.admitted, want.admitted, "{msg}: admission status");
}

fn sealed_prefix_len(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.iter().map(|s| s.count as usize).sum()
}

fn sealed_count(dir: &Path) -> usize {
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    man.sealed.len()
}

fn export_events(dir: &Path) -> Vec<Event> {
    store::export_jsonl(dir).unwrap();
    let text = fs::read_to_string(dir.join("events.jsonl")).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn rewrite_on_disk_log(dir: &Path, events: &[Event]) {
    let n = events.len();
    let (sealed, active) = if n >= 2 {
        (vec![n - 1], 1)
    } else {
        (Vec::new(), n)
    };
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
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

fn payload_listing(dir: &Path) -> BTreeMap<String, Vec<u8>> {
    let mut out = BTreeMap::new();
    for ent in fs::read_dir(dir).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        if name == "kutha.lock" {
            continue;
        }
        let path = ent.path();
        if path.is_file() {
            out.insert(
                name.to_string_lossy().into_owned(),
                fs::read(&path).unwrap(),
            );
        }
    }
    out
}

fn dummy_checkpoint() -> Op {
    Op::Checkpoint {
        first_event_id: EventId::nil(),
        last_event_id: EventId::nil(),
        dropped_count: 0,
        fact_count: 0,
        fingerprint_hex: String::new(),
        blob_digest: String::new(),
    }
}

fn persist_seed(dir: &Path) -> Seed {
    let seed = seed_runtime();
    store::persist(&seed.rt, dir).unwrap();
    assert!(
        dir.join(store::MANIFEST_REL).exists(),
        "segmented persist must write manifest.json"
    );
    assert!(
        sealed_count(dir) >= 1,
        "fixture must seal a prefix (got 0 sealed files)"
    );
    seed
}

fn must_fail_closed(dir: &Path, what: &str) {
    match store::open(dir) {
        Err(_) => {}
        Ok(rt) => panic!(
            "{what}: store::open succeeded; replay_check is_ok={} (tamper must fail closed)",
            rt.replay_check().is_ok()
        ),
    }
}

fn post_checkpoint_muts(rt: &mut Runtime, post_retract: EventId, post_correct: EventId) {
    rt.emit(Op::Retract {
        event_id: post_retract,
    })
    .expect("post-checkpoint Retract of prefix fact");
    let obj = rt.intern("90-kmh");
    rt.emit(Op::Correct {
        event_id: post_correct,
        object: obj,
        valid_from: 2010,
        valid_to: Some(2020),
    })
    .expect("post-checkpoint Correct of prefix fact");
    let b = rt.intern("B");
    let c = rt.intern("C");
    let rel = rt.intern("relatedTo");
    rt.emit(Op::Assert {
        subject: b,
        relation: rel,
        object: c,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: Some("horn-bc".into()),
        polarity: None,
    })
    .expect("post-checkpoint Horn premise");
    rt.emit_tt_anchor(2024).expect("post-checkpoint TtAnchor");
}

fn apply_suffix(dir: &Path, suffix: &[Event], dict: Vec<String>) {
    let opened = store::open(dir).unwrap();
    let mut events = opened.log().as_slice().to_vec();
    events.extend(suffix.iter().cloned());
    let rt = Runtime::from_dict_and_events(dict, events, opened.max_cascade)
        .expect("replay identical suffix onto checkpointed store");
    store::persist(&rt, dir).unwrap();
}

fn find_checkpoint_mut(events: &mut [Event]) -> &mut Op {
    events
        .iter_mut()
        .find(|e| matches!(e.op, Op::Checkpoint { .. }))
        .map(|e| &mut e.op)
        .expect("checkpointed log must contain Op::Checkpoint")
}

#[test]
fn r2_replay_fingerprint_and_pit_survive_checkpoint() {
    let _g = tiny_limits();
    let dir = persist_dir("r2");
    let seed = persist_seed(&dir);
    assert!(
        seed.rt
            .fold()
            .facts()
            .iter()
            .any(|f| f.invalidated_at.is_some()),
        "fixture must include a dead/retracted fact hashed into the fingerprint"
    );
    let before = capture(&seed.rt, &seed.jid);
    assert!(
        before
            .known_on
            .iter()
            .any(|(w, ans)| *w < 2015 && ans.is_none()),
        "as_known_on before the first TT anchor must be None"
    );
    assert!(
        before
            .known_on
            .iter()
            .any(|(w, ans)| *w == 2018 && ans.is_some()),
        "as_known_on between TT anchors must resolve"
    );

    let encoded = store::encoded_events(&seed.rt);
    let prefix_len = sealed_prefix_len(&dir);
    assert!(prefix_len > 0 && prefix_len < encoded.len());
    let prefix_assert = encoded[..prefix_len]
        .iter()
        .find(|e| match &e.op {
            Op::Assert { relation, .. } => seed
                .rt
                .dictionary()
                .lookup(*relation)
                .is_some_and(|n| !n.starts_with("kutha:")),
            _ => false,
        })
        .expect("sealed prefix must contain a product Assert");
    let prefix_assert_id = prefix_assert.id;
    let kept_blob_ids: Vec<EventId> = encoded[..prefix_len]
        .iter()
        .filter(|e| matches!(e.op, Op::StoreBlob { .. }))
        .map(|e| e.id)
        .collect();
    assert!(
        !kept_blob_ids.is_empty(),
        "prefix must keep rule/policy StoreBlobs"
    );

    store::checkpoint_sealed_prefix(&dir).unwrap();

    let opened = store::open(&dir).expect("open after checkpoint");
    opened
        .replay_check()
        .expect("replay_check after checkpoint");
    let after = capture(&opened, &seed.jid);
    assert_pit_eq(&after, &before, "R2 after checkpoint");
    assert_eq!(
        GraphFold::replay_with_dict(opened.log().as_slice(), opened.dictionary()).fingerprint(),
        before.fingerprint,
        "R2: replay_with_dict of the rewritten log must match the pre-checkpoint fingerprint"
    );
    assert!(
        opened
            .log()
            .iter()
            .any(|e| matches!(e.op, Op::Checkpoint { dropped_count, .. } if dropped_count > 0)),
        "R1/R2 kill: no-op checkpoint leaves no receipt"
    );
    assert!(
        !opened.log().iter().any(|e| e.id == prefix_assert_id),
        "R1/R2 kill: sealed-prefix Assert {prefix_assert_id} must leave the log"
    );
    for id in &kept_blob_ids {
        assert!(
            opened.log().iter().any(|e| e.id == *id),
            "R1: prefix StoreBlob {id} must be copied verbatim"
        );
    }
    assert!(
        opened
            .fold()
            .facts()
            .iter()
            .any(|f| f.event_id == prefix_assert_id),
        "R2: dropped Assert must still exist as a fold fact (event_id preserved)"
    );
    assert!(
        opened.dictionary().id("pad-term-0").is_some(),
        "R3: prefix intern names must survive via the checkpoint blob"
    );

    let _ = fs::remove_dir_all(&dir);
}

/// Minimised R2: dictionary Asserts (`kutha:admission` / `kutha:action`) sit in
/// the active tail with no trailing product quantum. After rewrite, prefix
/// `Define`s are gone so `committed_prefix` may treat those rows as
/// NeedsOutcome and hide them, leaving `snapshot.log_offset` ahead.
#[test]
fn r2_open_after_checkpoint_with_admission_in_active_tail() {
    let _g = tiny_limits();
    let dir = persist_dir("r2-admit-tail");
    let mut rt = Runtime::default();
    rt.emit_rule("derive_pq", 2010, None).unwrap();
    rt.emit_policy("leased-policy", 2010, None).unwrap();
    let statute = emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "statute-50",
    );
    for i in 0..6 {
        let _ = rt.intern(&format!("pad-term-{i}"));
        emit_keyed(
            &mut rt,
            "art-12-speed",
            "inForceAs",
            &format!("pad-obj-{i}"),
            2010,
            Some(2020),
            &format!("pad-{i}"),
        );
    }
    rt.emit_tt_anchor(2015).unwrap();
    let jid = rt
        .record_justification(
            statute,
            vec![statute],
            vec![statute],
            rule_definition_hash("derive_pq"),
            TransactionTime::MAX,
            ValidTime::from(2017),
        )
        .expect("record_justification");
    rt.record_action(&jid, TransactionTime::MAX, ValidTime::from(2017))
        .expect("record_action");
    store::persist(&rt, &dir).unwrap();
    assert!(
        sealed_count(&dir) >= 1,
        "need a sealed prefix to checkpoint"
    );
    store::checkpoint_sealed_prefix(&dir).unwrap();
    store::open(&dir).unwrap_or_else(|e| {
        panic!("HIGH R2: open after checkpoint with kutha:admission in the active tail: {e}")
    });
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r3_twin_post_checkpoint_retract_correct_horn_and_tt() {
    let _g = tiny_limits();
    let ckp = persist_dir("r3-ckp");
    let twin = persist_dir("r3-twin");
    let mut seed = persist_seed(&ckp);
    let orig_n = seed.rt.log().len();
    copy_store(&ckp, &twin);

    store::checkpoint_sealed_prefix(&ckp).unwrap();
    post_checkpoint_muts(&mut seed.rt, seed.post_retract, seed.post_correct);
    let suffix = seed.rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&seed.rt, &twin).unwrap();
    apply_suffix(&ckp, &suffix, seed.rt.dictionary().strings().to_vec());

    let a = store::open(&ckp).expect("checkpointed twin");
    let b = store::open(&twin).expect("uncheckpointed twin");
    a.replay_check().unwrap();
    b.replay_check().unwrap();
    let pit_a = capture(&a, &seed.jid);
    let pit_b = capture(&b, &seed.jid);
    assert_pit_eq(&pit_a, &pit_b, "R3 twin after post-checkpoint muts");

    let rel = a.dictionary().id("relatedTo").expect("relatedTo");
    let subj = a.dictionary().id("A").expect("A");
    let obj = a.dictionary().id("C").expect("C");
    assert!(
        a.fold().facts().iter().any(|f| {
            f.subject == subj
                && f.relation == rel
                && f.object() == obj
                && f.invalidated_at.is_none()
        }),
        "R3: prefix Horn rule must fire relatedTo(A,C) after post-checkpoint relatedTo(B,C)"
    );
    assert!(
        a.as_known_on(WallTime::from(2024)).is_some(),
        "R3: post-checkpoint TT anchor must resolve as_known_on(2024)"
    );
    assert!(
        a.fold()
            .facts()
            .iter()
            .any(|f| f.event_id == seed.post_retract && f.invalidated_at.is_some()),
        "R3: Retract of a prefix Assert must invalidate that fact"
    );

    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

#[test]
fn r3_persister_append_after_checkpoint() {
    let _g = tiny_limits();
    let dir = persist_dir("r3-persister");
    let seed = persist_seed(&dir);
    store::checkpoint_sealed_prefix(&dir).unwrap();
    let (mut rt, mut persister) = store::open_for_append(&dir).unwrap_or_else(|e| {
        panic!("HIGH R3: open_for_append after checkpoint: {e}");
    });
    post_checkpoint_muts(&mut rt, seed.post_retract, seed.post_correct);
    persister.append(&rt).unwrap_or_else(|e| {
        panic!("HIGH R3: Persister::append after checkpoint: {e}");
    });
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_emit_checkpoint_is_meta_op_rejected_and_self_commit() {
    let mut rt = Runtime::default();
    let n = rt.log().len();
    let err = rt.emit(dummy_checkpoint()).unwrap_err();
    assert!(
        matches!(err, RuntimeError::MetaOpRejected),
        "R4: emit(Checkpoint) must be MetaOpRejected, got {err:?}"
    );
    assert_eq!(n, rt.log().len(), "R4: rejected emit must not append");
    assert_eq!(
        store::op_commit_class(&dummy_checkpoint()),
        store::CommitClass::SelfCommit
    );
}

#[test]
fn r4_tamper_fingerprint_hex_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-fp-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src)
        .expect("untampered checkpointed store must open before fingerprint_hex tamper");
    let dir = persist_dir("r4-fp");
    copy_store(&src, &dir);

    let mut events = export_events(&dir);
    match find_checkpoint_mut(&mut events) {
        Op::Checkpoint {
            fingerprint_hex, ..
        } => {
            fingerprint_hex.push_str("00");
        }
        _ => unreachable!(),
    }
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 wrong fingerprint_hex");

    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_tamper_blob_digest_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-digest-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open before blob_digest tamper");
    let dir = persist_dir("r4-digest");
    copy_store(&src, &dir);

    let mut events = export_events(&dir);
    match find_checkpoint_mut(&mut events) {
        Op::Checkpoint { blob_digest, .. } => {
            blob_digest.push_str("ff");
        }
        _ => unreachable!(),
    }
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 wrong blob_digest");

    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_tamper_truncated_blob_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-trunc-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src)
        .expect("untampered checkpointed store must open before truncated-blob tamper");
    let dir = persist_dir("r4-trunc");
    copy_store(&src, &dir);

    let mut events = export_events(&dir);
    let digest = match events.iter().find_map(|e| match &e.op {
        Op::Checkpoint { blob_digest, .. } => Some(blob_digest.clone()),
        _ => None,
    }) {
        Some(d) => d,
        None => panic!("missing Checkpoint"),
    };
    let mut truncated = false;
    for e in &mut events {
        if let Op::StoreBlob { digest: d, bytes } = &mut e.op {
            if *d == digest && bytes.len() > 8 {
                bytes.truncate(bytes.len() / 2);
                truncated = true;
            }
        }
    }
    assert!(truncated, "must truncate the checkpoint StoreBlob");
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 truncated checkpoint blob");

    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_dropped_receipt_with_prefix_removed_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-drop-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src)
        .expect("untampered checkpointed store must open before dropped-receipt tamper");
    let dir = persist_dir("r4-drop");
    copy_store(&src, &dir);

    let events = export_events(&dir);
    let idx = events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Checkpoint receipt");
    let tail = events[idx + 1..].to_vec();
    assert!(
        !tail.iter().any(|e| matches!(e.op, Op::Checkpoint { .. })),
        "tail must not keep a receipt"
    );
    rewrite_on_disk_log(&dir, &tail);
    must_fail_closed(&dir, "R4 dropped receipt with prefix removed");

    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_legacy_and_empty_sealed_fail_closed_leave_dir() {
    let legacy = persist_dir("r4-legacy");
    store::ensure_legacy_dir(&legacy).unwrap();
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "legacy-one",
    );
    store::persist(&rt, &legacy).unwrap();
    assert!(
        !legacy.join(store::MANIFEST_REL).exists(),
        "legacy persist must not write manifest.json"
    );
    let before = payload_listing(&legacy);
    let err = store::checkpoint_sealed_prefix(&legacy).expect_err("legacy store");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData, "{err}");
    assert_eq!(
        payload_listing(&legacy),
        before,
        "R4: legacy checkpoint failure must leave payload files unchanged"
    );

    let empty = persist_dir("r4-empty-sealed");
    let mut rt = Runtime::default();
    emit_keyed(
        &mut rt,
        "art-12-speed",
        "inForceAs",
        "50-kmh",
        2010,
        Some(2020),
        "active-only",
    );
    store::persist(&rt, &empty).unwrap();
    assert_eq!(
        sealed_count(&empty),
        0,
        "small store under default limits must have an empty sealed prefix"
    );
    let before = payload_listing(&empty);
    let err = store::checkpoint_sealed_prefix(&empty).expect_err("empty sealed");
    assert_eq!(err.kind(), std::io::ErrorKind::InvalidData, "{err}");
    assert_eq!(
        payload_listing(&empty),
        before,
        "R4: empty-sealed failure must leave payload files unchanged"
    );

    let _ = fs::remove_dir_all(&legacy);
    let _ = fs::remove_dir_all(&empty);
}

#[test]
fn r5_checkpoint_twice_and_checkpoint_append_checkpoint() {
    let _g = tiny_limits();
    let twice = persist_dir("r5-twice");
    let seed = persist_seed(&twice);
    let before = capture(&seed.rt, &seed.jid);
    store::checkpoint_sealed_prefix(&twice).unwrap();
    store::checkpoint_sealed_prefix(&twice).unwrap();
    let opened = store::open(&twice).unwrap();
    opened.replay_check().unwrap();
    assert_pit_eq(
        &capture(&opened, &seed.jid),
        &before,
        "R5: checkpoint twice",
    );

    let ckp = persist_dir("r5-ckp");
    let twin = persist_dir("r5-twin");
    let mut seed = persist_seed(&ckp);
    let orig_n = seed.rt.log().len();
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();
    post_checkpoint_muts(&mut seed.rt, seed.post_retract, seed.post_correct);
    let suffix = seed.rt.log().as_slice()[orig_n..].to_vec();
    store::persist(&seed.rt, &twin).unwrap();
    apply_suffix(&ckp, &suffix, seed.rt.dictionary().strings().to_vec());
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let a = store::open(&ckp).unwrap();
    let b = store::open(&twin).unwrap();
    a.replay_check().unwrap();
    b.replay_check().unwrap();
    assert_pit_eq(
        &capture(&a, &seed.jid),
        &capture(&b, &seed.jid),
        "R5: checkpoint then append then checkpoint vs uncheckpointed twin",
    );

    let _ = fs::remove_dir_all(&twice);
    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

#[test]
fn r6_checkpoint_serde_externally_tagged_no_alias() {
    let op = Op::Checkpoint {
        first_event_id: EventId::nil(),
        last_event_id: EventId::nil(),
        dropped_count: 3,
        fact_count: 7,
        fingerprint_hex: "abc".into(),
        blob_digest: "def".into(),
    };
    let value = serde_json::to_value(&op).unwrap();
    assert!(
        value.get("Checkpoint").is_some(),
        "R6: modern externally tagged Checkpoint, got {value}"
    );
    assert!(
        value.get("checkpoint").is_none(),
        "R6: lowercase alias must not serialize, got {value}"
    );
    assert_eq!(serde_json::from_value::<Op>(value.clone()).unwrap(), op);
    let alias = serde_json::json!({"checkpoint": value.get("Checkpoint").cloned().unwrap()});
    assert!(
        serde_json::from_value::<Op>(alias).is_err(),
        "R6: serde must not accept a checkpoint alias"
    );
}
