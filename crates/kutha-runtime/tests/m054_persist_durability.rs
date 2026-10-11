//! M054 S02 / Phase 118 — persist durability (CE whole-repo #9 #10).
//! #6 poison stays a human decision-gate and is not leased here.

use kutha_common::{EventId, Op};
use kutha_runtime::{Runtime, store};
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!(
            "m054-persist-durability-{label}-{}-{}",
            std::process::id(),
            nanos
        ));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn seed_runtime() -> Runtime {
    let mut rt = Runtime::default();
    let s = rt.intern("m054-s02-article");
    let r = rt.intern("relatedTo");
    let o = rt.intern("m054-s02-object");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: Some(2020),
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
    rt.emit_tt_anchor(2015).unwrap();
    for i in 0..8 {
        let _ = rt.intern(&format!("m054-s02-pad-{i}"));
    }
    rt
}

fn log_ids(rt: &Runtime) -> Vec<EventId> {
    rt.log()
        .iter()
        .filter(|e| !matches!(e.op, Op::Define { .. }))
        .map(|e| e.id)
        .collect()
}

fn manifest_value(dir: &Path) -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(dir.join(store::MANIFEST_REL)).unwrap()).unwrap()
}

fn live_segment_names(dir: &Path) -> HashSet<String> {
    let man = manifest_value(dir);
    let mut names = HashSet::new();
    names.insert(man["active"].as_str().expect("manifest.active").to_string());
    for sealed in man["sealed"].as_array().expect("manifest.sealed") {
        names.insert(sealed["name"].as_str().expect("sealed.name").to_string());
    }
    names
}

fn drop_unreferenced_segments(dir: &Path) {
    let keep = live_segment_names(dir);
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("seg-") && name.ends_with(".log") && !keep.contains(&name) {
            let _ = fs::remove_file(entry.path());
        }
    }
}

fn overlay_empty_active(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    let events = store::encoded_events(rt);
    let n = events.len();
    assert!(n >= 1, "seed must persist at least one encoded event");
    store::write_test_segmented(dir, &events, &[n], 0).unwrap();
    drop_unreferenced_segments(dir);
}

fn tear_empty_active(dir: &Path) {
    let man = manifest_value(dir);
    let active = dir.join(man["active"].as_str().expect("manifest.active"));
    let mut bytes = fs::read(&active).unwrap();
    assert_eq!(
        bytes.len(),
        store::SEGMENT_HEADER_LEN as usize,
        "empty active must be header-only before the torn tail"
    );
    assert!(man["sealed"].as_array().expect("sealed").len() >= 1);
    bytes.extend_from_slice(&[0xAA, 0xBB, 0xCC]);
    fs::write(&active, &bytes).unwrap();
}

fn emit_follow_on(rt: &mut Runtime, label: &str) {
    let s = rt.intern(&format!("m054-s02-{label}-article"));
    let o = rt.intern(&format!("m054-s02-{label}-object"));
    let r = rt.dictionary().id("relatedTo").expect("relatedTo");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: 2010,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

/// #9 — torn empty-active tail must be healed under lock before append; the
/// follow-on quantum must survive exact reopen (writer_torn_active_tail path).
#[test]
fn open_for_append_heals_torn_empty_active_then_append_survives_reopen() {
    let rt = seed_runtime();
    let dir = persist_dir("torn-empty-active");
    overlay_empty_active(&rt, &dir);
    tear_empty_active(&dir);

    let opened = store::open(&dir).expect("read-only open drops the torn prefix");
    assert_eq!(log_ids(&opened), log_ids(&rt));

    let (mut healed, mut writer) = store::open_for_append(&dir)
        .expect("open_for_append must heal torn empty active under kutha.lock");
    assert_eq!(log_ids(&healed), log_ids(&rt));
    emit_follow_on(&mut healed, "torn");
    writer
        .append(&healed)
        .expect("append after torn-empty-active heal");
    drop(writer);

    let reopened = store::open(&dir).expect("reopen after post-heal append");
    assert_eq!(
        log_ids(&reopened),
        log_ids(&healed),
        "append after a torn empty active must not vanish on reopen"
    );
    assert_eq!(reopened.fold().fingerprint(), healed.fold().fingerprint());
    reopened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

/// #10 — a full persist must mint generation-unique segment names so a later
/// rewrite cannot clobber the files an older manifest still names.
#[test]
fn persist_log_uses_generation_unique_segment_names() {
    let mut rt = seed_runtime();
    let dir = persist_dir("unique-names");
    store::persist(&rt, &dir).unwrap();
    let gen1 = live_segment_names(&dir);
    assert!(!gen1.is_empty());

    emit_follow_on(&mut rt, "unique");
    store::persist(&rt, &dir).unwrap();
    let gen2 = live_segment_names(&dir);
    assert!(
        gen1.is_disjoint(&gen2),
        "re-persist must not reuse live segment names; gen1={gen1:?} gen2={gen2:?}"
    );
    for name in &gen1 {
        assert!(
            !dir.join(name).exists(),
            "obsolete generation {name} must be deleted only after the new manifest is durable"
        );
    }
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
}

/// #10 — writing the next persist generation must not mutate files the old
/// manifest still names. Model a crash before the manifest swap by copying the
/// new persist's segment files into a dir that still holds the previous
/// manifest: the old store must still open.
#[test]
fn persist_crash_before_manifest_swap_keeps_old_generation_openable() {
    let mut rt = seed_runtime();
    let dir = persist_dir("old-manifest");
    store::persist(&rt, &dir).unwrap();
    let old_ids = log_ids(&rt);
    let old_fp = rt.fold().fingerprint();
    let old_names = live_segment_names(&dir);
    let old_bytes: Vec<(String, Vec<u8>)> = old_names
        .iter()
        .map(|name| (name.clone(), fs::read(dir.join(name)).unwrap()))
        .collect();

    emit_follow_on(&mut rt, "crash");
    let other = persist_dir("new-generation");
    store::persist(&rt, &other).unwrap();
    for entry in fs::read_dir(&other).unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with("seg-") && name.ends_with(".log") {
            fs::copy(entry.path(), dir.join(&name)).unwrap();
        }
    }

    for (name, bytes) in &old_bytes {
        assert_eq!(
            fs::read(dir.join(name)).unwrap(),
            *bytes,
            "old generation {name} must stay byte-identical while the old manifest is live"
        );
    }
    let opened = store::open(&dir).expect("old manifest must still open");
    assert_eq!(log_ids(&opened), old_ids);
    assert_eq!(opened.fold().fingerprint(), old_fp);
    opened.replay_check().unwrap();
    let _ = fs::remove_dir_all(&dir);
    let _ = fs::remove_dir_all(&other);
}
