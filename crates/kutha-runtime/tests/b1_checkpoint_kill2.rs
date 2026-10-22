//! Second-pass independent kills for M154 sealed-prefix checkpoint (phase 228).
//! Public API only. Covers R3 Persister composition, prefix sidecar answers,
//! R4 tampers the first pass did not, Horn tail Behavior, and R7 crash windows.
//! `checkpoint_sealed_prefix` has no failpoint; persist_log uses unique segment
//! names. R7 is exercised by constructing each on-disk mix with public helpers.

use kutha_common::{
    Event, EventId, KUTHA_ACTION, KUTHA_ADMISSION, Op, RuleAst, RuleAtom, RuleTerm,
    TransactionTime, ValidTime, WallTime, rule_definition_hash,
};
use kutha_runtime::{GraphFold, Justification, PersistedQuantumOutcome, Runtime, store};
use std::fs;
use std::path::{Path, PathBuf};

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

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("b1-kill2-{label}-{nanos}"));
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
    outcomes: Vec<PersistedQuantumOutcome>,
    justifications: Vec<Justification>,
    admissions: Vec<(u32, u32, u32, Option<u64>)>,
    actions: Vec<(u32, u32, u32, Option<u64>)>,
}

fn dict_rel_rows(rt: &Runtime, rel_name: &str) -> Vec<(u32, u32, u32, Option<u64>)> {
    let Some(rel) = rt.dictionary().id(rel_name) else {
        return Vec::new();
    };
    let mut rows: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| f.relation == rel)
        .map(|f| (f.subject, f.relation, f.object(), f.invalidated_at))
        .collect();
    rows.sort();
    rows
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
        outcomes: rt.outcome_records().to_vec(),
        justifications: rt.justification_records(),
        admissions: dict_rel_rows(rt, KUTHA_ADMISSION),
        actions: dict_rel_rows(rt, KUTHA_ACTION),
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

/// Query answers after independently minted tail events (new EventIds, so fingerprint
/// cannot match). Still kills a fold that drops Retract/Horn/TT/admission.
fn assert_query_eq(got: &Pit, want: &Pit, msg: &str) {
    assert_eq!(got.dead, want.dead, "{msg}: dead fact count");
    assert_eq!(got.facts, want.facts, "{msg}: fact count");
    assert_eq!(got.as_of, want.as_of, "{msg}: as_of");
    assert_eq!(got.known_on, want.known_on, "{msg}: as_known_on");
    assert_eq!(got.dict_as_of, want.dict_as_of, "{msg}: dictionary_as_of");
    assert_eq!(got.policy, want.policy, "{msg}: live policy pin");
    assert_eq!(got.admitted, want.admitted, "{msg}: admission status");
    assert_eq!(
        got.admissions, want.admissions,
        "{msg}: kutha:admission fold rows"
    );
    assert_eq!(got.actions, want.actions, "{msg}: kutha:action fold rows");
}

