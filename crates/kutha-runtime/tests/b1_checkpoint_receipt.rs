//! Phase 228 / M154 S01 — sealed-prefix checkpoint receipt oracle (B1).
//! RED: `store::checkpoint_sealed_prefix` is missing (E0425). Do not treat
//! sealed-file deletion as the RED.

use kutha_common::{Op, ValidTime, WallTime};
use kutha_runtime::{store, GraphFold, Runtime};
use std::fs;
use std::path::PathBuf;

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

fn persist_dir() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!(
        "kutha-b1-checkpoint-{}-{}",
        std::process::id(),
        nanos
    ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn sealed_len(dir: &std::path::Path) -> usize {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    v["sealed"].as_array().expect("sealed array").len()
}

#[test]
fn checkpoint_sealed_prefix_preserves_fingerprint_and_pit() {
    let _limits = tiny_limits();
    let dir = persist_dir();
    let mut rt = Runtime::default();
    rt.emit_allow("inForceAs", 2010, None).unwrap();
    let relation = rt.intern("inForceAs");
    let mut live_assert = None;
    for i in 0..6 {
        let subject = rt.intern(&format!("art-{i}"));
        let object = rt.intern(&format!("obj-{i}"));
        let id = rt
            .emit(Op::Assert {
                subject,
                relation,
                object,
                valid_from: 2010,
                valid_to: Some(2020),
                claim: None,
                delivery_key: Some(format!("dk-{i}")),
                polarity: None,
            })
            .unwrap()
            .receipt
            .event_ids[0];
        if live_assert.is_none() {
            live_assert = Some(id);
        }
    }
    rt.emit(Op::Retract {
        event_id: live_assert.expect("one live Assert"),
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::persist(&rt, &dir).unwrap();
    assert!(
        sealed_len(&dir) >= 2,
        "fixture must span at least two sealed segments"
    );

    let fingerprint = rt.fold().fingerprint();
    let as_of_2009 = rt.as_of(ValidTime::from(2009));
    let as_of_2015 = rt.as_of(ValidTime::from(2015));
    let as_of_2025 = rt.as_of(ValidTime::from(2025));
    let known_2015 = rt.as_known_on(WallTime::from(2015));
    let known_2021 = rt.as_known_on(WallTime::from(2021));

    store::checkpoint_sealed_prefix(&dir).unwrap();

    let opened = store::open(&dir).expect("open after checkpoint");
    opened.replay_check().expect("replay_check after checkpoint");
    assert_eq!(opened.fold().fingerprint(), fingerprint);
    assert_eq!(opened.as_of(ValidTime::from(2009)), as_of_2009);
    assert_eq!(opened.as_of(ValidTime::from(2015)), as_of_2015);
    assert_eq!(opened.as_of(ValidTime::from(2025)), as_of_2025);
    assert_eq!(opened.as_known_on(WallTime::from(2015)), known_2015);
    assert_eq!(opened.as_known_on(WallTime::from(2021)), known_2021);
    assert_eq!(
        GraphFold::replay_with_dict(opened.log().as_slice(), opened.dictionary()).fingerprint(),
        fingerprint
    );
}
