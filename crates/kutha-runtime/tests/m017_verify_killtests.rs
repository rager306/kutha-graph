//! Independent Phase 42 kill-tests (verifier). Public API only; not the executor oracles.

use kutha_common::{Event, EventId, Op, OutcomeDisposition, TransactionTime, ValidTime};
use kutha_runtime::{FoldIndexMode, Justification, PersistedQuantumOutcome, Runtime, store};
use serde_json::{Value, json};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

static ENV: Mutex<()> = Mutex::new(());

struct EnvGuard {
    key: &'static str,
    prev: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: &str) -> Self {
        let prev = std::env::var(key).ok();
        // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
        unsafe { std::env::set_var(key, value) };
        Self { key, prev }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.prev {
            // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
            Some(v) => unsafe { std::env::set_var(self.key, v) },
            // SAFETY: test-only env mutation; callers serialize through the file's env mutex or run before other threads read the variable.
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
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
    let dir = std::env::temp_dir().join(format!("kutha-m017-verify-{label}-{}", uuid_like()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    store::ensure_legacy_dir(&dir).unwrap();
    dir
}

fn seed_two_anchors() -> Runtime {
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit_tt_anchor(2014).unwrap();
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2020).unwrap();
    rt
}

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(&l).expect("Event jsonl"))
        .collect()
}

fn write_jsonl_events(path: &Path, events: &[Event]) {
    let mut f = File::create(path).unwrap();
    for e in events {
        serde_json::to_writer(&mut f, e).unwrap();
        f.write_all(b"\n").unwrap();
    }
}

fn load_snapshot(dir: &Path) -> Value {
    serde_json::from_str(&fs::read_to_string(dir.join("snapshot.json")).unwrap()).unwrap()
}

fn save_snapshot(dir: &Path, snap: &Value) {
    fs::write(dir.join("snapshot.json"), serde_json::to_vec(snap).unwrap()).unwrap();
}

fn tamper_snapshot_wall(dir: &Path, from: u64, to: u64) {
    let mut snap = load_snapshot(dir);
    let anchors = snap
        .get_mut("fold")
        .and_then(|f| f.get_mut("tt_anchors"))
        .and_then(|a| a.as_object_mut())
        .expect("fold.tt_anchors");
    let val = anchors.remove(&from.to_string()).expect("anchor wall key");
    anchors.insert(to.to_string(), val);
    save_snapshot(dir, &snap);
}

fn tamper_snapshot_position(dir: &Path) {
    let mut snap = load_snapshot(dir);
    let anchors = snap
        .get_mut("fold")
        .and_then(|f| f.get_mut("tt_anchors"))
        .and_then(|a| a.as_object_mut())
        .expect("fold.tt_anchors");
    assert!(anchors.len() >= 2, "need two snapshot anchors");
    for (_, v) in anchors.iter_mut() {
        let n = v.as_u64().expect("ingested_at");
        *v = json!(n.wrapping_add(1_000_000));
    }
    save_snapshot(dir, &snap);
}

fn tamper_jsonl_anchor_wall(dir: &Path, from: u64, to: u64) {
    let path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&path);
    let mut n = 0usize;
    for e in &mut events {
        if let Op::TtAnchor { wall } = &mut e.op {
            if *wall == from {
                *wall = to;
                n += 1;
            }
        }
    }
    assert!(n > 0, "jsonl must contain Op::TtAnchor wall {from}");
    write_jsonl_events(&path, &events);
}

fn assert_open_err(dir: &Path, why: &str) {
    match store::open(dir) {
        Ok(_) => panic!("{why}: store::open must Err"),
        Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::InvalidData, "{why}: {e}"),
    }
    match Runtime::open(dir) {
        Ok(_) => panic!("{why}: Runtime::open must Err"),
        Err(_) => {}
    }
}

fn product_known_brute(rt: &Runtime, tt: u64) -> Vec<(u32, u32, u32)> {
    let mut v: Vec<_> = rt
        .fold()
        .facts()
        .iter()
        .filter(|f| {
            f.is_known_at(tt)
                && rt
                    .dictionary()
                    .lookup(f.relation)
                    .is_some_and(|n| !n.starts_with("kutha:"))
        })
        .map(|f| (f.subject, f.relation, f.object()))
        .collect();
    v.sort_unstable();
    v
}

fn sorted_known_at(rt: &Runtime, tt: u64) -> Vec<(u32, u32, u32)> {
    let mut v = rt.fold().known_at(tt);
    v.sort_unstable();
    v
}

