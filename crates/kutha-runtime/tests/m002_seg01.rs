//! M002 S06 / SEG-01 oracles (Phase 54 plan 54-01): format, manifest, open.
//! Persist still writes the legacy jsonl+WAL layout. RED: stub reader keeps jsonl SoT.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const VF_FIRST: u64 = 2010;
const VF_SECOND: u64 = 2020;

fn persist_dir(label: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("tmp")
        .join(format!("p54-store-{label}-{nanos}"));
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
        let _ = rt.intern(&format!("p54-pad-{i}"));
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

fn read_jsonl_events(path: &Path) -> Vec<Event> {
    let text = fs::read_to_string(path).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str::<Event>(l).expect("jsonl Event"))
        .collect()
}

fn persist_encoded(rt: &Runtime, dir: &Path) -> Vec<Event> {
    store::persist(rt, dir).unwrap();
    read_jsonl_events(&dir.join("events.jsonl"))
}

fn layout(n: usize) -> (Vec<usize>, usize) {
    assert!(
        n >= 10,
        "encoded log too small for sealed+active split: {n}"
    );
    (vec![4, 4], n - 8)
}

fn write_segments(dir: &Path, events: &[Event]) {
    let (sealed, active) = layout(events.len());
    store::write_test_segmented(dir, events, &sealed, active).unwrap();
}

fn drop_legacy_log(dir: &Path) {
    let _ = fs::remove_file(dir.join("events.jsonl"));
    let _ = fs::remove_file(dir.join("events.wal"));
}

fn smash_legacy_log(dir: &Path) {
    fs::write(dir.join("events.jsonl"), "{\"not\":\"the-segment-log\"}\n").unwrap();
    let _ = fs::remove_file(dir.join("events.wal"));
}

fn manifest_path(dir: &Path) -> PathBuf {
    dir.join(store::MANIFEST_REL)
}

fn first_sealed_path(dir: &Path) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    dir.join(v["sealed"][0]["name"].as_str().expect("sealed[0].name"))
}

fn active_path(dir: &Path) -> PathBuf {
    let v: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(dir)).unwrap()).unwrap();
    dir.join(v["active"].as_str().expect("active"))
}

fn is_seal_payload(payload: &[u8]) -> bool {
    serde_json::from_slice::<serde_json::Value>(payload)
        .ok()
        .and_then(|v| v.get("k").and_then(|k| k.as_str()).map(|s| s == "seal"))
        .unwrap_or(false)
}

fn event_frame_ranges(bytes: &[u8]) -> Vec<(usize, usize)> {
    assert!(bytes.len() >= 12, "segment header");
    assert_eq!(&bytes[0..8], b"KUTHASEG");
    let mut off = 12usize;
    let mut out = Vec::new();
    while off + 8 <= bytes.len() {
        let mut len_buf = [0u8; 4];
        len_buf.copy_from_slice(&bytes[off..off + 4]);
        let len = u32::from_le_bytes(len_buf) as usize;
        let end = off + 8 + len;
        if end > bytes.len() {
            break;
        }
        if is_seal_payload(&bytes[off + 8..end]) {
            break;
        }
        out.push((off, end));
        off = end;
    }
    out
}

fn assert_invalid_data(dir: &Path, needle: &str) {
    match store::open(dir) {
        Ok(_) => panic!("expected InvalidData containing {needle:?}"),
        Err(e) => {
            assert_eq!(
                e.kind(),
                ErrorKind::InvalidData,
                "expected InvalidData, got {e:?}"
            );
            let msg = e.to_string();
            assert!(
                msg.contains(needle),
                "error must mention {needle:?}; got {msg}"
            );
        }
    }
}

fn expect_round_trip(dir: &Path, rt: &Runtime) {
    let opened = store::open(dir).expect("segmented dir must open");
    assert_eq!(log_ids(&opened), log_ids(rt));
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    opened.replay_check().unwrap();
}

