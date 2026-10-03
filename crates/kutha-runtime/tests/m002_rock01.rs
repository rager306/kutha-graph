//! ROCK-01 oracles (Phase 55). Feature-gated; default workspace tests never compile RocksDB.

#![cfg(feature = "rocks-index")]

use kutha_common::{Event, EventId, Op, OutcomeDisposition};
use kutha_runtime::rocks_index::{Locator, SegmentIndex};
use kutha_runtime::{store, Runtime};
use rocksdb::{IteratorMode, Options, DB};
use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

struct LimitGuard;

impl Drop for LimitGuard {
    fn drop(&mut self) {
        store::reset_test_segment_limits();
    }
}

fn tiny_limits() -> LimitGuard {
    store::set_test_segment_limits(Some(4096), Some(3));
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
        .join(format!("p55-store-{label}-{nanos}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-12-speed");
    let in_force = rt.intern("inForceAs");
    let limit_50 = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit_50,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(T_OLD).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("p55-pad-{i}"));
    }
    rt
}

fn delivery_map_from_events<'a, I>(events: I) -> HashMap<String, EventId>
where
    I: IntoIterator<Item = &'a Event>,
{
    let mut map = HashMap::new();
    for e in events {
        if let Op::Assert {
            delivery_key: Some(k),
            ..
        } = &e.op
        {
            if !k.is_empty() {
                map.insert(k.clone(), e.id);
            }
        }
    }
    map
}

fn runtime_delivery_map(rt: &Runtime) -> HashMap<String, EventId> {
    delivery_map_from_events(rt.log().iter())
}

fn event_key(id: EventId) -> Vec<u8> {
    let mut k = b"e:".to_vec();
    k.extend_from_slice(id.as_bytes());
    k
}

fn open_rocks(rocks_dir: &Path) -> DB {
    DB::open(&Options::default(), rocks_dir).expect("open rocks dir")
}

fn crash_omit_last(p: &mut store::Persister, rt: &Runtime) {
    let n = store::pending_append_len(p, rt).unwrap();
    assert!(
        n >= 2,
        "pending batch must include payload and QuantumOutcome; n={n}"
    );
    store::append_with_event_failpoint(p, rt, n - 1)
        .expect_err("event failpoint must inject after N-1 events");
}

fn emit_keyed_assert(rt: &mut Runtime, key: &str, object: &str) -> EventId {
    let subject = rt.intern("p55-keyed-article");
    let relation = rt.intern("inForceAs");
    let object = rt.intern(object);
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from: VF_FIRST,
        valid_to: Some(VF_SECOND),
        claim: None,
        delivery_key: Some(key.to_string()),
        polarity: None,
    })
    .unwrap();
    rt.log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| {
            matches!(
                &e.op,
                Op::Assert {
                    delivery_key: Some(k),
                    ..
                } if k == key
            )
        })
        .map(|e| e.id)
        .expect("keyed assert must be on the log")
}

fn sealed_count(dir: &Path) -> usize {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    v["sealed"].as_array().expect("sealed array").len()
}

fn scan_pairs(rocks_dir: &Path) -> Vec<(Vec<u8>, Vec<u8>)> {
    let db = DB::open_for_read_only(&Options::default(), rocks_dir, false).unwrap();
    db.iterator(IteratorMode::Start)
        .map(|r| r.unwrap())
        .map(|(k, v)| (k.to_vec(), v.to_vec()))
        .collect()
}

