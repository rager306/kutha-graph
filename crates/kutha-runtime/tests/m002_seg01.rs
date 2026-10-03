//! M002 S06 / SEG-01 oracles (Phase 54). 54-01: format/manifest/open. 54-02: persist + Persister.

use kutha_common::{Event, EventId, Op};
use kutha_runtime::{store, Runtime};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

const T_OLD: u64 = 2015;
const T_NEW: u64 = 2021;
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
    store::encoded_events(rt)
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

fn overlay_segmented(rt: &Runtime, dir: &Path) {
    let events = persist_encoded(rt, dir);
    write_segments(dir, &events);
    drop_legacy_log(dir);
}

fn assert_no_legacy_log(dir: &Path) {
    assert!(
        !dir.join("events.jsonl").exists(),
        "segmented writer must not create events.jsonl"
    );
    assert!(
        !dir.join("events.wal").exists(),
        "segmented writer must not create events.wal"
    );
}

fn persist_expect_segmented(rt: &Runtime, dir: &Path) {
    store::persist(rt, dir).unwrap();
    assert!(
        manifest_path(dir).exists(),
        "empty dir (no manifest, no events.jsonl, no events.wal) must persist as segments"
    );
    assert_no_legacy_log(dir);
}

fn snap_log_offset(dir: &Path) -> usize {
    let snap: kutha_runtime::Snapshot =
        serde_json::from_reader(fs::File::open(dir.join("snapshot.json")).unwrap()).unwrap();
    snap.log_offset
}

fn emit_one_more(rt: &mut Runtime, wall: u64) {
    rt.emit_tt_anchor(wall).unwrap();
}

fn emit_gap_assert(rt: &mut Runtime, subject: &str, object: &str) {
    let s = rt.intern(subject);
    let o = rt.intern(object);
    let r = rt.intern("inForceAs");
    rt.emit(Op::Assert {
        subject: s,
        relation: r,
        object: o,
        valid_from: VF_FIRST,
        valid_to: None,
        claim: None,
        delivery_key: None,
        polarity: None,
    })
    .unwrap();
}

fn emit_product_assert(rt: &mut Runtime, subject: &str, object: &str) {
    emit_gap_assert(rt, subject, object);
}