#[test]
fn round_trip_tiny_segmented_dir() {
    let rt = seed_runtime();
    let dir = persist_dir("round");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_sealed_segment_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("miss-sealed");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    fs::remove_file(&path).unwrap();

    assert_invalid_data(&dir, "missing sealed segment");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn missing_active_segment_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("miss-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    fs::remove_file(active_path(&dir)).unwrap();

    assert_invalid_data(&dir, "missing active segment");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn sealed_without_trailer_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("no-trailer");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        !ranges.is_empty(),
        "sealed file must have event frames before the trailer"
    );
    fs::write(&path, &bytes[..ranges[ranges.len() - 1].1]).unwrap();

    assert_invalid_data(&dir, "truncated sealed");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn truncated_sealed_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("trunc-sealed");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    assert!(bytes.len() > 20, "sealed file must be larger than a header");
    bytes.truncate(bytes.len().saturating_sub(12));
    fs::write(&path, &bytes).unwrap();

    assert_invalid_data(&dir, "truncated sealed");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn flipped_byte_in_sealed_frame_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("flip");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 2, "need a payload byte to flip");
    let payload = ranges[1].0 + 8;
    bytes[payload] ^= 0x01;
    fs::write(&path, &bytes).unwrap();

    assert_invalid_data(&dir, "sealed");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn swapped_adjacent_sealed_frames_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("swap");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let path = first_sealed_path(&dir);
    let bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        ranges.len() >= 4,
        "swap two middle frames so first/last still match"
    );
    let mut rebuilt = bytes[..12].to_vec();
    let mut order: Vec<usize> = (0..ranges.len()).collect();
    order.swap(1, 2);
    for i in order {
        rebuilt.extend_from_slice(&bytes[ranges[i].0..ranges[i].1]);
    }
    rebuilt.extend_from_slice(&bytes[ranges[ranges.len() - 1].1..]);
    fs::write(&path, &rebuilt).unwrap();

    assert_invalid_data(&dir, "chain");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn reordered_or_deleted_sealed_frames_is_invalid_data() {
    let rt = seed_runtime();
    let dir = persist_dir("reorder-delete");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);

    let path = first_sealed_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 3);
    let drop_at = ranges[1];
    bytes.drain(drop_at.0..drop_at.1);
    fs::write(&path, &bytes).unwrap();
    assert_invalid_data(&dir, "sealed");

    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    let mut man: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
    let a = man["sealed"][0].clone();
    let b = man["sealed"][1].clone();
    man["sealed"][0] = b;
    man["sealed"][1] = a;
    fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
    assert_invalid_data(&dir, "out of order");

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn manifest_file_disagreement_is_invalid_data() {
    let fields: [&str; 5] = ["first", "last", "count", "bytes", "chain"];
    for field in fields {
        let rt = seed_runtime();
        let dir = persist_dir(&format!("disagree-{field}"));
        let events = persist_encoded(&rt, &dir);
        write_segments(&dir, &events);
        let mut man: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
        match field {
            "first" | "last" => {
                man["sealed"][0][field] = serde_json::json!("00000000-0000-0000-0000-000000000000");
            }
            "count" => man["sealed"][0][field] = serde_json::json!(1),
            "bytes" => man["sealed"][0][field] = serde_json::json!(12),
            "chain" => {
                man["sealed"][0][field] = serde_json::json!(
                    "0000000000000000000000000000000000000000000000000000000000000000"
                );
            }
            _ => unreachable!(),
        }
        fs::write(manifest_path(&dir), serde_json::to_vec(&man).unwrap()).unwrap();
        assert_invalid_data(&dir, "manifest disagrees");
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn torn_active_tail_dropped() {
    let rt = seed_runtime();
    let dir = persist_dir("torn-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    let path = active_path(&dir);
    let mut bytes = fs::read(&path).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(
        !ranges.is_empty(),
        "active must have at least one event frame so a torn tail is extra"
    );
    bytes.extend_from_slice(&[8, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
    fs::write(&path, &bytes).unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn seal_trailer_on_named_active_ends_stream() {
    let rt = seed_runtime();
    let dir = persist_dir("seal-active");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    store::rewrite_segment_with_seal_trailer(&active_path(&dir)).unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn unnamed_segment_leftover_ignored() {
    let rt = seed_runtime();
    let dir = persist_dir("leftover");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    drop_legacy_log(&dir);
    fs::write(dir.join("seg-99999999.log"), b"not a segment leftover").unwrap();

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn manifest_wins_over_disagreeing_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("manifest-wins");
    let events = persist_encoded(&rt, &dir);
    write_segments(&dir, &events);
    smash_legacy_log(&dir);

    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn legacy_dir_still_opens_and_appends_as_before() {
    let mut rt = seed_runtime();
    let dir = persist_dir("legacy");
    store::persist(&rt, &dir).unwrap();
    assert!(
        !manifest_path(&dir).exists(),
        "legacy persist must not write a manifest"
    );
    let opened = store::open(&dir).expect("legacy dir still opens");
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    rt.emit_tt_anchor(T_OLD + 1).unwrap();
    p.append(&rt).unwrap();
    let again = store::open(&dir).unwrap();
    assert_eq!(log_ids(&again), log_ids(&rt));

    let _ = fs::remove_dir_all(&dir);
}
