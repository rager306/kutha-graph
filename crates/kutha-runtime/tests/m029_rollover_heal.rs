//! M029 S01 / Phase 89 — interrupted rollover seal trailer must not wedge
//! attach / open_for_append (CE review P1 #3).

use kutha_common::Op;
use kutha_runtime::{Runtime, store};
use std::fs;
use std::path::{Path, PathBuf};

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
        .join(format!(
            "m029-rollover-heal-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let article = rt.intern("art-m029");
    let in_force = rt.intern("inForceAs");
    let limit = rt.intern("50-kmh");
    rt.emit(Op::Assert {
        subject: article,
        relation: in_force,
        object: limit,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("m029-pad-{i}"));
    }
    rt
}

fn overlay_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    let events = store::encoded_events(rt);
    let n = events.len();
    assert!(n >= 10);
    store::write_test_segmented(dir, &events, &[4, 4], n - 8).unwrap();
    // Tiny-limit persist may leave orphan seg-* files beyond the test layout;
    // remove anything not named in the rewritten manifest.
    let man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap();
    let mut keep = std::collections::HashSet::new();
    keep.insert(man["active"].as_str().unwrap().to_string());
    for s in man["sealed"].as_array().unwrap() {
        keep.insert(s["name"].as_str().unwrap().to_string());
    }
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("seg-") && name.ends_with(".log") && !keep.contains(&name) {
            let _ = fs::remove_file(entry.path());
        }
    }
    let _ = fs::remove_file(dir.join("events.jsonl"));
    let _ = fs::remove_file(dir.join("events.wal"));
}

#[test]
fn open_for_append_heals_after_rollover_trailer_crash() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("RolloverTrailer");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverTrailer)
        .expect_err("rollover failpoint must inject");

    let opened = store::open(&dir).expect("read path must still open after mid-rollover");
    assert_eq!(opened.fold().fingerprint(), committed_fp);

    // Crash = process gone: Drop unlocks. Live writers keep kutha.lock on Other (M030).
    drop(p);
    let (mut healed, mut writer) =
        store::open_for_append(&dir).expect("open_for_append must finish interrupted rollover");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    healed.emit_tt_anchor(2022).unwrap();
    writer.append(&healed).expect("append after heal");
    let again = store::open(&dir).expect("reopen after post-heal append");
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_for_append_heals_after_rollover_new_header_crash() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("RolloverNewHeader");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(2021).unwrap();
    store::append_with_failpoint(&mut p, &rt, store::AppendStep::RolloverNewHeader)
        .expect_err("rollover failpoint must inject");

    let opened = store::open(&dir).expect("read path must still open after mid-rollover");
    assert_eq!(opened.fold().fingerprint(), committed_fp);

    drop(p);
    let (mut healed, mut writer) =
        store::open_for_append(&dir).expect("open_for_append must finish interrupted rollover");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    healed.emit_tt_anchor(2022).unwrap();
    writer.append(&healed).expect("append after heal");
    let again = store::open(&dir).expect("reopen after post-heal append");
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    let _ = fs::remove_dir_all(&dir);
}