fn assert_dict_term_parity(src: &Runtime, opened: &Runtime) {
    assert_eq!(
        opened.dictionary().strings(),
        src.dictionary().strings(),
        "open must restore interned strings in intern order"
    );
    for s in src.dictionary().strings() {
        assert_eq!(
            opened.dictionary().id(s),
            src.dictionary().id(s),
            "TermId for {s} must match the source runtime"
        );
    }
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

fn last_event_frame_range(bytes: &[u8]) -> (usize, usize) {
    let ranges = event_frame_ranges(bytes);
    *ranges.last().expect("active must have an event frame")
}

fn replace_last_event_frame(bytes: &[u8], event: &Event) -> Vec<u8> {
    let (start, end) = last_event_frame_range(bytes);
    let frame = store::event_frame_bytes(event).unwrap();
    assert_eq!(
        frame.len(),
        end - start,
        "replacement frame must keep last-record length"
    );
    let mut out = bytes.to_vec();
    out[start..end].copy_from_slice(&frame);
    out
}

fn last_event_from_segment(bytes: &[u8]) -> Event {
    let (start, end) = last_event_frame_range(bytes);
    serde_json::from_slice(&bytes[start + 8..end]).expect("last Event")
}

fn assert_guard_writes_nothing(p: &mut store::Persister, rt: &Runtime, path: &Path, before: &[u8]) {
    match p.append(rt) {
        Ok(_) => panic!("tail-guard mismatch must return InvalidData"),
        Err(e) => assert_eq!(e.kind(), ErrorKind::InvalidData),
    }
    assert_eq!(
        fs::read(path).unwrap(),
        before,
        "tail-guard mismatch must write nothing to the active segment"
    );
    assert_no_legacy_log(path.parent().expect("active parent"));
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
    store::ensure_legacy_dir(&dir).unwrap();
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

#[test]
fn persist_empty_dir_writes_segments_not_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("empty-seg");
    persist_expect_segmented(&rt, &dir);
    expect_round_trip(&dir, &rt);
    assert!(
        store::SEGMENT_MAX_BYTES == 64 * 1024 * 1024 && store::SEGMENT_MAX_EVENTS == 65536,
        "rollover pub consts"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_legacy_dir_keeps_jsonl_and_wal() {
    let rt = seed_runtime();
    let dir = persist_dir("keep-legacy");
    store::ensure_legacy_dir(&dir).unwrap();
    store::persist(&rt, &dir).unwrap();
    assert!(
        !manifest_path(&dir).exists(),
        "legacy dir must not grow a manifest"
    );
    assert!(dir.join("events.jsonl").exists());
    assert!(dir.join("events.wal").exists());
    let from_jsonl = read_jsonl_events(&dir.join("events.jsonl"));
    assert_eq!(from_jsonl.len(), store::encoded_events(&rt).len());
    let opened = store::open(&dir).unwrap();
    assert_eq!(log_ids(&opened), log_ids(&rt));

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn persist_segmented_dir_does_not_create_jsonl() {
    let rt = seed_runtime();
    let dir = persist_dir("rewr-seg");
    overlay_segmented(&rt, &dir);
    store::persist(&rt, &dir).unwrap();
    assert!(manifest_path(&dir).exists());
    assert_no_legacy_log(&dir);
    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn prefix_sealed_bytes_unchanged_after_append() {
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-append");
    overlay_segmented(&rt, &dir);
    let sealed_path = first_sealed_path(&dir);
    let sealed_before = fs::read(&sealed_path).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        fs::read(&sealed_path).unwrap(),
        sealed_before,
        "append must not rewrite sealed bytes"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn prefix_sealed_bytes_unchanged_after_rollover() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-roll");
    overlay_segmented(&rt, &dir);
    let sealed_path = first_sealed_path(&dir);
    let sealed_before = fs::read(&sealed_path).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        fs::read(&sealed_path).unwrap(),
        sealed_before,
        "rollover must not rewrite earlier sealed bytes"
    );
    expect_round_trip(&dir, &rt);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn one_log_sync_per_quantum_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("one-sync");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_log_sync_count().saturating_sub(before);
    assert_eq!(
        delta, 1,
        "non-rollover segmented append must sync_data exactly once; count={delta}"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_creates_no_jsonl() {
    let mut rt = seed_runtime();
    let dir = persist_dir("no-jsonl");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn append_nothing_new_does_no_io_on_segmented() {
    let rt = seed_runtime();
    let dir = persist_dir("noop-seg");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let before = fs::read(&active).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let sync_before = store::persist_log_sync_count();
    p.append(&rt).unwrap();
    assert_eq!(
        store::persist_log_sync_count().saturating_sub(sync_before),
        0
    );
    assert_eq!(fs::read(&active).unwrap(), before);
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_length_mismatch_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-len");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let mut bytes = fs::read(&active).unwrap();
    bytes.extend_from_slice(&[0xFF, 0x00, 0xAA, 0xBB]);
    fs::write(&active, &bytes).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &bytes);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_last_id_mismatch_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-id");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let bytes = fs::read(&active).unwrap();
    let mut last = last_event_from_segment(&bytes);
    last.id = EventId::from_u128(last.id.as_u128().wrapping_add(1));
    let tampered = replace_last_event_frame(&bytes, &last);
    fs::write(&active, &tampered).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &tampered);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn tail_guard_same_length_different_last_record_writes_nothing() {
    let mut rt = seed_runtime();
    let dir = persist_dir("guard-swap");
    overlay_segmented(&rt, &dir);
    let active = active_path(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    let bytes = fs::read(&active).unwrap();
    let ranges = event_frame_ranges(&bytes);
    assert!(ranges.len() >= 2, "need a prior frame to swap");
    let last_len = ranges[ranges.len() - 1].1 - ranges[ranges.len() - 1].0;
    let prior = ranges
        .iter()
        .rev()
        .skip(1)
        .find(|(s, e)| e - s == last_len)
        .copied();
    let tampered = if let Some((s, e)) = prior {
        let mut out = bytes.clone();
        let last = last_event_frame_range(&bytes);
        let frame = bytes[s..e].to_vec();
        out[last.0..last.1].copy_from_slice(&frame);
        out
    } else {
        let mut last = last_event_from_segment(&bytes);
        last.id = EventId::from_u128(last.id.as_u128().wrapping_add(2));
        replace_last_event_frame(&bytes, &last)
    };
    assert_eq!(tampered.len(), bytes.len());
    fs::write(&active, &tampered).unwrap();
    emit_one_more(&mut rt, T_NEW);
    assert_guard_writes_nothing(&mut p, &rt, &active, &tampered);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn rollover_crash_matrix_reopens_to_prefix() {
    let _limits = tiny_limits();
    let steps = [
        store::AppendStep::RolloverTrailer,
        store::AppendStep::RolloverNewHeader,
        store::AppendStep::RolloverManifest,
    ];
    for step in steps {
        let mut rt = seed_runtime();
        let dir = persist_dir(&format!("roll-crash-{step:?}"));
        overlay_segmented(&rt, &dir);
        let committed = log_ids(&rt);
        let active_before = active_path(&dir)
            .file_name()
            .expect("active")
            .to_string_lossy()
            .into_owned();
        let mut p = store::Persister::attach(&rt, &dir).unwrap();
        emit_one_more(&mut rt, T_NEW);
        store::append_with_failpoint(&mut p, &rt, step)
            .expect_err("rollover failpoint must inject");
        match store::open(&dir) {
            Ok(opened) => {
                assert_eq!(
                    log_ids(&opened),
                    committed,
                    "crash at {step:?} must reopen to the committed prefix"
                );
                opened.replay_check().unwrap();
            }
            Err(e) => assert_eq!(
                e.kind(),
                ErrorKind::InvalidData,
                "crash at {step:?} may fail closed; {e}"
            ),
        }
        if step == store::AppendStep::RolloverNewHeader {
            let man: serde_json::Value =
                serde_json::from_str(&fs::read_to_string(manifest_path(&dir)).unwrap()).unwrap();
            assert_eq!(
                man["active"].as_str(),
                Some(active_before.as_str()),
                "manifest must still name the old active after RolloverNewHeader"
            );
        }
        let _ = fs::remove_dir_all(&dir);
    }
}

#[test]
fn rollover_manifest_flip_fsyncs_dir() {
    let _limits = tiny_limits();
    let mut rt = seed_runtime();
    let dir = persist_dir("roll-fsync");
    overlay_segmented(&rt, &dir);
    let old_active = active_path(&dir);
    let old_name = old_active
        .file_name()
        .expect("active name")
        .to_string_lossy()
        .into_owned();
    let old_len = fs::metadata(&old_active).unwrap().len();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    let before = store::persist_dir_fsync_count();
    p.append(&rt).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    assert!(delta >= 1, "manifest flip must dir-fsync; count={delta}");
    let newly = dir.join(&old_name);
    let sealed = fs::read(&newly).unwrap();
    assert!(
        sealed.len() as u64 > old_len,
        "rollover must append a seal trailer without rewriting the prefix"
    );
    let ranges = event_frame_ranges(&sealed);
    let trailer_off = ranges.last().map(|r| r.1).unwrap_or(12);
    assert!(
        trailer_off + 8 < sealed.len() && is_seal_payload(&sealed[trailer_off + 8..]),
        "rollover must write a seal trailer on the former active"
    );
    assert_ne!(
        active_path(&dir)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned()),
        Some(old_name),
        "manifest active must advance"
    );

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn checkpoint_dir_fsync_count_exact_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("ckpt-fsync");
    overlay_segmented(&rt, &dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let before = store::persist_dir_fsync_count();
    p.checkpoint(&rt).unwrap();
    let delta = store::persist_dir_fsync_count().saturating_sub(before);
    assert_eq!(
        delta, 5,
        "checkpoint dir fsync: snapshot+terms+outcomes+justifications+final; count={delta}"
    );
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn open_for_append_heals_and_appends_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("heal-seg");
    overlay_segmented(&rt, &dir);
    let committed_fp = rt.fold().fingerprint();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_product_assert(&mut rt, "p54-heal-article", "p54-heal-80");
    crash_omit_last(&mut p, &rt);

    let (mut healed, mut p) = store::open_for_append(&dir)
        .expect("open_for_append must persist the committed prefix then attach");
    assert_eq!(healed.fold().fingerprint(), committed_fp);
    emit_product_assert(&mut healed, "p54-heal2-article", "p54-heal2-90");
    p.append(&healed).unwrap();
    let again = store::open(&dir).unwrap();
    assert_eq!(again.fold().fingerprint(), healed.fold().fingerprint());
    again.replay_check().unwrap();
    assert_no_legacy_log(&dir);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn stale_leases_between_checkpoints_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("stale-seg");
    overlay_segmented(&rt, &dir);
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "small append must not rewrite snapshot.json"
    );
    let opened = store::open(&dir).expect("stale snapshot + log ahead");
    assert_eq!(log_ids(&opened), log_ids(&rt));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn interned_terms_between_appends_survive_on_segmented() {
    let mut rt = seed_runtime();
    let dir = persist_dir("intern-seg");
    overlay_segmented(&rt, &dir);
    let offset_a = snap_log_offset(&dir);
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    emit_gap_assert(&mut rt, "p54-gap-article", "p54-gap-80");
    p.append(&rt).unwrap();
    assert_eq!(
        snap_log_offset(&dir),
        offset_a,
        "GAP A: intern-append must not checkpoint"
    );
    let opened = store::open(&dir).expect("open after intern-append without checkpoint");
    assert_dict_term_parity(&rt, &opened);
    assert_eq!(opened.fold().fingerprint(), rt.fold().fingerprint());
    assert_eq!(opened.as_of(VF_FIRST), rt.as_of(VF_FIRST));
    opened.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn second_and_third_appends_keep_earlier_bytes_as_prefix() {
    let mut rt = seed_runtime();
    let dir = persist_dir("prefix-3");
    overlay_segmented(&rt, &dir);
    let sealed0 = fs::read(first_sealed_path(&dir)).unwrap();
    let active = active_path(&dir);
    let a0 = fs::read(&active).unwrap();
    let mut p = store::Persister::attach(&rt, &dir).unwrap();
    emit_one_more(&mut rt, T_NEW);
    p.append(&rt).unwrap();
    let a1 = fs::read(&active).unwrap();
    assert!(a1.starts_with(&a0) && a1.len() > a0.len());
    emit_one_more(&mut rt, T_NEW + 1);
    p.append(&rt).unwrap();
    let a2 = fs::read(&active).unwrap();
    assert!(a2.starts_with(&a1) && a2.len() > a1.len());
    p.append(&rt).unwrap();
    assert_eq!(fs::read(&active).unwrap(), a2);
    emit_one_more(&mut rt, T_NEW + 5);
    p.append(&rt).unwrap();
    let a3 = fs::read(&active).unwrap();
    assert!(a3.starts_with(&a2) && a3.len() > a2.len());
    assert_eq!(fs::read(first_sealed_path(&dir)).unwrap(), sealed0);

    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn segmented_replay_parity_with_legacy_full_persist() {
    let rt = seed_runtime();
    let dir_seg = persist_dir("parity-seg");
    persist_expect_segmented(&rt, &dir_seg);
    let opened_s = store::open(&dir_seg).unwrap();

    let dir_leg = persist_dir("parity-leg");
    store::ensure_legacy_dir(&dir_leg).unwrap();
    store::persist(&rt, &dir_leg).unwrap();
    let opened_l = store::open(&dir_leg).unwrap();

    assert_eq!(log_ids(&opened_s), log_ids(&opened_l));
    assert_dict_term_parity(&opened_l, &opened_s);
    assert_eq!(opened_s.fold().fingerprint(), opened_l.fold().fingerprint());
    assert_eq!(opened_s.as_of(VF_FIRST), opened_l.as_of(VF_FIRST));
    opened_s.replay_check().unwrap();
    opened_l.replay_check().unwrap();

    let _ = fs::remove_dir_all(&dir_seg);
    let _ = fs::remove_dir_all(&dir_leg);
}