#[test]
fn a_snapshot_anchor_wall_tamper_fails_open() {
    let rt = seed_two_anchors();
    let dir = temp_dir("a-wall");
    store::persist(&rt, &dir).unwrap();
    rt.replay_check().unwrap();
    tamper_snapshot_wall(&dir, 2014, 1999);
    assert_open_err(&dir, "REPL-01 snapshot wall");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_snapshot_anchor_position_tamper_fails_open() {
    let rt = seed_two_anchors();
    let dir = temp_dir("a-pos");
    store::persist(&rt, &dir).unwrap();
    tamper_snapshot_position(&dir);
    assert_open_err(&dir, "REPL-01 snapshot position");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn a_jsonl_and_snapshot_wall_agree_wal_intact_fails_open() {
    let rt = seed_two_anchors();
    let dir = temp_dir("a-jsonl-snap");
    store::persist(&rt, &dir).unwrap();
    tamper_jsonl_anchor_wall(&dir, 2014, 1999);
    tamper_snapshot_wall(&dir, 2014, 1999);
    // WAL still has original payloads; ids still cover jsonl → prefer WAL → snap mismatch.
    assert_open_err(&dir, "REPL-01 jsonl+snapshot wall with intact WAL");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn b_truncated_wal_never_yields_shorter_log() {
    let rt = seed_two_anchors();
    let dir = temp_dir("b-wal");
    store::persist(&rt, &dir).unwrap();
    let jsonl_ids: Vec<EventId> = read_jsonl_events(&dir.join("events.jsonl"))
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect();
    assert!(
        jsonl_ids.len() >= 2,
        "jsonl graph must keep product+anchor events"
    );
    let wal = dir.join("events.wal");
    let full = fs::read(&wal).unwrap();
    assert!(full.len() > 16, "WAL must contain framed records");
    for n in [0usize, 8, 12, full.len().saturating_sub(3)] {
        fs::write(&wal, &full[..n.min(full.len())]).unwrap();
        match store::open(&dir) {
            Ok(opened) => {
                let got: Vec<EventId> = opened.log().iter().map(|e| e.id).collect();
                assert_eq!(
                    got.len(),
                    jsonl_ids.len(),
                    "truncated WAL ({n} bytes) must not yield a shorter log than jsonl"
                );
                assert_eq!(
                    got, jsonl_ids,
                    "truncated WAL ({n} bytes) must replay intact jsonl ids"
                );
            }
            Err(_) => {}
        }
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn c_jsonl_wal_event_id_disagreement_fails_open() {
    let rt = seed_two_anchors();
    let dir = temp_dir("c-ids");
    store::persist(&rt, &dir).unwrap();
    let path = dir.join("events.jsonl");
    let mut events = read_jsonl_events(&path);
    let idx = events
        .iter()
        .rposition(|e| matches!(e.op, Op::TtAnchor { .. }))
        .expect("TtAnchor in jsonl");
    events[idx].id = EventId::from_u128(0x1111_2222_3333_4444);
    write_jsonl_events(&path, &events);
    assert_open_err(&dir, "REPL-02 id disagreement");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn d_spurious_outcomes_sidecar_fails_absent_ok() {
    let rt = seed_two_anchors();
    let dir = temp_dir("d-out");
    store::persist(&rt, &dir).unwrap();
    store::open(&dir).expect("honest persist");
    fs::remove_file(dir.join(store::OUTCOMES_REL)).unwrap();
    store::open(&dir).expect("REPL-03 absent outcomes sidecar");
    let fake = PersistedQuantumOutcome {
        quantum_id: "spurious-not-on-log".into(),
        disposition: OutcomeDisposition::Full,
        aborted_on_budget: false,
        events_in_quantum: 9,
        event_ids: vec![EventId::from_u128(1)],
        receipt_digest_hex: "00".into(),
        resume_of: None,
    };
    let mut f = File::create(dir.join(store::OUTCOMES_REL)).unwrap();
    serde_json::to_writer(&mut f, &fake).unwrap();
    f.write_all(b"\n").unwrap();
    assert_open_err(&dir, "REPL-03 spurious outcomes");
    let mut jf = File::create(dir.join(store::JUSTIFICATIONS_REL)).unwrap();
    let cite = Justification {
        justification_id: "not-on-log".into(),
        target_claim: EventId::from_u128(2),
        source_claim_ids: vec![],
        source_event_ids: vec![],
        rule_version: "x".into(),
        tt: 0,
        vt: 0,
    };
    serde_json::to_writer(&mut jf, &cite).unwrap();
    jf.write_all(b"\n").unwrap();
    fs::remove_file(dir.join(store::OUTCOMES_REL)).unwrap();
    assert_open_err(&dir, "REPL-03 spurious justifications");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn e_open_uses_snapshot_index_mode_not_env() {
    let _lock = ENV.lock().unwrap_or_else(|e| e.into_inner());
    let dir = temp_dir("e-mode");
    let persist_guard = EnvGuard::set("KUTHA_FOLD_INDEX_MODE", "eager");
    let rt = seed_two_anchors();
    assert_eq!(rt.index_mode(), FoldIndexMode::Eager);
    store::persist(&rt, &dir).unwrap();
    drop(persist_guard);
    let _open_guard = EnvGuard::set("KUTHA_FOLD_INDEX_MODE", "lazy");
    let opened = Runtime::open(&dir).expect("REPL-04 open");
    assert_eq!(
        opened.index_mode(),
        FoldIndexMode::Eager,
        "open must not take FoldIndexMode from env"
    );
    let vt = ValidTime::from(2015);
    let mut a = rt.as_of(vt);
    let mut b = opened.as_of(vt);
    a.sort_unstable();
    b.sort_unstable();
    assert_eq!(a, b);
    let mut ka = rt.as_known_on(2015).expect("anchor");
    let mut kb = opened.as_known_on(2015).expect("anchor rebuilt");
    ka.sort_unstable();
    kb.sort_unstable();
    assert_eq!(ka, kb);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn f_abort_before_rename_leaves_dest_bytes() {
    let rt = seed_two_anchors();
    let dir = temp_dir("f-abort");
    store::persist(&rt, &dir).unwrap();
    let jsonl = dir.join("events.jsonl");
    let wal = dir.join("events.wal");
    let snap = dir.join("snapshot.json");
    let jsonl_b = fs::read(&jsonl).unwrap();
    let wal_b = fs::read(&wal).unwrap();
    let snap_b = fs::read(&snap).unwrap();
    store::abort_events_jsonl_replace_before_rename(&dir, &read_jsonl_events(&jsonl))
        .expect_err("abort jsonl");
    assert_eq!(fs::read(&jsonl).unwrap(), jsonl_b);
    store::abort_wal_replace_before_rename(&dir, &rt).expect_err("abort wal");
    assert_eq!(fs::read(&wal).unwrap(), wal_b);
    store::abort_snapshot_replace_before_rename(&dir, &rt).expect_err("abort snap");
    assert_eq!(fs::read(&snap).unwrap(), snap_b);
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn g_closed_vt_as_known_on_differs_live_at_max() {
    let mut rt = Runtime::with_index_mode(32, FoldIndexMode::Eager);
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    let statute = (article, in_force, limit_50);
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    let tt = rt
        .log()
        .iter()
        .find_map(|e| match &e.op {
            Op::TtAnchor { wall } if *wall == 2015 => Some(e.ingested_at),
            _ => None,
        })
        .expect("2015 anchor");
    let known = rt.as_known_on(2015).expect("TT-only cut");
    assert!(known.contains(&statute), "as_known_on ignores closed VT");
    let live_tt_max = rt.live_at(tt, ValidTime::MAX);
    assert!(
        !live_tt_max.contains(&statute),
        "KNOWN-03: as_known_on != live_at(tt, MAX) on closed VT"
    );
    let live_max_vt = rt.live_at(TransactionTime::MAX, ValidTime::from(2025));
    assert!(
        !live_max_vt.contains(&statute),
        "as_known_on(D) != live_at(MAX, vt) outside interval"
    );
    assert_eq!(sorted_known_at(&rt, tt), product_known_brute(&rt, tt));
    let src = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/quantum.rs"));
    assert!(
        src.contains("TT-only"),
        "Runtime::as_known_on rustdoc must say TT-only"
    );
}

#[test]
fn h_tt_anchor_does_not_enter_facts_fingerprint() {
    let mut rt = Runtime::default();
    let n = rt.fold().facts().len();
    let fp = rt.fold().fingerprint();
    rt.emit_tt_anchor(2014).unwrap();
    rt.emit_tt_anchor(2020).unwrap();
    assert_eq!(
        rt.fold().facts().len(),
        n,
        "TtAnchor must not mint fold facts"
    );
    assert_eq!(
        rt.fold().fingerprint(),
        fp,
        "facts fingerprint must ignore TtAnchor"
    );
}