#[test]
fn index_answers_equal_the_log() {
    let rt = seed_runtime();
    let dir = persist_dir("eq-log");
    let events = store::encoded_events(&rt);
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    for e in &events {
        let got = idx
            .get_event(e.id)
            .unwrap()
            .expect("every committed event id must locate");
        assert_eq!(got, *e);
        assert!(idx.locate(e.id).unwrap().is_some());
    }
    assert!(
        idx.get_event(EventId::nil()).unwrap().is_none(),
        "absent id is None"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn delivery_key_index_equals_runtime_map() {
    let mut rt = Runtime::default();
    let kept = emit_keyed_assert(&mut rt, "p55-keep", "50-kmh");
    let retracted = emit_keyed_assert(&mut rt, "p55-retract", "60-kmh");
    rt.emit(Op::Retract {
        event_id: retracted,
    })
    .unwrap();
    let corrected = emit_keyed_assert(&mut rt, "p55-correct", "70-kmh");
    rt.correct("p55-keyed-article", "inForceAs", "80-kmh", VF_FIRST)
        .unwrap();
    let expected = runtime_delivery_map(&rt);
    assert_eq!(expected.get("p55-keep").copied(), Some(kept));
    assert_eq!(expected.get("p55-retract").copied(), Some(retracted));
    assert_eq!(expected.get("p55-correct").copied(), Some(corrected));

    let dir = persist_dir("dkey");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    for (key, id) in &expected {
        assert_eq!(idx.by_delivery_key(key).unwrap(), Some(*id), "key={key}");
    }
    assert!(idx.by_delivery_key("p55-absent").unwrap().is_none());

    let a = Event::stable_define("s", 0);
    let b = Event::stable_define("r", 0);
    let c = Event::stable_define("o", 0);
    let first = Event::new(
        Op::Assert {
            subject: 0,
            relation: 1,
            object: 2,
            valid_from: VF_FIRST,
            valid_to: None,
            claim: None,
            delivery_key: Some("p55-twice".into()),
            polarity: None,
        },
        1,
    );
    let live = Event::new(
        Op::Assert {
            subject: 0,
            relation: 1,
            object: 2,
            valid_from: VF_FIRST,
            valid_to: None,
            claim: None,
            delivery_key: Some("p55-twice".into()),
            polarity: None,
        },
        2,
    );
    let qo = Event::new(
        Op::QuantumOutcome {
            quantum_id: "p55-twice-q".into(),
            disposition: OutcomeDisposition::Full,
            aborted_on_budget: false,
            events_in_quantum: 2,
            event_ids: vec![first.id, live.id],
            receipt_digest_hex: "00".repeat(32),
            resume_of: None,
        },
        3,
    );
    let twice_dir = persist_dir("dkey-twice");
    store::write_test_segmented(
        &twice_dir,
        &[a, b, c, first.clone(), live.clone(), qo],
        &[3],
        3,
    )
    .unwrap();
    let twice_rocks = twice_dir.join("rocks");
    let twice = SegmentIndex::build(&twice_dir, &twice_rocks).unwrap();
    assert_eq!(
        twice.by_delivery_key("p55-twice").unwrap(),
        Some(live.id),
        "last Assert with the key wins (Runtime rebuild_log_indexes)"
    );
    assert_ne!(first.id, live.id);

    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&twice_dir);
}

#[test]
fn dropping_rocks_dir_changes_no_answer() {
    let rt = seed_runtime();
    let dir = persist_dir("drop");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let events = store::encoded_events(&rt);
    {
        let idx = SegmentIndex::build(&dir, &rocks).unwrap();
        assert!(idx.get_event(events[0].id).unwrap().is_some());
    }
    fs::remove_dir_all(&rocks).unwrap();
    assert!(!rocks.exists());
    let opened = store::open(&dir).unwrap();
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    for e in &events {
        assert_eq!(idx.get_event(e.id).unwrap().as_ref(), Some(e));
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn stale_index_is_rebuilt() {
    let mut rt = seed_runtime();
    let dir = persist_dir("stale");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let before = store::encoded_events(&rt);
    drop(SegmentIndex::build(&dir, &rocks).unwrap());

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let article = rt.intern("p55-stale-article");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("p55-stale-90");
    rt.emit(Op::Assert {
        subject: article,
        relation: rel,
        object: obj,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    p.append(&rt).unwrap();
    let after = store::encoded_events(&rt);
    assert!(after.len() > before.len());
    let new_id = after.last().unwrap().id;

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    assert!(
        idx.get_event(new_id).unwrap().is_some(),
        "open_or_rebuild must see Persister-appended events"
    );
    drop(idx);

    let db = open_rocks(&rocks);
    db.put(b"m:tip", b"tampered-tip").unwrap();
    drop(db);
    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    assert!(idx.get_event(new_id).unwrap().is_some());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn catch_up_indexes_only_the_tail() {
    let mut rt = seed_runtime();
    let dir = persist_dir("catch");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let old_id = store::encoded_events(&rt)[0].id;
    drop(SegmentIndex::build(&dir, &rocks).unwrap());

    let db = open_rocks(&rocks);
    db.put(&event_key(old_id), [0u8; 16]).unwrap();
    drop(db);

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let article = rt.intern("p55-catch-article");
    let rel = rt.intern("inForceAs");
    let obj = rt.intern("p55-catch-90");
    rt.emit(Op::Assert {
        subject: article,
        relation: rel,
        object: obj,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    p.append(&rt).unwrap();
    let new_id = store::encoded_events(&rt).last().unwrap().id;

    let n = idx.catch_up().unwrap();
    assert!(n > 0, "catch_up must index the appended tail");
    assert!(idx.get_event(new_id).unwrap().is_some());
    match idx.get_event(old_id) {
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
        Ok(_) => panic!("catch_up must not rewrite a poisoned prefix locator"),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn locator_pointing_at_wrong_frame_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("wrong-frame");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let events = store::encoded_events(&rt);
    assert!(events.len() >= 2);
    let a = events[0].id;
    let b = events[1].id;
    drop(SegmentIndex::build(&dir, &rocks).unwrap());

    let db = open_rocks(&rocks);
    let va = db.get(event_key(a)).unwrap().expect("locator a");
    let vb = db.get(event_key(b)).unwrap().expect("locator b");
    db.put(&event_key(a), &vb).unwrap();
    db.put(&event_key(b), &va).unwrap();
    drop(db);

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    match idx.get_event(a) {
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
        Ok(other) => panic!("wrong-frame locator must be InvalidData, got {other:?}"),
    }
    drop(idx);

    let last = events.last().unwrap().id;
    let db = open_rocks(&rocks);
    db.put(&event_key(last), [0u8; 16]).unwrap();
    drop(db);
    let recovered = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    assert_eq!(
        recovered.get_event(last).unwrap().as_ref().map(|e| e.id),
        Some(last)
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn rocks_values_hold_no_payload() {
    let mut rt = seed_runtime();
    let _ = emit_keyed_assert(&mut rt, "p55-meta-key", "50-kmh");
    let dir = persist_dir("no-payload");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    drop(SegmentIndex::build(&dir, &rocks).unwrap());
    for (k, v) in scan_pairs(&rocks) {
        assert!(
            k.starts_with(b"e:") || k.starts_with(b"d:") || k.starts_with(b"m:"),
            "unexpected key prefix {k:?}"
        );
        let as_str = String::from_utf8_lossy(&v);
        assert!(
            serde_json::from_slice::<serde_json::Value>(&v).is_err(),
            "value must not be JSON: {as_str}"
        );
        assert!(
            !as_str.contains("inForceAs"),
            "value must not hold a term string: {as_str}"
        );
        if k.starts_with(b"e:") {
            assert_eq!(v.len(), 16, "e: locator is seq+offset+len");
        }
        if k.starts_with(b"d:") {
            assert_eq!(v.len(), 16, "d: value is EventId");
        }
        if k == b"m:v" {
            assert_eq!(v.len(), 4, "m:v is u32be");
        }
        if k == b"m:tip" {
            assert_eq!(v.len(), 88, "m:tip is hex+count+EventId");
        }
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn legacy_dir_rejected_until_migrate() {
    let rt = seed_runtime();
    let dir = persist_dir("legacy");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    assert!(!dir.join(store::MANIFEST_REL).exists());
    let rocks = dir.join("rocks");
    match SegmentIndex::build(&dir, &rocks) {
        Err(e) => {
            assert_eq!(e.kind(), ErrorKind::InvalidData);
            assert!(
                e.to_string().contains("run store::migrate first"),
                "got {e}"
            );
        }
        Ok(_) => panic!("legacy jsonl dir must be rejected"),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn uncommitted_suffix_is_not_indexed() {
    let mut rt = seed_runtime();
    let dir = persist_dir("uncommitted");
    store::persist(&rt, &dir).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let subject = rt.intern("p55-uncommitted-article");
    let relation = rt.intern("inForceAs");
    let object = rt.intern("p55-uncommitted-80");
    rt.emit(Op::Assert {
        subject,
        relation,
        object,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    let uncommitted_id = rt
        .log()
        .as_slice()
        .iter()
        .rev()
        .find(|e| matches!(e.op, Op::Assert { .. }))
        .map(|e| e.id)
        .unwrap();
    crash_omit_last(&mut p, &rt);

    let rocks = dir.join("rocks");
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    assert!(
        idx.get_event(uncommitted_id).unwrap().is_none(),
        "uncommitted suffix must not be indexed"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn multi_segment_store_locates_across_rollover() {
    let _guard = tiny_limits();
    let rt = seed_runtime();
    let dir = persist_dir("multi");
    store::persist(&rt, &dir).unwrap();
    assert!(
        sealed_count(&dir) >= 1,
        "lowered rollover seam must seal at least one segment"
    );
    let events = store::encoded_events(&rt);
    let rocks = dir.join("rocks");
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    let mut seqs = std::collections::BTreeSet::new();
    for e in &events {
        let loc: Locator = idx.locate(e.id).unwrap().expect("id across rollover");
        seqs.insert(loc.seq);
        assert_eq!(idx.get_event(e.id).unwrap().as_ref(), Some(e));
    }
    assert!(
        seqs.len() >= 2,
        "locators must span more than one segment seq, got {seqs:?}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persister_source_does_not_write_rocks() {
    let persist = include_str!("../src/persist_append.rs");
    let store = include_str!("../src/store.rs");
    let quantum = include_str!("../src/quantum.rs");
    for (name, src) in [
        ("persist_append.rs", persist),
        ("store.rs", store),
        ("quantum.rs", quantum),
    ] {
        let lower = src.to_ascii_lowercase();
        assert!(
            !lower.contains("rocksdb") && !lower.contains("rocks_index"),
            "{name} must not write a Rocks lease"
        );
    }
}