fn assert_sidecars_eq(got: &Pit, want: &Pit, msg: &str) {
    let mut fails = Vec::new();
    if got.justifications != want.justifications {
        fails.push(format!(
            "justifications got={} want={}",
            got.justifications.len(),
            want.justifications.len()
        ));
    }
    if got.outcomes != want.outcomes {
        fails.push(format!(
            "outcomes got={} want={}",
            got.outcomes.len(),
            want.outcomes.len()
        ));
    }
    if got.admissions != want.admissions {
        fails.push(format!(
            "admissions got={:?} want={:?}",
            got.admissions, want.admissions
        ));
    }
    if got.actions != want.actions {
        fails.push(format!(
            "actions got={:?} want={:?}",
            got.actions, want.actions
        ));
    }
    assert!(fails.is_empty(), "{msg}: {}", fails.join("; "));
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

#[derive(Debug, PartialEq, Eq)]
enum OpenReplay {
    OpenErr(std::io::ErrorKind),
    OpenOk { replay_ok: bool },
}

fn open_replay_outcome(dir: &Path) -> OpenReplay {
    match store::open(dir) {
        Err(e) => OpenReplay::OpenErr(e.kind()),
        Ok(rt) => OpenReplay::OpenOk {
            replay_ok: rt.replay_check().is_ok(),
        },
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

fn further_append(rt: &mut Runtime) {
    emit_keyed(
        rt,
        "art-post-reopen",
        "inForceAs",
        "110-kmh",
        2010,
        Some(2020),
        "post-reopen-1",
    );
}

fn find_checkpoint_mut(events: &mut [Event]) -> &mut Op {
    events
        .iter_mut()
        .find(|e| matches!(e.op, Op::Checkpoint { .. }))
        .map(|e| &mut e.op)
        .expect("checkpointed log must contain Op::Checkpoint")
}

fn checkpoint_idx(events: &[Event]) -> usize {
    events
        .iter()
        .position(|e| matches!(e.op, Op::Checkpoint { .. }))
        .expect("Checkpoint receipt")
}

fn related_to_ac_live(rt: &Runtime) -> bool {
    let rel = rt.dictionary().id("relatedTo").expect("relatedTo");
    let subj = rt.dictionary().id("A").expect("A");
    let obj = rt.dictionary().id("C").expect("C");
    rt.fold().facts().iter().any(|f| {
        f.subject == subj && f.relation == rel && f.object() == obj && f.invalidated_at.is_none()
    })
}

fn seg_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for ent in fs::read_dir(dir).unwrap() {
        let ent = ent.unwrap();
        let name = ent.file_name();
        let name = name.to_string_lossy();
        if name.starts_with("seg-") && name.ends_with(".log") {
            out.push(ent.path());
        }
    }
    out.sort();
    out
}

fn copy_named(src: &Path, dst: &Path, names: &[&str]) {
    for name in names {
        let from = src.join(name);
        if from.exists() {
            fs::copy(&from, dst.join(name)).unwrap();
        }
    }
}

fn overlay_segs(src: &Path, dst: &Path) {
    for p in seg_files(src) {
        let name = p.file_name().unwrap();
        fs::copy(&p, dst.join(name)).unwrap();
    }
}

fn remove_segs(dir: &Path) {
    for p in seg_files(dir) {
        fs::remove_file(p).unwrap();
    }
}

fn sidecar_open_verdict(dir: &Path, old: &Pit, new: &Pit, jid: &str, window: &str) {
    match store::open(dir) {
        Err(_) => {}
        Ok(rt) => {
            let check = rt.replay_check();
            if let Err(e) = &check {
                panic!(
                    "HIGH R7 {window}: open Ok but replay_check Err ({e}) — not a clean OLD or NEW picture"
                );
            }
            let got = capture(&rt, jid);
            let matches_old = got.fingerprint == old.fingerprint
                && got.as_of == old.as_of
                && got.known_on == old.known_on;
            let matches_new = got.fingerprint == new.fingerprint
                && got.as_of == new.as_of
                && got.known_on == new.known_on;
            if !matches_old && !matches_new {
                panic!(
                    "HIGH R7 {window}: open succeeded with mixed/neither answers; facts={} old={} new={}",
                    got.facts, old.facts, new.facts
                );
            }
        }
    }
}

/// R3: Persister append after checkpoint, then full persist, reopen, further Persister append,
/// compared to an un-checkpointed twin with the same event sequence.
#[test]
fn r3_persister_append_composed_with_persist_reopen_and_further_append() {
    let _g = tiny_limits();
    let ckp = persist_dir("r3-compose-ckp");
    let twin = persist_dir("r3-compose-twin");
    let mut seed = persist_seed(&ckp);
    copy_store(&ckp, &twin);
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let (mut rt, mut persister) = store::open_for_append(&ckp).unwrap_or_else(|e| {
        panic!("HIGH R3: open_for_append after checkpoint: {e}");
    });
    post_checkpoint_muts(&mut rt, seed.post_retract, seed.post_correct);
    persister.append(&rt).unwrap_or_else(|e| {
        panic!("HIGH R3: Persister::append after checkpoint: {e}");
    });
    drop(persister);
    store::persist(&rt, &ckp).unwrap_or_else(|e| {
        panic!("HIGH R3: persist after Persister append: {e}");
    });

    let (mut rt, mut persister) = store::open_for_append(&ckp).unwrap_or_else(|e| {
        panic!("HIGH R3: open_for_append after persist+reopen: {e}");
    });
    further_append(&mut rt);
    persister.append(&rt).unwrap_or_else(|e| {
        panic!("HIGH R3: further Persister::append after reopen: {e}");
    });
    drop(persister);

    post_checkpoint_muts(&mut seed.rt, seed.post_retract, seed.post_correct);
    further_append(&mut seed.rt);
    store::persist(&seed.rt, &twin).unwrap();

    let a = store::open(&ckp).expect("checkpointed composed store");
    let b = store::open(&twin).expect("uncheckpointed twin");
    a.replay_check()
        .expect("HIGH R3: replay_check on checkpointed composed store");
    b.replay_check().unwrap();
    let pit_a = capture(&a, &seed.jid);
    let pit_b = capture(&b, &seed.jid);
    assert_query_eq(
        &pit_a,
        &pit_b,
        "HIGH R3: Persister+persist+reopen+append vs twin",
    );
    assert!(
        related_to_ac_live(&a),
        "HIGH R3: prefix Horn rule must fire relatedTo(A,C) after Persister relatedTo(B,C)"
    );
    assert!(
        a.as_known_on(WallTime::from(2024)).is_some(),
        "HIGH R3: post-checkpoint TT anchor must resolve as_known_on(2024)"
    );

    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// R3: prefix QuantumOutcome / JustificationCite / admission / action must still
/// answer like the un-checkpointed twin (replay_check + sidecar/fold).
#[test]
fn r3_prefix_quantum_justification_admission_action_vs_twin() {
    let _g = tiny_limits();
    let ckp = persist_dir("r3-side-ckp");
    let twin = persist_dir("r3-side-twin");
    let seed = persist_seed(&ckp);
    copy_store(&ckp, &twin);
    let encoded = store::encoded_events(&seed.rt);
    let prefix_len = sealed_prefix_len(&ckp);
    assert!(prefix_len > 0 && prefix_len < encoded.len());
    let prefix = &encoded[..prefix_len];
    assert!(
        prefix
            .iter()
            .any(|e| matches!(e.op, Op::QuantumOutcome { .. })),
        "fixture must seal a prefix QuantumOutcome"
    );
    assert!(
        prefix
            .iter()
            .any(|e| matches!(e.op, Op::JustificationCite { .. })),
        "fixture must seal a prefix JustificationCite"
    );
    let has_admission = prefix.iter().any(|e| match &e.op {
        Op::Assert { relation, .. } => seed
            .rt
            .dictionary()
            .lookup(*relation)
            .is_some_and(|n| n == KUTHA_ADMISSION),
        _ => false,
    });
    let has_action = prefix.iter().any(|e| match &e.op {
        Op::Assert { relation, .. } => seed
            .rt
            .dictionary()
            .lookup(*relation)
            .is_some_and(|n| n == KUTHA_ACTION),
        _ => false,
    });
    assert!(has_admission, "fixture must seal a prefix kutha:admission");
    assert!(has_action, "fixture must seal a prefix kutha:action");

    store::checkpoint_sealed_prefix(&ckp).unwrap();
    let a = store::open(&ckp).expect("checkpointed store");
    let b = store::open(&twin).expect("twin");
    a.replay_check()
        .expect("HIGH R3: replay_check after checkpoint with prefix cite/outcome");
    b.replay_check().unwrap();
    let pit_a = capture(&a, &seed.jid);
    let pit_b = capture(&b, &seed.jid);
    assert_pit_eq(&pit_a, &pit_b, "HIGH R3: prefix sidecar PIT vs twin");
    assert_sidecars_eq(
        &pit_a,
        &pit_b,
        "HIGH R3: prefix QuantumOutcome/JustificationCite/admission/action vs twin",
    );

    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

/// R3: Horn rule body lives in a kept StoreBlob; a tail Behavior must replay_check.
#[test]
fn r3_horn_kept_rule_blob_tail_behavior_replay_check() {
    let _g = tiny_limits();
    let ckp = persist_dir("r3-horn-ckp");
    let twin = persist_dir("r3-horn-twin");
    let mut seed = persist_seed(&ckp);
    copy_store(&ckp, &twin);
    let encoded = store::encoded_events(&seed.rt);
    let prefix_len = sealed_prefix_len(&ckp);
    let prefix_rule_blobs = encoded[..prefix_len]
        .iter()
        .filter(|e| matches!(e.op, Op::StoreBlob { .. }))
        .count();
    assert!(
        prefix_rule_blobs >= 2,
        "prefix must keep rule/policy StoreBlobs (got {prefix_rule_blobs})"
    );
    store::checkpoint_sealed_prefix(&ckp).unwrap();

    let (mut rt, mut persister) = store::open_for_append(&ckp).unwrap();
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
    .unwrap();
    persister.append(&rt).unwrap();
    drop(persister);

    let b = seed.rt.intern("B");
    let c = seed.rt.intern("C");
    let rel = seed.rt.intern("relatedTo");
    seed.rt
        .emit(Op::Assert {
            subject: b,
            relation: rel,
            object: c,
            valid_from: 2010,
            valid_to: None,
            claim: None,
            delivery_key: Some("horn-bc".into()),
            polarity: None,
        })
        .unwrap();
    store::persist(&seed.rt, &twin).unwrap();

    let a = store::open(&ckp).expect("checkpointed horn store");
    let t = store::open(&twin).expect("twin horn store");
    a.replay_check()
        .expect("HIGH R3: replay_check must accept tail Behavior after checkpoint");
    t.replay_check().unwrap();
    assert!(
        a.log().iter().any(|e| matches!(e.op, Op::Behavior { .. })),
        "HIGH R3: tail must contain Op::Behavior from prefix Horn + kept rule blob"
    );
    assert!(
        related_to_ac_live(&a),
        "HIGH R3: relatedTo(A,C) must be live after tail Horn fire"
    );
    let pit_a = capture(&a, &seed.jid);
    let pit_t = capture(&t, &seed.jid);
    assert_eq!(
        GraphFold::replay_with_dict(a.log().as_slice(), a.dictionary()).fingerprint(),
        a.fold().fingerprint(),
        "HIGH R3: replay_with_dict of checkpointed log must match opened fold"
    );
    assert_query_eq(&pit_a, &pit_t, "HIGH R3: Horn tail Behavior vs twin");

    let _ = fs::remove_dir_all(&ckp);
    let _ = fs::remove_dir_all(&twin);
}

#[test]
fn r4_tamper_fact_count_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-fc-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open");
    let dir = persist_dir("r4-fc");
    copy_store(&src, &dir);
    let mut events = export_events(&dir);
    match find_checkpoint_mut(&mut events) {
        Op::Checkpoint { fact_count, .. } => {
            *fact_count = fact_count.saturating_add(1);
        }
        _ => unreachable!(),
    }
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 wrong fact_count");
    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_receipt_moved_after_tail_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-moved-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open");
    let dir = persist_dir("r4-moved");
    copy_store(&src, &dir);
    let mut events = export_events(&dir);
    let idx = checkpoint_idx(&events);
    let receipt = events.remove(idx);
    assert!(
        !events.iter().any(|e| matches!(e.op, Op::Checkpoint { .. })),
        "move must leave no Checkpoint in the prefix"
    );
    events.push(receipt);
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 receipt moved after tail events");
    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn r4_duplicate_receipt_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-dup-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open");
    let dir = persist_dir("r4-dup");
    copy_store(&src, &dir);
    let mut events = export_events(&dir);
    let idx = checkpoint_idx(&events);
    let dup = events[idx].clone();
    events.insert(idx + 1, dup);
    rewrite_on_disk_log(&dir, &events);
    must_fail_closed(&dir, "R4 duplicate Checkpoint receipt");
    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

/// Truncate a *kept* prefix StoreBlob (rule/policy), not the checkpoint blob.
/// AMENDED R4: plain stores do not verify StoreBlob body-vs-digest on open.
/// Requirement is PARITY with an un-checkpointed twin under the same tamper.
#[test]
fn r4_truncated_kept_storeblob_parity_with_plain_store() {
    let _g = tiny_limits();
    let src = persist_dir("r4-kept-src");
    persist_seed(&src);
    let twin = persist_dir("r4-kept-twin");
    copy_store(&src, &twin);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open");
    let dir = persist_dir("r4-kept");
    copy_store(&src, &dir);
    let mut events = export_events(&dir);
    let ck_digest = events.iter().find_map(|e| match &e.op {
        Op::Checkpoint { blob_digest, .. } => Some(blob_digest.clone()),
        _ => None,
    });
    let ck_digest = ck_digest.expect("Checkpoint");
    let mut target_digest = None;
    for e in &mut events {
        if let Op::StoreBlob { digest, bytes } = &mut e.op {
            if *digest != ck_digest && bytes.len() > 8 {
                bytes.truncate(bytes.len() / 2);
                target_digest = Some(digest.clone());
                break;
            }
        }
    }
    let target_digest = target_digest.expect("must truncate a kept (non-checkpoint) StoreBlob");
    rewrite_on_disk_log(&dir, &events);

    let mut twin_events = export_events(&twin);
    let mut truncated_twin = false;
    for e in &mut twin_events {
        if let Op::StoreBlob { digest, bytes } = &mut e.op {
            if *digest == target_digest {
                bytes.truncate(bytes.len() / 2);
                truncated_twin = true;
                break;
            }
        }
    }
    assert!(
        truncated_twin,
        "twin must contain the same kept StoreBlob digest"
    );
    rewrite_on_disk_log(&twin, &twin_events);

    assert_eq!(
        open_replay_outcome(&dir),
        open_replay_outcome(&twin),
        "AMENDED R4: checkpointed store must match un-checkpointed twin under the same kept-StoreBlob body tamper"
    );
    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&twin);
}

/// Physically truncate a sealed segment that holds the kept StoreBlob prefix.
#[test]
fn r4_truncated_segment_holding_kept_blobs_fails_closed() {
    let _g = tiny_limits();
    let src = persist_dir("r4-seg-src");
    persist_seed(&src);
    store::checkpoint_sealed_prefix(&src).unwrap();
    store::open(&src).expect("untampered checkpointed store must open");
    let dir = persist_dir("r4-seg");
    copy_store(&src, &dir);
    let man: store::Manifest =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    let first = man
        .sealed
        .first()
        .expect("checkpointed store must have a sealed segment");
    let path = dir.join(&first.name);
    let bytes = fs::read(&path).unwrap();
    assert!(
        bytes.len() > 16,
        "sealed segment must be large enough to truncate"
    );
    fs::write(&path, &bytes[..bytes.len() / 2]).unwrap();
    must_fail_closed(&dir, "R4 truncated segment file holding kept StoreBlobs");
    let _ = fs::remove_dir_all(&src);
    let _ = fs::remove_dir_all(&dir);
}

/// R7: interrupt windows. No product failpoint on `checkpoint_sealed_prefix`.
/// persist_log writes unique names then swaps the manifest, then drops obsolete
/// segs, then checkpoint refreshes terms/snapshot/sidecars. Each mix must open
/// as the OLD picture, the NEW picture, or fail closed — never a blend.
#[test]
fn r7_crash_windows_open_old_or_new_never_mix() {
    let _g = tiny_limits();
    let old_dir = persist_dir("r7-old");
    let seed = persist_seed(&old_dir);
    let jid = seed.jid.clone();
    let old_rt = store::open(&old_dir).unwrap();
    let old_pit = capture(&old_rt, &jid);
    drop(old_rt);

    let new_dir = persist_dir("r7-new");
    copy_store(&old_dir, &new_dir);
    store::checkpoint_sealed_prefix(&new_dir).unwrap();
    let new_rt = store::open(&new_dir).unwrap();
    new_rt.replay_check().unwrap();
    let new_pit = capture(&new_rt, &jid);
    assert_pit_eq(
        &new_pit,
        &old_pit,
        "R7 setup: successful checkpoint must keep OLD query answers",
    );
    drop(new_rt);

    // W1: new unique segs present, old manifest still names old segs.
    let w1 = persist_dir("r7-w1");
    copy_store(&old_dir, &w1);
    overlay_segs(&new_dir, &w1);
    sidecar_open_verdict(&w1, &old_pit, &new_pit, &jid, "W1 new segs + old manifest");

    // W2: new manifest+segs, leftover old segs (remove_obsolete not run).
    let w2 = persist_dir("r7-w2");
    copy_store(&new_dir, &w2);
    overlay_segs(&old_dir, &w2);
    sidecar_open_verdict(
        &w2,
        &old_pit,
        &new_pit,
        &jid,
        "W2 new manifest leftover old segs",
    );

    // W3: persist_log complete (new segs+manifest, old segs gone), snapshot/sidecars still OLD.
    let w3 = persist_dir("r7-w3");
    copy_store(&old_dir, &w3);
    remove_segs(&w3);
    overlay_segs(&new_dir, &w3);
    fs::copy(
        new_dir.join(store::MANIFEST_REL),
        w3.join(store::MANIFEST_REL),
    )
    .unwrap();
    sidecar_open_verdict(
        &w3,
        &old_pit,
        &new_pit,
        &jid,
        "W3 new log+manifest, old snapshot/sidecars",
    );

    // W4: persist_log + terms, snapshot still OLD.
    let w4 = persist_dir("r7-w4");
    copy_store(&w3, &w4);
    copy_named(&new_dir, &w4, &["terms.jsonl"]);
    sidecar_open_verdict(
        &w4,
        &old_pit,
        &new_pit,
        &jid,
        "W4 new log+terms, old snapshot",
    );

    // W5: snapshot refreshed, outcomes/justifications still OLD.
    let w5 = persist_dir("r7-w5");
    copy_store(&new_dir, &w5);
    copy_named(
        &old_dir,
        &w5,
        &[store::OUTCOMES_REL, store::JUSTIFICATIONS_REL],
    );
    sidecar_open_verdict(
        &w5,
        &old_pit,
        &new_pit,
        &jid,
        "W5 new snapshot, old outcome/justification sidecars",
    );

    // W6: outcomes refreshed, justifications still OLD.
    let w6 = persist_dir("r7-w6");
    copy_store(&new_dir, &w6);
    copy_named(&old_dir, &w6, &[store::JUSTIFICATIONS_REL]);
    sidecar_open_verdict(
        &w6,
        &old_pit,
        &new_pit,
        &jid,
        "W6 new outcomes, old justifications sidecar",
    );

    let _ = fs::remove_dir_all(&old_dir);
    let _ = fs::remove_dir_all(&new_dir);
    let _ = fs::remove_dir_all(&w1);
    let _ = fs::remove_dir_all(&w2);
    let _ = fs::remove_dir_all(&w3);
    let _ = fs::remove_dir_all(&w4);
    let _ = fs::remove_dir_all(&w5);
    let _ = fs::remove_dir_all(&w6);
}
