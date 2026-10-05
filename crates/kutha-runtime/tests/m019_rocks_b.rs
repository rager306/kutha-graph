//! ROCK-02 oracles (Phase 61). Feature-gated; default workspace tests never compile RocksDB.
//! D-07: FF5/FF6/M010–M017/`m002_*` assertions unchanged.

#![cfg(feature = "rocks-index")]

use kutha_common::{EventId, Op};
use kutha_runtime::rocks_index::SegmentIndex;
use kutha_runtime::{Runtime, store};
use rocksdb::{DB, Options};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;
const KEY_SEALED: &[u8] = b"m:sealed";

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
        .join(format!("p61-rocks-{label}-{nanos}"));
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
        let _ = rt.intern(&format!("p61-pad-{i}"));
    }
    rt
}

fn event_key(id: EventId) -> Vec<u8> {
    let mut k = b"e:".to_vec();
    k.extend_from_slice(id.as_bytes());
    k
}

fn open_rocks(rocks_dir: &Path) -> DB {
    DB::open(&Options::default(), rocks_dir).expect("open rocks dir")
}

fn middle_and_tip(rt: &Runtime) -> (EventId, EventId) {
    let events = store::encoded_events(rt);
    assert!(events.len() >= 2, "need a non-tip event to poison");
    let tip = events.last().unwrap().id;
    let mid = events
        .iter()
        .find(|e| e.id != tip)
        .map(|e| e.id)
        .expect("non-tip id");
    (mid, tip)
}

fn poison_locator(rocks: &Path, id: EventId) {
    let db = open_rocks(rocks);
    db.put(&event_key(id), [0u8; 16]).unwrap();
    drop(db);
}

/// D-01 pin (not a bug): middle-locator poison with matching `m:tip` is not an
/// `open_or_rebuild` trigger. `get_event` already fail-closes. Pair with `verify_full`.
#[test]
fn middle_locator_poison_survives_open_or_rebuild() {
    let rt = seed_runtime();
    let dir = persist_dir("mid-survive");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let (mid, tip) = middle_and_tip(&rt);
    drop(SegmentIndex::build(&dir, &rocks).unwrap());
    poison_locator(&rocks, mid);

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    assert!(
        idx.get_event(tip).unwrap().is_some(),
        "tip event must still locate"
    );
    match idx.get_event(mid) {
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
        Ok(other) => panic!("poisoned middle locator must be InvalidData, got {other:?}"),
    }
    drop(idx);
    let db = open_rocks(&rocks);
    let stored = db
        .get(event_key(mid))
        .unwrap()
        .expect("locator still present");
    assert_eq!(
        stored.as_slice(),
        [0u8; 16],
        "open_or_rebuild must not rebuild"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn verify_full_rejects_poisoned_middle_locator() {
    let rt = seed_runtime();
    let dir = persist_dir("mid-digest");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let (mid, _tip) = middle_and_tip(&rt);
    let idx = SegmentIndex::build(&dir, &rocks).unwrap();
    idx.verify_full()
        .expect("clean index must pass verify_full");
    drop(idx);
    poison_locator(&rocks, mid);

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    match idx.verify_full() {
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
        Ok(()) => panic!("verify_full must reject a poisoned middle locator"),
    }
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_or_rebuild_rebuilds_when_sealed_list_binding_differs() {
    let _guard = tiny_limits();
    let rt = seed_runtime();
    let dir = persist_dir("sealed-bind");
    store::persist(&rt, &dir).unwrap();
    let rocks = dir.join("rocks");
    let (mid, _tip) = middle_and_tip(&rt);
    drop(SegmentIndex::build(&dir, &rocks).unwrap());
    poison_locator(&rocks, mid);
    let db = open_rocks(&rocks);
    db.put(KEY_SEALED, b"tampered-sealed-list").unwrap();
    drop(db);

    let idx = SegmentIndex::open_or_rebuild(&dir, &rocks).unwrap();
    assert_eq!(
        idx.get_event(mid).unwrap().as_ref().map(|e| e.id),
        Some(mid),
        "sealed-list mismatch must rebuild locators"
    );
    idx.verify_full()
        .expect("rebuilt index must pass verify_full");
    let _ = fs::remove_dir_all(&dir);
}
